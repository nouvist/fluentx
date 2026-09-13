use std::{ffi::c_void, mem, sync::OnceLock};

use flutter_rust_bridge::frb;
use windows::{
    Win32::{
        Foundation::{HWND, LPARAM}, Graphics::Dwm::{
            DWM_SYSTEMBACKDROP_TYPE, DWMSBT_MAINWINDOW, DWMSBT_NONE, DWMSBT_TABBEDWINDOW, DWMSBT_TRANSIENTWINDOW, DWMWA_SYSTEMBACKDROP_TYPE, DwmExtendFrameIntoClientArea, DwmSetWindowAttribute,
        }, System::Threading::GetCurrentProcessId, UI::{
            Controls::MARGINS,
            WindowsAndMessaging::{EnumWindows, GetWindowThreadProcessId},
        },
    }, core::BOOL,
};

#[frb(opaque)]
pub struct FluentxNativeBackdrop;

impl FluentxNativeBackdrop {
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
}
