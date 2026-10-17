use crate::config::{Action, AppConfig};
use crate::win32::*;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, AtomicU8, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

// Lock-free atomic hook configuration (zero lock contention in callback)
pub static IS_SUSPENDED: AtomicBool = AtomicBool::new(false);
pub static ENABLE_GESTURES: AtomicBool = AtomicBool::new(true);
pub static INVERT_SCROLL: AtomicBool = AtomicBool::new(false);
pub static SCROLL_MULTIPLIER: AtomicU32 = AtomicU32::new(1);
pub static SNIPER_SPEED: AtomicU32 = AtomicU32::new(5);
pub static AUTOCLICK_INTERVAL_MS: AtomicU32 = AtomicU32::new(100);
pub static XBUTTON1_ACTION: AtomicU8 = AtomicU8::new(0);
pub static XBUTTON2_ACTION: AtomicU8 = AtomicU8::new(0);
pub static MIDDLE_CLICK_ACTION: AtomicU8 = AtomicU8::new(0);

pub static LAST_EVENT: Mutex<Option<String>> = Mutex::new(None);
pub static IS_RAPID_FIRE: AtomicBool = AtomicBool::new(false);

static X1_HELD: AtomicBool = AtomicBool::new(false);
static X2_HELD: AtomicBool = AtomicBool::new(false);
static X1_DID_CHORD: AtomicBool = AtomicBool::new(false);
static X2_DID_CHORD: AtomicBool = AtomicBool::new(false);

static GLOBAL_HOOK: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());
static SNIPER_SENDER: OnceLock<std::sync::mpsc::SyncSender<bool>> = OnceLock::new();

pub const LLMHF_INJECTED: u32 = 0x00000001;
const XBUTTON1: u16 = 0x0001;
const XBUTTON2: u16 = 0x0002;

pub fn take_last_event() -> Option<String> {
    LAST_EVENT.try_lock().ok().and_then(|mut g| g.take())
}

pub fn is_rapid_fire() -> bool {
    IS_RAPID_FIRE.load(Ordering::Relaxed)
}

pub fn toggle_rapid_fire() -> bool {
    let current = IS_RAPID_FIRE.load(Ordering::Relaxed);
    let next = !current;
    IS_RAPID_FIRE.store(next, Ordering::Relaxed);
    next
}

pub fn set_last_event(text: String) {
    if let Ok(mut g) = LAST_EVENT.try_lock() {
        *g = Some(text);
    }
}

pub fn update_hook_config(cfg: &AppConfig) {
    IS_SUSPENDED.store(cfg.is_suspended, Ordering::Relaxed);
    ENABLE_GESTURES.store(cfg.enable_gestures, Ordering::Relaxed);
    INVERT_SCROLL.store(cfg.invert_scroll, Ordering::Relaxed);
    SCROLL_MULTIPLIER.store(cfg.scroll_multiplier, Ordering::Relaxed);
    SNIPER_SPEED.store(cfg.sniper_speed, Ordering::Relaxed);
    set_autoclick_interval(cfg.autoclick_interval_ms);
    XBUTTON1_ACTION.store(cfg.buttons.xbutton1.to_u8(), Ordering::Relaxed);
    XBUTTON2_ACTION.store(cfg.buttons.xbutton2.to_u8(), Ordering::Relaxed);
    MIDDLE_CLICK_ACTION.store(cfg.buttons.middle_click.to_u8(), Ordering::Relaxed);
}

pub fn set_autoclick_interval(ms: u32) {
    let clamped = crate::autoclick::clamp_interval(ms);
    AUTOCLICK_INTERVAL_MS.store(clamped, Ordering::Relaxed);
}

pub fn precise_sleep_until(deadline: std::time::Instant) {
    let mut now = std::time::Instant::now();
    while deadline > now {
        let rem = deadline - now;
        if rem > std::time::Duration::from_millis(2) {
            std::thread::sleep(rem - std::time::Duration::from_micros(1500));
        } else if rem > std::time::Duration::from_micros(300) {
            std::thread::yield_now();
        } else {
            std::hint::spin_loop();
        }
        now = std::time::Instant::now();
    }
}

pub fn start_mouse_hook(config: Arc<std::sync::RwLock<AppConfig>>) {
    if let Ok(cfg) = config.read() {
        update_hook_config(&cfg);
    }

    // Spawn Sniper Clutch background worker to offload synchronous SystemParametersInfoW calls
    let (sniper_tx, sniper_rx) = std::sync::mpsc::sync_channel::<bool>(4);
    let _ = SNIPER_SENDER.set(sniper_tx);

    std::thread::spawn(move || {
        let mut original_speed: u32 = 10;
        let mut is_sniper_active = false;
        while let Ok(activate) = sniper_rx.recv() {
            if activate && !is_sniper_active {
                unsafe {
                    let mut cur_spd: u32 = 10;
                    SystemParametersInfoW(
                        SPI_GETMOUSESPEED,
                        0,
                        &mut cur_spd as *mut _ as *mut std::ffi::c_void,
                        0,
                    );
                    original_speed = cur_spd;
                    let target = SNIPER_SPEED.load(Ordering::Relaxed);
                    SystemParametersInfoW(
                        SPI_SETMOUSESPEED,
                        0,
                        target as usize as *mut std::ffi::c_void,
                        0,
                    );
                }
                is_sniper_active = true;
            } else if !activate && is_sniper_active {
                unsafe {
                    SystemParametersInfoW(
                        SPI_SETMOUSESPEED,
                        0,
                        original_speed as usize as *mut std::ffi::c_void,
                        0,
                    );
                }
                is_sniper_active = false;
            }
        }
    });

    // Spawn High-Precision Rapid Fire auto-clicker worker thread with monotonic deadline pacing
    std::thread::spawn(|| {
        unsafe {
            SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_HIGHEST);
        }
        loop {
            if IS_RAPID_FIRE.load(Ordering::Relaxed) {
                let mut next_cycle_start = std::time::Instant::now();
                while IS_RAPID_FIRE.load(Ordering::Relaxed) {
                    let interval = AUTOCLICK_INTERVAL_MS.load(Ordering::Relaxed);
                    let (down_ms, _) = crate::autoclick::calc_click_timings(interval);
                    let cycle_duration = std::time::Duration::from_millis(interval as u64);
                    let down_duration = std::time::Duration::from_millis(down_ms);

                    unsafe {
                        send_left_click(true);
                    }
                    let up_deadline = next_cycle_start + down_duration;
                    precise_sleep_until(up_deadline);

                    unsafe {
                        send_left_click(false);
                    }
                    next_cycle_start += cycle_duration;

                    if !IS_RAPID_FIRE.load(Ordering::Relaxed) {
                        break;
                    }

                    let now = std::time::Instant::now();
                    if now > next_cycle_start {
                        next_cycle_start = now;
                    } else {
                        precise_sleep_until(next_cycle_start);
                    }
                }
            } else {
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        }
    });

    // Spawn low-level mouse hook on dedicated background thread
    std::thread::spawn(|| unsafe {
        SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_HIGHEST);

        let h_instance = GetModuleHandleW(std::ptr::null());
        let hook = SetWindowsHookExW(
            WH_MOUSE_LL,
            Some(mouse_hook_callback),
            h_instance,
            0,
        );
        if hook.is_null() {
            let err = GetLastError();
            eprintln!("Failed to register mouse hook: {}", err);
            return;
        }
        GLOBAL_HOOK.store(hook, Ordering::Release);

        let mut msg = MSG {
            hwnd: std::ptr::null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        let h = GLOBAL_HOOK.load(Ordering::Acquire);
        if !h.is_null() {
            UnhookWindowsHookEx(h);
        }
    });
}

unsafe extern "system" fn mouse_hook_callback(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code < 0 {
        return CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param);
    }

    let msg = w_param as u32;

    // FAST-PATH: Mouse movement accounts for >99.8% of events.
    // Return immediately to avoid any lock/atomic overhead or pointer dereferences.
    if msg == WM_MOUSEMOVE {
        return CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param);
    }

    let info = *(l_param as *const MSLLHOOKSTRUCT);

    // Ignore synthetic events (software injected or our own auto-clicker)
    if (info.flags & LLMHF_INJECTED) != 0
        || info.dwExtraInfo == OMNIMOUSE_INJECTED_SIGNATURE
        || info.dwExtraInfo == RIOT_INJECTED_SIGNATURE
    {
        return CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param);
    }

    // Check suspension state lock-free
    if IS_SUSPENDED.load(Ordering::Relaxed) {
        return CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param);
    }

    match msg {
        WM_XBUTTONDOWN => {
            let button = (info.mouseData >> 16) as u16;
            let (action_u8, is_x1) = if button == XBUTTON1 {
                (XBUTTON1_ACTION.load(Ordering::Relaxed), true)
            } else if button == XBUTTON2 {
                (XBUTTON2_ACTION.load(Ordering::Relaxed), false)
            } else {
                (0, false)
            };
            let action = Action::from_u8(action_u8);

            if is_x1 {
                X1_HELD.store(true, Ordering::Relaxed);
                X1_DID_CHORD.store(false, Ordering::Relaxed);
            } else {
                X2_HELD.store(true, Ordering::Relaxed);
                X2_DID_CHORD.store(false, Ordering::Relaxed);
            }

            if action == Action::RapidFire {
                IS_RAPID_FIRE.store(true, Ordering::Relaxed);
                let btn_name = if is_x1 { "Tombol Samping 1 (X1)" } else { "Tombol Samping 2 (X2)" };
                let interval = AUTOCLICK_INTERVAL_MS.load(Ordering::Relaxed);
                set_last_event(format!("{}: Auto-Clicker Kiri AKTIF ({} ms)", btn_name, interval));
                return 1;
            }

            if action == Action::SniperPrecision {
                if let Some(tx) = SNIPER_SENDER.get() {
                    let _ = tx.try_send(true);
                }
                set_last_event("Sniper Clutch: AKTIF (Sensitivitas Diturunkan)".to_string());
                return 1;
            }

            let enable_gestures = ENABLE_GESTURES.load(Ordering::Relaxed);
            if enable_gestures {
                if action != Action::Default {
                    return 1;
                }
            } else if action != Action::Default {
                let btn_name = if is_x1 { "Tombol Samping 1 (X1)" } else { "Tombol Samping 2 (X2)" };
                set_last_event(format!("{}: {}", btn_name, action.label()));
                execute_action(action);
                return 1;
            }
        }
        WM_XBUTTONUP => {
            let button = (info.mouseData >> 16) as u16;
            let (action_u8, is_x1) = if button == XBUTTON1 {
                (XBUTTON1_ACTION.load(Ordering::Relaxed), true)
            } else if button == XBUTTON2 {
                (XBUTTON2_ACTION.load(Ordering::Relaxed), false)
            } else {
                (0, false)
            };
            let action = Action::from_u8(action_u8);

            let did_chord = if is_x1 {
                let c = X1_DID_CHORD.swap(false, Ordering::Relaxed);
                X1_HELD.store(false, Ordering::Relaxed);
                c
            } else {
                let c = X2_DID_CHORD.swap(false, Ordering::Relaxed);
                X2_HELD.store(false, Ordering::Relaxed);
                c
            };

            if action == Action::RapidFire {
                IS_RAPID_FIRE.store(false, Ordering::Relaxed);
                let btn_name = if is_x1 { "Tombol Samping 1 (X1)" } else { "Tombol Samping 2 (X2)" };
                set_last_event(format!("{}: Auto-Clicker NONAKTIF", btn_name));
                return 1;
            }

            if action == Action::SniperPrecision {
                if let Some(tx) = SNIPER_SENDER.get() {
                    let _ = tx.try_send(false);
                }
                set_last_event("Sniper Clutch: NONAKTIF (Sensitivitas Normal)".to_string());
                return 1;
            }

            let enable_gestures = ENABLE_GESTURES.load(Ordering::Relaxed);
            if enable_gestures {
                if !did_chord && action != Action::Default {
                    let btn_name = if is_x1 { "Tombol Samping 1 (X1)" } else { "Tombol Samping 2 (X2)" };
                    set_last_event(format!("{}: {}", btn_name, action.label()));
                    execute_action(action);
                    return 1;
                } else if did_chord || action != Action::Default {
                    return 1;
                }
            } else if action != Action::Default {
                return 1;
            }
        }
        WM_MOUSEWHEEL => {
            let delta = ((info.mouseData >> 16) as i16) as i32;
            let enable_gestures = ENABLE_GESTURES.load(Ordering::Relaxed);
            let x1_held = X1_HELD.load(Ordering::Relaxed);
            let x2_held = X2_HELD.load(Ordering::Relaxed);

            if enable_gestures && (x1_held || x2_held) {
                if x1_held {
                    X1_DID_CHORD.store(true, Ordering::Relaxed);
                    if delta > 0 {
                        send_single(VK_VOLUME_UP);
                        set_last_event("Gestur X1 + Scroll Atas: Volume Naik (+)".to_string());
                    } else {
                        send_single(VK_VOLUME_DOWN);
                        set_last_event("Gestur X1 + Scroll Bawah: Volume Turun (-)".to_string());
                    }
                    return 1;
                } else if x2_held {
                    X2_DID_CHORD.store(true, Ordering::Relaxed);
                    mouse_event(MOUSEEVENTF_HWHEEL, 0, 0, delta as u32, 0);
                    set_last_event("Gestur X2 + Scroll: Horizontal Scroll".to_string());
                    return 1;
                }
            } else {
                let invert_scroll = INVERT_SCROLL.load(Ordering::Relaxed);
                let scroll_multiplier = SCROLL_MULTIPLIER.load(Ordering::Relaxed);
                if invert_scroll || scroll_multiplier > 1 {
                    let mult = if scroll_multiplier > 0 { scroll_multiplier as i32 } else { 1 };
                    let modified_delta = if invert_scroll {
                        -delta * mult
                    } else {
                        delta * mult
                    };
                    mouse_event(MOUSEEVENTF_WHEEL, 0, 0, modified_delta as u32, 0);
                    return 1;
                }
            }
        }
        WM_MBUTTONDOWN => {
            let enable_gestures = ENABLE_GESTURES.load(Ordering::Relaxed);
            let x1_held = X1_HELD.load(Ordering::Relaxed);
            if enable_gestures && x1_held {
                X1_DID_CHORD.store(true, Ordering::Relaxed);
                send_single(VK_VOLUME_MUTE);
                set_last_event("Gestur X1 + Middle Click: Mute / Unmute Audio".to_string());
                return 1;
            }

            let action = Action::from_u8(MIDDLE_CLICK_ACTION.load(Ordering::Relaxed));
            if action == Action::RapidFire {
                IS_RAPID_FIRE.store(true, Ordering::Relaxed);
                let interval = AUTOCLICK_INTERVAL_MS.load(Ordering::Relaxed);
                set_last_event(format!("Tombol Tengah: Auto-Clicker Kiri AKTIF ({} ms)", interval));
                return 1;
            }

            set_last_event(format!("Tombol Tengah: {}", action.label()));
            if action != Action::Default {
                execute_action(action);
                return 1;
            }
        }
        WM_MBUTTONUP => {
            let action = Action::from_u8(MIDDLE_CLICK_ACTION.load(Ordering::Relaxed));
            if action == Action::RapidFire {
                IS_RAPID_FIRE.store(false, Ordering::Relaxed);
                set_last_event("Tombol Tengah: Auto-Clicker NONAKTIF".to_string());
                return 1;
            }
            if action != Action::Default {
                return 1;
            }
        }
        _ => {}
    }

    CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param)
}

fn execute_action(action: Action) {
    match action {
        Action::Default | Action::Disabled => {}
        Action::Copy => send_combo(&[VK_CONTROL, VK_C]),
        Action::Paste => send_combo(&[VK_CONTROL, VK_V]),
        Action::Cut => send_combo(&[VK_CONTROL, VK_X]),
        Action::Undo => send_combo(&[VK_CONTROL, VK_Z]),
        Action::SelectAll => send_combo(&[VK_CONTROL, VK_A]),
        Action::ShowDesktop => send_combo(&[VK_LWIN, VK_D]),
        Action::TaskView => send_combo(&[VK_LWIN, VK_TAB]),
        Action::SnippingTool => send_combo(&[VK_LWIN, VK_SHIFT, VK_S]),
        Action::CloseTab => send_combo(&[VK_CONTROL, VK_W]),
        Action::ReopenTab => send_combo(&[VK_CONTROL, VK_SHIFT, VK_T]),
        Action::NewTab => send_combo(&[VK_CONTROL, VK_T]),
        Action::Enter => send_single(VK_RETURN),
        Action::Escape => send_single(VK_ESCAPE),
        Action::AltTab => send_combo(&[VK_MENU, VK_TAB]),
        Action::MediaPlayPause => send_single(VK_MEDIA_PLAY_PAUSE),
        Action::MediaNext => send_single(VK_MEDIA_NEXT_TRACK),
        Action::MediaPrev => send_single(VK_MEDIA_PREV_TRACK),
        Action::VolumeUp => send_single(VK_VOLUME_UP),
        Action::VolumeDown => send_single(VK_VOLUME_DOWN),
        Action::VolumeMute => send_single(VK_VOLUME_MUTE),
        Action::BrowserBack => send_single(VK_BROWSER_BACK),
        Action::BrowserForward => send_single(VK_BROWSER_FORWARD),
        Action::SniperPrecision => {}
        Action::RapidFire => {}
    }
}

fn send_single(key: u16) {
    let inputs = [make_key_input(key, false), make_key_input(key, true)];
    unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        );
    }
}

fn send_combo(keys: &[u16]) {
    if keys.is_empty() || keys.len() > 4 {
        return;
    }
    let mut inputs = [make_key_input(0, false); 8];
    let count = keys.len() * 2;
    for (i, &k) in keys.iter().enumerate() {
        inputs[i] = make_key_input(k, false);
    }
    for (i, &k) in keys.iter().rev().enumerate() {
        inputs[keys.len() + i] = make_key_input(k, true);
    }
    unsafe {
        SendInput(
            count as u32,
            inputs.as_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        );
    }
}

fn make_key_input(key: u16, key_up: bool) -> INPUT {
    let mut flags = 0;
    if key_up {
        flags |= KEYEVENTF_KEYUP;
    }
    INPUT {
        r#type: INPUT_KEYBOARD,
        _pad: 0,
        u: INPUT_UNION {
            ki: KEYBDINPUT {
                wVk: key,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_autoclick_interval() {
        set_autoclick_interval(10);
        assert_eq!(AUTOCLICK_INTERVAL_MS.load(Ordering::Relaxed), 10);

        set_autoclick_interval(500);
        assert_eq!(AUTOCLICK_INTERVAL_MS.load(Ordering::Relaxed), 500);

        // Clamping checks
        set_autoclick_interval(1);
        assert_eq!(AUTOCLICK_INTERVAL_MS.load(Ordering::Relaxed), 10);

        set_autoclick_interval(1000);
        assert_eq!(AUTOCLICK_INTERVAL_MS.load(Ordering::Relaxed), 500);
    }

    #[test]
    fn test_update_hook_config() {
        let mut cfg = AppConfig::default();
        cfg.is_suspended = true;
        cfg.enable_gestures = false;
        cfg.invert_scroll = true;
        cfg.scroll_multiplier = 3;
        cfg.sniper_speed = 3;
        cfg.autoclick_interval_ms = 50;
        cfg.buttons.xbutton1 = Action::Copy;
        cfg.buttons.xbutton2 = Action::Paste;
        cfg.buttons.middle_click = Action::Undo;

        update_hook_config(&cfg);

        assert_eq!(IS_SUSPENDED.load(Ordering::Relaxed), true);
        assert_eq!(ENABLE_GESTURES.load(Ordering::Relaxed), false);
        assert_eq!(INVERT_SCROLL.load(Ordering::Relaxed), true);
        assert_eq!(SCROLL_MULTIPLIER.load(Ordering::Relaxed), 3);
        assert_eq!(SNIPER_SPEED.load(Ordering::Relaxed), 3);
        assert_eq!(AUTOCLICK_INTERVAL_MS.load(Ordering::Relaxed), 50);
        assert_eq!(XBUTTON1_ACTION.load(Ordering::Relaxed), Action::Copy.to_u8());
        assert_eq!(XBUTTON2_ACTION.load(Ordering::Relaxed), Action::Paste.to_u8());
        assert_eq!(MIDDLE_CLICK_ACTION.load(Ordering::Relaxed), Action::Undo.to_u8());
    }

    #[test]
    fn test_precise_sleep_until_accuracy() {
        let target_duration = std::time::Duration::from_millis(10);
        let start = std::time::Instant::now();
        precise_sleep_until(start + target_duration);
        let elapsed = start.elapsed();

        assert!(elapsed >= target_duration);
        assert!(elapsed < target_duration + std::time::Duration::from_millis(10));
    }

    #[test]
    fn test_precise_sleep_until_past_deadline() {
        let past = std::time::Instant::now() - std::time::Duration::from_millis(10);
        let start = std::time::Instant::now();
        precise_sleep_until(past);
        let elapsed = start.elapsed();
        assert!(elapsed < std::time::Duration::from_millis(5));
    }

    #[test]
    fn test_injected_signature_constant() {
        assert_eq!(OMNIMOUSE_INJECTED_SIGNATURE, 0x4F4D4E49);
        assert_eq!(RIOT_INJECTED_SIGNATURE, 0x52494F54);
    }

    #[test]
    fn test_click_cycle_pacing_simulation() {
        let interval_ms = 10u32;
        let cycle_duration = std::time::Duration::from_millis(interval_ms as u64);
        let (down_ms, _) = crate::autoclick::calc_click_timings(interval_ms);
        let down_duration = std::time::Duration::from_millis(down_ms);

        let iterations = 10;
        let start = std::time::Instant::now();
        let mut next_cycle_start = start;

        for _ in 0..iterations {
            let up_deadline = next_cycle_start + down_duration;
            precise_sleep_until(up_deadline);
            next_cycle_start += cycle_duration;
            let now = std::time::Instant::now();
            if now <= next_cycle_start {
                precise_sleep_until(next_cycle_start);
            } else {
                next_cycle_start = now;
            }
        }

        let total_elapsed = start.elapsed();
        // 10 cycles of 10ms should take ~100ms
        assert!(total_elapsed >= std::time::Duration::from_millis(90));
        assert!(total_elapsed <= std::time::Duration::from_millis(160));
    }

    #[test]
    fn test_last_event_set_and_take() {
        set_last_event("Event A".to_string());
        assert_eq!(take_last_event(), Some("Event A".to_string()));
        assert_eq!(take_last_event(), None);
    }

    #[test]
    fn test_is_and_toggle_rapid_fire() {
        IS_RAPID_FIRE.store(false, Ordering::Relaxed);
        assert!(!is_rapid_fire());
        let toggled = toggle_rapid_fire();
        assert!(toggled);
        assert!(is_rapid_fire());
        let toggled_back = toggle_rapid_fire();
        assert!(!toggled_back);
        assert!(!is_rapid_fire());
    }
}
