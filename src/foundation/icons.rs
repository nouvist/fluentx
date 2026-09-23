use std::borrow::Cow;

use gpui::App;

include!(concat!(env!("OUT_DIR"), "/icons_generated.rs"));

pub const ICON_REGULAR_TTF: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/Regular.ttf"));

pub const ICON_FILLED_TTF: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/Filled.ttf"));

pub fn init_icon_regular(cx: &App) -> gpui::Result<()> {
    cx.text_system()
        .add_fonts(vec![Cow::Borrowed(ICON_REGULAR_TTF)])
}

pub fn init_icon_filled(cx: &App) -> gpui::Result<()> {
    cx.text_system()
        .add_fonts(vec![Cow::Borrowed(ICON_FILLED_TTF)])
}

#[derive(Clone, Copy)]
pub struct IconNameRegular(&'static str, f32);

#[derive(Clone, Copy)]
pub struct IconNameFilled(&'static str, f32);

impl IconNameRegular {
    #[inline]
    pub fn content(&self) -> &'static str {
        self.0
    }

    #[inline]
    pub fn size(&self) -> f32 {
        self.1
    }
}

impl IconNameFilled {
    #[inline]
    pub fn content(&self) -> &'static str {
        self.0
    }

    #[inline]
    pub fn size(&self) -> f32 {
        self.1
    }
}
