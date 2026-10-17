use crate::win32::{
    to_wide, RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegSetValueExW, HKEY_CURRENT_USER,
    KEY_SET_VALUE, REG_SZ,
};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Default,
    Copy,
    Paste,
    Cut,
    Undo,
    SelectAll,
    MediaPlayPause,
    MediaNext,
    MediaPrev,
    VolumeUp,
    VolumeDown,
    VolumeMute,
    ShowDesktop,
    TaskView,
    SnippingTool,
    CloseTab,
    ReopenTab,
    NewTab,
    Enter,
    Escape,
    AltTab,
    BrowserBack,
    BrowserForward,
    SniperPrecision,
    RapidFire,
    Disabled,
}

impl Action {
    pub const ALL: &'static [Action] = &[
        Action::Default,
        Action::Copy,
        Action::Paste,
        Action::Cut,
        Action::Undo,
        Action::SelectAll,
        Action::MediaPlayPause,
        Action::MediaNext,
        Action::MediaPrev,
        Action::VolumeUp,
        Action::VolumeDown,
        Action::VolumeMute,
        Action::ShowDesktop,
        Action::TaskView,
        Action::SnippingTool,
        Action::CloseTab,
        Action::ReopenTab,
        Action::NewTab,
        Action::Enter,
        Action::Escape,
        Action::AltTab,
        Action::BrowserBack,
        Action::BrowserForward,
        Action::SniperPrecision,
        Action::RapidFire,
        Action::Disabled,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Action::Default => "Default",
            Action::Copy => "Copy",
            Action::Paste => "Paste",
            Action::Cut => "Cut",
            Action::Undo => "Undo",
            Action::SelectAll => "SelectAll",
            Action::MediaPlayPause => "MediaPlayPause",
            Action::MediaNext => "MediaNext",
            Action::MediaPrev => "MediaPrev",
            Action::VolumeUp => "VolumeUp",
            Action::VolumeDown => "VolumeDown",
            Action::VolumeMute => "VolumeMute",
            Action::ShowDesktop => "ShowDesktop",
            Action::TaskView => "TaskView",
            Action::SnippingTool => "SnippingTool",
            Action::CloseTab => "CloseTab",
            Action::ReopenTab => "ReopenTab",
            Action::NewTab => "NewTab",
            Action::Enter => "Enter",
            Action::Escape => "Escape",
            Action::AltTab => "AltTab",
            Action::BrowserBack => "BrowserBack",
            Action::BrowserForward => "BrowserForward",
            Action::SniperPrecision => "SniperPrecision",
            Action::RapidFire => "RapidFire",
            Action::Disabled => "Disabled",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Copy" => Action::Copy,
            "Paste" => Action::Paste,
            "Cut" => Action::Cut,
            "Undo" => Action::Undo,
            "SelectAll" => Action::SelectAll,
            "MediaPlayPause" => Action::MediaPlayPause,
            "MediaNext" => Action::MediaNext,
            "MediaPrev" => Action::MediaPrev,
            "VolumeUp" => Action::VolumeUp,
            "VolumeDown" => Action::VolumeDown,
            "VolumeMute" => Action::VolumeMute,
            "ShowDesktop" => Action::ShowDesktop,
            "TaskView" => Action::TaskView,
            "SnippingTool" => Action::SnippingTool,
            "CloseTab" => Action::CloseTab,
            "ReopenTab" => Action::ReopenTab,
            "NewTab" => Action::NewTab,
            "Enter" => Action::Enter,
            "Escape" => Action::Escape,
            "AltTab" => Action::AltTab,
            "BrowserBack" => Action::BrowserBack,
            "BrowserForward" => Action::BrowserForward,
            "SniperPrecision" => Action::SniperPrecision,
            "RapidFire" => Action::RapidFire,
            "Disabled" => Action::Disabled,
            _ => Action::Default,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Action::Default => "Bawaan (Default Windows)",
            Action::Copy => "Salin (Ctrl + C)",
            Action::Paste => "Tempel (Ctrl + V)",
            Action::Cut => "Potong (Ctrl + X)",
            Action::Undo => "Undo (Ctrl + Z)",
            Action::SelectAll => "Pilih Semua (Ctrl + A)",
            Action::MediaPlayPause => "Media: Play / Pause",
            Action::MediaNext => "Media: Lagu Berikutnya",
            Action::MediaPrev => "Media: Lagu Sebelumnya",
            Action::VolumeUp => "Volume Naik",
            Action::VolumeDown => "Volume Turun",
            Action::VolumeMute => "Volume Mute",
            Action::ShowDesktop => "Tampilkan Desktop (Win + D)",
            Action::TaskView => "Task View (Win + Tab)",
            Action::SnippingTool => "Tangkapan Layar (Win + Shift + S)",
            Action::CloseTab => "Tutup Tab (Ctrl + W)",
            Action::ReopenTab => "Buka Kembali Tab (Ctrl + Shift + T)",
            Action::NewTab => "Tab Baru (Ctrl + T)",
            Action::Enter => "Tombol Enter",
            Action::Escape => "Tombol Escape",
            Action::AltTab => "Ganti Jendela (Alt + Tab)",
            Action::BrowserBack => "Browser: Back",
            Action::BrowserForward => "Browser: Forward",
            Action::SniperPrecision => "Sniper Clutch (Tahan untuk Presisi)",
            Action::RapidFire => "Rapid Fire (Auto-Clicker Kiri)",
            Action::Disabled => "Nonaktifkan Tombol",
        }
    }

    pub fn to_u8(self) -> u8 {
        Action::ALL.iter().position(|&a| a == self).unwrap_or(0) as u8
    }

    pub fn from_u8(val: u8) -> Self {
        Action::ALL.get(val as usize).copied().unwrap_or(Action::Default)
    }
}

#[derive(Debug, Clone)]
pub struct ButtonConfig {
    pub xbutton1: Action,
    pub xbutton2: Action,
    pub middle_click: Action,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            xbutton1: Action::Copy,
            xbutton2: Action::Paste,
            middle_click: Action::Default,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub buttons: ButtonConfig,
    pub is_suspended: bool,
    pub auto_start: bool,
    pub minimize_to_tray: bool,
    pub enable_gestures: bool,
    pub sniper_speed: u32,
    pub invert_scroll: bool,
    pub scroll_multiplier: u32,
    pub autoclick_interval_ms: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            buttons: ButtonConfig::default(),
            is_suspended: false,
            auto_start: false,
            minimize_to_tray: true,
            enable_gestures: true,
            sniper_speed: 3,
            invert_scroll: false,
            scroll_multiplier: 1,
            autoclick_interval_ms: crate::autoclick::DEFAULT_INTERVAL_MS,
        }
    }
}

pub fn get_config_path() -> PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        PathBuf::from(appdata).join("OmniMouse").join("config.ini")
    } else {
        PathBuf::from("config.ini")
    }
}

pub fn get_legacy_config_path() -> PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        PathBuf::from(appdata).join("RiotMQ10").join("config.ini")
    } else {
        PathBuf::from("config_legacy.ini")
    }
}

pub fn parse_config_str(content: &str) -> AppConfig {
    let mut config = AppConfig::default();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            match k.trim() {
                "xbutton1" => config.buttons.xbutton1 = Action::from_str(v.trim()),
                "xbutton2" => config.buttons.xbutton2 = Action::from_str(v.trim()),
                "middle_click" => config.buttons.middle_click = Action::from_str(v.trim()),
                "is_suspended" => config.is_suspended = v.trim() == "1" || v.trim() == "true",
                "auto_start" => config.auto_start = v.trim() == "1" || v.trim() == "true",
                "minimize_to_tray" => {
                    config.minimize_to_tray = v.trim() == "1" || v.trim() == "true"
                }
                "enable_gestures" => {
                    config.enable_gestures = v.trim() == "1" || v.trim() == "true"
                }
                "sniper_speed" => {
                    if let Ok(spd) = v.trim().parse::<u32>() {
                        config.sniper_speed = spd.clamp(1, 20);
                    }
                }
                "invert_scroll" => config.invert_scroll = v.trim() == "1" || v.trim() == "true",
                "scroll_multiplier" => {
                    if let Ok(mult) = v.trim().parse::<u32>() {
                        config.scroll_multiplier = mult.clamp(1, 5);
                    }
                }
                "autoclick_interval_ms" => {
                    if let Ok(ms) = v.trim().parse::<u32>() {
                        config.autoclick_interval_ms = crate::autoclick::clamp_interval(ms);
                    }
                }
                _ => {}
            }
        }
    }
    config
}

pub fn serialize_config(config: &AppConfig) -> String {
    format!(
        "xbutton1={}\nxbutton2={}\nmiddle_click={}\nis_suspended={}\nauto_start={}\nminimize_to_tray={}\nenable_gestures={}\nsniper_speed={}\ninvert_scroll={}\nscroll_multiplier={}\nautoclick_interval_ms={}\n",
        config.buttons.xbutton1.as_str(),
        config.buttons.xbutton2.as_str(),
        config.buttons.middle_click.as_str(),
        if config.is_suspended { "1" } else { "0" },
        if config.auto_start { "1" } else { "0" },
        if config.minimize_to_tray { "1" } else { "0" },
        if config.enable_gestures { "1" } else { "0" },
        config.sniper_speed,
        if config.invert_scroll { "1" } else { "0" },
        config.scroll_multiplier,
        config.autoclick_interval_ms,
    )
}

pub fn load_config() -> AppConfig {
    let primary = get_config_path();
    let legacy = get_legacy_config_path();
    load_config_from_paths(&primary, &legacy)
}

pub fn load_config_from_paths(primary: &std::path::Path, legacy: &std::path::Path) -> AppConfig {
    // 1. If primary exists, load it directly
    if primary.exists() {
        if let Ok(content) = fs::read_to_string(primary) {
            return parse_config_str(&content);
        }
    }

    // 2. If primary does not exist, but legacy exists, auto-migrate
    if legacy.exists() {
        if let Ok(content) = fs::read_to_string(legacy) {
            let config = parse_config_str(&content);
            if let Some(parent) = primary.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if fs::copy(legacy, primary).is_err() {
                let _ = fs::write(primary, &content);
            }
            return config;
        }
    }

    // 3. Fallback: create default and save to primary
    let config = AppConfig::default();
    let _ = save_config_to_path(&config, primary);
    config
}

pub fn save_config(config: &AppConfig) -> Result<(), std::io::Error> {
    let path = get_config_path();
    save_config_to_path(config, &path)
}

pub fn save_config_to_path(
    config: &AppConfig,
    path: &std::path::Path,
) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(path, serialize_config(config))
}

pub fn configure_windows_autostart(enable: bool) -> Result<(), String> {
    let subkey = to_wide("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
    let value_name = to_wide("OmniMouse");
    let legacy_value_name = to_wide("RiotMQ10Companion");

    unsafe {
        let mut hkey = std::ptr::null_mut();
        let res = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            subkey.as_ptr(),
            0,
            KEY_SET_VALUE,
            &mut hkey,
        );
        if res != 0 {
            return Err("Gagal membuka Registry Run key".to_string());
        }

        if enable {
            if let Ok(exe_path) = std::env::current_exe() {
                let exe_str = format!("\"{}\"", exe_path.to_string_lossy());
                let exe_wide = to_wide(&exe_str);
                let _ = RegSetValueExW(
                    hkey,
                    value_name.as_ptr(),
                    0,
                    REG_SZ,
                    exe_wide.as_ptr() as *const u8,
                    (exe_wide.len() * 2) as u32,
                );
            }
            // Clean up legacy autostart entry if present
            let _ = RegDeleteValueW(hkey, legacy_value_name.as_ptr());
        } else {
            let _ = RegDeleteValueW(hkey, value_name.as_ptr());
            let _ = RegDeleteValueW(hkey, legacy_value_name.as_ptr());
        }

        let _ = RegCloseKey(hkey);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_u8_roundtrip() {
        for &action in Action::ALL {
            let u = action.to_u8();
            let recovered = Action::from_u8(u);
            assert_eq!(action, recovered);
        }
    }

    #[test]
    fn test_action_from_invalid_u8_defaults() {
        assert_eq!(Action::from_u8(255), Action::Default);
        assert_eq!(Action::from_u8(Action::ALL.len() as u8), Action::Default);
    }

    #[test]
    fn test_action_str_roundtrip() {
        for &action in Action::ALL {
            let s = action.as_str();
            let recovered = Action::from_str(s);
            assert_eq!(action, recovered);
        }
    }

    #[test]
    fn test_action_from_str_unknown() {
        assert_eq!(Action::from_str("UnknownActionXYZ"), Action::Default);
        assert_eq!(Action::from_str(""), Action::Default);
    }

    #[test]
    fn test_action_labels_non_empty() {
        for &action in Action::ALL {
            assert!(!action.label().is_empty());
        }
    }

    #[test]
    fn test_serialize_and_parse_roundtrip_default() {
        let original = AppConfig::default();
        let serialized = serialize_config(&original);
        let parsed = parse_config_str(&serialized);

        assert_eq!(parsed.buttons.xbutton1, original.buttons.xbutton1);
        assert_eq!(parsed.buttons.xbutton2, original.buttons.xbutton2);
        assert_eq!(parsed.buttons.middle_click, original.buttons.middle_click);
        assert_eq!(parsed.is_suspended, original.is_suspended);
        assert_eq!(parsed.auto_start, original.auto_start);
        assert_eq!(parsed.minimize_to_tray, original.minimize_to_tray);
        assert_eq!(parsed.enable_gestures, original.enable_gestures);
        assert_eq!(parsed.sniper_speed, original.sniper_speed);
        assert_eq!(parsed.invert_scroll, original.invert_scroll);
        assert_eq!(parsed.scroll_multiplier, original.scroll_multiplier);
        assert_eq!(parsed.autoclick_interval_ms, original.autoclick_interval_ms);
    }

    #[test]
    fn test_serialize_and_parse_roundtrip_custom() {
        let mut original = AppConfig::default();
        original.buttons.xbutton1 = Action::SnippingTool;
        original.buttons.xbutton2 = Action::TaskView;
        original.buttons.middle_click = Action::VolumeMute;
        original.is_suspended = true;
        original.auto_start = true;
        original.minimize_to_tray = false;
        original.enable_gestures = false;
        original.sniper_speed = 7;
        original.invert_scroll = true;
        original.scroll_multiplier = 3;
        original.autoclick_interval_ms = 40;

        let serialized = serialize_config(&original);
        let parsed = parse_config_str(&serialized);

        assert_eq!(parsed.buttons.xbutton1, Action::SnippingTool);
        assert_eq!(parsed.buttons.xbutton2, Action::TaskView);
        assert_eq!(parsed.buttons.middle_click, Action::VolumeMute);
        assert_eq!(parsed.is_suspended, true);
        assert_eq!(parsed.auto_start, true);
        assert_eq!(parsed.minimize_to_tray, false);
        assert_eq!(parsed.enable_gestures, false);
        assert_eq!(parsed.sniper_speed, 7);
        assert_eq!(parsed.invert_scroll, true);
        assert_eq!(parsed.scroll_multiplier, 3);
        assert_eq!(parsed.autoclick_interval_ms, 40);
    }

    #[test]
    fn test_parse_config_comments_and_blank_lines() {
        let ini = r#"
# Konfigurasi OmniMouse
   
xbutton1=Copy
# Ini baris komentar di tengah
xbutton2=Paste
middle_click=Undo

"#;
        let parsed = parse_config_str(ini);
        assert_eq!(parsed.buttons.xbutton1, Action::Copy);
        assert_eq!(parsed.buttons.xbutton2, Action::Paste);
        assert_eq!(parsed.buttons.middle_click, Action::Undo);
    }

    #[test]
    fn test_parse_config_boolean_formats() {
        let ini1 = "is_suspended=1\nauto_start=true\nminimize_to_tray=0\nenable_gestures=false\n";
        let parsed1 = parse_config_str(ini1);
        assert!(parsed1.is_suspended);
        assert!(parsed1.auto_start);
        assert!(!parsed1.minimize_to_tray);
        assert!(!parsed1.enable_gestures);
    }

    #[test]
    fn test_parse_config_value_clamping() {
        // Clamping bounds:
        // sniper_speed: 1..=20
        // scroll_multiplier: 1..=5
        // autoclick_interval_ms: 10..=500
        let ini_low = "sniper_speed=0\nscroll_multiplier=0\nautoclick_interval_ms=1\n";
        let parsed_low = parse_config_str(ini_low);
        assert_eq!(parsed_low.sniper_speed, 1);
        assert_eq!(parsed_low.scroll_multiplier, 1);
        assert_eq!(parsed_low.autoclick_interval_ms, 10);

        let ini_high = "sniper_speed=999\nscroll_multiplier=999\nautoclick_interval_ms=9999\n";
        let parsed_high = parse_config_str(ini_high);
        assert_eq!(parsed_high.sniper_speed, 20);
        assert_eq!(parsed_high.scroll_multiplier, 5);
        assert_eq!(parsed_high.autoclick_interval_ms, 500);
    }

    #[test]
    fn test_parse_config_malformed_numbers_preserve_default() {
        let default_cfg = AppConfig::default();
        let ini_invalid = "sniper_speed=abc\nscroll_multiplier=xyz\nautoclick_interval_ms=bad\n";
        let parsed = parse_config_str(ini_invalid);
        assert_eq!(parsed.sniper_speed, default_cfg.sniper_speed);
        assert_eq!(parsed.scroll_multiplier, default_cfg.scroll_multiplier);
        assert_eq!(parsed.autoclick_interval_ms, default_cfg.autoclick_interval_ms);
    }

    #[test]
    fn test_config_path_resolution() {
        let primary = get_config_path();
        let legacy = get_legacy_config_path();
        let primary_str = primary.to_string_lossy();
        let legacy_str = legacy.to_string_lossy();

        if std::env::var("APPDATA").is_ok() {
            assert!(primary_str.contains("OmniMouse"));
            assert!(primary_str.ends_with("config.ini"));
            assert!(legacy_str.contains("RiotMQ10"));
            assert!(legacy_str.ends_with("config.ini"));
        } else {
            assert_eq!(primary, PathBuf::from("config.ini"));
            assert_eq!(legacy, PathBuf::from("config_legacy.ini"));
        }
    }

    #[test]
    fn test_config_migration_from_legacy_to_primary() {
        let temp_dir =
            std::env::temp_dir().join(format!("omnimouse_test_mig_{}", std::process::id()));
        let _ = fs::create_dir_all(&temp_dir);

        let legacy_path = temp_dir.join("RiotMQ10").join("config.ini");
        let primary_path = temp_dir.join("OmniMouse").join("config.ini");

        let _ = fs::remove_file(&primary_path);
        let _ = fs::remove_file(&legacy_path);
        if let Some(p) = legacy_path.parent() {
            let _ = fs::create_dir_all(p);
        }

        let custom_legacy =
            "xbutton1=VolumeUp\nxbutton2=VolumeDown\nsniper_speed=8\nautoclick_interval_ms=45\n";
        fs::write(&legacy_path, custom_legacy).unwrap();

        assert!(!primary_path.exists());
        assert!(legacy_path.exists());

        // Load config with migration
        let loaded = load_config_from_paths(&primary_path, &legacy_path);

        assert_eq!(loaded.buttons.xbutton1, Action::VolumeUp);
        assert_eq!(loaded.buttons.xbutton2, Action::VolumeDown);
        assert_eq!(loaded.sniper_speed, 8);
        assert_eq!(loaded.autoclick_interval_ms, 45);

        // Verify primary file now exists with migrated content
        assert!(primary_path.exists());
        let migrated_content = fs::read_to_string(&primary_path).unwrap();
        assert!(migrated_content.contains("xbutton1=VolumeUp"));
        assert!(migrated_content.contains("sniper_speed=8"));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_config_primary_takes_precedence_over_legacy() {
        let temp_dir =
            std::env::temp_dir().join(format!("omnimouse_test_prec_{}", std::process::id()));
        let _ = fs::create_dir_all(&temp_dir);

        let legacy_path = temp_dir.join("legacy.ini");
        let primary_path = temp_dir.join("primary.ini");

        let legacy_content = "sniper_speed=2\n";
        let primary_content = "sniper_speed=18\n";

        fs::write(&legacy_path, legacy_content).unwrap();
        fs::write(&primary_path, primary_content).unwrap();

        let loaded = load_config_from_paths(&primary_path, &legacy_path);
        assert_eq!(loaded.sniper_speed, 18);

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
