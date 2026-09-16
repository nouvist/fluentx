use std::{ffi::c_void, mem};

use windows::Win32::{
    Graphics::Dwm::{
        DwmExtendFrameIntoClientArea, DwmSetWindowAttribute, DWMSBT_MAINWINDOW, DWMSBT_NONE,
        DWMSBT_TABBEDWINDOW, DWMWA_SYSTEMBACKDROP_TYPE, DWM_SYSTEMBACKDROP_TYPE,
    },
    UI::Controls::MARGINS,
};

use crate::foundations::window::{backdrop::FxNativeWindowBackdrop, FxNativeWindow};

impl FxNativeWindowBackdrop for FxNativeWindow {
    fn extend(&self) {
        _ = unsafe {
            DwmExtendFrameIntoClientArea(
                self.root_hwnd(),
                &MARGINS {
                    cxLeftWidth: -1,
                    cxRightWidth: -1,
                    cyTopHeight: -1,
                    cyBottomHeight: -1,
                },
            )
        };
    }

    fn none(&self) {
        _ = unsafe {
            DwmSetWindowAttribute(
                self.root_hwnd(),
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_NONE as *const _ as *const c_void,
                mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
            )
        };
    }

    fn mica(&self) {
        _ = unsafe {
            DwmSetWindowAttribute(
                self.root_hwnd(),
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_MAINWINDOW as *const _ as *const c_void,
                mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
            )
        };
    }

    fn tabbed(&self) {
        _ = unsafe {
            DwmSetWindowAttribute(
                self.root_hwnd(),
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_TABBEDWINDOW as *const _ as *const c_void,
                mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
            )
        };
    }
}
