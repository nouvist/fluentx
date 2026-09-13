use flutter_rust_bridge::frb;
#[cfg(windows)]
use windows::UI::{
    Color,
    ViewManagement::{UIColorType, UISettings},
};

#[frb]
pub enum FluentxNativeBrightness {
    Dark,
    Light,
}

impl FluentxNativeBrightness {
    #[frb(sync)]
    #[cfg(windows)]
    pub fn current() -> Option<Self> {
        let settings = UISettings::new().ok()?;
        let background = settings.GetColorValue(UIColorType::Background).ok()?;
        let foreground = settings.GetColorValue(UIColorType::Foreground).ok()?;

        Some(match background.R < foreground.R {
            true => Self::Dark,
            false => Self::Light,
        })
    }

    #[frb(sync)]
    #[cfg(not(windows))]
    pub fn current() -> Option<Self> {
        None
    }
}

#[frb]
pub struct FluentxNativeColors {
    pub light3: u32,
    pub light2: u32,
    pub light1: u32,
    pub base: u32,
    pub dark1: u32,
    pub dark2: u32,
    pub dark3: u32,
}

impl FluentxNativeColors {
    #[frb(sync)]
    #[cfg(windows)]
    pub fn current() -> Option<Self> {
        let settings = UISettings::new().ok()?;
        let light3 = settings.GetColorValue(UIColorType::AccentLight3).ok()?;
        let light2 = settings.GetColorValue(UIColorType::AccentLight2).ok()?;
        let light1 = settings.GetColorValue(UIColorType::AccentLight1).ok()?;
        let base = settings.GetColorValue(UIColorType::Accent).ok()?;
        let dark1 = settings.GetColorValue(UIColorType::AccentDark1).ok()?;
        let dark2 = settings.GetColorValue(UIColorType::AccentDark2).ok()?;
        let dark3 = settings.GetColorValue(UIColorType::AccentDark3).ok()?;

        fn c(color: Color) -> u32 {
            (color.A as u32) << 24
                | (color.R as u32) << 16
                | (color.G as u32) << 8
                | (color.B as u32)
        }

        Some(Self {
            light3: c(light3),
            light2: c(light2),
            light1: c(light1),
            base: c(base),
            dark1: c(dark1),
            dark2: c(dark2),
            dark3: c(dark3),
        })
    }

    #[frb(sync)]
    #[cfg(not(windows))]
    pub fn current() -> Option<Self> {
        None
    }
}
