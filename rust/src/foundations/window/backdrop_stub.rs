use crate::foundations::window::{backdrop::FxNativeWindowBackdrop, FxNativeWindow};

impl FxNativeWindowBackdrop for FxNativeWindow {
    fn extend(&self) {}
    fn none(&self) {}
    fn mica(&self) {}
    fn tabbed(&self) {}
}
