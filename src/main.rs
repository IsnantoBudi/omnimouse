#![cfg_attr(not(test), windows_subsystem = "windows")]
#![allow(linker_messages)]

mod autoclick;
mod config;
mod device;
mod gui;
mod hook;
mod win32;

use config::load_config;
use device::start_device_monitor;
use gui::run_gui;
use hook::start_mouse_hook;
use std::sync::{Arc, RwLock};

fn main() {
    // 1. Single-instance check via named Win32 mutex
    let mutex_name = win32::to_wide(r"Local\OmniMouse_SingleInstance");
    let mutex = unsafe {
        win32::CreateMutexW(std::ptr::null_mut(), win32::TRUE, mutex_name.as_ptr())
    };

    if !mutex.is_null() && unsafe { win32::GetLastError() } == win32::ERROR_ALREADY_EXISTS {
        // Locate existing instance window and restore focus
        let class_name = win32::to_wide("OmniMouse_Class");
        let mut existing_hwnd = unsafe {
            win32::FindWindowW(class_name.as_ptr(), std::ptr::null())
        };

        // Fallback for previous class names
        if existing_hwnd.is_null() {
            let fallback_class = win32::to_wide("RiotMQ10_Class");
            existing_hwnd = unsafe {
                win32::FindWindowW(fallback_class.as_ptr(), std::ptr::null())
            };
        }

        if existing_hwnd.is_null() {
            let fallback_class = win32::to_wide("RiotMQ10ControlClass");
            existing_hwnd = unsafe {
                win32::FindWindowW(fallback_class.as_ptr(), std::ptr::null())
            };
        }

        if !existing_hwnd.is_null() {
            unsafe {
                win32::ShowWindow(existing_hwnd, win32::SW_RESTORE);
                win32::SetForegroundWindow(existing_hwnd);
            }
        }

        unsafe {
            win32::CloseHandle(mutex);
        }
        return;
    }

    unsafe {
        win32::timeBeginPeriod(1);
    }

    std::panic::set_hook(Box::new(|info| {
        let _ = std::fs::write("crash.log", format!("PANIC: {:?}", info));
    }));

    let app_config = load_config();
    let config_arc = Arc::new(RwLock::new(app_config));

    // Start background Win32 mouse hook thread
    start_mouse_hook(Arc::clone(&config_arc));

    // Start background device monitor thread
    let device_status = start_device_monitor();

    // Run native Windows GUI
    run_gui(config_arc, device_status);

    unsafe {
        win32::timeEndPeriod(1);
        if !mutex.is_null() {
            win32::CloseHandle(mutex);
        }
    }
}
