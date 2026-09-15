use std::{
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
    core::BOOL,
    Win32::{
        Foundation::{FALSE, HWND, LPARAM, LRESULT, TRUE, WPARAM},
        Graphics::Dwm::{
            DwmExtendFrameIntoClientArea, DwmSetWindowAttribute, DWMSBT_MAINWINDOW, DWMSBT_NONE,
            DWMSBT_TABBEDWINDOW, DWMWA_SYSTEMBACKDROP_TYPE, DWMWA_USE_IMMERSIVE_DARK_MODE,
            DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND, DWM_SYSTEMBACKDROP_TYPE,
            DWM_WINDOW_CORNER_PREFERENCE,
        },
        System::Threading::GetCurrentProcessId,
        UI::{
            Controls::MARGINS,
            Shell::{DefSubclassProc, SetWindowSubclass},
            WindowsAndMessaging::{
                EnumWindows, GetWindowLongPtrW, GetWindowThreadProcessId, SetWindowLongPtrW,
                SetWindowPos, GWL_STYLE, SWP_DRAWFRAME, SWP_FRAMECHANGED, SWP_NOMOVE,
                SWP_NOOWNERZORDER, SWP_NOSIZE, SWP_NOZORDER, WM_NCCALCSIZE, WM_SETTINGCHANGE,
                WS_SYSMENU,
            },
        },
    },
};

use crate::foundations::colors::FluentxNativeBrightness;

struct FluentxNativeWindowInner {
    hwnd: usize,
    next: AtomicU64,
    listeners: RwLock<
        Vec<(
            u64,
            Box<dyn Fn() -> DartFnFuture<()> + Send + Sync + 'static>,
        )>,
    >,
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

    #[cfg(not(windows))]
    fn init() -> Self {
        Self(Arc::new(FluentxNativeWindowInner {
            hwnd: 0,
            listeners: RwLock::new(Vec::new()),
            next: AtomicU64::new(0),
        }))
    }

    #[cfg(windows)]
    fn init() -> Self {
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

        unsafe {
            _ = EnumWindows(Some(callback), LPARAM(&mut lparam as *mut _ as isize));
            let it = Self(Arc::new(FluentxNativeWindowInner {
                hwnd: lparam.hwnd.0 as usize,
                listeners: RwLock::new(Vec::new()),
                next: AtomicU64::new(0),
            }));
            let it_ptr = Arc::into_raw(it.clone().0);

            _ = SetWindowSubclass(
                lparam.hwnd,
                Some(Self::handle_subclass),
                &Self::handle_subclass as *const _ as usize,
                it_ptr as usize,
            );

            let corner = DWMWCP_ROUND;
            _ = DwmSetWindowAttribute(
                it.raw(),
                DWMWA_WINDOW_CORNER_PREFERENCE,
                &corner as *const _ as *const c_void,
                mem::size_of::<DWM_WINDOW_CORNER_PREFERENCE>() as u32,
            );

            let mut style = GetWindowLongPtrW(it.raw(), GWL_STYLE) as u32;
            style &= !WS_SYSMENU.0;
            SetWindowLongPtrW(it.raw(), GWL_STYLE, style as isize);

            it
        }
    }

    #[frb(ignore)]
    #[cfg(windows)]
    pub fn raw(&self) -> HWND {
        HWND(self.0.hwnd as *mut _)
    }

    #[frb(sync)]
    #[cfg(windows)]
    pub fn refresh(&self) {
        unsafe {
            let swp = SWP_NOMOVE
                | SWP_NOSIZE
                | SWP_NOZORDER
                | SWP_NOOWNERZORDER
                | SWP_FRAMECHANGED
                | SWP_DRAWFRAME;
            _ = SetWindowPos(self.raw(), None, 0, 0, 0, 0, swp);
        }
    }

    #[cfg(windows)]
    unsafe extern "system" fn handle_subclass(
        hwnd: HWND,
        umsg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        _uidsubclass: usize,
        dwrefdata: usize,
    ) -> LRESULT {
        let it = dwrefdata as *const FluentxNativeWindowInner;
        Arc::increment_strong_count(it);
        let it = Self(Arc::from_raw(it));

        if umsg == WM_SETTINGCHANGE {
            let brightness = FluentxNativeBrightness::current();
            let is_dark = match brightness {
                Some(FluentxNativeBrightness::Dark) => TRUE,
                _ => FALSE,
            };

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
    #[cfg(not(windows))]
    pub fn is_native() -> bool {
        false
    }

    #[frb(sync)]
    #[cfg(windows)]
    pub fn is_native() -> bool {
        true
    }

    #[frb(sync)]
    #[cfg(not(windows))]
    pub fn extend() {}

    #[frb(sync)]
    #[cfg(windows)]
    pub fn extend() {
        _ = unsafe {
            DwmExtendFrameIntoClientArea(
                FluentxNativeWindow::instance().raw(),
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
    #[cfg(not(windows))]
    pub fn none() {}

    #[frb(sync)]
    #[cfg(windows)]
    pub fn none() {
        _ = unsafe {
            DwmSetWindowAttribute(
                FluentxNativeWindow::instance().raw(),
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_NONE as *const _ as *const c_void,
                mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
            )
        };
    }

    #[frb(sync)]
    #[cfg(not(windows))]
    pub fn mica() {}

    #[frb(sync)]
    #[cfg(windows)]
    pub fn mica() {
        _ = unsafe {
            DwmSetWindowAttribute(
                FluentxNativeWindow::instance().raw(),
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
                FluentxNativeWindow::instance().raw(),
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_TABBEDWINDOW as *const _ as *const c_void,
                mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
            )
        };
    }

    #[frb(sync)]
    #[cfg(not(windows))]
    pub fn tabbed() {}
}
