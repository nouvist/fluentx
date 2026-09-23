use gpui::{App, Global, Rgba, Window};

mod implementations;
mod namespaces;

use implementations::*;
use namespaces::*;
#[cfg(windows)]
use windows::UI::ViewManagement::{UIColorType, UISettings};

const fn rgba_const(hex: u32) -> Rgba {
    let r = ((hex >> 24) & 0xFF) as f32 / 255.;
    let g = ((hex >> 16) & 0xFF) as f32 / 255.;
    let b = ((hex >> 8) & 0xFF) as f32 / 255.;
    let a = (hex & 0xFF) as f32 / 255.;
    Rgba { r, g, b, a }
}

const fn rgba_from_windows(color: windows::UI::Color) -> Rgba {
    Rgba {
        r: (color.R as f32) / 255.,
        g: (color.G as f32) / 255.,
        b: (color.B as f32) / 255.,
        a: (color.A as f32) / 255.,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Brightness {
    Dark,
    Light,
}

impl Brightness {
    pub fn is_dark(&self) -> bool {
        *self == Brightness::Dark
    }

    pub fn is_light(&self) -> bool {
        *self == Brightness::Light
    }

    #[cfg(windows)]
    fn current_impl() -> Option<Self> {
        let settings = UISettings::new().ok()?;
        let background = settings.GetColorValue(UIColorType::Background).ok()?;
        let foreground = settings.GetColorValue(UIColorType::Foreground).ok()?;
        Some(match background.R < foreground.R {
            true => Self::Dark,
            false => Self::Light,
        })
    }

    #[cfg(windows)]
    pub fn current() -> Self {
        Self::current_impl().unwrap_or(Brightness::Light)
    }

    #[cfg(not(windows))]
    pub fn current() -> Self {
        Brightness::Light
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Colors {
    pub brightness: Brightness,
    pub accent: AccentColors,
    pub foreground: ForegroundColors,
    pub control: ControlColors,
    pub stroke: StrokeColors,
    pub background: BackgroundColors,
    pub system: SystemColors,
}

impl Global for Colors {}

pub trait AppColors {
    fn colors(&self) -> &Colors;
}

impl AppColors for App {
    fn colors(&self) -> &Colors {
        self.global()
    }
}

impl Colors {
    pub fn dark(accent: AccentColors) -> Self {
        Self {
            accent,
            brightness: Brightness::Dark,
            foreground: ForegroundColors::dark(&accent),
            control: ControlColors::dark(&accent),
            stroke: StrokeColors::DARK,
            background: BackgroundColors::DARK,
            system: SystemColors::DARK,
        }
    }

    pub fn light(accent: AccentColors) -> Self {
        Self {
            brightness: Brightness::Light,
            accent,
            foreground: ForegroundColors::light(&accent),
            control: ControlColors::light(&accent),
            stroke: StrokeColors::LIGHT,
            background: BackgroundColors::LIGHT,
            system: SystemColors::LIGHT,
        }
    }

    #[cfg(windows)]
    fn current_impl() -> Option<Self> {
        let settings = UISettings::new().ok()?;
        let light3 = settings.GetColorValue(UIColorType::AccentLight3).ok()?;
        let light2 = settings.GetColorValue(UIColorType::AccentLight2).ok()?;
        let light1 = settings.GetColorValue(UIColorType::AccentLight1).ok()?;
        let base = settings.GetColorValue(UIColorType::Accent).ok()?;
        let dark1 = settings.GetColorValue(UIColorType::AccentDark1).ok()?;
        let dark2 = settings.GetColorValue(UIColorType::AccentDark2).ok()?;
        let dark3 = settings.GetColorValue(UIColorType::AccentDark3).ok()?;

        let accent = AccentColors {
            light3: rgba_from_windows(light3),
            light2: rgba_from_windows(light2),
            light1: rgba_from_windows(light1),
            base: rgba_from_windows(base),
            dark1: rgba_from_windows(dark1),
            dark2: rgba_from_windows(dark2),
            dark3: rgba_from_windows(dark3),
        };

        match Brightness::current() {
            Brightness::Dark => Some(Self::dark(accent)),
            Brightness::Light => Some(Self::light(accent)),
        }
    }

    #[cfg(windows)]
    pub fn current() -> Self {
        Self::current_impl().unwrap_or_else(|| Self::light(AccentColors::FALLBACK))
    }

    #[cfg(not(windows))]
    pub fn current() -> Self {
        Self::light(AccentColors::FALLBACK)
    }

    pub fn init(window: &mut Window, cx: &mut App) {
        cx.set_global(Colors::current());

        #[cfg(windows)]
        window.observe_window_appearance(Self::handle_init).detach();
        #[cfg(not(windows))]
        let _ = window;
    }

    #[cfg(windows)]
    fn handle_init(window: &mut Window, cx: &mut App) {
        cx.set_global(Colors::current());
        window.refresh();
    }
}
