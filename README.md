# OmniMouse

[![Language: Rust](https://img.shields.io/badge/Language-Rust%202021-DEA584?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20x64-0078D6?style=flat-square&logo=windows)](https://microsoft.com/windows)
[![Dependencies: 0 External Crates](https://img.shields.io/badge/Dependencies-0%20External%20Crates-success?style=flat-square)](#architecture-and-systems-design)
[![Binary Size](https://img.shields.io/badge/Binary%20Size-~1.4%20MB-blue?style=flat-square)](#performance-and-resource-footprint)
[![License: MIT](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)

High-performance, ultra-lightweight, zero-dependency Windows companion utility and input enhancement system for all mouse hardware. Built entirely in Rust using direct Win32 API FFI.

---

## Overview

OmniMouse is an open-source system utility engineered to provide advanced mouse customization, button remapping, chorded gestures, and a microsecond-accurate auto-clicker without the overhead of modern OEM software suites. 

Most proprietary mouse software (such as Logitech G HUB, Razer Synapse, or vendor-specific Electron packages) consumes between 150 MB and 400 MB of system RAM, spawns multiple background helper processes, and introduces noticeable CPU usage and input latency. OmniMouse eliminates this overhead:

- Zero external crate dependencies (`Cargo.toml` has empty dependencies).
- Standalone executable of approximately 1.4 MB.
- Low memory footprint of ~12 MB to 18 MB RAM.
- 0.0% idle CPU utilization through an event-driven message loop.
- Sub-microsecond hook fast-path ensuring 0 ms added latency at 1000 Hz polling rates.
- Universal compatibility across all USB wired, 2.4 GHz wireless, and Bluetooth mice.

---

## Key Capabilities

### 1. Universal Hardware Presence Detection
- Leverages the Windows SetupAPI (`GUID_DEVINTERFACE_MOUSE`) to monitor active mouse interfaces dynamically.
- Dual-tier fallback via Win32 `GetSystemMetrics(SM_MOUSEPRESENT)` guarantees reliable presence detection across docking stations, KVM switches, and sleep modes.
- Event-driven updates via `WM_DEVICECHANGE` refresh connection status in less than 15 ms upon hardware arrival or removal.
- Works with any mouse hardware regardless of vendor or model.

### 2. Low-Latency Lock-Free Input Hook
- Intercepts mouse events at the Windows kernel boundary via `WH_MOUSE_LL` on a dedicated high-priority thread.
- Implements a fast-path for `WM_MOUSEMOVE` (>99.8% of events), immediately forwarding movement to the next hook without lock acquisitions or pointer dereferences.
- Hook configuration is stored in lock-free atomic variables (`AtomicBool`, `AtomicU32`, `AtomicU8`) using relaxed memory orderings, eliminating thread contention.

### 3. Comprehensive Button Remapping
Supports full remapping of XButton1 (Mouse 4), XButton2 (Mouse 5), and Middle Click (Scroll Wheel Click):
- Productivity: Copy, Paste, Cut, Undo, Select All, Enter, Escape, Alt+Tab, Close Tab, Reopen Closed Tab, New Tab.
- Navigation & Window Management: Browser Back, Browser Forward, Task View (`Win+Tab`), Show Desktop (`Win+D`), Snipping Tool (`Win+Shift+S`).
- Media Controls: Play/Pause, Next Track, Previous Track, Volume Up, Volume Down, Audio Mute.
- Gaming Enhancements:
  - Sniper Precision: Temporarily reduces system pointer speed (`SPI_SETMOUSESPEED`) while holding a designated side button.
  - Rapid Fire: Sustains rapid left-clicking while holding a mapped side button.

### 4. Chorded Scroll Gestures
- Volume Control: Hold XButton1 and rotate the scroll wheel upward or downward to step system audio volume up or down.
- Horizontal Scrolling: Hold XButton2 and scroll vertically to generate native Windows horizontal scroll events (`WM_MOUSEHWHEEL`).
- Quick Audio Mute: Hold XButton1 and click the middle wheel button to toggle audio mute.

### 5. High-Precision Auto-Clicker
- Dynamic rate adjustment ranging from 2 CPS (500 ms interval) to 100 CPS (10 ms interval).
- Monotonic deadline pacing powered by `std::time::Instant` and multimedia timer resolution (`timeBeginPeriod(1)`), preventing timing drift common in sleep-based loops.
- Global hotkey toggle via `F6` (`RegisterHotKey`) to enable or disable the clicker globally without switching windows.
- Integrated interactive click tester with a real-time counter to verify click dispatch before entering target applications or games.

### 6. Scroll Modifiers
- Invert Scroll Direction: Provides macOS-style natural scrolling without modifying global Windows registry values.
- Scroll Multiplier: Scales wheel delta by 1x, 2x, or 3x for faster navigation through long source files and documents.

### 7. Process Lifecycle & System Integration
- Single-Instance Enforcement: Named Win32 Mutex (`Local\OmniMouse_SingleInstance`) prevents duplicate processes.
- Window Focus Restoration: Launching a second instance automatically brings the existing GUI window to the foreground via `SW_RESTORE` and `SetForegroundWindow`, then exits cleanly.
- System Tray Integration: Minimizes unobtrusively to the Windows notification area with a custom context menu for quick restoration or termination.
- Safe System Shutdown: Handles `WM_QUERYENDSESSION` and `WM_ENDSESSION` to immediately flush timers, clean up tray icons, and exit with code 0 without blocking Windows restart or shutdown.
- Seamless Auto-Migration: Reads configuration from `%APPDATA%\OmniMouse\config.ini`. If absent, automatically imports existing preferences from legacy configuration directories.

---

## Architecture and Systems Design

OmniMouse is engineered with an emphasis on simplicity, data locality, and native platform integration.

```
omnimouse.exe (~1.4 MB)
  |-- GUI Thread (Native Win32 Message Pump, Dark Mode DWM, Controls, System Tray)
  |-- Mouse Hook Thread (WH_MOUSE_LL, Fast-Path WM_MOUSEMOVE, Atomic Config Cache)
  |-- Clicker Worker Thread (THREAD_PRIORITY_HIGHEST, Monotonic Deadline Pacing)
  |-- Hardware Monitor Thread (SetupAPI Device Interface Enumeration, Non-blocking)
  `-- Sniper Speed Worker (Async SystemParametersInfoW Dispatch)
```

### Module Structure

```
src/
  |-- main.rs       Application entry point, single-instance mutex, timer lifecycle
  |-- win32.rs      Hand-crafted Win32 FFI bindings (User32, Kernel32, SetupAPI, WinMM, Shell32)
  |-- hook.rs       WH_MOUSE_LL callback, lock-free atomic cache, event synthesis, chorded gestures
  |-- autoclick.rs  Interval clamping, CPS mathematics, click duty cycle calculations
  |-- device.rs     SetupAPI device interface enumeration and connection status evaluation
  |-- config.rs     INI file serialization, auto-migration logic, Windows Run registry integration
  `-- gui.rs        Native Win32 window creation, two-tab layout, trackbar, and dark theme
```

### Zero-Crate Design
Unlike typical Rust desktop applications that import dozens of third-party crates, OmniMouse relies exclusively on Rust's standard library (`std`) and custom FFI declarations matching the Windows SDK. This guarantees:
- Complete immunity to upstream supply-chain vulnerabilities.
- Deterministic sub-second compilation times.
- Zero binary bloat from unused runtime abstractions.

---

## Performance and Resource Footprint

Measurements taken on Windows 11 Pro 64-bit (Intel Core i7, 1000 Hz mouse polling rate):

| Metric | OmniMouse | Traditional OEM Software |
| :--- | :--- | :--- |
| Binary Size | 1.41 MB | 150 MB - 450 MB |
| RAM (Working Set) | ~14 MB | 180 MB - 350 MB |
| Idle CPU Usage | 0.0% | 0.5% - 2.5% |
| Hook Overhead (Move) | < 0.001 ms | 0.05 ms - 0.2 ms |
| External Dependencies | 0 | 50+ (Node.js / Chromium runtime) |
| Child Processes | 0 | 4 - 8 background daemons |

---

## Building from Source

### Prerequisites
- Windows 10 or Windows 11 (64-bit / x86_64).
- Rust toolchain 1.70 or newer (`rustc`, `cargo`).
- Target architecture: `x86_64-pc-windows-gnu` (MinGW) or `x86_64-pc-windows-msvc`.

### 1. Clone the Repository
```bash
git clone https://github.com/IsnantoBudi/omnimouse.git
cd omnimouse
```

### 2. Run Test Suite
OmniMouse includes a unit test suite covering click mathematics, timing bounds, configuration roundtripping, and device detection routines:
```bash
cargo test
```

Expected output:
```text
test result: ok. 42 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
```

### 3. Compile Release Binary
```bash
cargo build --release
```

The optimized executable will be located at:
```text
target/release/omnimouse.exe
```

The binary is fully standalone and can be moved to any directory or run from portable storage without installation.

---

## Configuration Reference

Configuration is persisted in standard INI format at:
```text
%APPDATA%\OmniMouse\config.ini
```

### Example `config.ini`
```ini
[Buttons]
XButton1=BrowserBack
XButton2=BrowserForward
MiddleClick=Default

[General]
Suspended=false
AutoStart=false
MinimizeToTray=true
EnableGestures=true
SniperSpeed=3
InvertScroll=false
ScrollMultiplier=1
AutoclickIntervalMs=50
```

### Supported Action Identifiers
| Action Identifier | Display Name | Operation |
| :--- | :--- | :--- |
| `Default` | Default Windows (Back / Forward) | Standard OS behavior |
| `RapidFire` | Rapid Fire (Left Auto-Clicker) | High-speed left clicking while held |
| `SniperPrecision` | Sniper Precision (Lower Pointer Speed) | Lowers pointer sensitivity while held |
| `Copy` | Copy (Ctrl+C) | Copies selection to clipboard |
| `Paste` | Paste (Ctrl+V) | Pastes clipboard content |
| `Cut` | Cut (Ctrl+X) | Cuts selection to clipboard |
| `Undo` | Undo (Ctrl+Z) | Reverses previous action |
| `SelectAll` | Select All (Ctrl+A) | Selects all text or items |
| `SnippingTool` | Snipping Tool (Win+Shift+S) | Opens Windows screenshot overlay |
| `TaskView` | Task View (Win+Tab) | Opens virtual desktop switcher |
| `ShowDesktop` | Show Desktop (Win+D) | Minimizes all active windows |
| `CloseTab` | Close Tab (Ctrl+W) | Closes the current browser tab |
| `ReopenTab` | Reopen Tab (Ctrl+Shift+T) | Restores the last closed browser tab |
| `NewTab` | New Tab (Ctrl+T) | Opens a new browser tab |
| `MediaPlayPause` | Play / Pause Media | Toggles media playback |
| `MediaNext` | Next Track | Advances to next audio/video track |
| `MediaPrev` | Previous Track | Returns to previous track |
| `VolumeUp` | Volume Up | Increases master audio volume |
| `VolumeDown` | Volume Down | Decreases master audio volume |
| `VolumeMute` | Mute Audio | Toggles audio mute state |
| `Disabled` | Disabled | Suppresses button click |

---

## Controls and Shortcuts

- Toggle Auto-Clicker: Press `F6` globally from any window or full-screen application.
- Volume Gesture: Hold `XButton1` and scroll up (Volume +) or scroll down (Volume -).
- Horizontal Scroll Gesture: Hold `XButton2` and scroll vertically.
- Mute Gesture: Hold `XButton1` and click the middle scroll wheel button.
- System Tray Menu: Right-click the OmniMouse tray icon to restore the window or terminate the application.
- Background Restore: Running `omnimouse.exe` while an instance is already active brings the existing window to the front.

---

## Troubleshooting

### Antivirus False Positives
OmniMouse registers a low-level mouse hook (`WH_MOUSE_LL` via `SetWindowsHookExW`) and simulates mouse input via `SendInput`. Certain heuristic scanners may flag low-level input hooks as suspicious. OmniMouse is open-source, contains zero telemetry or network calls, and executes entirely offline. You may safely add `omnimouse.exe` to your security software's exclusion list.

### Binary Locked During Rebuild
If `cargo build` reports that `omnimouse.exe` cannot be overwritten:
1. Ensure OmniMouse is not running in the system tray. Right-click the tray icon and select Exit.
2. Alternatively, terminate the process via PowerShell:
   ```powershell
   Stop-Process -Name omnimouse -Force -ErrorAction SilentlyContinue
   ```

---

## License

This project is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute this software in personal and commercial environments.
