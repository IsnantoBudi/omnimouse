use crate::win32::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceStatus {
    pub is_connected: bool,
    pub connection_mode: String,
    pub mode_badge: String,
    pub battery_info: String,
}

impl Default for DeviceStatus {
    fn default() -> Self {
        Self {
            is_connected: false,
            connection_mode: "Mendeteksi perangkat...".to_string(),
            mode_badge: "Memeriksa...".to_string(),
            battery_info: "Indikator LED / Sistem Windows".to_string(),
        }
    }
}

pub fn start_device_monitor() -> Arc<Mutex<DeviceStatus>> {
    let status = Arc::new(Mutex::new(DeviceStatus::default()));
    let status_clone = Arc::clone(&status);

    std::thread::spawn(move || {
        loop {
            let (connected, mode, badge) = query_device_connection();

            if let Ok(mut current) = status_clone.lock() {
                current.is_connected = connected;
                current.connection_mode = mode;
                current.mode_badge = badge;
            }

            std::thread::sleep(Duration::from_secs(2));
        }
    });

    status
}

#[inline]
pub fn is_bluetooth_mouse_device(instance_id: &str) -> bool {
    let upper = instance_id.to_uppercase();
    upper.starts_with("BTH") || upper.contains("BTHENUM") || upper.contains("{00001124")
}

#[inline]
pub fn is_usb_mouse_device(instance_id: &str) -> bool {
    let upper = instance_id.to_uppercase();
    upper.contains("USB") || upper.contains("VID_") || upper.starts_with("HID\\")
}

pub fn determine_connection_mode(usb_found: bool, bt_found: bool) -> (bool, String, String) {
    if usb_found && bt_found {
        (
            true,
            "Dual-Mode (USB/Dongle & Bluetooth)".to_string(),
            "Mouse Terhubung (Aktif)".to_string(),
        )
    } else if usb_found {
        (
            true,
            "USB / Wireless 2.4GHz Dongle".to_string(),
            "Mouse Terhubung (Aktif)".to_string(),
        )
    } else if bt_found {
        (
            true,
            "Bluetooth Wireless".to_string(),
            "Mouse Terhubung (Aktif)".to_string(),
        )
    } else {
        (
            false,
            "Tidak ada mouse terdeteksi".to_string(),
            "Terputus".to_string(),
        )
    }
}

pub fn query_hardware_presence() -> (bool, bool) {
    let h_dev_info = unsafe {
        SetupDiGetClassDevsW(
            &GUID_DEVINTERFACE_MOUSE,
            std::ptr::null(),
            std::ptr::null_mut(),
            DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
        )
    };

    if h_dev_info == INVALID_HANDLE_VALUE || h_dev_info.is_null() {
        // Fallback: Check SystemMetrics directly if SetupAPI fails
        let mouse_present = unsafe { GetSystemMetrics(SM_MOUSEPRESENT) } != 0;
        return (mouse_present, false);
    }

    let mut dev_info_data = SP_DEVINFO_DATA {
        cbSize: std::mem::size_of::<SP_DEVINFO_DATA>() as u32,
        ClassGuid: GUID {
            Data1: 0,
            Data2: 0,
            Data3: 0,
            Data4: [0; 8],
        },
        DevInst: 0,
        Reserved: 0,
    };

    let mut index = 0;
    let mut found_usb = false;
    let mut found_bt = false;
    let mut found_any = false;

    let mut id_buf = [0u16; 512];

    loop {
        let ok = unsafe { SetupDiEnumDeviceInfo(h_dev_info, index, &mut dev_info_data) };
        if ok == 0 {
            break;
        }
        index += 1;
        found_any = true;

        let mut req_len: u32 = 0;
        let id_ok = unsafe {
            SetupDiGetDeviceInstanceIdW(
                h_dev_info,
                &mut dev_info_data,
                id_buf.as_mut_ptr(),
                id_buf.len() as u32,
                &mut req_len,
            )
        };

        if id_ok != 0 && req_len > 1 {
            let char_count = (req_len as usize).saturating_sub(1).min(id_buf.len());
            let id_upper = String::from_utf16_lossy(&id_buf[..char_count]).to_uppercase();

            if is_bluetooth_mouse_device(&id_upper) {
                found_bt = true;
            } else if is_usb_mouse_device(&id_upper) {
                found_usb = true;
            } else {
                found_usb = true;
            }
        } else {
            found_usb = true;
        }

        if found_usb && found_bt {
            break;
        }
    }

    unsafe {
        SetupDiDestroyDeviceInfoList(h_dev_info);
    }

    // Safety fallback: If SetupAPI returned 0 devices, verify with GetSystemMetrics
    if !found_any {
        let mouse_present = unsafe { GetSystemMetrics(SM_MOUSEPRESENT) } != 0;
        if mouse_present {
            return (true, false);
        }
    }

    (found_usb, found_bt)
}

pub fn query_device_connection() -> (bool, String, String) {
    let (usb_found, bt_found) = query_hardware_presence();
    determine_connection_mode(usb_found, bt_found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_usb_mouse_device_matches_various_vendors() {
        assert!(is_usb_mouse_device("USB\\VID_3151&PID_3020\\6&26E6625&0&1"));
        assert!(is_usb_mouse_device(
            "HID\\VID_046D&PID_C539&MI_01&COL01\\8&2D030F4&0&0000"
        ));
        assert!(is_usb_mouse_device("HID\\VID_1532&PID_008A&MI_00"));
        assert!(is_usb_mouse_device(
            "HID\\ELAN06FA&COL01\\4&1846BBDB&0&0000"
        ));
        assert!(!is_usb_mouse_device(
            "BTHENUM\\{00001124-0000-1000-8000-00805F9B34FB}"
        ));
    }

    #[test]
    fn test_is_bluetooth_mouse_device_matches_patterns() {
        assert!(is_bluetooth_mouse_device(
            "BTHENUM\\{00001124-0000-1000-8000-00805F9B34FB}_LOCALMFG&0000"
        ));
        assert!(is_bluetooth_mouse_device(
            "HID\\{00001124-0000-1000-8000-00805F9B34FB}_VID&0002046D_PID&B016"
        ));
        assert!(is_bluetooth_mouse_device(
            "BTH\\MS_BTHLE\\7&2090CE7A&2&0001"
        ));
        assert!(!is_bluetooth_mouse_device(
            "USB\\VID_046D&PID_C539\\5&1C4A5A1&0&1"
        ));
        assert!(!is_bluetooth_mouse_device(""));
    }

    #[test]
    fn test_determine_connection_mode_all_combinations() {
        // Dual mode
        let (conn, mode, badge) = determine_connection_mode(true, true);
        assert!(conn);
        assert_eq!(badge, "Mouse Terhubung (Aktif)");
        assert_eq!(mode, "Dual-Mode (USB/Dongle & Bluetooth)");

        // USB only
        let (conn, mode, badge) = determine_connection_mode(true, false);
        assert!(conn);
        assert_eq!(badge, "Mouse Terhubung (Aktif)");
        assert_eq!(mode, "USB / Wireless 2.4GHz Dongle");

        // Bluetooth only
        let (conn, mode, badge) = determine_connection_mode(false, true);
        assert!(conn);
        assert_eq!(badge, "Mouse Terhubung (Aktif)");
        assert_eq!(mode, "Bluetooth Wireless");

        // Disconnected
        let (conn, mode, badge) = determine_connection_mode(false, false);
        assert!(!conn);
        assert_eq!(badge, "Terputus");
        assert_eq!(mode, "Tidak ada mouse terdeteksi");
    }

    #[test]
    fn test_default_device_status_state() {
        let dev = DeviceStatus::default();
        assert!(!dev.is_connected);
        assert_eq!(dev.mode_badge, "Memeriksa...");
        assert_eq!(dev.battery_info, "Indikator LED / Sistem Windows");
    }

    #[test]
    fn test_live_hardware_presence_query_execution() {
        // Live SetupAPI query must execute without panicking or memory faults
        let (usb, bt) = query_hardware_presence();
        let _ = (usb, bt);
    }

    #[test]
    fn test_live_query_device_connection_execution() {
        // Live query must return a valid non-empty status tuple
        let (conn, mode, badge) = query_device_connection();
        assert!(!mode.is_empty());
        assert!(!badge.is_empty());
        if conn {
            assert_eq!(badge, "Mouse Terhubung (Aktif)");
        } else {
            assert_eq!(badge, "Terputus");
        }
    }
}
