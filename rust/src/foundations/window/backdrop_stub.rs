use crate::foundations::window::{backdrop::FluentxNativeWindowBackdrop, FluentxNativeWindow};

impl FluentxNativeWindowBackdrop for FluentxNativeWindow {
    fn extend(&self) {}
    fn none(&self) {}
    fn mica(&self) {}
    fn tabbed(&self) {}
}
