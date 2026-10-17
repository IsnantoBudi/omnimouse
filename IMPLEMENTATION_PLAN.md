# 📋 IMPLEMENTATION PLAN: Single-Instance Mutex & Dialog Konfirmasi Exit

> **Proyek**: `riot-mq10` (Gaming Mouse Utility in Pure Rust)  
> **Target Masalah**: Menghilangkan proses lingering/zombie di background saat aplikasi ditutup, mencegah penolakan terminate di Task Manager, dan mencegah proses ganda (*multiple instances*).  
> **Standar Kode**: Ponytail Mode (Pure Rust Standard Library + Direct Win32 FFI, Zero External Crates).

---

## 1. Pemahaman Masalah & Analisis Akar Masalah (Root Cause)

1. **Jendela Hanya Sembunyi (*Hidden to Tray*), Bukan Exit**:
   - Di `src/gui.rs` (`WM_CLOSE`), saat tombol silang (X) ditekan, opsi `minimize_to_tray` aktif secara default sehingga kode memanggil `ShowWindow(hwnd, SW_HIDE); return 0;`.
   - Proses tetap hidup di memori dan mouse hook tetap terpasang.
2. **Error Saat Di-Terminate Lewat Task Manager**:
   - Saat pengguna menekan "End Task", Task Manager mengirim pesan `WM_CLOSE`. Karena `WM_CLOSE` dicegat dan hanya disembunyikan, aplikasi menolak perintah penutupan dari OS sehingga Task Manager menampilkan error atau pesan *Access Denied*.
3. **Akumulasi Banyak Proses (Multiple Instances)**:
   - Karena jendela tidak terlihat di taskbar, pengguna membuka file `.exe` berulang kali. Ketiadaan *Single-Instance Mutex* menyebabkan setiap peluncuran membentuk proses baru (sempat ada 5 proses bersamaan dengan 5 hook mouse aktif paralel).

---

## 2. Validasi Standar & Arsitektur (Ponytail Mode)

- [x] **Zero External Crates**: Menggunakan Win32 API bawaan OS (`kernel32.dll`, `user32.dll`) tanpa library tambahan.
- [x] **Clean Concurrency**: Pelepasan mutex dan hook secara higienis saat aplikasi ditutup.
- [x] **Safe Termination**: Penanganan sinyal penutupan sistem (`WM_QUERYENDSESSION`, `WM_ENDSESSION`).

---

## 3. Rencana Perubahan File

| File | Perubahan yang Diperlukan |
| :--- | :--- |
| [`src/win32.rs`](file:///D:/Project/mouse%20riot%20mq-10/src/win32.rs) | Menambahkan deklarasi FFI: `CreateMutexW`, `CloseHandle`, `FindWindowW`, `SetForegroundWindow`, `MessageBoxW`, serta konstanta `MB_YESNOCANCEL`, `MB_ICONQUESTION`, `IDYES`, `IDNO`, `IDCANCEL`, `ERROR_ALREADY_EXISTS`, `WM_QUERYENDSESSION`, `WM_ENDSESSION`. |
| [`src/main.rs`](file:///D:/Project/mouse%20riot%20mq-10/src/main.rs) | Memasang *Single-Instance Mutex* di awal `main()`. Jika sudah ada instance aktif, fokuskan jendela lama ke depan lalu langsung `return` tanpa memuat hook/GUI baru. |
| [`src/gui.rs`](file:///D:/Project/mouse%20riot%20mq-10/src/gui.rs) | Mengubah `WM_CLOSE` agar menampilkan popup konfirmasi `MessageBoxW` (Yes = Keluar Tuntas, No = Sembunyi ke Tray, Cancel = Batal). Menambahkan handler `WM_QUERYENDSESSION` & `WM_ENDSESSION` agar force close/shutdown keluar seketika. |

---

## 4. Rincian Teknis Implementasi

### A. Modifikasi `src/win32.rs`
Tambahkan deklarasi FFI berikut ke dalam blok `extern "system"` dan konstanta Win32:
```rust
pub const ERROR_ALREADY_EXISTS: u32 = 183;
pub const WM_QUERYENDSESSION: u32 = 0x0011;
pub const WM_ENDSESSION: u32 = 0x0016;

pub const MB_YESNOCANCEL: u32 = 0x00000003;
pub const MB_ICONQUESTION: u32 = 0x00000020;
pub const IDYES: i32 = 6;
pub const IDNO: i32 = 7;
pub const IDCANCEL: i32 = 2;

extern "system" {
    pub fn CreateMutexW(
        lpMutexAttributes: *mut std::ffi::c_void,
        bInitialOwner: BOOL,
        lpName: *const u16,
    ) -> *mut std::ffi::c_void;

    pub fn CloseHandle(hObject: *mut std::ffi::c_void) -> BOOL;
    pub fn FindWindowW(lpClassName: *const u16, lpWindowName: *const u16) -> HWND;
    pub fn MessageBoxW(hWnd: HWND, lpText: *const u16, lpCaption: *const u16, uType: u32) -> i32;
}
```

---

### B. Modifikasi `src/main.rs` (Single-Instance Mutex)
Di awal fungsi `main()` sebelum inisialisasi:
```rust
fn main() {
    unsafe {
        win32::timeBeginPeriod(1);
    }

    // 1. Cek Single-Instance Mutex
    let mutex_name = win32::to_wide("Local\\RiotMQ10_Companion_SingleInstance");
    let h_mutex = unsafe {
        win32::CreateMutexW(std::ptr::null_mut(), win32::FALSE, mutex_name.as_ptr())
    };

    if !h_mutex.is_null() && unsafe { win32::GetLastError() } == win32::ERROR_ALREADY_EXISTS {
        // Jika sudah ada instance yang berjalan, cari jendelanya dan bawa ke depan
        unsafe {
            let class_name = win32::to_wide("RiotMQ10_Class");
            let existing_hwnd = win32::FindWindowW(class_name.as_ptr(), std::ptr::null());
            if !existing_hwnd.is_null() {
                win32::ShowWindow(existing_hwnd, win32::SW_RESTORE);
                win32::SetForegroundWindow(existing_hwnd);
            }
            win32::CloseHandle(h_mutex);
            win32::timeEndPeriod(1);
        }
        return; // Keluar seketika tanpa membuat thread/hook baru
    }

    // ... sisa kode inisialisasi config, hook, monitor, dan gui ...

    unsafe {
        if !h_mutex.is_null() {
            win32::CloseHandle(h_mutex);
        }
        win32::timeEndPeriod(1);
    }
}
```

---

### C. Modifikasi `src/gui.rs` (Dialog Konfirmasi Tombol X & OS Shutdown)
Perbarui blok `wndproc`:
```rust
        WM_QUERYENDSESSION => 1,
        WM_ENDSESSION => {
            if wparam != 0 {
                // Windows sedang shutdown atau Task Manager force terminate
                KillTimer(hwnd, 1);
                timeEndPeriod(1);
                setup_tray_icon(hwnd, false);
                std::process::exit(0);
            }
            0
        }
        WM_CLOSE => {
            let title = to_wide("Keluar dari RIOT MQ-10?");
            let msg = to_wide("Apakah Anda ingin keluar dari aplikasi sepenuhnya?\n\n• Pilih [Yes] untuk KELUAR TUNTAS (Hentikan semua proses & hook).\n• Pilih [No] untuk Sembunyi ke System Tray (Tetap aktif di latar belakang).\n• Pilih [Cancel] untuk Batal.");
            
            let res = MessageBoxW(
                hwnd,
                msg.as_ptr(),
                title.as_ptr(),
                MB_YESNOCANCEL | MB_ICONQUESTION,
            );

            match res {
                IDYES => {
                    // Keluar tuntas: bersihkan timer, tray, destroy window, exit
                    KillTimer(hwnd, 1);
                    timeEndPeriod(1);
                    setup_tray_icon(hwnd, false);
                    DestroyWindow(hwnd);
                    std::process::exit(0);
                }
                IDNO => {
                    // Sembunyi ke System Tray
                    ShowWindow(hwnd, SW_HIDE);
                    0
                }
                _ => {
                    // Cancel / abaikan penutupan
                    0
                }
            }
        }
```

---

## 5. Langkah Eksekusi (Saat Anda Melanjutkan Nanti)

1. **Terapkan Perubahan Kode**:
   - Terapkan edit pada `src/win32.rs`, `src/main.rs`, dan `src/gui.rs` sesuai rincian di atas.
2. **Kompilasi & Uji Coba**:
   ```powershell
   cargo check
   cargo test
   cargo build --release
   ```
3. **Pembersihan Proses Lama (Jika Masih Tersangkut)**:
   ```powershell
   powershell -Command "Get-Process -Name 'riot*' | Stop-Process -Force"
   ```

---

## 6. Checklist Pengujian Manual

- [ ] **Uji Tombol X - Opsi "Yes"**: Klik tombol silang (X), pilih tombol **Yes**. Periksa Task Manager untuk memastikan proses `riot-mq10-companion.exe` langsung hilang tuntas.
- [ ] **Uji Tombol X - Opsi "No"**: Klik tombol silang (X), pilih tombol **No**. Pastikan jendela sembunyi ke System Tray di dekat jam Windows dan dapat dibuka kembali dengan klik ganda ikon tray.
- [ ] **Uji Single-Instance**: Saat aplikasi sedang aktif (atau di tray), jalankan kembali file `.exe`. Pastikan tidak ada proses kedua yang tercipta, dan jendela yang ada langsung muncul ke layar depan.
- [ ] **Uji "End Task" di Task Manager**: Buka Task Manager, klik "End Task" pada aplikasi. Pastikan aplikasi dapat ditutup tanpa muncul pesan error akses ditolak.
