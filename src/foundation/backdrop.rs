use std::{ffi::c_void, mem};

use gpui::Window;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::{
    Foundation::HWND,
    Graphics::Dwm::{
        DWM_SYSTEMBACKDROP_TYPE, DWMSBT_MAINWINDOW, DWMSBT_NONE, DWMSBT_TABBEDWINDOW,
        DWMWA_SYSTEMBACKDROP_TYPE, DwmExtendFrameIntoClientArea, DwmSetWindowAttribute,
    },
    UI::{
        Controls::MARGINS,
        WindowsAndMessaging::{GWL_STYLE, GetWindowLongPtrW, SetWindowLongPtrW, WS_SYSMENU},
    },
};

pub struct Backdrop(HWND);

unsafe impl Send for Backdrop {}
unsafe impl Sync for Backdrop {}

impl Backdrop {
    pub fn new(window: &Window) -> Option<Self> {
        let handle = HasWindowHandle::window_handle(window).ok()?;
        let handle = match handle.as_raw() {
            RawWindowHandle::Win32(it) => Some(it),
            _ => None,
        };

        Some(Self(HWND(handle?.hwnd.get() as *mut c_void)))
    }

    pub fn extend(&self) {
        unsafe {
            let mut style = GetWindowLongPtrW(self.0, GWL_STYLE) as u32;
            style &= !WS_SYSMENU.0;
            SetWindowLongPtrW(self.0, GWL_STYLE, style as isize);

            let _ = DwmExtendFrameIntoClientArea(
                self.0,
                &MARGINS {
                    cxLeftWidth: -1,
                    cxRightWidth: -1,
                    cyTopHeight: -1,
                    cyBottomHeight: -1,
                },
            );
        }
    }

    pub fn none(&self) {
        let _ = unsafe {
            DwmSetWindowAttribute(
                self.0,
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_NONE as *const _ as *const c_void,
                mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
            )
        };
    }

    pub fn mica(&self) {
        let _ = unsafe {
            DwmSetWindowAttribute(
                self.0,
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_MAINWINDOW as *const _ as *const c_void,
                mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
            )
        };
    }

    pub fn tabbed(&self) {
        let _ = unsafe {
            DwmSetWindowAttribute(
                self.0,
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_TABBEDWINDOW as *const _ as *const c_void,
                mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
            )
        };
    }
}
