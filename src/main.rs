//! Snap Tap (SOCD "last input wins") for Windows: A/D and W/S never cancel each other out.
use std::cell::RefCell;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, time::Duration};

use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::Threading::*;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

const MAGIC: usize = 0x534E_4150; // marks our own injected events so the hook skips them
const PAIRS: [(u32, u32); 2] = [(0x41, 0x44), (0x57, 0x53)]; // A/D, W/S

static ACTIVE: AtomicBool = AtomicBool::new(false);

#[derive(Debug, PartialEq)]
enum Out {
    Pass,
    Block,
    Send(Vec<(u32, bool)>), // (vk, down), the original event is blocked
}

struct Socd {
    phys: [bool; 256],
    supp: [bool; 256], // physically held but released in-game by its opposite
}

impl Default for Socd {
    fn default() -> Self {
        Socd { phys: [false; 256], supp: [false; 256] }
    }
}

fn opposite(vk: u32) -> Option<u32> {
    PAIRS.iter().find_map(|&(a, b)| if vk == a { Some(b) } else if vk == b { Some(a) } else { None })
}

impl Socd {
    fn on_key(&mut self, vk: u32, down: bool) -> Out {
        let Some(opp) = opposite(vk) else { return Out::Pass };
        let (v, o) = (vk as usize, opp as usize);
        if down {
            if self.phys[v] {
                // auto-repeat: keep a suppressed key silent
                return if self.supp[v] { Out::Block } else { Out::Pass };
            }
            self.phys[v] = true;
            if self.phys[o] && !self.supp[o] {
                self.supp[o] = true;
                return Out::Send(vec![(opp, false), (vk, true)]);
            }
            Out::Pass
        } else {
            self.phys[v] = false;
            if std::mem::take(&mut self.supp[v]) {
                return Out::Block; // already up in-game
            }
            if self.phys[o] && self.supp[o] {
                self.supp[o] = false;
                return Out::Send(vec![(vk, false), (opp, true)]);
            }
            Out::Pass
        }
    }
}

thread_local!(static STATE: RefCell<Socd> = RefCell::new(Socd::default()));

fn send(keys: &[(u32, bool)]) {
    let inputs: Vec<INPUT> = keys
        .iter()
        .map(|&(vk, down)| INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: 0,
                    wScan: unsafe { MapVirtualKeyW(vk, MAPVK_VK_TO_VSC) } as u16,
                    dwFlags: KEYEVENTF_SCANCODE | if down { 0 } else { KEYEVENTF_KEYUP },
                    time: 0,
                    dwExtraInfo: MAGIC,
                },
            },
        })
        .collect();
    unsafe { SendInput(inputs.len() as u32, inputs.as_ptr(), std::mem::size_of::<INPUT>() as i32) };
}

unsafe extern "system" fn hook(code: i32, w: WPARAM, l: LPARAM) -> LRESULT {
    let info = &*(l as *const KBDLLHOOKSTRUCT);
    if code == HC_ACTION as i32 && info.dwExtraInfo != MAGIC {
        let down = w == WM_KEYDOWN as usize || w == WM_SYSKEYDOWN as usize;
        let out = STATE.with(|s| {
            let mut s = s.borrow_mut();
            if ACTIVE.load(Ordering::Relaxed) {
                s.on_key(info.vkCode, down)
            } else {
                *s = Socd::default();
                Out::Pass
            }
        });
        match out {
            Out::Pass => {}
            Out::Block => return 1,
            Out::Send(k) => {
                send(&k);
                return 1;
            }
        }
    }
    CallNextHookEx(null_mut(), code, w, l)
}

/// Executable name of the foreground window's process, lowercase ("cs2.exe").
fn foreground_exe() -> String {
    unsafe {
        let mut pid = 0u32;
        GetWindowThreadProcessId(GetForegroundWindow(), &mut pid);
        let h = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if h.is_null() {
            return String::new();
        }
        let mut name = [0u16; 520];
        let mut len = name.len() as u32;
        let ok = QueryFullProcessImageNameW(h, 0, name.as_mut_ptr(), &mut len) != 0;
        CloseHandle(h);
        if !ok {
            return String::new();
        }
        let path = String::from_utf16_lossy(&name[..len as usize]).to_lowercase();
        path.rsplit('\\').next().unwrap_or_default().to_string()
    }
}

fn main() {
    // snaptap.exe            -> active while cs2.exe is focused
    // snaptap.exe game.exe   -> active while that exe is focused
    // snaptap.exe --global   -> active everywhere
    let target = std::env::args().nth(1).unwrap_or_else(|| "cs2.exe".into()).to_lowercase();
    let global = target == "--global";
    println!("Snap Tap: A/D and W/S, last pressed wins. Ctrl+C to quit.");
    println!("{}
", if global { "Active in every window.".to_string() } else { format!("Active while {target} is focused.") });
    thread::spawn(move || {
        let mut last = None;
        loop {
            let on = global || foreground_exe() == target;
            ACTIVE.store(on, Ordering::Relaxed);
            if last != Some(on) {
                println!("{}", if on { "[ON]" } else { "[OFF]" });
                last = Some(on);
            }
            thread::sleep(Duration::from_millis(250));
        }
    });
    unsafe {
        if SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook), null_mut(), 0).is_null() {
            eprintln!("SetWindowsHookExW failed");
            return;
        }
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const A: u32 = 0x41;
    const D: u32 = 0x44;

    #[test]
    fn last_input_wins_and_restores() {
        let mut s = Socd::default();
        assert_eq!(s.on_key(A, true), Out::Pass);
        assert_eq!(s.on_key(D, true), Out::Send(vec![(A, false), (D, true)]));
        assert_eq!(s.on_key(A, true), Out::Block); // auto-repeat of suppressed A
        assert_eq!(s.on_key(D, false), Out::Send(vec![(D, false), (A, true)]));
        assert_eq!(s.on_key(A, false), Out::Pass);
        assert_eq!(s.on_key(0x20, true), Out::Pass); // unrelated key
    }

    #[test]
    fn release_of_suppressed_key_is_silent() {
        let mut s = Socd::default();
        s.on_key(A, true);
        s.on_key(D, true);
        assert_eq!(s.on_key(A, false), Out::Block);
        assert_eq!(s.on_key(D, false), Out::Pass);
        assert_eq!(s.on_key(A, true), Out::Pass);
    }

    #[test]
    fn repress_of_suppressed_key_takes_over() {
        let mut s = Socd::default();
        s.on_key(A, true);
        s.on_key(D, true); // A suppressed
        s.on_key(A, false); // silent
        assert_eq!(s.on_key(A, true), Out::Send(vec![(D, false), (A, true)]));
    }
}
