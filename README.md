<p align="center">
  <img src="assets/banner.svg" alt="snaptap" width="100%">
</p>

<p align="center">
  <a href="https://github.com/attent10nc/snaptap/releases/latest"><img src="https://img.shields.io/github/v/release/attent10nc/snaptap?style=for-the-badge&color=7c5cff&label=release" alt="release"></a>
  <a href="https://github.com/attent10nc/snaptap/releases"><img src="https://img.shields.io/github/downloads/attent10nc/snaptap/total?style=for-the-badge&color=00b4d8" alt="downloads"></a>
  <a href="https://www.virustotal.com/gui/file/9fd3822f48b5a225b3c1ab2ecbf3c67538534f691520a362ddb8f95e1c30080e"><img src="https://img.shields.io/badge/VirusTotal-0%2F71%20clean-2ea043?style=for-the-badge&logo=virustotal&logoColor=white" alt="VirusTotal"></a>
  <a href="https://github.com/attent10nc/snaptap/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/attent10nc/snaptap/ci.yml?style=for-the-badge&label=build" alt="build"></a>
  <img src="https://img.shields.io/badge/Windows-10%20%7C%2011-0078d4?style=for-the-badge&logo=windows&logoColor=white" alt="windows">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-555?style=for-the-badge" alt="MIT"></a>
</p>

<p align="center">
  <b>Snap Tap / SOCD for any keyboard.</b> When two opposite keys are held, the last one pressed wins.<br>
  One small open-source exe. No install, no drivers, no config.
</p>

<p align="center">
  <a href="https://github.com/attent10nc/snaptap/releases/latest/download/snaptap.exe"><b>⬇ Download snaptap.exe</b></a>
  &nbsp;·&nbsp; <a href="#-русский">Русский</a>
</p>

---

## ✨ What it does

<p align="center"><img src="assets/how-it-works.svg" alt="how it works" width="90%"></p>

| You do | The game gets |
|---|---|
| hold **A**, press **D** | A released, D pressed |
| release **D** while still holding **A** | A pressed again |
| hold **D**, press **A**, release **D** | nothing changes, A stays |

- 🎯 Pairs: **A / D** and **W / S**
- ⚡ Low-level keyboard hook in Rust, reacts within the same keystroke
- 🔁 Auto-repeat of an overridden key is swallowed; every other key passes through untouched
- 🪶 &lt;200 KB, portable, no admin rights needed

## 🚀 Usage

1. Download [`snaptap.exe`](https://github.com/attent10nc/snaptap/releases/latest/download/snaptap.exe).
2. Run it and switch to your game. The console shows `[ON]` / `[OFF]` as the game gains or loses focus.

```text
snaptap.exe              active while cs2.exe is focused (default)
snaptap.exe game.exe     active while game.exe is focused
snaptap.exe --global     active in every window
```

Close the window or press <kbd>Ctrl</kbd>+<kbd>C</kbd> to quit.

> [!WARNING]
> Since August 2024 Valve kicks players using SOCD / Snap Tap in matches on **official CS2 servers**.
> Check the rules of the servers and leagues you play on. Use at your own risk.

## 🛡️ Is it safe?

| | |
|---|---|
| **VirusTotal** | **0 / 71** engines detect anything — [full report](https://www.virustotal.com/gui/file/9fd3822f48b5a225b3c1ab2ecbf3c67538534f691520a362ddb8f95e1c30080e) |
| **SHA-256 (v1.0.0)** | `9fd3822f48b5a225b3c1ab2ecbf3c67538534f691520a362ddb8f95e1c30080e` |
| **Built by** | [GitHub Actions](https://github.com/attent10nc/snaptap/actions/workflows/release.yml) straight from this source, see the release log |
| **Code** | one file, [`src/main.rs`](src/main.rs), ~200 lines |
| **Network / files / registry** | none: it only reads keys and sends keys |

Check your download yourself:

```powershell
Get-FileHash .\snaptap.exe -Algorithm SHA256
```

The hash must match `SHA256SUMS.txt` from the release.

<p align="center">
  <a href="https://www.virustotal.com/gui/file/9fd3822f48b5a225b3c1ab2ecbf3c67538534f691520a362ddb8f95e1c30080e"><img src="assets/virustotal.png" alt="VirusTotal: 0/71" width="90%"></a>
  <br><sub>VirusTotal scan of the release build, click for the live report</sub>
</p>

<details>
<summary><b>Why can an antivirus be suspicious of a keyboard tool at all?</b></summary>

Any program that installs a global keyboard hook (`SetWindowsHookEx`) and sends keystrokes (`SendInput`)
looks similar to a keylogger for heuristic engines, especially when it is new and unsigned. snaptap never stores,
logs or sends keystrokes anywhere: read `src/main.rs` or build it yourself.
</details>

## ⚙️ How it works

A `WH_KEYBOARD_LL` hook tracks which keys are physically held. When the opposite key of a held key goes
down, the original event is blocked and `SendInput` emits `<held key up>, <new key down>` by scancode.
Releasing the newer key re-presses the older one if it is still held. Injected events carry a marker in
`dwExtraInfo` so the hook skips its own input.

## 🔨 Build from source

```bash
git clone https://github.com/attent10nc/snaptap
cd snaptap
cargo test            # SOCD logic tests
cargo build --release # target/release/snaptap.exe
```

Releases are built by [`.github/workflows/release.yml`](.github/workflows/release.yml) when a `v*` tag is pushed.

---

## 🇷🇺 Русский

**snaptap** — Snap Tap (SOCD) для любой клавиатуры: из двух противоположных клавиш (A/D, W/S) работает
последняя нажатая. Зажал A, нажал D, и игра видит только D. Отпустил D, и снова работает A. Нет момента,
когда обе клавиши гасят друг друга и персонаж останавливается.

**Запуск:** скачай [`snaptap.exe`](https://github.com/attent10nc/snaptap/releases/latest/download/snaptap.exe)
и запусти.
- `snaptap.exe` — работает только когда в фокусе CS2;
- `snaptap.exe game.exe` — в другой игре;
- `snaptap.exe --global` — везде.

**Безопасность:** [отчёт VirusTotal](https://www.virustotal.com/gui/file/9fd3822f48b5a225b3c1ab2ecbf3c67538534f691520a362ddb8f95e1c30080e), открытый код (один файл), exe собирает GitHub Actions.
Программа не пишет нажатия в файлы и не выходит в сеть.

⚠️ В матчах на официальных серверах Valve за SOCD кикают. Правила других серверов проверяй сам.

## 📄 License

[MIT](LICENSE)
