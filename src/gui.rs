#![allow(static_mut_refs)]

use crate::config::{configure_windows_autostart, save_config, Action, AppConfig};
use crate::device::DeviceStatus;
use crate::win32::*;
use std::sync::{Arc, Mutex, RwLock};

static mut APP_STATE: Option<GuiState> = None;

struct GuiState {
    config: Arc<RwLock<AppConfig>>,
    local_config: AppConfig,
    device_status: Arc<Mutex<DeviceStatus>>,
    current_tab: usize,

    // Navigation buttons
    h_btn_tab1: HWND,
    h_btn_tab2: HWND,

    // Tab 1 controls (Tombol & Perangkat)
    h_lbl_sec_assign: HWND,
    h_lbl_x1: HWND,
    h_combo_x1: HWND,
    h_lbl_x2: HWND,
    h_combo_x2: HWND,
    h_lbl_mid: HWND,
    h_combo_mid: HWND,
    h_lbl_sec_pref: HWND,
    h_chk_gestures: HWND,
    h_chk_invert_scroll: HWND,
    h_lbl_scroll_speed: HWND,
    h_combo_scroll_speed: HWND,
    h_chk_autostart: HWND,
    h_chk_tray: HWND,

    // Tab 2 controls (Auto Clicker)
    h_lbl_autoclick_title: HWND,
    h_lbl_autoclick_info: HWND,
    h_lbl_autoclick_val: HWND,
    h_trackbar_autoclick: HWND,
    h_lbl_slider_min: HWND,
    h_lbl_slider_max: HWND,
    h_lbl_presets: HWND,
    h_btn_preset_10: HWND,
    h_btn_preset_50: HWND,
    h_btn_preset_100: HWND,
    h_btn_preset_200: HWND,
    h_btn_toggle_autoclick: HWND,
    h_lbl_test_sec: HWND,
    h_btn_test_click: HWND,
    h_btn_test_reset: HWND,
    h_lbl_test_tip: HWND,
    click_test_counter: u64,

    // Common footer controls
    h_lbl_status: HWND,
    h_lbl_battery: HWND,
    h_lbl_event: HWND,
    h_btn_suspend: HWND,

    // GDI resources
    hbr_bg: HBRUSH,
    h_font_bold: HFONT,
    h_font_semi: HFONT,
    h_font_reg: HFONT,

    // UI state cache to avoid redundant SetWindowTextW updates
    last_battery_text: String,
    last_status_text: String,
    last_clicking_state: Option<bool>,
}

const IDC_SAVE_BTN: usize = 101;
const IDC_SUSPEND_BTN: usize = 102;
const IDC_COMBO_X1: usize = 103;
const IDC_COMBO_X2: usize = 104;
const IDC_COMBO_MID: usize = 105;
const IDC_CHK_AUTOSTART: usize = 106;
const IDC_CHK_TRAY: usize = 107;
const IDC_CHK_GESTURES: usize = 108;
const IDC_CHK_INVERT_SCROLL: usize = 109;
const IDC_COMBO_SCROLL_SPEED: usize = 110;

// Tab navigation IDs
const IDC_TAB_BTN1: usize = 120;
const IDC_TAB_BTN2: usize = 121;

// Tab 2 Auto Clicker IDs
const IDC_TRACKBAR_AUTOCLICK: usize = 130;
const IDC_PRESET_10: usize = 131;
const IDC_PRESET_50: usize = 132;
const IDC_PRESET_100: usize = 133;
const IDC_PRESET_200: usize = 134;
const IDC_TEST_CLICK: usize = 135;
const IDC_TEST_RESET: usize = 136;
const IDC_TOGGLE_AUTOCLICK: usize = 137;
const HOTKEY_AUTOCLICK_ID: i32 = 1;

// System Tray Menu IDs
const IDM_TRAY_RESTORE: usize = 2001;
const IDM_TRAY_EXIT: usize = 2002;

pub fn run_gui(config: Arc<RwLock<AppConfig>>, device_status: Arc<Mutex<DeviceStatus>>) {
    let initial_cfg = config.read().map(|c| c.clone()).unwrap_or_default();

    unsafe {
        // Initialize common controls for Trackbar (Slider)
        let iccex = INITCOMMONCONTROLSEX {
            dwSize: std::mem::size_of::<INITCOMMONCONTROLSEX>() as u32,
            dwICC: ICC_BAR_CLASSES,
        };
        InitCommonControlsEx(&iccex);

        let class_name = to_wide("OmniMouse_Class");
        let wnd_title = to_wide("OmniMouse Companion");

        // Charcoal Dark Palette (#18181b / RGB: 24, 24, 27)
        let hbr_bg = CreateSolidBrush(RGB(24, 24, 27));

        let h_instance = GetModuleHandleW(std::ptr::null());
        let custom_icon = LoadIconW(h_instance, 1 as *const u16);
        let h_icon = if !custom_icon.is_null() {
            custom_icon
        } else {
            LoadIconW(std::ptr::null_mut(), 32512 as *const u16)
        };

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: h_icon,
            hCursor: LoadCursorW(std::ptr::null_mut(), 32512 as *const u16), // IDC_ARROW
            hbrBackground: hbr_bg,
            lpszMenuName: std::ptr::null(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: h_icon,
        };

        RegisterClassExW(&wc);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            wnd_title.as_ptr(),
            WS_OVERLAPPEDWINDOW & !0x00040000, // Non-resizable
            150,
            100,
            515,
            600,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            h_instance,
            std::ptr::null_mut(),
        );

        if hwnd.is_null() {
            let err = GetLastError();
            let _ = std::fs::write("gui_debug.log", format!("Failed CreateWindow: err={}", err));
            return;
        }
        let _ = std::fs::write("gui_debug.log", "CreateWindow succeeded\n");

        SendMessageW(hwnd, WM_SETICON, ICON_BIG, h_icon as LPARAM);
        SendMessageW(hwnd, WM_SETICON, ICON_SMALL, h_icon as LPARAM);

        // Register Global Hotkey F6 for instant Auto-Clicker Toggle
        RegisterHotKey(hwnd, HOTKEY_AUTOCLICK_ID, 0, VK_F6 as u32);

        // Apply dark mode titlebar (Windows 10/11 DWM)
        let dark_mode: i32 = 1;
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE,
            &dark_mode as *const i32 as *const std::ffi::c_void,
            4,
        );

        // Create Modern Segoe UI Fonts with ClearType
        let font_name = to_wide("Segoe UI");
        let h_font_bold = CreateFontW(
            -18, 0, 0, 0, FW_BOLD, 0, 0, 0,
            DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
            CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE,
            font_name.as_ptr(),
        );
        let h_font_semi = CreateFontW(
            -14, 0, 0, 0, FW_SEMIBOLD, 0, 0, 0,
            DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
            CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE,
            font_name.as_ptr(),
        );
        let h_font_reg = CreateFontW(
            -14, 0, 0, 0, FW_NORMAL, 0, 0, 0,
            DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
            CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE,
            font_name.as_ptr(),
        );

        let controls = create_controls(hwnd, &initial_cfg, h_font_bold, h_font_semi, h_font_reg);

        APP_STATE = Some(GuiState {
            config,
            local_config: initial_cfg,
            device_status,
            current_tab: 0,
            h_btn_tab1: controls.btn_tab1,
            h_btn_tab2: controls.btn_tab2,
            h_lbl_sec_assign: controls.lbl_sec_assign,
            h_lbl_x1: controls.lbl_x1,
            h_combo_x1: controls.combo_x1,
            h_lbl_x2: controls.lbl_x2,
            h_combo_x2: controls.combo_x2,
            h_lbl_mid: controls.lbl_mid,
            h_combo_mid: controls.combo_mid,
            h_lbl_sec_pref: controls.lbl_sec_pref,
            h_chk_gestures: controls.chk_gestures,
            h_chk_invert_scroll: controls.chk_invert_scroll,
            h_lbl_scroll_speed: controls.lbl_scroll_speed,
            h_combo_scroll_speed: controls.combo_scroll_speed,
            h_chk_autostart: controls.chk_autostart,
            h_chk_tray: controls.chk_tray,
            h_lbl_autoclick_title: controls.lbl_autoclick_title,
            h_lbl_autoclick_info: controls.lbl_autoclick_info,
            h_lbl_autoclick_val: controls.lbl_autoclick_val,
            h_trackbar_autoclick: controls.trackbar_autoclick,
            h_lbl_slider_min: controls.lbl_slider_min,
            h_lbl_slider_max: controls.lbl_slider_max,
            h_lbl_presets: controls.lbl_presets,
            h_btn_preset_10: controls.btn_preset_10,
            h_btn_preset_50: controls.btn_preset_50,
            h_btn_preset_100: controls.btn_preset_100,
            h_btn_preset_200: controls.btn_preset_200,
            h_btn_toggle_autoclick: controls.btn_toggle_autoclick,
            h_lbl_test_sec: controls.lbl_test_sec,
            h_btn_test_click: controls.btn_test_click,
            h_btn_test_reset: controls.btn_test_reset,
            h_lbl_test_tip: controls.lbl_test_tip,
            click_test_counter: 0,
            h_lbl_status: controls.lbl_status,
            h_lbl_battery: controls.lbl_battery,
            h_lbl_event: controls.lbl_event,
            h_btn_suspend: controls.btn_suspend,
            hbr_bg,
            h_font_bold,
            h_font_semi,
            h_font_reg,
            last_battery_text: String::new(),
            last_status_text: String::new(),
            last_clicking_state: None,
        });

        // Initialize active tab visibility (Tab 1 by default)
        if let Some(state) = &mut APP_STATE {
            switch_tab(state, 0);
        }

        // Setup System Tray icon
        setup_tray_icon(hwnd, true);

        // Win32 timer (200 ms) for status polling directly on the UI message pump thread
        SetTimer(hwnd, 1, 200, None);

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

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

        setup_tray_icon(hwnd, false); // Remove tray icon on exit
    }
}

struct CreatedControls {
    btn_tab1: HWND,
    btn_tab2: HWND,
    lbl_status: HWND,
    lbl_battery: HWND,
    // Tab 1
    lbl_sec_assign: HWND,
    lbl_x1: HWND,
    combo_x1: HWND,
    lbl_x2: HWND,
    combo_x2: HWND,
    lbl_mid: HWND,
    combo_mid: HWND,
    lbl_sec_pref: HWND,
    chk_gestures: HWND,
    chk_invert_scroll: HWND,
    lbl_scroll_speed: HWND,
    combo_scroll_speed: HWND,
    chk_autostart: HWND,
    chk_tray: HWND,
    // Tab 2
    lbl_autoclick_title: HWND,
    lbl_autoclick_info: HWND,
    lbl_autoclick_val: HWND,
    trackbar_autoclick: HWND,
    lbl_slider_min: HWND,
    lbl_slider_max: HWND,
    lbl_presets: HWND,
    btn_preset_10: HWND,
    btn_preset_50: HWND,
    btn_preset_100: HWND,
    btn_preset_200: HWND,
    btn_toggle_autoclick: HWND,
    lbl_test_sec: HWND,
    btn_test_click: HWND,
    btn_test_reset: HWND,
    lbl_test_tip: HWND,
    // Footer
    btn_suspend: HWND,
    lbl_event: HWND,
}

unsafe fn create_controls(
    hwnd: HWND,
    cfg: &AppConfig,
    h_bold: HFONT,
    h_semi: HFONT,
    h_reg: HFONT,
) -> CreatedControls {
    let lbl_class = to_wide("STATIC");
    let edit_class = to_wide("COMBOBOX");
    let btn_class = to_wide("BUTTON");
    let trackbar_class = to_wide("msctls_trackbar32");

    // HEADER SECTION
    let lbl_title = create_label(hwnd, "OmniMouse Companion", 25, 14, 450, 22, &lbl_class);
    SendMessageW(lbl_title, WM_SETFONT, h_bold as WPARAM, 1);

    let lbl_status = create_label(hwnd, "Status: Mendeteksi koneksi...", 25, 38, 450, 18, &lbl_class);
    SendMessageW(lbl_status, WM_SETFONT, h_semi as WPARAM, 1);

    let lbl_battery = create_label(hwnd, "Daya Baterai: Indikator LED / Sistem Windows", 25, 58, 450, 18, &lbl_class);
    SendMessageW(lbl_battery, WM_SETFONT, h_reg as WPARAM, 1);

    // TWO-TIER UNDERLINE TAB NAVIGATION
    let btn_tab1 = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("[ 1. Tombol & Perangkat ]").as_ptr(),
        WS_CHILD | WS_VISIBLE,
        25,
        82,
        220,
        28,
        hwnd,
        IDC_TAB_BTN1 as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(btn_tab1, WM_SETFONT, h_semi as WPARAM, 1);

    let btn_tab2 = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("[ 2. Auto Clicker ]").as_ptr(),
        WS_CHILD | WS_VISIBLE,
        255,
        82,
        220,
        28,
        hwnd,
        IDC_TAB_BTN2 as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(btn_tab2, WM_SETFONT, h_semi as WPARAM, 1);

    // ================= TAB 1: TOMBOL & PERANGKAT =================
    let lbl_sec_assign = create_label(hwnd, "PENUGASAN TOMBOL (ASSIGNMENTS)", 25, 118, 450, 18, &lbl_class);
    SendMessageW(lbl_sec_assign, WM_SETFONT, h_semi as WPARAM, 1);

    let lbl_x1 = create_label(hwnd, "Tombol Samping 1 (Back / X1):", 25, 138, 450, 16, &lbl_class);
    SendMessageW(lbl_x1, WM_SETFONT, h_reg as WPARAM, 1);
    let combo_x1 = create_combobox(hwnd, 25, 156, 450, 240, IDC_COMBO_X1, &edit_class);
    SendMessageW(combo_x1, WM_SETFONT, h_reg as WPARAM, 1);
    populate_combobox(combo_x1, cfg.buttons.xbutton1);

    let lbl_x2 = create_label(hwnd, "Tombol Samping 2 (Forward / X2):", 25, 186, 450, 16, &lbl_class);
    SendMessageW(lbl_x2, WM_SETFONT, h_reg as WPARAM, 1);
    let combo_x2 = create_combobox(hwnd, 25, 204, 450, 240, IDC_COMBO_X2, &edit_class);
    SendMessageW(combo_x2, WM_SETFONT, h_reg as WPARAM, 1);
    populate_combobox(combo_x2, cfg.buttons.xbutton2);

    let lbl_mid = create_label(hwnd, "Tombol Roda Tengah (Middle Click):", 25, 234, 450, 16, &lbl_class);
    SendMessageW(lbl_mid, WM_SETFONT, h_reg as WPARAM, 1);
    let combo_mid = create_combobox(hwnd, 25, 252, 450, 240, IDC_COMBO_MID, &edit_class);
    SendMessageW(combo_mid, WM_SETFONT, h_reg as WPARAM, 1);
    populate_combobox(combo_mid, cfg.buttons.middle_click);

    let lbl_sec_pref = create_label(hwnd, "PREFERENSI SISTEM & SCROLL WHEEL", 25, 286, 450, 18, &lbl_class);
    SendMessageW(lbl_sec_pref, WM_SETFONT, h_semi as WPARAM, 1);

    let chk_gestures = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("Aktifkan Gestur: Tahan X1+Scroll = Volume, X2+Scroll = Horiz").as_ptr(),
        WS_CHILD | WS_VISIBLE | BS_AUTOCHECKBOX,
        25,
        308,
        450,
        20,
        hwnd,
        IDC_CHK_GESTURES as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(chk_gestures, WM_SETFONT, h_reg as WPARAM, 1);
    if cfg.enable_gestures {
        SendMessageW(chk_gestures, BM_SETCHECK, BST_CHECKED, 0);
    }

    let chk_invert_scroll = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("Balik Arah Scroll (Natural / Inverted Scrolling)").as_ptr(),
        WS_CHILD | WS_VISIBLE | BS_AUTOCHECKBOX,
        25,
        330,
        450,
        20,
        hwnd,
        IDC_CHK_INVERT_SCROLL as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(chk_invert_scroll, WM_SETFONT, h_reg as WPARAM, 1);
    if cfg.invert_scroll {
        SendMessageW(chk_invert_scroll, BM_SETCHECK, BST_CHECKED, 0);
    }

    let lbl_scroll_speed = create_label(hwnd, "Kecepatan Gulir Scroll Wheel:", 25, 354, 250, 18, &lbl_class);
    SendMessageW(lbl_scroll_speed, WM_SETFONT, h_reg as WPARAM, 1);

    let combo_scroll_speed = create_combobox(hwnd, 285, 352, 190, 120, IDC_COMBO_SCROLL_SPEED, &edit_class);
    SendMessageW(combo_scroll_speed, WM_SETFONT, h_reg as WPARAM, 1);
    let speed_options = ["1x (Kecepatan Normal)", "2x (Cepat)", "3x (Super Cepat)"];
    for opt in speed_options.iter() {
        let opt_wide = to_wide(opt);
        SendMessageW(combo_scroll_speed, CB_ADDSTRING, 0, opt_wide.as_ptr() as LPARAM);
    }
    let speed_idx = match cfg.scroll_multiplier {
        2 => 1,
        3 => 2,
        _ => 0,
    };
    SendMessageW(combo_scroll_speed, CB_SETCURSEL, speed_idx, 0);

    let chk_autostart = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("Jalankan otomatis saat Windows startup (Auto-Start)").as_ptr(),
        WS_CHILD | WS_VISIBLE | BS_AUTOCHECKBOX,
        25,
        378,
        450,
        20,
        hwnd,
        IDC_CHK_AUTOSTART as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(chk_autostart, WM_SETFONT, h_reg as WPARAM, 1);
    if cfg.auto_start {
        SendMessageW(chk_autostart, BM_SETCHECK, BST_CHECKED, 0);
    }

    let chk_tray = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("Minimize ke System Tray saat tombol silang (X) ditekan").as_ptr(),
        WS_CHILD | WS_VISIBLE | BS_AUTOCHECKBOX,
        25,
        400,
        450,
        20,
        hwnd,
        IDC_CHK_TRAY as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(chk_tray, WM_SETFONT, h_reg as WPARAM, 1);
    if cfg.minimize_to_tray {
        SendMessageW(chk_tray, BM_SETCHECK, BST_CHECKED, 0);
    }

    // ================= TAB 2: AUTO CLICKER =================
    let lbl_autoclick_title = create_label(
        hwnd,
        "PENGATURAN KECEPATAN AUTO CLICKER (KLIK KIRI)",
        25,
        118,
        450,
        18,
        &lbl_class,
    );
    SendMessageW(lbl_autoclick_title, WM_SETFONT, h_semi as WPARAM, 1);

    let lbl_autoclick_info = create_label(
        hwnd,
        "Geser slider untuk kecepatan klik (10 ms = 100 CPS, 500 ms = 2 CPS):",
        25,
        138,
        450,
        18,
        &lbl_class,
    );
    SendMessageW(lbl_autoclick_info, WM_SETFONT, h_reg as WPARAM, 1);

    let init_interval_label = crate::autoclick::format_interval_label(cfg.autoclick_interval_ms);
    let lbl_autoclick_val = create_label(hwnd, &init_interval_label, 25, 158, 450, 22, &lbl_class);
    SendMessageW(lbl_autoclick_val, WM_SETFONT, h_bold as WPARAM, 1);

    // Trackbar control (10 ms s/d 500 ms)
    let trackbar_autoclick = CreateWindowExW(
        0,
        trackbar_class.as_ptr(),
        std::ptr::null(),
        WS_CHILD | TBS_AUTOTICKS,
        25,
        182,
        450,
        32,
        hwnd,
        IDC_TRACKBAR_AUTOCLICK as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(trackbar_autoclick, TBM_SETRANGE, TRUE as WPARAM, ((500 << 16) | 10) as LPARAM);
    SendMessageW(trackbar_autoclick, TBM_SETPAGESIZE, 0, 10);
    SendMessageW(
        trackbar_autoclick,
        TBM_SETPOS,
        TRUE as WPARAM,
        cfg.autoclick_interval_ms as LPARAM,
    );

    let lbl_slider_min = create_label(
        hwnd,
        "◄ 10 ms (100 CPS / Super Cepat)",
        25,
        216,
        220,
        16,
        &lbl_class,
    );
    SendMessageW(lbl_slider_min, WM_SETFONT, h_reg as WPARAM, 1);

    let lbl_slider_max = create_label(
        hwnd,
        "500 ms (2 CPS / Santai) ►",
        260,
        216,
        215,
        16,
        &lbl_class,
    );
    SendMessageW(lbl_slider_max, WM_SETFONT, h_reg as WPARAM, 1);

    let lbl_presets = create_label(hwnd, "PILIHAN CEPAT (PRESET INTERVAL):", 25, 238, 450, 18, &lbl_class);
    SendMessageW(lbl_presets, WM_SETFONT, h_semi as WPARAM, 1);

    let btn_preset_10 = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("10 ms (100 CPS)").as_ptr(),
        WS_CHILD,
        25,
        258,
        105,
        26,
        hwnd,
        IDC_PRESET_10 as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(btn_preset_10, WM_SETFONT, h_reg as WPARAM, 1);

    let btn_preset_50 = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("50 ms (20 CPS)").as_ptr(),
        WS_CHILD,
        140,
        258,
        105,
        26,
        hwnd,
        IDC_PRESET_50 as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(btn_preset_50, WM_SETFONT, h_reg as WPARAM, 1);

    let btn_preset_100 = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("100 ms (10 CPS)").as_ptr(),
        WS_CHILD,
        255,
        258,
        105,
        26,
        hwnd,
        IDC_PRESET_100 as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(btn_preset_100, WM_SETFONT, h_reg as WPARAM, 1);

    let btn_preset_200 = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("200 ms (5 CPS)").as_ptr(),
        WS_CHILD,
        370,
        258,
        105,
        26,
        hwnd,
        IDC_PRESET_200 as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(btn_preset_200, WM_SETFONT, h_reg as WPARAM, 1);

    // TOGGLE ON/OFF BUTTON (PROMINENT ACTION)
    let btn_toggle_autoclick = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("▶ MULAI AUTO CLICKER (Hotkey: F6 / Klik Tombol Ini)").as_ptr(),
        WS_CHILD,
        25,
        292,
        450,
        34,
        hwnd,
        IDC_TOGGLE_AUTOCLICK as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(btn_toggle_autoclick, WM_SETFONT, h_semi as WPARAM, 1);

    let lbl_test_sec = create_label(hwnd, "AREA UJI COBA KLIK (CLICK TESTER):", 25, 334, 450, 18, &lbl_class);
    SendMessageW(lbl_test_sec, WM_SETFONT, h_semi as WPARAM, 1);

    let btn_test_click = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("Target Uji Klik (Total: 0)").as_ptr(),
        WS_CHILD,
        25,
        354,
        320,
        32,
        hwnd,
        IDC_TEST_CLICK as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(btn_test_click, WM_SETFONT, h_semi as WPARAM, 1);

    let btn_test_reset = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("Reset Hitungan").as_ptr(),
        WS_CHILD,
        355,
        354,
        120,
        32,
        hwnd,
        IDC_TEST_RESET as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(btn_test_reset, WM_SETFONT, h_semi as WPARAM, 1);

    let lbl_test_tip = create_label(
        hwnd,
        "Tips: Klik 'Mulai Auto Clicker' atau tekan F6 pada keyboard kapan saja untuk aktivasi instan!",
        25,
        392,
        450,
        34,
        &lbl_class,
    );
    SendMessageW(lbl_test_tip, WM_SETFONT, h_reg as WPARAM, 1);

    // ================= COMMON FOOTER =================
    let btn_save = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide("Simpan & Terapkan Perubahan").as_ptr(),
        WS_CHILD | WS_VISIBLE | BS_DEFPUSHBUTTON,
        25,
        432,
        270,
        34,
        hwnd,
        IDC_SAVE_BTN as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(btn_save, WM_SETFONT, h_semi as WPARAM, 1);

    let suspend_label = if cfg.is_suspended {
        "Aktifkan Hook"
    } else {
        "Tangguhkan Hook"
    };
    let btn_suspend = CreateWindowExW(
        0,
        btn_class.as_ptr(),
        to_wide(suspend_label).as_ptr(),
        WS_CHILD | WS_VISIBLE,
        305,
        432,
        170,
        34,
        hwnd,
        IDC_SUSPEND_BTN as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    SendMessageW(btn_suspend, WM_SETFONT, h_semi as WPARAM, 1);

    let lbl_sec_telem = create_label(hwnd, "MONITOR AKTIVITAS INPUT & GESTUR", 25, 476, 450, 18, &lbl_class);
    SendMessageW(lbl_sec_telem, WM_SETFONT, h_semi as WPARAM, 1);

    let lbl_event = create_label(
        hwnd,
        "Aksi Terakhir: (Menunggu input tombol mouse...)",
        25,
        496,
        450,
        22,
        &lbl_class,
    );
    SendMessageW(lbl_event, WM_SETFONT, h_reg as WPARAM, 1);

    CreatedControls {
        btn_tab1,
        btn_tab2,
        lbl_status,
        lbl_battery,
        lbl_sec_assign,
        lbl_x1,
        combo_x1,
        lbl_x2,
        combo_x2,
        lbl_mid,
        combo_mid,
        lbl_sec_pref,
        chk_gestures,
        chk_invert_scroll,
        lbl_scroll_speed,
        combo_scroll_speed,
        chk_autostart,
        chk_tray,
        lbl_autoclick_title,
        lbl_autoclick_info,
        lbl_autoclick_val,
        trackbar_autoclick,
        lbl_slider_min,
        lbl_slider_max,
        lbl_presets,
        btn_preset_10,
        btn_preset_50,
        btn_preset_100,
        btn_preset_200,
        btn_toggle_autoclick,
        lbl_test_sec,
        btn_test_click,
        btn_test_reset,
        lbl_test_tip,
        btn_suspend,
        lbl_event,
    }
}

unsafe fn switch_tab(state: &mut GuiState, new_tab: usize) {
    state.current_tab = new_tab;
    if new_tab == 0 {
        // Tab 1 Active
        SetWindowTextW(state.h_btn_tab1, to_wide("[● 1. Tombol & Perangkat]").as_ptr());
        SetWindowTextW(state.h_btn_tab2, to_wide("[ 2. Auto Clicker ]").as_ptr());

        // Show Tab 1 controls
        ShowWindow(state.h_lbl_sec_assign, SW_SHOW);
        ShowWindow(state.h_lbl_x1, SW_SHOW);
        ShowWindow(state.h_combo_x1, SW_SHOW);
        ShowWindow(state.h_lbl_x2, SW_SHOW);
        ShowWindow(state.h_combo_x2, SW_SHOW);
        ShowWindow(state.h_lbl_mid, SW_SHOW);
        ShowWindow(state.h_combo_mid, SW_SHOW);
        ShowWindow(state.h_lbl_sec_pref, SW_SHOW);
        ShowWindow(state.h_chk_gestures, SW_SHOW);
        ShowWindow(state.h_chk_invert_scroll, SW_SHOW);
        ShowWindow(state.h_lbl_scroll_speed, SW_SHOW);
        ShowWindow(state.h_combo_scroll_speed, SW_SHOW);
        ShowWindow(state.h_chk_autostart, SW_SHOW);
        ShowWindow(state.h_chk_tray, SW_SHOW);

        // Hide Tab 2 controls
        ShowWindow(state.h_lbl_autoclick_title, SW_HIDE);
        ShowWindow(state.h_lbl_autoclick_info, SW_HIDE);
        ShowWindow(state.h_lbl_autoclick_val, SW_HIDE);
        ShowWindow(state.h_trackbar_autoclick, SW_HIDE);
        ShowWindow(state.h_lbl_slider_min, SW_HIDE);
        ShowWindow(state.h_lbl_slider_max, SW_HIDE);
        ShowWindow(state.h_lbl_presets, SW_HIDE);
        ShowWindow(state.h_btn_preset_10, SW_HIDE);
        ShowWindow(state.h_btn_preset_50, SW_HIDE);
        ShowWindow(state.h_btn_preset_100, SW_HIDE);
        ShowWindow(state.h_btn_preset_200, SW_HIDE);
        ShowWindow(state.h_btn_toggle_autoclick, SW_HIDE);
        ShowWindow(state.h_lbl_test_sec, SW_HIDE);
        ShowWindow(state.h_btn_test_click, SW_HIDE);
        ShowWindow(state.h_btn_test_reset, SW_HIDE);
        ShowWindow(state.h_lbl_test_tip, SW_HIDE);
    } else {
        // Tab 2 Active
        SetWindowTextW(state.h_btn_tab1, to_wide("[ 1. Tombol & Perangkat ]").as_ptr());
        SetWindowTextW(state.h_btn_tab2, to_wide("[● 2. Auto Clicker]").as_ptr());

        // Hide Tab 1 controls
        ShowWindow(state.h_lbl_sec_assign, SW_HIDE);
        ShowWindow(state.h_lbl_x1, SW_HIDE);
        ShowWindow(state.h_combo_x1, SW_HIDE);
        ShowWindow(state.h_lbl_x2, SW_HIDE);
        ShowWindow(state.h_combo_x2, SW_HIDE);
        ShowWindow(state.h_lbl_mid, SW_HIDE);
        ShowWindow(state.h_combo_mid, SW_HIDE);
        ShowWindow(state.h_lbl_sec_pref, SW_HIDE);
        ShowWindow(state.h_chk_gestures, SW_HIDE);
        ShowWindow(state.h_chk_invert_scroll, SW_HIDE);
        ShowWindow(state.h_lbl_scroll_speed, SW_HIDE);
        ShowWindow(state.h_combo_scroll_speed, SW_HIDE);
        ShowWindow(state.h_chk_autostart, SW_HIDE);
        ShowWindow(state.h_chk_tray, SW_HIDE);

        // Show Tab 2 controls
        ShowWindow(state.h_lbl_autoclick_title, SW_SHOW);
        ShowWindow(state.h_lbl_autoclick_info, SW_SHOW);
        ShowWindow(state.h_lbl_autoclick_val, SW_SHOW);
        ShowWindow(state.h_trackbar_autoclick, SW_SHOW);
        ShowWindow(state.h_lbl_slider_min, SW_SHOW);
        ShowWindow(state.h_lbl_slider_max, SW_SHOW);
        ShowWindow(state.h_lbl_presets, SW_SHOW);
        ShowWindow(state.h_btn_preset_10, SW_SHOW);
        ShowWindow(state.h_btn_preset_50, SW_SHOW);
        ShowWindow(state.h_btn_preset_100, SW_SHOW);
        ShowWindow(state.h_btn_preset_200, SW_SHOW);
        ShowWindow(state.h_btn_toggle_autoclick, SW_SHOW);
        ShowWindow(state.h_lbl_test_sec, SW_SHOW);
        ShowWindow(state.h_btn_test_click, SW_SHOW);
        ShowWindow(state.h_btn_test_reset, SW_SHOW);
        ShowWindow(state.h_lbl_test_tip, SW_SHOW);
    }
}

unsafe fn create_label(
    hwnd: HWND,
    text: &str,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    class_name: &[u16],
) -> HWND {
    CreateWindowExW(
        0,
        class_name.as_ptr(),
        to_wide(text).as_ptr(),
        WS_CHILD | WS_VISIBLE,
        x,
        y,
        w,
        h,
        hwnd,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    )
}

unsafe fn create_combobox(
    hwnd: HWND,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    id: usize,
    class_name: &[u16],
) -> HWND {
    CreateWindowExW(
        0,
        class_name.as_ptr(),
        std::ptr::null(),
        WS_CHILD | WS_VISIBLE | WS_TABSTOP | CBS_DROPDOWNLIST,
        x,
        y,
        w,
        h,
        hwnd,
        id as HMENU,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    )
}

unsafe fn populate_combobox(combo: HWND, selected: Action) {
    let mut selected_idx = 0;
    for (i, action) in Action::ALL.iter().enumerate() {
        let label_wide = to_wide(action.label());
        SendMessageW(combo, CB_ADDSTRING, 0, label_wide.as_ptr() as LPARAM);
        if *action == selected {
            selected_idx = i;
        }
    }
    SendMessageW(combo, CB_SETCURSEL, selected_idx, 0);
}

unsafe fn setup_tray_icon(hwnd: HWND, add: bool) {
    let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1;
    nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
    nid.uCallbackMessage = WM_TRAYICON;
    let h_instance = GetModuleHandleW(std::ptr::null());
    let custom_icon = LoadIconW(h_instance, 1 as *const u16);
    let h_icon = if !custom_icon.is_null() {
        custom_icon
    } else {
        LoadIconW(std::ptr::null_mut(), 32512 as *const u16)
    };
    nid.hIcon = h_icon;

    let tip = to_wide("OmniMouse Companion");
    for (i, &c) in tip.iter().take(127).enumerate() {
        nid.szTip[i] = c;
    }

    if add {
        Shell_NotifyIconW(NIM_ADD, &mut nid);
    } else {
        Shell_NotifyIconW(NIM_DELETE, &mut nid);
    }
}

unsafe fn update_autoclick_speed(state: &mut GuiState, ms: u32) {
    let clamped = crate::autoclick::clamp_interval(ms);
    state.local_config.autoclick_interval_ms = clamped;
    crate::hook::set_autoclick_interval(clamped);
    SendMessageW(
        state.h_trackbar_autoclick,
        TBM_SETPOS,
        TRUE as WPARAM,
        clamped as LPARAM,
    );
    let label_str = crate::autoclick::format_interval_label(clamped);
    SetWindowTextW(state.h_lbl_autoclick_val, to_wide(&label_str).as_ptr());
}

unsafe fn toggle_autoclick_execution() {
    let next = crate::hook::toggle_rapid_fire();
    if let Some(state) = &APP_STATE {
        if next {
            crate::hook::set_last_event(format!(
                "Auto-Clicker Kiri: AKTIF ({} ms / Toggle)",
                state.local_config.autoclick_interval_ms
            ));
        } else {
            crate::hook::set_last_event("Auto-Clicker Kiri: NONAKTIF (Toggle)".to_string());
        }
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CTLCOLORDLG => {
            if let Some(state) = &APP_STATE {
                return state.hbr_bg as LRESULT;
            }
            0
        }
        WM_CTLCOLORSTATIC => {
            let hdc = wparam as HDC;
            SetBkMode(hdc, TRANSPARENT);
            SetTextColor(hdc, RGB(244, 244, 245)); // Bright white-zinc
            if let Some(state) = &APP_STATE {
                return state.hbr_bg as LRESULT;
            }
            0
        }
        WM_CTLCOLORBTN => {
            if let Some(state) = &APP_STATE {
                return state.hbr_bg as LRESULT;
            }
            0
        }
        WM_HOTKEY => {
            if wparam as i32 == HOTKEY_AUTOCLICK_ID {
                toggle_autoclick_execution();
            }
            0
        }
        WM_HSCROLL => {
            let trackbar_hwnd = lparam as HWND;
            if let Some(state) = &mut APP_STATE {
                if trackbar_hwnd == state.h_trackbar_autoclick {
                    let pos = SendMessageW(state.h_trackbar_autoclick, TBM_GETPOS, 0, 0) as u32;
                    let clamped = crate::autoclick::clamp_interval(pos);
                    state.local_config.autoclick_interval_ms = clamped;
                    crate::hook::set_autoclick_interval(clamped);
                    let label_str = crate::autoclick::format_interval_label(clamped);
                    SetWindowTextW(state.h_lbl_autoclick_val, to_wide(&label_str).as_ptr());
                }
            }
            0
        }
        WM_TIMER => {
            if wparam == 1 {
                if let Some(state) = &mut APP_STATE {
                    // Update battery and status texts (SetWindowTextW only if text changed)
                    if let Ok(dev) = state.device_status.lock() {
                        let status_str = format!("Status: {} ({})", dev.mode_badge, dev.connection_mode);
                        if state.last_status_text != status_str {
                            state.last_status_text = status_str.clone();
                            SetWindowTextW(state.h_lbl_status, to_wide(&status_str).as_ptr());
                        }

                        let bat_str = format!("Daya Baterai: {}", dev.battery_info);
                        if state.last_battery_text != bat_str {
                            state.last_battery_text = bat_str.clone();
                            SetWindowTextW(state.h_lbl_battery, to_wide(&bat_str).as_ptr());
                        }
                    }

                    // Update last event text
                    if let Some(event_str) = crate::hook::take_last_event() {
                        let ev_text = format!("Aksi Terakhir: {}", event_str);
                        SetWindowTextW(state.h_lbl_event, to_wide(&ev_text).as_ptr());
                    }

                    // Update auto-clicker toggle button text based on crate::hook::is_rapid_fire()
                    let is_clicking = crate::hook::is_rapid_fire();
                    if state.last_clicking_state != Some(is_clicking) {
                        state.last_clicking_state = Some(is_clicking);
                        let btn_text = if is_clicking {
                            "⏹ HENTIKAN AUTO CLICKER (Sedang Berjalan...)"
                        } else {
                            "▶ MULAI AUTO CLICKER (Hotkey: F6 / Klik Tombol Ini)"
                        };
                        SetWindowTextW(state.h_btn_toggle_autoclick, to_wide(btn_text).as_ptr());
                    }
                }
            }
            0
        }
        WM_COMMAND => {
            let id = (wparam & 0xFFFF) as usize;
            match id {
                IDC_TAB_BTN1 => {
                    if let Some(state) = &mut APP_STATE {
                        switch_tab(state, 0);
                    }
                }
                IDC_TAB_BTN2 => {
                    if let Some(state) = &mut APP_STATE {
                        switch_tab(state, 1);
                    }
                }
                IDC_TOGGLE_AUTOCLICK => {
                    toggle_autoclick_execution();
                }
                IDC_PRESET_10 => {
                    if let Some(state) = &mut APP_STATE {
                        update_autoclick_speed(state, 10);
                    }
                }
                IDC_PRESET_50 => {
                    if let Some(state) = &mut APP_STATE {
                        update_autoclick_speed(state, 50);
                    }
                }
                IDC_PRESET_100 => {
                    if let Some(state) = &mut APP_STATE {
                        update_autoclick_speed(state, 100);
                    }
                }
                IDC_PRESET_200 => {
                    if let Some(state) = &mut APP_STATE {
                        update_autoclick_speed(state, 200);
                    }
                }
                IDC_TEST_CLICK => {
                    if let Some(state) = &mut APP_STATE {
                        state.click_test_counter += 1;
                        let text = format!("Target Uji Klik (Total: {})", state.click_test_counter);
                        SetWindowTextW(state.h_btn_test_click, to_wide(&text).as_ptr());
                    }
                }
                IDC_TEST_RESET => {
                    if let Some(state) = &mut APP_STATE {
                        state.click_test_counter = 0;
                        SetWindowTextW(
                            state.h_btn_test_click,
                            to_wide("Target Uji Klik (Total: 0)").as_ptr(),
                        );
                    }
                }
                IDC_SAVE_BTN => {
                    if let Some(state) = &mut APP_STATE {
                        let idx1 = SendMessageW(state.h_combo_x1, CB_GETCURSEL, 0, 0) as usize;
                        let idx2 = SendMessageW(state.h_combo_x2, CB_GETCURSEL, 0, 0) as usize;
                        let idx_mid =
                            SendMessageW(state.h_combo_mid, CB_GETCURSEL, 0, 0) as usize;

                        if idx1 < Action::ALL.len() {
                            state.local_config.buttons.xbutton1 = Action::ALL[idx1];
                        }
                        if idx2 < Action::ALL.len() {
                            state.local_config.buttons.xbutton2 = Action::ALL[idx2];
                        }
                        if idx_mid < Action::ALL.len() {
                            state.local_config.buttons.middle_click = Action::ALL[idx_mid];
                        }

                        let auto_start =
                            SendMessageW(state.h_chk_autostart, BM_GETCHECK, 0, 0) == (BST_CHECKED as isize);
                        let min_tray =
                            SendMessageW(state.h_chk_tray, BM_GETCHECK, 0, 0) == (BST_CHECKED as isize);
                        let enable_gestures =
                            SendMessageW(state.h_chk_gestures, BM_GETCHECK, 0, 0) == (BST_CHECKED as isize);
                        let invert_scroll =
                            SendMessageW(state.h_chk_invert_scroll, BM_GETCHECK, 0, 0) == (BST_CHECKED as isize);
                        let speed_idx =
                            SendMessageW(state.h_combo_scroll_speed, CB_GETCURSEL, 0, 0) as usize;
                        let scroll_multiplier = match speed_idx {
                            1 => 2,
                            2 => 3,
                            _ => 1,
                        };

                        state.local_config.auto_start = auto_start;
                        state.local_config.minimize_to_tray = min_tray;
                        state.local_config.enable_gestures = enable_gestures;
                        state.local_config.invert_scroll = invert_scroll;
                        state.local_config.scroll_multiplier = scroll_multiplier;

                        let _ = configure_windows_autostart(auto_start);
                        let _ = save_config(&state.local_config);

                        if let Ok(mut cfg) = state.config.write() {
                            *cfg = state.local_config.clone();
                        }
                        crate::hook::update_hook_config(&state.local_config);

                        SetWindowTextW(
                            state.h_lbl_event,
                            to_wide("Pengaturan berhasil disimpan dan langsung aktif!").as_ptr(),
                        );
                    }
                }
                IDC_SUSPEND_BTN => {
                    if let Some(state) = &mut APP_STATE {
                        state.local_config.is_suspended = !state.local_config.is_suspended;
                        let btn_text = if state.local_config.is_suspended {
                            "Aktifkan Hook"
                        } else {
                            "Tangguhkan Hook"
                        };
                        SetWindowTextW(state.h_btn_suspend, to_wide(btn_text).as_ptr());

                        let _ = save_config(&state.local_config);
                        if let Ok(mut cfg) = state.config.write() {
                            *cfg = state.local_config.clone();
                        }
                        crate::hook::update_hook_config(&state.local_config);
                    }
                }
                IDC_CHK_TRAY => {
                    if let Some(state) = &mut APP_STATE {
                        let is_checked = SendMessageW(state.h_chk_tray, BM_GETCHECK, 0, 0) == (BST_CHECKED as isize);
                        state.local_config.minimize_to_tray = is_checked;
                    }
                }
                IDM_TRAY_RESTORE => {
                    ShowWindow(hwnd, SW_RESTORE);
                    SetForegroundWindow(hwnd);
                }
                IDM_TRAY_EXIT => {
                    KillTimer(hwnd, 1);
                    setup_tray_icon(hwnd, false);
                    DestroyWindow(hwnd);
                    timeEndPeriod(1);
                    std::process::exit(0);
                }
                _ => {}
            }
            0
        }
        WM_TRAYICON => {
            let event = lparam as u32;
            if event == WM_LBUTTONDOWN || event == WM_LBUTTONDBLCLK {
                ShowWindow(hwnd, SW_RESTORE);
                SetForegroundWindow(hwnd);
            } else if event == WM_RBUTTONUP || event == WM_CONTEXTMENU {
                let h_menu = CreatePopupMenu();
                let label_restore = to_wide("Buka OmniMouse");
                let label_exit = to_wide("Keluar / Tutup Aplikasi");
                AppendMenuW(h_menu, MF_STRING, IDM_TRAY_RESTORE, label_restore.as_ptr());
                AppendMenuW(h_menu, MF_SEPARATOR, 0, std::ptr::null());
                AppendMenuW(h_menu, MF_STRING, IDM_TRAY_EXIT, label_exit.as_ptr());

                let mut pt = POINT { x: 0, y: 0 };
                GetCursorPos(&mut pt);
                SetForegroundWindow(hwnd);
                TrackPopupMenu(
                    h_menu,
                    TPM_RIGHTBUTTON | TPM_BOTTOMALIGN,
                    pt.x,
                    pt.y,
                    0,
                    hwnd,
                    std::ptr::null(),
                );
                PostMessageW(hwnd, 0, 0, 0);
                DestroyMenu(h_menu);
            }
            0
        }
        WM_DEVICECHANGE => {
            if let Some(state) = &APP_STATE {
                let (connected, mode, badge) = crate::device::query_device_connection();
                if let Ok(mut dev) = state.device_status.lock() {
                    dev.is_connected = connected;
                    dev.connection_mode = mode;
                    dev.mode_badge = badge;
                    let status_str = format!("Status: {} ({})", dev.mode_badge, dev.connection_mode);
                    SetWindowTextW(state.h_lbl_status, to_wide(&status_str).as_ptr());
                }
            }
            0
        }
        WM_QUERYENDSESSION => 1,
        WM_ENDSESSION => {
            if wparam != 0 {
                KillTimer(hwnd, 1);
                timeEndPeriod(1);
                UnregisterHotKey(hwnd, HOTKEY_AUTOCLICK_ID);
                setup_tray_icon(hwnd, false);
                std::process::exit(0);
            }
            0
        }
        WM_CLOSE => {
            let title = to_wide("OmniMouse Companion");
            let message = to_wide(
                "Do you want to exit OmniMouse Companion?\n\n[Yes] Exit Application\n[No] Minimize to System Tray\n[Cancel] Stay in Application",
            );
            let res = MessageBoxW(
                hwnd,
                message.as_ptr(),
                title.as_ptr(),
                MB_YESNOCANCEL | MB_ICONQUESTION,
            );

            match res {
                IDYES => {
                    KillTimer(hwnd, 1);
                    timeEndPeriod(1);
                    setup_tray_icon(hwnd, false);
                    DestroyWindow(hwnd);
                    std::process::exit(0);
                }
                IDNO => {
                    ShowWindow(hwnd, SW_HIDE);
                    0
                }
                _ => 0,
            }
        }
        WM_DESTROY => {
            KillTimer(hwnd, 1);
            timeEndPeriod(1);
            UnregisterHotKey(hwnd, HOTKEY_AUTOCLICK_ID);
            setup_tray_icon(hwnd, false);
            if let Some(state) = &APP_STATE {
                DeleteObject(state.hbr_bg);
                DeleteObject(state.h_font_bold);
                DeleteObject(state.h_font_semi);
                DeleteObject(state.h_font_reg);
            }
            PostQuitMessage(0);
            std::process::exit(0);
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
