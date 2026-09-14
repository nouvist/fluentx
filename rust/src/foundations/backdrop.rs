use std::{ffi::c_void, mem, sync::OnceLock};

use flutter_rust_bridge::frb;
#[cfg(windows)]
use windows::{
    core::BOOL,
    Win32::{
        Foundation::{HWND, LPARAM},
        Graphics::Dwm::{
            DwmExtendFrameIntoClientArea, DwmSetWindowAttribute, DWMSBT_MAINWINDOW, DWMSBT_NONE,
            DWMSBT_TABBEDWINDOW, DWMWA_SYSTEMBACKDROP_TYPE, DWM_SYSTEMBACKDROP_TYPE,
        },
        System::Threading::GetCurrentProcessId,
        UI::{
            Controls::MARGINS,
            WindowsAndMessaging::{EnumWindows, GetWindowThreadProcessId},
        },
    },
};

#[frb(opaque)]
pub struct FluentxNativeBackdrop;

impl FluentxNativeBackdrop {
    #[cfg(windows)]
    fn window() -> HWND {
        static INSTANCE: OnceLock<usize> = OnceLock::new();
        let instance = INSTANCE.get_or_init(|| {
            struct Data {
                pid: u32,
                hwnd: HWND,
            }

            let mut lparam = Data {
                pid: unsafe { GetCurrentProcessId() },
                hwnd: HWND::default(),
            };

            unsafe extern "system" fn callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
                let data = &mut *(lparam.0 as *mut Data);

                let mut pid: u32 = 0;
                GetWindowThreadProcessId(hwnd, Some(&mut pid));
                if pid == data.pid {
                    data.hwnd = hwnd;
                    return BOOL(0);
                }

                BOOL(1)
            }

            _ = unsafe { EnumWindows(Some(callback), LPARAM(&mut lparam as *mut _ as isize)) };

            lparam.hwnd.0 as usize
        });

        HWND(*instance as *mut c_void)
    }

    #[frb(sync)]
    #[cfg(windows)]
    pub fn is_native() -> bool {
        true
    }

    #[frb(sync)]
    #[cfg(windows)]
    pub fn extend() {
        _ = unsafe {
            DwmExtendFrameIntoClientArea(
                Self::window(),
                &MARGINS {
                    cxLeftWidth: -1,
                    cxRightWidth: -1,
                    cyTopHeight: -1,
                    cyBottomHeight: -1,
                },
            )
        };
    }

    #[frb(sync)]
    #[cfg(windows)]
    pub fn none() {
        _ = unsafe {
            DwmSetWindowAttribute(
                Self::window(),
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_NONE as *const _ as *const c_void,
                mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
            )
        };
    }

    #[frb(sync)]
    #[cfg(windows)]
    pub fn mica() {
        _ = unsafe {
            DwmSetWindowAttribute(
                Self::window(),
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_MAINWINDOW as *const _ as *const c_void,
                mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
            )
        };
    }

    #[frb(sync)]
    #[cfg(windows)]
    pub fn tabbed() {
        _ = unsafe {
            DwmSetWindowAttribute(
                Self::window(),
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_TABBEDWINDOW as *const _ as *const c_void,
                mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
            )
        };
    }

    #[frb(sync)]
    #[cfg(not(windows))]
    pub fn is_native() -> bool {
        false
    }

    #[frb(sync)]
    #[cfg(not(windows))]
    pub fn extend() {}

    #[frb(sync)]
    #[cfg(not(windows))]
    pub fn none() {}

    #[frb(sync)]
    #[cfg(not(windows))]
    pub fn mica() {}

    #[frb(sync)]
    #[cfg(not(windows))]
    pub fn tabbed() {}
}
