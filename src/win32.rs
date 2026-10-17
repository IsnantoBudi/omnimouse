#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

pub type HWND = *mut std::ffi::c_void;
pub type HINSTANCE = *mut std::ffi::c_void;
pub type HHOOK = *mut std::ffi::c_void;
pub type HICON = *mut std::ffi::c_void;
pub type HCURSOR = *mut std::ffi::c_void;
pub type HBRUSH = *mut std::ffi::c_void;
pub type HMENU = *mut std::ffi::c_void;
pub type HKEY = *mut std::ffi::c_void;
pub type HDC = *mut std::ffi::c_void;
pub type HFONT = *mut std::ffi::c_void;
pub type HDEVINFO = *mut std::ffi::c_void;
pub type COLORREF = u32;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type BOOL = i32;
pub type HANDLE = *mut std::ffi::c_void;

pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;
pub const INVALID_HANDLE_VALUE: *mut std::ffi::c_void = -1isize as *mut std::ffi::c_void;
pub const ERROR_ALREADY_EXISTS: u32 = 183;

pub const WM_DEVICECHANGE: u32 = 0x0219;
pub const DBT_DEVICEARRIVAL: usize = 0x8000;
pub const DBT_DEVICEREMOVECOMPLETE: usize = 0x8004;
pub const DBT_DEVNODES_CHANGED: usize = 0x0007;

pub const DIGCF_DEFAULT: u32 = 0x00000001;
pub const DIGCF_PRESENT: u32 = 0x00000002;
pub const DIGCF_ALLCLASSES: u32 = 0x00000004;
pub const DIGCF_DEVICEINTERFACE: u32 = 0x00000010;
pub const SPDRP_FRIENDLYNAME: u32 = 0x0000000C;

pub const SM_MOUSEPRESENT: i32 = 19;
pub const SM_CMOUSEBUTTONS: i32 = 43;

pub const WH_MOUSE_LL: i32 = 14;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_QUERYENDSESSION: u32 = 0x0011;
pub const WM_ENDSESSION: u32 = 0x0016;
pub const WM_COMMAND: u32 = 0x0111;
pub const WM_TIMER: u32 = 0x0113;
pub const WM_HSCROLL: u32 = 0x0114;
pub const WM_SETFONT: u32 = 0x0030;
pub type TIMERPROC = Option<unsafe extern "system" fn(HWND, u32, usize, u32)>;
pub const WM_CTLCOLORDLG: u32 = 0x0136;
pub const WM_CTLCOLORSTATIC: u32 = 0x0138;
pub const WM_CTLCOLORBTN: u32 = 0x0135;
pub const WM_USER: u32 = 0x0400;
pub const WM_APP: u32 = 0x8000;
pub const WM_TRAYICON: u32 = WM_USER + 100;
pub const WM_SETICON: u32 = 0x0080;
pub const ICON_SMALL: usize = 0;
pub const ICON_BIG: usize = 1;

pub const TBM_GETPOS: u32 = WM_USER;
pub const TBM_SETPOS: u32 = WM_USER + 5;
pub const TBM_SETRANGE: u32 = WM_USER + 6;
pub const TBM_SETPAGESIZE: u32 = WM_USER + 22;
pub const TBS_AUTOTICKS: u32 = 0x0001;

pub const ICC_BAR_CLASSES: u32 = 0x00000004;

#[repr(C)]
pub struct INITCOMMONCONTROLSEX {
    pub dwSize: u32,
    pub dwICC: u32,
}

pub const DEFAULT_CHARSET: u32 = 1;
pub const OUT_DEFAULT_PRECIS: u32 = 0;
pub const CLIP_DEFAULT_PRECIS: u32 = 0;
pub const CLEARTYPE_QUALITY: u32 = 5;
pub const DEFAULT_PITCH: u32 = 0;
pub const FF_DONTCARE: u32 = 0;
pub const FW_NORMAL: i32 = 400;
pub const FW_SEMIBOLD: i32 = 600;
pub const FW_BOLD: i32 = 700;
pub const TRANSPARENT: i32 = 1;
pub const OPAQUE: i32 = 2;

#[inline]
pub fn RGB(r: u8, g: u8, b: u8) -> COLORREF {
    (r as u32) | ((g as u32) << 8) | ((b as u32) << 16)
}

pub const WM_CONTEXTMENU: u32 = 0x007B;
pub const WM_MOUSEMOVE: u32 = 0x0200;
pub const WM_LBUTTONDOWN: u32 = 0x0201;
pub const WM_LBUTTONDBLCLK: u32 = 0x0203;
pub const WM_RBUTTONUP: u32 = 0x0205;
pub const WM_MBUTTONDOWN: u32 = 0x0207;
pub const WM_MBUTTONUP: u32 = 0x0208;
pub const WM_MOUSEWHEEL: u32 = 0x020A;
pub const WM_XBUTTONDOWN: u32 = 0x020B;
pub const WM_XBUTTONUP: u32 = 0x020C;
pub const WM_MOUSEHWHEEL: u32 = 0x020E;

pub const OMNIMOUSE_INJECTED_SIGNATURE: usize = 0x4F4D4E49; // 'OMNI' in ASCII
pub const RIOT_INJECTED_SIGNATURE: usize = 0x52494F54; // 'RIOT' in ASCII (legacy)
pub const THREAD_PRIORITY_HIGHEST: i32 = 2;

pub const MF_STRING: u32 = 0x0000;
pub const MF_SEPARATOR: u32 = 0x0800;
pub const TPM_RIGHTBUTTON: u32 = 0x0002;
pub const TPM_BOTTOMALIGN: u32 = 0x0020;

pub const SPI_GETMOUSESPEED: u32 = 0x0070;
pub const SPI_SETMOUSESPEED: u32 = 0x0071;
pub const SPIF_SENDCHANGE: u32 = 0x02;

pub const MOUSEEVENTF_LEFTDOWN: u32 = 0x0002;
pub const MOUSEEVENTF_LEFTUP: u32 = 0x0004;
pub const MOUSEEVENTF_HWHEEL: u32 = 0x1000;
pub const MOUSEEVENTF_WHEEL: u32 = 0x0800;

pub const WS_OVERLAPPEDWINDOW: u32 = 0x00CF0000;
pub const WS_VISIBLE: u32 = 0x10000000;
pub const WS_CHILD: u32 = 0x40000000;
pub const WS_TABSTOP: u32 = 0x00010000;

pub const BS_DEFPUSHBUTTON: u32 = 0x00000001;
pub const BS_AUTOCHECKBOX: u32 = 0x00000003;
pub const CBS_DROPDOWNLIST: u32 = 0x00000003;

pub const CB_ADDSTRING: u32 = 0x0143;
pub const CB_SETCURSEL: u32 = 0x014E;
pub const CB_GETCURSEL: u32 = 0x0147;
pub const BM_GETCHECK: u32 = 0x00F0;
pub const BM_SETCHECK: u32 = 0x00F1;
pub const BST_CHECKED: usize = 1;
pub const BST_UNCHECKED: usize = 0;

pub const SW_HIDE: i32 = 0;
pub const SW_SHOW: i32 = 5;
pub const SW_RESTORE: i32 = 9;

pub const MB_YESNOCANCEL: u32 = 0x00000003;
pub const MB_ICONQUESTION: u32 = 0x00000020;
pub const IDOK: i32 = 1;
pub const IDCANCEL: i32 = 2;
pub const IDYES: i32 = 6;
pub const IDNO: i32 = 7;

pub const NIM_ADD: u32 = 0x00000000;
pub const NIM_MODIFY: u32 = 0x00000001;
pub const NIM_DELETE: u32 = 0x00000002;

pub const NIF_MESSAGE: u32 = 0x00000001;
pub const NIF_ICON: u32 = 0x00000002;
pub const NIF_TIP: u32 = 0x00000004;

pub const INPUT_MOUSE: u32 = 0;
pub const INPUT_KEYBOARD: u32 = 1;
pub const KEYEVENTF_KEYUP: u32 = 0x0002;
pub const KEYEVENTF_EXTENDEDKEY: u32 = 0x0001;
pub const WM_HOTKEY: u32 = 0x0312;
pub const VK_F6: u16 = 0x75;

pub const VK_SHIFT: u16 = 0x10;
pub const VK_CONTROL: u16 = 0x11;
pub const VK_MENU: u16 = 0x12; // Alt
pub const VK_RETURN: u16 = 0x0D;
pub const VK_ESCAPE: u16 = 0x1B;
pub const VK_TAB: u16 = 0x09;
pub const VK_LWIN: u16 = 0x5B;
pub const VK_A: u16 = 0x41;
pub const VK_C: u16 = 0x43;
pub const VK_D: u16 = 0x44;
pub const VK_S: u16 = 0x53;
pub const VK_T: u16 = 0x54;
pub const VK_V: u16 = 0x56;
pub const VK_W: u16 = 0x57;
pub const VK_X: u16 = 0x58;
pub const VK_Y: u16 = 0x59;
pub const VK_Z: u16 = 0x5A;

pub const VK_VOLUME_MUTE: u16 = 0xAD;
pub const VK_VOLUME_DOWN: u16 = 0xAE;
pub const VK_VOLUME_UP: u16 = 0xAF;
pub const VK_MEDIA_NEXT_TRACK: u16 = 0xB0;
pub const VK_MEDIA_PREV_TRACK: u16 = 0xB1;
pub const VK_MEDIA_PLAY_PAUSE: u16 = 0xB3;
pub const VK_BROWSER_BACK: u16 = 0xA6;
pub const VK_BROWSER_FORWARD: u16 = 0xA7;

pub const HKEY_CURRENT_USER: HKEY = 0x80000001usize as HKEY;
pub const KEY_SET_VALUE: u32 = 0x0002;
pub const REG_SZ: u32 = 1;

pub const DWMWA_USE_IMMERSIVE_DARK_MODE: u32 = 20;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MSLLHOOKSTRUCT {
    pub pt: POINT,
    pub mouseData: u32,
    pub flags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct WNDCLASSEXW {
    pub cbSize: u32,
    pub style: u32,
    pub lpfnWndProc: Option<unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT>,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: *const u16,
    pub lpszClassName: *const u16,
    pub hIconSm: HICON,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: HICON,
    pub szTip: [u16; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u16; 256],
    pub uTimeoutOrVersion: u32,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: u32,
    pub guidItem: [u8; 16],
    pub hBalloonIcon: HICON,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct KEYBDINPUT {
    pub wVk: u16,
    pub wScan: u16,
    pub dwFlags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MOUSEINPUT {
    pub dx: i32,
    pub dy: i32,
    pub mouseData: u32,
    pub dwFlags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union INPUT_UNION {
    pub mi: MOUSEINPUT,
    pub ki: KEYBDINPUT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct INPUT {
    pub r#type: u32,
    pub _pad: u32,
    pub u: INPUT_UNION,
}

const _: () = assert!(std::mem::size_of::<INPUT>() == 40);

pub unsafe fn send_left_click(down: bool) {
    let flag = if down {
        MOUSEEVENTF_LEFTDOWN
    } else {
        MOUSEEVENTF_LEFTUP
    };
    let mut input: INPUT = std::mem::zeroed();
    input.r#type = INPUT_MOUSE;
    input.u.mi = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: flag,
        time: 0,
        dwExtraInfo: OMNIMOUSE_INJECTED_SIGNATURE,
    };
    SendInput(1, &input, std::mem::size_of::<INPUT>() as i32);
}

pub type HOOKPROC = Option<unsafe extern "system" fn(i32, WPARAM, LPARAM) -> LRESULT>;

#[link(name = "kernel32")]
extern "system" {
    pub fn GetModuleHandleW(lpModuleName: *const u16) -> HINSTANCE;
    pub fn GetLastError() -> u32;
    pub fn GetCurrentThread() -> *mut std::ffi::c_void;
    pub fn SetThreadPriority(hThread: *mut std::ffi::c_void, nPriority: i32) -> BOOL;
    pub fn CreateMutexW(
        lpMutexAttributes: *mut std::ffi::c_void,
        bInitialOwner: BOOL,
        lpName: *const u16,
    ) -> HANDLE;
    pub fn CloseHandle(hObject: HANDLE) -> BOOL;
}

#[link(name = "winmm")]
extern "system" {
    pub fn timeBeginPeriod(uPeriod: u32) -> u32;
    pub fn timeEndPeriod(uPeriod: u32) -> u32;
}

#[link(name = "user32")]
extern "system" {
    pub fn GetSystemMetrics(nIndex: i32) -> i32;
    pub fn FindWindowW(lpClassName: *const u16, lpWindowName: *const u16) -> HWND;
    pub fn MessageBoxW(
        hWnd: HWND,
        lpText: *const u16,
        lpCaption: *const u16,
        uType: u32,
    ) -> i32;
    pub fn SetWindowsHookExW(idHook: i32, lpfn: HOOKPROC, hmod: HINSTANCE, dwThreadId: u32) -> HHOOK;
    pub fn UnhookWindowsHookEx(hhk: HHOOK) -> BOOL;
    pub fn CallNextHookEx(hhk: HHOOK, nCode: i32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn SendInput(cInputs: u32, pInputs: *const INPUT, cbSize: i32) -> u32;
    pub fn GetMessageW(lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: u32, wMsgFilterMax: u32) -> BOOL;
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
    pub fn PostQuitMessage(nExitCode: i32);
    pub fn DefWindowProcW(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> u16;
    pub fn CreateWindowExW(
        dwExStyle: u32,
        lpClassName: *const u16,
        lpWindowName: *const u16,
        dwStyle: u32,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: *mut std::ffi::c_void,
    ) -> HWND;
    pub fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> BOOL;
    pub fn UpdateWindow(hWnd: HWND) -> BOOL;
    pub fn SetForegroundWindow(hWnd: HWND) -> BOOL;
    pub fn SendMessageW(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn SetWindowTextW(hWnd: HWND, lpString: *const u16) -> BOOL;
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: *const u16) -> HCURSOR;
    pub fn LoadIconW(hInstance: HINSTANCE, lpIconName: *const u16) -> HICON;
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    pub fn SystemParametersInfoW(
        uiAction: u32,
        uiParam: u32,
        pvParam: *mut std::ffi::c_void,
        fWinIni: u32,
    ) -> BOOL;
    pub fn mouse_event(dwFlags: u32, dx: u32, dy: u32, dwData: u32, dwExtraInfo: usize);
    pub fn RegisterHotKey(hWnd: HWND, id: i32, fsModifiers: u32, vk: u32) -> BOOL;
    pub fn UnregisterHotKey(hWnd: HWND, id: i32) -> BOOL;
    pub fn CreatePopupMenu() -> HMENU;
    pub fn AppendMenuW(hMenu: HMENU, uFlags: u32, uIDNewItem: usize, lpNewItem: *const u16) -> BOOL;
    pub fn TrackPopupMenu(
        hMenu: HMENU,
        uFlags: u32,
        x: i32,
        y: i32,
        nReserved: i32,
        hWnd: HWND,
        prcRect: *const std::ffi::c_void,
    ) -> BOOL;
    pub fn DestroyMenu(hMenu: HMENU) -> BOOL;
    pub fn GetCursorPos(lpPoint: *mut POINT) -> BOOL;
    pub fn PostMessageW(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> BOOL;
    pub fn SetTimer(hWnd: HWND, nIDEvent: usize, uElapse: u32, lpTimerFunc: TIMERPROC) -> usize;
    pub fn KillTimer(hWnd: HWND, uIDEvent: usize) -> BOOL;
}

#[link(name = "shell32")]
extern "system" {
    pub fn Shell_NotifyIconW(dwMessage: u32, lpData: *mut NOTIFYICONDATAW) -> BOOL;
}

#[link(name = "comctl32")]
extern "system" {
    pub fn InitCommonControlsEx(picce: *const INITCOMMONCONTROLSEX) -> BOOL;
}

#[link(name = "dwmapi")]
extern "system" {
    pub fn DwmSetWindowAttribute(
        hwnd: HWND,
        dwAttribute: u32,
        pvAttribute: *const std::ffi::c_void,
        cbAttribute: u32,
    ) -> i32;
}

#[link(name = "advapi32")]
extern "system" {
    pub fn RegOpenKeyExW(
        hKey: HKEY,
        lpSubKey: *const u16,
        ulOptions: u32,
        samDesired: u32,
        phkResult: *mut HKEY,
    ) -> i32;
    pub fn RegSetValueExW(
        hKey: HKEY,
        lpValueName: *const u16,
        Reserved: u32,
        dwType: u32,
        lpData: *const u8,
        cbData: u32,
    ) -> i32;
    pub fn RegDeleteValueW(hKey: HKEY, lpValueName: *const u16) -> i32;
    pub fn RegCloseKey(hKey: HKEY) -> i32;
}

#[link(name = "gdi32")]
extern "system" {
    pub fn CreateFontW(
        cHeight: i32,
        cWidth: i32,
        cEscapement: i32,
        cOrientation: i32,
        cWeight: i32,
        bItalic: u32,
        bUnderline: u32,
        bStrikeOut: u32,
        iCharSet: u32,
        iOutPrecision: u32,
        iClipPrecision: u32,
        iQuality: u32,
        iPitchAndFamily: u32,
        pszFaceName: *const u16,
    ) -> HFONT;
    pub fn CreateSolidBrush(color: COLORREF) -> HBRUSH;
    pub fn SetTextColor(hdc: HDC, color: COLORREF) -> COLORREF;
    pub fn SetBkColor(hdc: HDC, color: COLORREF) -> COLORREF;
    pub fn SetBkMode(hdc: HDC, mode: i32) -> i32;
    pub fn DeleteObject(ho: *mut std::ffi::c_void) -> BOOL;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GUID {
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8],
}

// Device Interface Class for Mouse: {378DE44C-56EF-11D1-BC8C-00A0C91405DD}
pub const GUID_DEVINTERFACE_MOUSE: GUID = GUID {
    Data1: 0x378DE44C,
    Data2: 0x56EF,
    Data3: 0x11D1,
    Data4: [0xBC, 0x8C, 0x00, 0xA0, 0xC9, 0x14, 0x05, 0xDD],
};

#[repr(C)]
pub struct SP_DEVINFO_DATA {
    pub cbSize: u32,
    pub ClassGuid: GUID,
    pub DevInst: u32,
    pub Reserved: usize,
}

#[link(name = "setupapi")]
extern "system" {
    pub fn SetupDiGetClassDevsW(
        ClassGuid: *const GUID,
        Enumerator: *const u16,
        hwndParent: HWND,
        Flags: u32,
    ) -> HDEVINFO;

    pub fn SetupDiEnumDeviceInfo(
        DeviceInfoSet: HDEVINFO,
        MemberIndex: u32,
        DeviceInfoData: *mut SP_DEVINFO_DATA,
    ) -> BOOL;

    pub fn SetupDiGetDeviceRegistryPropertyW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: *mut SP_DEVINFO_DATA,
        Property: u32,
        PropertyRegDataType: *mut u32,
        PropertyBuffer: *mut u8,
        PropertyBufferSize: u32,
        RequiredSize: *mut u32,
    ) -> BOOL;

    pub fn SetupDiGetDeviceInstanceIdW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: *mut SP_DEVINFO_DATA,
        DeviceInstanceId: *mut u16,
        DeviceInstanceIdSize: u32,
        RequiredSize: *mut u32,
    ) -> BOOL;

    pub fn SetupDiDestroyDeviceInfoList(DeviceInfoSet: HDEVINFO) -> BOOL;
}

pub fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_wide_empty_string() {
        let wide = to_wide("");
        assert_eq!(wide, vec![0]);
    }

    #[test]
    fn test_to_wide_ascii() {
        let wide = to_wide("OMNI");
        assert_eq!(wide, vec!['O' as u16, 'M' as u16, 'N' as u16, 'I' as u16, 0]);
        assert_eq!(wide.len(), 5);
        assert_eq!(*wide.last().unwrap(), 0);
    }

    #[test]
    fn test_to_wide_unicode_and_symbols() {
        let wide = to_wide("Tombol ▶ ⏹ Daya: 100%");
        assert_eq!(*wide.last().unwrap(), 0);
        let back: String = String::from_utf16(&wide[..wide.len() - 1]).unwrap();
        assert_eq!(back, "Tombol ▶ ⏹ Daya: 100%");
    }

    #[test]
    fn test_rgb_macro() {
        assert_eq!(RGB(0, 0, 0), 0x00000000);
        assert_eq!(RGB(255, 0, 0), 0x000000FF);
        assert_eq!(RGB(0, 255, 0), 0x0000FF00);
        assert_eq!(RGB(0, 0, 255), 0x00FF0000);
        assert_eq!(RGB(24, 24, 27), 24 | (24 << 8) | (27 << 16));
    }

    #[test]
    fn test_input_size_assertion() {
        assert_eq!(std::mem::size_of::<INPUT>(), 40);
    }

    #[test]
    fn test_win32_constants_values() {
        assert_eq!(ERROR_ALREADY_EXISTS, 183);
        assert_eq!(SW_HIDE, 0);
        assert_eq!(SW_RESTORE, 9);
        assert_eq!(MB_YESNOCANCEL, 0x00000003);
        assert_eq!(MB_ICONQUESTION, 0x00000020);
        assert_eq!(IDOK, 1);
        assert_eq!(IDCANCEL, 2);
        assert_eq!(IDYES, 6);
        assert_eq!(IDNO, 7);
        assert_eq!(WM_QUERYENDSESSION, 0x0011);
        assert_eq!(WM_ENDSESSION, 0x0016);
        assert_eq!(DIGCF_DEVICEINTERFACE, 0x00000010);
        assert_eq!(SM_MOUSEPRESENT, 19);
        assert_eq!(SM_CMOUSEBUTTONS, 43);
    }

    #[test]
    fn test_omnimouse_signature_constant() {
        assert_eq!(OMNIMOUSE_INJECTED_SIGNATURE, 0x4F4D4E49);
        assert_eq!(RIOT_INJECTED_SIGNATURE, 0x52494F54);
    }

    #[test]
    fn test_guid_devinterface_mouse() {
        assert_eq!(GUID_DEVINTERFACE_MOUSE.Data1, 0x378DE44C);
        assert_eq!(GUID_DEVINTERFACE_MOUSE.Data2, 0x56EF);
        assert_eq!(GUID_DEVINTERFACE_MOUSE.Data3, 0x11D1);
        assert_eq!(
            GUID_DEVINTERFACE_MOUSE.Data4,
            [0xBC, 0x8C, 0x00, 0xA0, 0xC9, 0x14, 0x05, 0xDD]
        );
    }

    #[test]
    fn test_mutex_creation_and_already_exists() {
        let name = to_wide(r"Local\Test_OmniMouse_WorkerR3_Unique");
        unsafe {
            let m1 = CreateMutexW(std::ptr::null_mut(), TRUE, name.as_ptr());
            assert!(!m1.is_null());

            let m2 = CreateMutexW(std::ptr::null_mut(), TRUE, name.as_ptr());
            assert!(!m2.is_null());
            assert_eq!(GetLastError(), ERROR_ALREADY_EXISTS);

            assert_ne!(CloseHandle(m2), 0);
            assert_ne!(CloseHandle(m1), 0);
        }
    }

    #[test]
    fn test_find_window_nonexistent() {
        let class = to_wide("NonExistentClassXYZ12345");
        unsafe {
            let hwnd = FindWindowW(class.as_ptr(), std::ptr::null());
            assert!(hwnd.is_null());
        }
    }
}

