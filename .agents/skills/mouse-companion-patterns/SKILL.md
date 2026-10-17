---
name: mouse-companion-patterns
description: Taksonomi lengkap, arsitektur teknis, dan pola implementasi companion software mouse native Windows (Win32, Rust, Logitech G HUB patterns, chorded gestures, and low-level mouse hooks).
---

# Mouse Companion Software Patterns & Architecture

Panduan arsitektur dan pola implementasi untuk membangun perangkat lunak pendamping (*companion software*) mouse native Windows berkinerja tinggi (Zero Webview, Pure Win32, Super Ringan < 30 MB RAM).

---

## 1. Taksonomi 9 Domain Fitur Mouse Modern

1. **Button Remapping & Keystrokes**:
   * Single keys, modifiers (`Ctrl`, `Shift`, `Alt`, `Win`), dan key combinations (`Ctrl+C`, `Ctrl+V`, `Win+Shift+S`, `Win+D`, `Win+Tab`).
   * Multimedia actions (`VK_VOLUME_*`, `VK_MEDIA_*`).
   * Browser actions (`VK_BROWSER_BACK`, `VK_BROWSER_FORWARD`, `Ctrl+W`, `Ctrl+Shift+T`).

2. **Chorded Gestures & Multi-Layer Inputs**:
   * Menahan tombol samping (X1/X2) mengaktifkan fungsi alternatif pada scroll wheel atau tombol lainnya.
   * *Contoh*: Tahan X1 + Scroll = Volume Up/Down; Tahan X1 + Middle Click = Mute; Tahan X2 + Scroll = Horizontal Scroll.
   * *Short Press vs Chord*: Terapkan flag `DID_CHORD`. Jika pengguna hanya mengetuk tombol samping tanpa memutar scroll, eksekusi aksi reguler saat `WM_XBUTTONUP`.

3. **Sensor, DPI & Kalibrasi**:
   * Multi-stage DPI switcher.
   * **Sniper Clutch Mode**: Menurunkan kecepatan pointer Windows secara instan via `SystemParametersInfoW(SPI_SETMOUSESPEED)` saat tombol ditahan, lalu memulihkannya saat tombol dilepas.
   * Polling rate selection (125Hz, 250Hz, 500Hz, 1000Hz).

4. **Scroll Wheel & Cursor Enhancements**:
   * Smooth inertia scrolling.
   * Natural / inverted scrolling direction.
   * Horizontal scroll emulation via `mouse_event(MOUSEEVENTF_HWHEEL)`.

5. **Macro Engine**:
   * Keystroke and mouse click recording dengan timestamp delay.
   * Play modes: One-shot, Repeat while holding, Toggle loop.
   * Rapid-fire / Auto-clicker.

6. **Per-Application Profiles**:
   * Auto-switch profile dengan mendeteksi active foreground window via `GetForegroundWindow()`.

7. **On-Screen Display (OSD)**:
   * Floating toast overlay saat pergantian DPI/profil/sniper mode.

8. **RGB Lighting & Aesthetics**:
   * Efek pencahayaan (Static, Breathing, Wave, Reactive).
   * Status baterai via indikator warna LED.

9. **Power Management & Dual-Mode Connection**:
   * Deteksi hardware real-time via PnP device ID (2.4GHz USB Dongle vs Bluetooth Classic HID vs Wired Type-C).
   * Sleep timeout timer.

---

## 2. Pola Kritis Win32 & Rust Native

### A. Penyelarasan Memori `INPUT` Struct (x64)
Windows kernel menolak `SendInput` dengan `ERROR_INVALID_PARAMETER` (87) jika struct `INPUT` tidak berukuran tepat 40 byte pada x64:
```rust
#[repr(C)]
pub struct KEYBDINPUT {
    pub wVk: u16,
    pub wScan: u16,
    pub dwFlags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}

#[repr(C)]
pub struct INPUT {
    pub r#type: u32,
    pub _pad: u32,        // 32-bit padding alignment
    pub ki: KEYBDINPUT,   // 24 byte union
    pub _pad2: [u8; 8],   // total tepat 40 byte
}
const _: () = assert!(std::mem::size_of::<INPUT>() == 40);
```

### B. Registrasi Low-Level Mouse Hook (`WH_MOUSE_LL`)
* Wajib gunakan `GetModuleHandleW(std::ptr::null())` sebagai parameter `hMod`. Melewatkan null menghasilkan Win32 Error `1428` (`ERROR_HOOK_NEEDS_HMOD`).
* Abaikan event sintetis yang diinjeksi software: `if (info.flags & LLMHF_INJECTED) != 0 { return CallNextHookEx(...); }`.

### C. Resource Icon PE Header via MinGW
* Script `app.rc`: `1 ICON "assets/icon.ico"`.
* Kompilasi COFF: `windres -i app.rc -O coff -o assets/app.res`.
* Link di `.cargo/config.toml`: `rustflags = ["-C", "link-arg=...app.res"]` dengan linker MinGW `gcc.exe`.
