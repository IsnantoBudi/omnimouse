# Project: Riot MQ-10 Performance & Native Win32 Optimization

## Architecture
- **Language / Runtime**: Rust (x86_64-pc-windows-gnu) with pure Win32 raw FFI and zero external dependencies (`Cargo.toml` dependencies empty).
- **Subsystems**:
  1. `src/win32.rs`: Raw Win32 FFI declarations (`user32.dll`, `kernel32.dll`, `setupapi.dll`, `winmm.dll`).
  2. `src/device.rs`: Device connection monitoring via SetupAPI (`SetupDiGetClassDevsW`, `SetupDiEnumDeviceInfo`, `SetupDiGetDeviceInstanceIdW`, `SetupDiGetDeviceRegistryPropertyW`).
  3. `src/hook.rs`: Low-level mouse hook (`WH_MOUSE_LL`), lock-free atomic configuration cache, message pump, and rapid-fire auto-clicker.
  4. `src/autoclick.rs`: Timing calculations, CPS math, and interval clamping.
  5. `src/gui.rs`: Native Win32 GUI window, message loop, tab views, event handling (`WM_TIMER`, `WM_DEVICECHANGE`, `WM_COMMAND`), and control bindings.
  6. `src/config.rs`: INI file configuration parser and persistence.
  7. `src/main.rs`: Application entry point, single-instance mutex check, timer resolution management (`timeBeginPeriod(1)`), and subsystem lifecycle initialization.

## Feature Inventory
| # | Feature | Description | Milestone | Source |
|---|---------|-------------|-----------|--------|
| 1 | Native Win32 SetupAPI Enumeration | Enumerate HID and Bluetooth devices using SetupAPI FFI with zero `powershell.exe` execution | M1 | Survey Device Explorer |
| 2 | Riot MQ-10 USB 2.4GHz Dongle Matching | Identify USB dongle via `VID_3151&PID_3020` in InstanceId | M1 | Survey Device Explorer |
| 3 | Riot MQ-10 Bluetooth Mode Matching | Identify Bluetooth mode via FriendlyName `NEMESIS MQ` / `RIOT` under `BTHENUM` | M1 | Survey Device Explorer |
| 4 | Event-Driven Hardware Update | Hook `WM_DEVICECHANGE` (`0x0219`) in GUI to refresh status immediately (<15ms) on plug/unplug | M1 | Survey Device Explorer |
| 5 | Lock-Free Mouse Hook Callback | Replace `GLOBAL_CONFIG: Arc<RwLock<AppConfig>>` with atomic state variables in `hook.rs` | M2 | Survey Hook Explorer |
| 6 | Fast-path for `WM_MOUSEMOVE` | Return immediately on `WM_MOUSEMOVE` to prevent hook contention at 1000Hz | M2 | Survey Hook Explorer |
| 7 | High-Precision Multimedia Timer | Initialize `timeBeginPeriod(1)` at startup and `timeEndPeriod(1)` on shutdown | M2 | Survey Hook Explorer |
| 8 | Monotonic Deadline-Paced Clicker | Replace dual relative `thread::sleep` with monotonic `Instant` deadline pacing | M2 | Survey Hook Explorer |
| 9 | Non-Blocking Event Reporting & Async DPI | Use `try_lock` for `LAST_EVENT` and offload `SystemParametersInfoW` to async worker | M2 | Survey Hook Explorer |
| 10 | Hook Message Pump Fix | Fix message pump condition from `!= 0` to `> 0` | M2 | Survey Hook Explorer |
| 11 | Eliminate GUI Poller Thread Data Race | Replace background thread in `gui.rs` with Win32 `SetTimer` / `WM_TIMER` | M3 | Survey Arch Explorer |
| 12 | Zero Compiler & Linker Warnings | Clean/re-generate `assets/app.o` to eliminate `corrupt .drectve` linker warning | M3 | Survey Arch Explorer |
| 13 | Comprehensive Unit Test Suite | Add unit tests for `device.rs`, `config.rs`, `win32.rs`, and verify `autoclick.rs` | M3 | Survey Arch Explorer |
| 14 | E2E Performance & Integrity Verification | Pass automated validation: 0 powershell calls, CPU <0.1%, clean cargo check & test | M4 | Survey Arch / Request |
| 15 | Single-Instance Named Mutex | Create named Win32 mutex `Local\RiotMQ10_Companion_SingleInstance` | M5 | Request R1 |
| 16 | Duplicate Window Focus Restoration | Locate existing `RiotMQ10_Class` window, SW_RESTORE, SetForegroundWindow, exit cleanly | M5 | Request R1 |
| 17 | Win32 FFI Expansion | Declare CreateMutexW, CloseHandle, FindWindowW, SetForegroundWindow, MessageBoxW, etc. in `win32.rs` | M5 | Request R4 |
| 18 | Exit Confirmation Dialog on WM_CLOSE | MessageBoxW with MB_YESNOCANCEL: Yes (full exit), No (hide to tray), Cancel (abort close) | M6 | Request R2 |
| 19 | Safe System Termination (WM_QUERYENDSESSION / WM_ENDSESSION) | Immediate cleanup of timers and tray icon with clean exit 0 | M6 | Request R3 |
| 20 | Zero Warnings & Test Verification | cargo check 0 warnings, cargo test passes, clean code | M7 | Request Criteria |

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| 1 | Native Win32 SetupAPI Device Detection | `src/win32.rs`, `src/device.rs`, `src/gui.rs` | None | DONE |
| 2 | Low-Latency Hook & Precision Timing | `src/win32.rs`, `src/hook.rs`, `src/autoclick.rs`, `src/main.rs`, `src/gui.rs` | M1 | DONE |
| 3 | Concurrency Safety & Warning Elimination | `src/gui.rs`, `src/main.rs`, `assets/app.o`, `build.rs`, unit tests | M2 | DONE |
| 4 | Verification, Hardening & Forensic Audit | Verification commands, reviewer/challenger audit, gate pass | M1, M2, M3 | DONE |
| 5 | Single-Instance Mutex & Window Focus Restoration | `src/win32.rs`, `src/main.rs` | M4 | DONE |
| 6 | Exit Confirmation Dialog & Safe Shutdown Handling | `src/gui.rs`, `src/win32.rs` | M5 | DONE |
| 7 | Round 2 Verification, Review & Audit | Build, test, reviewer and auditor gate | M5, M6 | DONE |

## Interface Contracts
### `src/device.rs` ↔ `src/gui.rs`
- `pub fn query_device_connection() -> DeviceStatus`:
  - Pure Win32 SetupAPI call (synchronous, ~10ms execution, zero child processes).
- `pub fn start_device_monitor(...) -> (Arc<Mutex<DeviceStatus>>, std::sync::mpsc::Sender<()>)`:
  - Background monitor thread sleeps for 3s (or responds to refresh signals).
- `gui.rs` handles `WM_DEVICECHANGE` (`0x0219`) to invoke `trigger_device_refresh()` or query immediately.

### `src/hook.rs` ↔ `src/gui.rs` / `src/main.rs`
- Atomic configuration cache:
  - `pub fn update_hook_config(cfg: &AppConfig)` updates atomics (`IS_SUSPENDED`, `ENABLE_GESTURES`, `AUTOCLICK_INTERVAL_MS`, etc.).
  - `pub fn set_autoclick_interval(ms: u32)` updates `AUTOCLICK_INTERVAL_MS` instantly from Tab 2 slider / presets.
- `pub fn set_last_event(text: &str)` uses `try_lock()` to avoid blocking `mouse_hook_callback`.

## Code Layout
- `src/main.rs`: Entry point, calls `timeBeginPeriod(1)`, checks single instance, launches GUI.
- `src/win32.rs`: Native FFI imports from `kernel32`, `user32`, `setupapi`, `winmm`.
- `src/device.rs`: Device detection logic using SetupAPI.
- `src/hook.rs`: Low-level hook callback and clicker worker thread.
- `src/autoclick.rs`: Timing calculations and CPS conversion with unit tests.
- `src/gui.rs`: Native Win32 window, control creation, message dispatch (`WM_TIMER`, `WM_DEVICECHANGE`).
- `src/config.rs`: INI persistence.
- `build.rs`: Resource linking.
