use std::{
    alloc::{alloc, dealloc, Layout},
    ffi::c_void,
    mem,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, OnceLock,
    },
};

use flutter_rust_bridge::{frb, DartFnFuture};
use tokio::sync::RwLock;
use windows::{
    core::{BOOL, HSTRING},
    Win32::{
        Foundation::{FALSE, HWND, LPARAM, LRESULT, POINT, RECT, TRUE, WPARAM},
        Graphics::{
            Dwm::{
                DwmExtendFrameIntoClientArea, DwmSetWindowAttribute, DWMSBT_MAINWINDOW,
                DWMSBT_NONE, DWMSBT_TABBEDWINDOW, DWMWA_SYSTEMBACKDROP_TYPE,
                DWMWA_USE_IMMERSIVE_DARK_MODE, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND,
                DWM_SYSTEMBACKDROP_TYPE, DWM_WINDOW_CORNER_PREFERENCE,
            },
            Gdi::ScreenToClient,
        },
        System::Threading::GetCurrentProcessId,
        UI::{
            Controls::MARGINS,
            Shell::{DefSubclassProc, SetWindowSubclass},
            WindowsAndMessaging::{
                EnumChildWindows, EnumWindows, GetClassNameW, GetCursorPos, GetSystemMetrics,
                GetWindowLongPtrW, GetWindowRect, GetWindowThreadProcessId, SetWindowLongPtrW,
                SetWindowPos, GWL_STYLE, HTBOTTOM, HTBOTTOMLEFT, HTBOTTOMRIGHT, HTCAPTION,
                HTCLIENT, HTLEFT, HTRIGHT, HTTOP, HTTOPLEFT, HTTOPRIGHT, HTTRANSPARENT,
                SM_CXSIZEFRAME, SWP_DRAWFRAME, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOOWNERZORDER,
                SWP_NOSIZE, SWP_NOZORDER, WM_NCCALCSIZE, WM_NCHITTEST, WM_SETTINGCHANGE,
                WS_SYSMENU,
            },
        },
    },
};

use crate::foundations::colors::FluentxNativeBrightness;

const LAYOUT_U32: Layout = Layout::new::<LRESULT>();

struct FluentxNativeWindowInner {
    next: AtomicU64,
    root_hwnd: HWND,
    flutter_hwnd: HWND,
    hittest_ptr: *mut u32,
    listeners: RwLock<
        Vec<(
            u64,
            Box<dyn Fn() -> DartFnFuture<()> + Send + Sync + 'static>,
        )>,
    >,
}

unsafe impl Send for FluentxNativeWindowInner {}
unsafe impl Sync for FluentxNativeWindowInner {}

impl Drop for FluentxNativeWindowInner {
    fn drop(&mut self) {
        unsafe { dealloc(self.hittest_ptr as *mut u8, LAYOUT_U32) };
    }
}

macro_rules! it {
    ($ref:ident) => {{
        let it = $ref as *const FluentxNativeWindowInner;
        Arc::increment_strong_count(it);
        Self(Arc::from_raw(it))
    }};
}

#[frb(opaque)]
#[derive(Clone)]
#[repr(transparent)]
pub struct FluentxNativeWindow(Arc<FluentxNativeWindowInner>);

#[frb(non_opaque)]
pub struct FluentxNativeWindowListener(pub u64);

impl FluentxNativeWindow {
    #[frb(sync)]
    pub fn instance() -> Self {
        static INSTANCE: OnceLock<FluentxNativeWindow> = OnceLock::new();
        INSTANCE.get_or_init(|| Self::init()).clone()
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

    fn init() -> Self {
        let root_hwnd = Self::find_root_hwnd();
        let flutter_hwnd = Self::find_flutter_hwnd(root_hwnd);

        unsafe {
            let it = Self(Arc::new(FluentxNativeWindowInner {
                root_hwnd,
                flutter_hwnd,
                hittest_ptr: alloc(LAYOUT_U32) as *mut _,
                next: AtomicU64::new(0),
                listeners: RwLock::new(Vec::new()),
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
            let swp = SWP_NOMOVE
                | SWP_NOSIZE
                | SWP_NOZORDER
                | SWP_NOOWNERZORDER
                | SWP_FRAMECHANGED
                | SWP_DRAWFRAME;
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

        let is_top = cursor.y <= threshold;
        let is_bottom = cursor.y >= rect.bottom - rect.top - threshold;
        let is_left = cursor.x <= threshold;
        let is_right = cursor.x >= rect.right - rect.left - threshold;

        macro_rules! cache {
            ($param:ident) => {
                *self.0.hittest_ptr = $param;
                return
            };
        }

        if is_top {
            if is_left {
                cache!(HTTOPLEFT);
            } else if is_right {
                cache!(HTTOPRIGHT);
            }
            cache!(HTTOP);
        } else if is_bottom {
            if is_left {
                cache!(HTBOTTOMLEFT);
            } else if is_right {
                cache!(HTBOTTOMRIGHT);
            }
            cache!(HTBOTTOM);
        } else if is_left {
            cache!(HTLEFT);
        } else if is_right {
            cache!(HTRIGHT);
        }

        cache!(HTCLIENT);
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
            let brightness = FluentxNativeBrightness::current();
            let is_dark = match brightness {
                Some(FluentxNativeBrightness::Dark) => TRUE,
                _ => FALSE,
            };

            let it = it!(dwrefdata);
            crate::spawn(async move {
                let listeners = it.0.listeners.read().await;
                for (_, listener) in listeners.iter() {
                    (listener)().await;
                }
            });

            _ = DwmSetWindowAttribute(
                hwnd,
                DWMWA_USE_IMMERSIVE_DARK_MODE,
                &is_dark as *const _ as *const c_void,
                mem::size_of::<BOOL>() as u32,
            );
        } else if umsg == WM_NCCALCSIZE {
            return LRESULT(0);
        } else if umsg == WM_NCHITTEST {
            let it = it!(dwrefdata);
            return LRESULT(*it.0.hittest_ptr as _);
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
            if *it.0.hittest_ptr == HTCLIENT {
                return LRESULT(HTCLIENT as _);
            } else {
                return LRESULT(HTTRANSPARENT as _);
            }
        }
        DefSubclassProc(hwnd, umsg, wparam, lparam)
    }

    pub async fn listen(
        &self,
        callback: impl Fn() -> DartFnFuture<()> + Send + Sync + 'static,
    ) -> FluentxNativeWindowListener {
        let mut listeners = self.0.listeners.write().await;
        let id = self.0.next.fetch_add(1, Ordering::Relaxed);
        listeners.push((id, Box::new(callback)));
        FluentxNativeWindowListener(id)
    }

    pub async fn cancel(&self, listener: FluentxNativeWindowListener) {
        let mut listeners = self.0.listeners.write().await;
        listeners.retain(|it| it.0 != listener.0);
    }

    #[frb(sync)]
    pub fn extend() {
        _ = unsafe {
            DwmExtendFrameIntoClientArea(
                FluentxNativeWindow::instance().root_hwnd(),
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
                FluentxNativeWindow::instance().root_hwnd(),
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
                FluentxNativeWindow::instance().root_hwnd(),
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
                FluentxNativeWindow::instance().root_hwnd(),
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_TABBEDWINDOW as *const _ as *const c_void,
                mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
            )
        };
    }
}
