# snaptap

Tiny Windows **Snap Tap / SOCD** tool: when two opposite keys are held, the last one pressed wins.
Works with any keyboard. ~150 KB, no install, no config.

| You do | The game gets |
|---|---|
| hold **A**, press **D** | A released, D pressed |
| release **D** while still holding **A** | A pressed again |
| hold **D**, press **A**, release **D** | nothing changes, A stays |

Pairs: **A/D** and **W/S**. Auto-repeat of an overridden key is swallowed, every other key passes through.

## Usage

Download `snaptap.exe` from [Releases](../../releases) and run it.

```
snaptap.exe              active while cs2.exe is focused (default)
snaptap.exe game.exe     active while game.exe is focused
snaptap.exe --global     active in every window
```

The console prints `[ON]` / `[OFF]` as focus changes. Ctrl+C quits.

> Valve kicks players using SOCD/Snap Tap in matches on official CS2 servers (since Aug 2024).
> Check the rules of the servers or leagues you play on.

## How it works

A low-level keyboard hook (`WH_KEYBOARD_LL`) tracks which keys are physically down. When the opposite
key of a held key is pressed, the original event is blocked and `SendInput` sends
`<held key up>, <new key down>` by scancode. Injected events are tagged via `dwExtraInfo`, so the hook ignores its own input.

## Build

```
cargo build --release      # target/release/snaptap.exe
cargo test                 # SOCD logic tests
```

Pushing a `v*` tag builds the exe and attaches it to a GitHub Release (`.github/workflows/release.yml`).

---

### RU

Snap Tap для любой клавиатуры: из двух противоположных клавиш (A/D, W/S) работает последняя нажатая.
Запуск: `snaptap.exe` (только в CS2), `snaptap.exe game.exe` (другая игра), `snaptap.exe --global` (везде).
В матчах на официальных серверах Valve за SOCD кикают.

## License

MIT
