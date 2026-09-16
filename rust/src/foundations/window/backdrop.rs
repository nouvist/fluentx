use flutter_rust_bridge::frb;

pub trait FxNativeWindowBackdrop {
    #[frb(sync)]
    fn extend(&self);

    #[frb(sync)]
    fn none(&self);

    #[frb(sync)]
    fn mica(&self);

    #[frb(sync)]
    fn tabbed(&self);
}
