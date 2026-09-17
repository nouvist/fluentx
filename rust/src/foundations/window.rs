use std::{
    alloc::{alloc, dealloc, Layout},
    ffi::c_void,
    mem,
    sync::{Arc, OnceLock},
};

use flutter_rust_bridge::{frb, DartFnFuture};
use tokio::sync::RwLock;
use windows::{
    core::{BOOL, HSTRING},
    Win32::{
        Foundation::{FALSE, HWND, LPARAM, LRESULT, POINT, RECT, TRUE, WPARAM},
        Graphics::{
            Dwm::{
                DwmSetWindowAttribute, DWMWA_USE_IMMERSIVE_DARK_MODE,
                DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND, DWM_WINDOW_CORNER_PREFERENCE,
            },
            Gdi::{ClientToScreen, ScreenToClient},
        },
        System::Threading::GetCurrentProcessId,
        UI::{
            Shell::{DefSubclassProc, SetWindowSubclass},
            WindowsAndMessaging::{
                DestroyWindow, EnumChildWindows, EnumWindows, GetClassNameW, GetCursorPos,
                GetForegroundWindow, GetSystemMetrics, GetWindowLongPtrW, GetWindowPlacement,
                GetWindowRect, GetWindowThreadProcessId, SetWindowLongPtrW, SetWindowPos,
                ShowWindow, GWL_STYLE, HTBOTTOM, HTBOTTOMLEFT, HTBOTTOMRIGHT, HTCLIENT, HTCLOSE,
                HTLEFT, HTMAXBUTTON, HTMINBUTTON, HTRIGHT, HTTOP, HTTOPLEFT, HTTOPRIGHT,
                HTTRANSPARENT, NCCALCSIZE_PARAMS, SM_CXSIZEFRAME, SWP_FRAMECHANGED, SWP_NOMOVE,
                SWP_NOSIZE, SWP_NOZORDER, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, TITLEBARINFOEX,
                WINDOWPLACEMENT, WM_ACTIVATE, WM_GETTITLEBARINFOEX, WM_NCCALCSIZE, WM_NCHITTEST,
                WM_NCLBUTTONDOWN, WM_NCLBUTTONUP, WM_SETTINGCHANGE, WM_SIZE,
                WS_SYSMENU,
            },
        },
    },
};

use crate::foundations::{colors::FxNativeBrightness, event::FxVecEvent};

pub mod backdrop;
#[cfg(windows)]
pub mod backdrop_impl;
#[cfg(not(windows))]
pub mod backdrop_stub;

const LAYOUT_U32: Layout = Layout::new::<u32>();
const LAYOUT_HIT_EVENT: Layout = Layout::new::<FxNativeWindowHitEvent>();

struct FxNativeWindowInner {
    root_hwnd: HWND,
    flutter_hwnd: HWND,
    hit_ptr: *mut u32,
    hit_event_ptr: *mut FxNativeWindowHitEvent,
    listeners: FxVecEvent<FxNativeWindowEvent>,
    hit_listeners: FxVecEvent<FxNativeWindowHitEvent>,
    mouse_listeners: FxVecEvent<FxNativeWindowMouseEvent>,

    close_rect: RwLock<FxNativeWindowRect>,
    maximize_rect: RwLock<FxNativeWindowRect>,
    minimize_rect: RwLock<FxNativeWindowRect>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FxNativeWindowMouseEvent {
    Down,
    Up,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FxNativeWindowHitEvent {
    None,
    Close,
    Maximize,
    Minimize,
}

impl From<u32> for FxNativeWindowHitEvent {
    fn from(value: u32) -> Self {
        match value {
            HTCLOSE => FxNativeWindowHitEvent::Close,
            HTMAXBUTTON => FxNativeWindowHitEvent::Maximize,
            HTMINBUTTON => FxNativeWindowHitEvent::Minimize,
            _ => FxNativeWindowHitEvent::None,
        }
    }
}

impl Default for FxNativeWindowHitEvent {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy)]
pub enum FxNativeWindowEvent {
    Settings,
    Maximize,
    Foreground,
}

#[derive(Default)]
pub struct FxNativeWindowRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl FxNativeWindowRect {
    fn test(&self, point: POINT) -> bool {
        point.x >= self.left
            && point.x <= self.right
            && point.y >= self.top
            && point.y <= self.bottom
    }

    fn to_native(&self, hwnd: HWND) -> RECT {
        let mut top_left = POINT {
            x: self.left,
            y: self.top,
        };
        let mut bottom_right = POINT {
            x: self.right,
            y: self.bottom,
        };

        unsafe {
            _ = ClientToScreen(hwnd, &mut top_left);
            _ = ClientToScreen(hwnd, &mut bottom_right);
        }

        RECT {
            left: top_left.x,
            top: top_left.y,
            right: bottom_right.x,
            bottom: bottom_right.y,
        }
    }
}

unsafe impl Send for FxNativeWindowInner {}
unsafe impl Sync for FxNativeWindowInner {}

impl Drop for FxNativeWindowInner {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.hit_ptr as *mut u8, LAYOUT_U32);
            dealloc(self.hit_event_ptr as *mut u8, LAYOUT_HIT_EVENT);
        }
    }
}

macro_rules! it {
    ($ref:ident) => {{
        let it = $ref as *const FxNativeWindowInner;
        Arc::increment_strong_count(it);
        Self(Arc::from_raw(it))
    }};
}

#[frb(opaque)]
#[derive(Clone)]
#[repr(transparent)]
pub struct FxNativeWindow(Arc<FxNativeWindowInner>);

#[frb(non_opaque)]
pub struct FxNativeWindowListener(pub u64);

impl FxNativeWindow {
    #[frb(sync)]
    pub fn instance() -> Self {
        static INSTANCE: OnceLock<FxNativeWindow> = OnceLock::new();
        INSTANCE.get_or_init(|| Self::new()).clone()
    }

    fn find_root_hwnd() -> HWND {
        struct Param {
            pid: u32,
            hwnd: HWND,
        }

        let mut lparam = Param {
            pid: unsafe { GetCurrentProcessId() },
            hwnd: HWND::default(),
        };

        unsafe extern "system" fn callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
            let param = &mut *(lparam.0 as *mut Param);

            let mut pid: u32 = 0;
            _ = GetWindowThreadProcessId(hwnd, Some(&mut pid));
            if pid == param.pid {
                param.hwnd = hwnd;
                return FALSE;
            }

            TRUE
        }

        _ = unsafe { EnumWindows(Some(callback), LPARAM(&mut lparam as *mut _ as isize)) };
        lparam.hwnd
    }

    fn find_flutter_hwnd(root: HWND) -> HWND {
        let mut lparam = HWND::default();

        unsafe extern "system" fn callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
            let param = &mut *(lparam.0 as *mut HWND);

            let mut buf = [0u16; 16];
            GetClassNameW(hwnd, &mut buf);
            let str = HSTRING::from_wide(&buf);
            let str = str.to_string();
            if str.starts_with("FLUTTERVIEW") {
                *param = hwnd;
                println!("DAPAT");
                return FALSE;
            }

            TRUE
        }

        _ = unsafe {
            EnumChildWindows(
                Some(root),
                Some(callback),
                LPARAM(&mut lparam as *mut _ as isize),
            )
        };

        lparam
    }

    fn new() -> Self {
        let root_hwnd = Self::find_root_hwnd();
        let flutter_hwnd = Self::find_flutter_hwnd(root_hwnd);

        unsafe {
            let it = Self(Arc::new(FxNativeWindowInner {
                root_hwnd,
                flutter_hwnd,
                hit_ptr: alloc(LAYOUT_U32) as *mut _,
                hit_event_ptr: alloc(LAYOUT_U32) as *mut _,
                listeners: FxVecEvent::default(),
                hit_listeners: FxVecEvent::default(),
                mouse_listeners: FxVecEvent::default(),

                close_rect: RwLock::default(),
                maximize_rect: RwLock::default(),
                minimize_rect: RwLock::default(),
            }));

            _ = SetWindowSubclass(
                root_hwnd,
                Some(Self::handle_root),
                &Self::handle_root as *const _ as usize,
                Arc::into_raw(it.clone().0) as usize,
            );

            _ = SetWindowSubclass(
                flutter_hwnd,
                Some(Self::handle_flutter),
                &Self::handle_flutter as *const _ as usize,
                Arc::into_raw(it.clone().0) as usize,
            );

            let corner = DWMWCP_ROUND;
            _ = DwmSetWindowAttribute(
                it.root_hwnd(),
                DWMWA_WINDOW_CORNER_PREFERENCE,
                &corner as *const _ as *const c_void,
                mem::size_of::<DWM_WINDOW_CORNER_PREFERENCE>() as u32,
            );

            let mut style = GetWindowLongPtrW(it.root_hwnd(), GWL_STYLE) as u32;
            style &= !WS_SYSMENU.0;
            SetWindowLongPtrW(it.root_hwnd(), GWL_STYLE, style as isize);

            it
        }
    }

    #[frb(sync)]
    pub fn init(&self) {
        self.0.listeners.clear();
        self.0.hit_listeners.clear();
    }

    #[inline]
    #[frb(ignore)]
    pub fn root_hwnd(&self) -> HWND {
        self.0.root_hwnd
    }

    #[inline]
    #[frb(ignore)]
    pub fn flutter_hwnd(&self) -> HWND {
        self.0.flutter_hwnd
    }

    #[frb(sync)]
    pub fn refresh(&self) {
        unsafe {
            let swp = SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED;
            _ = SetWindowPos(self.root_hwnd(), None, 0, 0, 0, 0, swp);
        }
    }

    unsafe fn handle_hittest(&self) {
        let mut cursor = POINT::default();
        let mut rect = RECT::default();
        let threshold = GetSystemMetrics(SM_CXSIZEFRAME);
        _ = GetCursorPos(&mut cursor as *mut _);
        _ = ScreenToClient(self.root_hwnd(), &mut cursor as *mut _);
        _ = GetWindowRect(self.root_hwnd(), &mut rect as *mut _);

        let cache = |param: u32| {
            *self.0.hit_ptr = param;
            let next = FxNativeWindowHitEvent::from(param);
            if next != *self.0.hit_event_ptr {
                *self.0.hit_event_ptr = next;
                self.0.hit_listeners.invoke(next);
            }
        };

        if !self.is_maximized() {
            let is_top = cursor.y <= threshold;
            let is_bottom = cursor.y >= rect.bottom - rect.top - threshold - 8;
            let is_left = cursor.x <= threshold;
            let is_right = cursor.x >= rect.right - rect.left - threshold - 16;

            if is_top {
                if is_left {
                    cache(HTTOPLEFT);
                    return;
                } else if is_right {
                    cache(HTTOPRIGHT);
                    return;
                } else {
                    cache(HTTOP);
                    return;
                }
            } else if is_bottom {
                if is_left {
                    cache(HTBOTTOMLEFT);
                    return;
                } else if is_right {
                    cache(HTBOTTOMRIGHT);
                    return;
                } else {
                    cache(HTBOTTOM);
                    return;
                }
            } else if is_left {
                cache(HTLEFT);
                return;
            } else if is_right {
                cache(HTRIGHT);
                return;
            }
        }

        if self.0.close_rect.blocking_read().test(cursor) {
            cache(HTCLOSE);
            return;
        } else if self.0.maximize_rect.blocking_read().test(cursor) {
            cache(HTMAXBUTTON);
            return;
        } else if self.0.minimize_rect.blocking_read().test(cursor) {
            cache(HTMINBUTTON);
            return;
        }

        cache(HTCLIENT);
    }

    unsafe extern "system" fn handle_root(
        hwnd: HWND,
        umsg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        _uidsubclass: usize,
        dwrefdata: usize,
    ) -> LRESULT {
        if umsg == WM_SETTINGCHANGE {
            let brightness = FxNativeBrightness::current();
            let is_dark = match brightness {
                Some(FxNativeBrightness::Dark) => TRUE,
                _ => FALSE,
            };

            let it = it!(dwrefdata);
            it.0.listeners.invoke(FxNativeWindowEvent::Settings);

            _ = DwmSetWindowAttribute(
                hwnd,
                DWMWA_USE_IMMERSIVE_DARK_MODE,
                &is_dark as *const _ as *const c_void,
                mem::size_of::<BOOL>() as u32,
            );
        } else if umsg == WM_SIZE {
            let it = it!(dwrefdata);
            it.0.listeners.invoke(FxNativeWindowEvent::Maximize);
        } else if umsg == WM_ACTIVATE {
            let it = it!(dwrefdata);
            it.0.listeners.invoke(FxNativeWindowEvent::Foreground);
        } else if umsg == WM_NCCALCSIZE {
            let it = it!(dwrefdata);
            let size = lparam.0 as *mut c_void as *mut NCCALCSIZE_PARAMS;
            let size = &mut *size;

            size.rgrc[0].left += 8;
            size.rgrc[0].right -= 8;
            size.rgrc[0].bottom -= 8;
            size.rgrc[0].top += match it.is_maximized() {
                true => 8,
                false => 0,
            };

            return LRESULT(0);
        } else if umsg == WM_GETTITLEBARINFOEX {
            let it = it!(dwrefdata);
            let lparam = lparam.0 as *mut TITLEBARINFOEX;
            let lparam = &mut *lparam;
            lparam.rgrect[2] = it.0.minimize_rect.blocking_read().to_native(hwnd);
            lparam.rgrect[3] = it.0.maximize_rect.blocking_read().to_native(hwnd);
            lparam.rgrect[4] = it.0.close_rect.blocking_read().to_native(hwnd);
            return LRESULT(1);
        } else if umsg == WM_NCHITTEST {
            let it = it!(dwrefdata);
            return LRESULT(*it.0.hit_ptr as _);
        } else if umsg == WM_NCLBUTTONDOWN {
            let it = it!(dwrefdata);
            let wparam = wparam.0 as u32;
            if *it.0.hit_event_ptr != FxNativeWindowHitEvent::None {
                it.0.mouse_listeners.invoke(FxNativeWindowMouseEvent::Down);
            }
            if wparam == HTMAXBUTTON || wparam == HTMINBUTTON || wparam == HTCLOSE {
                return LRESULT(0);
            }
        } else if umsg == WM_NCLBUTTONUP {
            let it = it!(dwrefdata);
            let wparam = wparam.0 as u32;
            if *it.0.hit_event_ptr != FxNativeWindowHitEvent::None {
                it.0.mouse_listeners.invoke(FxNativeWindowMouseEvent::Up);
            }
            if wparam == HTMAXBUTTON {
                _ = ShowWindow(
                    hwnd,
                    match it.is_maximized() {
                        true => SW_RESTORE,
                        false => SW_MAXIMIZE,
                    },
                );
                return LRESULT(0);
            } else if wparam == HTMINBUTTON {
                _ = ShowWindow(hwnd, SW_MINIMIZE);
                return LRESULT(0);
            } else if wparam == HTCLOSE {
                _ = DestroyWindow(hwnd);
                return LRESULT(0);
            }
        }

        DefSubclassProc(hwnd, umsg, wparam, lparam)
    }

    unsafe extern "system" fn handle_flutter(
        hwnd: HWND,
        umsg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        _uidsubclass: usize,
        dwrefdata: usize,
    ) -> LRESULT {
        if umsg == WM_NCHITTEST {
            let it = it!(dwrefdata);
            it.handle_hittest();
            return LRESULT(HTTRANSPARENT as _);
        }
        DefSubclassProc(hwnd, umsg, wparam, lparam)
    }

    #[frb(sync)]
    pub fn is_maximized(&self) -> bool {
        let mut placement = WINDOWPLACEMENT::default();
        _ = unsafe { GetWindowPlacement(self.root_hwnd(), &mut placement) };
        placement.showCmd == SW_MAXIMIZE.0 as u32
    }

    #[frb(sync)]
    pub fn is_foreground(&self) -> bool {
        unsafe { GetForegroundWindow() == self.0.root_hwnd }
    }

    #[frb(sync)]
    pub fn set_close_rect(&self, rect: FxNativeWindowRect) {
        let mut close_rect = self.0.close_rect.blocking_write();
        *close_rect = rect;
    }

    #[frb(sync)]
    pub fn set_maximize_rect(&self, rect: FxNativeWindowRect) {
        let mut maximize_rect = self.0.maximize_rect.blocking_write();
        *maximize_rect = rect;
    }

    #[frb(sync)]
    pub fn set_minimize_rect(&self, rect: FxNativeWindowRect) {
        let mut minimize_rect = self.0.minimize_rect.blocking_write();
        *minimize_rect = rect;
    }

    #[frb(sync)]
    pub fn add_hit_listener(
        &self,
        callback: impl Fn(FxNativeWindowHitEvent) -> DartFnFuture<()> + Send + Sync + 'static,
    ) -> u32 {
        self.0.hit_listeners.add(callback)
    }

    #[frb(sync)]
    pub fn remove_hit_listener(&self, id: u32) {
        self.0.hit_listeners.remove(id);
    }

    #[frb(sync)]
    pub fn add_mouse_listener(
        &self,
        callback: impl Fn(FxNativeWindowMouseEvent) -> DartFnFuture<()> + Send + Sync + 'static,
    ) -> u32 {
        self.0.mouse_listeners.add(callback)
    }

    #[frb(sync)]
    pub fn remove_mouse_listener(&self, id: u32) {
        self.0.mouse_listeners.remove(id);
    }

    #[frb(sync)]
    pub fn add_listener(
        &self,
        callback: impl Fn(FxNativeWindowEvent) -> DartFnFuture<()> + Send + Sync + 'static,
    ) -> u32 {
        self.0.listeners.add(callback)
    }

    #[frb(sync)]
    pub fn remove_listener(&self, id: u32) {
        self.0.listeners.remove(id);
    }
}
