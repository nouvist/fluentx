use gpui::Rgba;

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct AccentColors {
    pub light3: Rgba,
    pub light2: Rgba,
    pub light1: Rgba,
    pub base: Rgba,
    pub dark1: Rgba,
    pub dark2: Rgba,
    pub dark3: Rgba,
}

impl AccentColors {
    pub const FALLBACK: Self = Self {
        light3: rgba_const(0x99ebffff),
        light2: rgba_const(0x4cc2ffff),
        light1: rgba_const(0x0091f8ff),
        base: rgba_const(0x0078d4ff),
        dark1: rgba_const(0x0067c0ff),
        dark2: rgba_const(0x003e92ff),
        dark3: rgba_const(0x001a68ff),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ForegroundPrimaryColors {
    pub primary: Rgba,
    pub secondary: Rgba,
    pub tertiary: Rgba,
    pub disabled: Rgba,
}

impl ForegroundPrimaryColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0xffffffff),
        secondary: rgba_const(0xffffffc5),
        tertiary: rgba_const(0xffffff87),
        disabled: rgba_const(0xffffff5d),
    };

    pub const LIGHT: Self = Self {
        primary: rgba_const(0x000000e4),
        secondary: rgba_const(0x0000009e),
        tertiary: rgba_const(0x00000072),
        disabled: rgba_const(0x0000005c),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ForegroundAccentColors {
    pub primary: Rgba,
    pub secondary: Rgba,
    pub tertiary: Rgba,
    pub disabled: Rgba,
}

impl ForegroundAccentColors {
    pub fn dark(accent: &AccentColors) -> Self {
        Self {
            primary: accent.light3,
            secondary: accent.light3,
            tertiary: accent.light2,
            disabled: rgba_const(0xffffff5d),
        }
    }

    pub fn light(accent: &AccentColors) -> Self {
        Self {
            primary: accent.dark2,
            secondary: accent.dark3,
            tertiary: accent.dark1,
            disabled: rgba_const(0x0000005c),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ForegroundOnAccentColors {
    pub primary: Rgba,
    pub secondary: Rgba,
    pub tertiary: Rgba,
    pub disabled: Rgba,
}

impl ForegroundOnAccentColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0x000000ff),
        secondary: rgba_const(0x00000080),
        tertiary: rgba_const(0x00000087),
        disabled: rgba_const(0xffffffff),
    };

    pub const LIGHT: Self = Self {
        primary: rgba_const(0xffffffff),
        secondary: rgba_const(0xffffffb3),
        tertiary: rgba_const(0xffffffb3),
        disabled: rgba_const(0xffffffff),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ControlPrimaryColors {
    pub primary: Rgba,
    pub secondary: Rgba,
    pub tertiary: Rgba,
    pub quarternary: Rgba,
    pub disabled: Rgba,
    pub transparent: Rgba,
    pub active: Rgba,
}

impl ControlPrimaryColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0xffffff0f),
        secondary: rgba_const(0xffffff15),
        tertiary: rgba_const(0xffffff0b),
        quarternary: rgba_const(0xffffff0f),
        disabled: rgba_const(0xf9f9f94d),
        transparent: rgba_const(0xffffff00),
        active: rgba_const(0x1e1e1eb3),
    };

    pub const LIGHT: Self = Self {
        primary: rgba_const(0xffffffb3),
        secondary: rgba_const(0xf9f9f980),
        tertiary: rgba_const(0xf9f9f94d),
        quarternary: rgba_const(0xf3f3f3c2),
        disabled: rgba_const(0xf9f9f94d),
        transparent: rgba_const(0xffffff00),
        active: rgba_const(0xffffffff),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ControlSecondaryColors {
    pub primary: Rgba,
    pub secondary: Rgba,
    pub tertiary: Rgba,
    pub quarternary: Rgba,
    pub disabled: Rgba,
}

impl ControlSecondaryColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0xffffff00),
        secondary: rgba_const(0x00000019),
        tertiary: rgba_const(0xffffff0b),
        quarternary: rgba_const(0xffffff12),
        disabled: rgba_const(0xffffff00),
    };

    pub const LIGHT: Self = Self {
        primary: rgba_const(0xffffff00),
        secondary: rgba_const(0x00000006),
        tertiary: rgba_const(0x0000000f),
        quarternary: rgba_const(0x00000018),
        disabled: rgba_const(0xffffff00),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ControlSolidColors {
    pub primary: Rgba,
}

impl ControlSolidColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0x454545ff),
    };
    pub const LIGHT: Self = Self {
        primary: rgba_const(0xffffffff),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ControlStrongColors {
    pub primary: Rgba,
    pub disabled: Rgba,
}

impl ControlStrongColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0xffffff8b),
        disabled: rgba_const(0xffffff3f),
    };

    pub const LIGHT: Self = Self {
        primary: rgba_const(0x00000072),
        disabled: rgba_const(0x00000051),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ControlSubtleColors {
    pub primary: Rgba,
    pub secondary: Rgba,
    pub tertiary: Rgba,
    pub disabled: Rgba,
}

impl ControlSubtleColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0xffffff00),
        secondary: rgba_const(0xffffff0f),
        tertiary: rgba_const(0xffffff0a),
        disabled: rgba_const(0xffffff00),
    };

    pub const LIGHT: Self = Self {
        primary: rgba_const(0x00000000),
        secondary: rgba_const(0x00000009),
        tertiary: rgba_const(0x00000006),
        disabled: rgba_const(0x00000000),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ControlOnImageColors {
    pub primary: Rgba,
    pub secondary: Rgba,
    pub tertiary: Rgba,
    pub disabled: Rgba,
}

impl ControlOnImageColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0x1c1c1cb3),
        secondary: rgba_const(0x1a1a1aff),
        tertiary: rgba_const(0x131313ff),
        disabled: rgba_const(0x1e1e1eff),
    };

    pub const LIGHT: Self = Self {
        primary: rgba_const(0xffffffc9),
        secondary: rgba_const(0xf3f3f3ff),
        tertiary: rgba_const(0xebebebff),
        disabled: rgba_const(0xffffff00),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ControlAccentColors {
    pub primary: Rgba,
    pub secondary: Rgba,
    pub tertiary: Rgba,
    pub disabled: Rgba,
    pub selected: Rgba,
}

impl ControlAccentColors {
    pub fn dark(accent: &AccentColors) -> Self {
        let mut secondary = accent.light2;
        secondary.a = 0.9;

        let mut tertiary = accent.light2;
        tertiary.a = 0.8;

        Self {
            primary: accent.light2,
            secondary,
            tertiary,
            disabled: rgba_const(0xffffff28),
            selected: accent.base,
        }
    }

    pub fn light(accent: &AccentColors) -> Self {
        let mut secondary = accent.dark2;
        secondary.a = 0.9;

        let mut tertiary = accent.dark2;
        tertiary.a = 0.8;

        Self {
            primary: accent.dark2,
            secondary,
            tertiary,
            disabled: rgba_const(0x00000037),
            selected: accent.base,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StrokeControlColors {
    pub primary: Rgba,
    pub secondary: Rgba,
    pub on_accent_primary: Rgba,
    pub on_accent_secondary: Rgba,
    pub on_accent_tertiary: Rgba,
    pub on_accent_disabled: Rgba,
    pub on_image: Rgba,
}

impl StrokeControlColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0xffffff12),
        secondary: rgba_const(0xffffff18),
        on_accent_primary: rgba_const(0xffffff14),
        on_accent_secondary: rgba_const(0x00000023),
        on_accent_tertiary: rgba_const(0x00000037),
        on_accent_disabled: rgba_const(0x00000033),
        on_image: rgba_const(0x0000006b),
    };

    pub const LIGHT: Self = Self {
        primary: rgba_const(0x0000000f),
        secondary: rgba_const(0x00000029),
        on_accent_primary: rgba_const(0xffffff14),
        on_accent_secondary: rgba_const(0x00000066),
        on_accent_tertiary: rgba_const(0x00000037),
        on_accent_disabled: rgba_const(0x0000000f),
        on_image: rgba_const(0xffffff59),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct StrokeCardColors {
    pub primary_transparent: Rgba,
    pub primary_opaque: Rgba,
}

impl StrokeCardColors {
    pub const DARK: Self = Self {
        primary_transparent: rgba_const(0x00000019),
        primary_opaque: rgba_const(0x1c1c1cff),
    };

    pub const LIGHT: Self = Self {
        primary_transparent: rgba_const(0x0000000f),
        primary_opaque: rgba_const(0xebebebff),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct StrokeSurfaceColors {
    pub primary: Rgba,
    pub snapped: Rgba,
    pub flyout: Rgba,
}

impl StrokeSurfaceColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0x75757566),
        snapped: rgba_const(0x757575ff),
        flyout: rgba_const(0x00000033),
    };

    pub const LIGHT: Self = Self {
        primary: rgba_const(0x75757566),
        snapped: rgba_const(0x757575ff),
        flyout: rgba_const(0x0000000f),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct StrokeDividerColors {
    pub primary: Rgba,
}

impl StrokeDividerColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0xffffff15),
    };
    pub const LIGHT: Self = Self {
        primary: rgba_const(0x0000000f),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct StrokeFocusColors {
    pub outer: Rgba,
    pub inner: Rgba,
}

impl StrokeFocusColors {
    pub const DARK: Self = Self {
        outer: rgba_const(0xffffffff),
        inner: rgba_const(0x000000b3),
    };

    pub const LIGHT: Self = Self {
        outer: rgba_const(0x000000e4),
        inner: rgba_const(0xffffffb3),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct BackgroundCardColors {
    pub primary: Rgba,
    pub secondary: Rgba,
    pub tertiary: Rgba,
}

impl BackgroundCardColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0xffffff0d),
        secondary: rgba_const(0xffffff08),
        tertiary: rgba_const(0xffffff12),
    };

    pub const LIGHT: Self = Self {
        primary: rgba_const(0xffffffb3),
        secondary: rgba_const(0xf6f6f680),
        tertiary: rgba_const(0xffffffff),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct BackgroundSmokeColors {
    pub primary: Rgba,
}

impl BackgroundSmokeColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0x0000004d),
    };
    pub const LIGHT: Self = Self {
        primary: rgba_const(0x0000004d),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct BackgroundLayerColors {
    pub primary: Rgba,
    pub secondary: Rgba,
}

impl BackgroundLayerColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0x3a3a3a4c),
        secondary: rgba_const(0xffffff0d),
    };

    pub const LIGHT: Self = Self {
        primary: rgba_const(0xffffff80),
        secondary: rgba_const(0xffffffff),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct BackgroundSolidColors {
    pub primary: Rgba,
    pub alternative: Rgba,
    pub secondary: Rgba,
    pub tertiary: Rgba,
    pub quarternary: Rgba,
    pub quinary: Rgba,
    pub senary: Rgba,
}

impl BackgroundSolidColors {
    pub const DARK: Self = Self {
        primary: rgba_const(0x202020ff),
        alternative: rgba_const(0x0a0a0aff),
        secondary: rgba_const(0x1c1c1cff),
        tertiary: rgba_const(0x282828ff),
        quarternary: rgba_const(0x2c2c2cff),
        quinary: rgba_const(0x333333ff),
        senary: rgba_const(0x373737ff),
    };

    pub const LIGHT: Self = Self {
        primary: rgba_const(0xf3f3f3ff),
        alternative: rgba_const(0xdadadaff),
        secondary: rgba_const(0xeeeeeeff),
        tertiary: rgba_const(0xf9f9f9ff),
        quarternary: rgba_const(0xffffffff),
        quinary: rgba_const(0xfdfdfdff),
        senary: rgba_const(0xffffffff),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct SystemItemColors {
    pub foreground: Rgba,
    pub background: Rgba,
}

impl SystemItemColors {
    pub const ATTENTION_DARK: Self = Self {
        foreground: rgba_const(0x60cdffff),
        background: rgba_const(0x2e2e2eff),
    };
    pub const SUCCESS_DARK: Self = Self {
        foreground: rgba_const(0x6ccb5fff),
        background: rgba_const(0x393d1bff),
    };
    pub const CAUTION_DARK: Self = Self {
        foreground: rgba_const(0xfce100ff),
        background: rgba_const(0x433519ff),
    };
    pub const CRITICAL_DARK: Self = Self {
        foreground: rgba_const(0xffff99a4),
        background: rgba_const(0x442726ff),
    };
    pub const NEUTRAL_DARK: Self = Self {
        foreground: rgba_const(0xffffff8b),
        background: rgba_const(0xffffff08),
    };
    pub const NEUTRAL_SOLID_DARK: Self = Self {
        foreground: rgba_const(0x9d9d9dff),
        background: rgba_const(0x2e2e2eff),
    };

    pub const ATTENTION_LIGHT: Self = Self {
        foreground: rgba_const(0x0070cbff),
        background: rgba_const(0xf7f7f7ff),
    };
    pub const SUCCESS_LIGHT: Self = Self {
        foreground: rgba_const(0x0f7b0fff),
        background: rgba_const(0xdff6ddff),
    };
    pub const CAUTION_LIGHT: Self = Self {
        foreground: rgba_const(0x9d5d00ff),
        background: rgba_const(0xfff4ceff),
    };
    pub const CRITICAL_LIGHT: Self = Self {
        foreground: rgba_const(0xc42b1cff),
        background: rgba_const(0xfde7e9ff),
    };
    pub const NEUTRAL_LIGHT: Self = Self {
        foreground: rgba_const(0x00000072),
        background: rgba_const(0x00000006),
    };
    pub const NEUTRAL_SOLID_LIGHT: Self = Self {
        foreground: rgba_const(0x8a8a8aff),
        background: rgba_const(0xf3f3f3ff),
    };
}
