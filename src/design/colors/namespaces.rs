use super::*;

#[derive(Debug, Clone, Copy)]
pub struct ForegroundColors {
    pub primary: ForegroundPrimaryColors,
    pub accent: ForegroundAccentColors,
    pub on_accent: ForegroundOnAccentColors,
}

impl ForegroundColors {
    pub fn dark(accent: &AccentColors) -> Self {
        Self {
            primary: ForegroundPrimaryColors::DARK,
            accent: ForegroundAccentColors::dark(accent),
            on_accent: ForegroundOnAccentColors::DARK,
        }
    }

    pub fn light(accent: &AccentColors) -> Self {
        Self {
            primary: ForegroundPrimaryColors::LIGHT,
            accent: ForegroundAccentColors::light(accent),
            on_accent: ForegroundOnAccentColors::LIGHT,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ControlColors {
    pub primary: ControlPrimaryColors,
    pub secondary: ControlSecondaryColors,
    pub accent: ControlAccentColors,
    pub solid: ControlSolidColors,
    pub strong: ControlStrongColors,
    pub subtle: ControlSubtleColors,
    pub on_image: ControlOnImageColors,
}

impl ControlColors {
    pub fn dark(accent: &AccentColors) -> Self {
        Self {
            primary: ControlPrimaryColors::DARK,
            secondary: ControlSecondaryColors::DARK,
            accent: ControlAccentColors::dark(accent),
            solid: ControlSolidColors::DARK,
            strong: ControlStrongColors::DARK,
            subtle: ControlSubtleColors::DARK,
            on_image: ControlOnImageColors::DARK,
        }
    }

    pub fn light(accent: &AccentColors) -> Self {
        Self {
            primary: ControlPrimaryColors::LIGHT,
            secondary: ControlSecondaryColors::LIGHT,
            accent: ControlAccentColors::light(accent),
            solid: ControlSolidColors::LIGHT,
            strong: ControlStrongColors::LIGHT,
            subtle: ControlSubtleColors::LIGHT,
            on_image: ControlOnImageColors::LIGHT,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StrokeColors {
    pub control: StrokeControlColors,
    pub card: StrokeCardColors,
    pub surface: StrokeSurfaceColors,
    pub divider: StrokeDividerColors,
    pub focus: StrokeFocusColors,
}

impl StrokeColors {
    pub const DARK: Self = Self {
        control: StrokeControlColors::DARK,
        card: StrokeCardColors::DARK,
        surface: StrokeSurfaceColors::DARK,
        divider: StrokeDividerColors::DARK,
        focus: StrokeFocusColors::DARK,
    };

    pub const LIGHT: Self = Self {
        control: StrokeControlColors::LIGHT,
        card: StrokeCardColors::LIGHT,
        surface: StrokeSurfaceColors::LIGHT,
        divider: StrokeDividerColors::LIGHT,
        focus: StrokeFocusColors::LIGHT,
    };
}

#[derive(Debug, Clone, Copy)]
pub struct BackgroundColors {
    pub card: BackgroundCardColors,
    pub smoke: BackgroundSmokeColors,
    pub layer: BackgroundLayerColors,
    pub solid: BackgroundSolidColors,
}

impl BackgroundColors {
    pub const DARK: Self = Self {
        card: BackgroundCardColors::DARK,
        smoke: BackgroundSmokeColors::DARK,
        layer: BackgroundLayerColors::DARK,
        solid: BackgroundSolidColors::DARK,
    };

    pub const LIGHT: Self = Self {
        card: BackgroundCardColors::LIGHT,
        smoke: BackgroundSmokeColors::LIGHT,
        layer: BackgroundLayerColors::LIGHT,
        solid: BackgroundSolidColors::LIGHT,
    };
}

#[derive(Debug, Clone, Copy)]
pub struct SystemColors {
    pub attention: SystemItemColors,
    pub success: SystemItemColors,
    pub caution: SystemItemColors,
    pub critical: SystemItemColors,
    pub neutral: SystemItemColors,
    pub neutral_solid: SystemItemColors,
}

impl SystemColors {
    pub const DARK: Self = Self {
        attention: SystemItemColors::ATTENTION_DARK,
        success: SystemItemColors::SUCCESS_DARK,
        caution: SystemItemColors::CAUTION_DARK,
        critical: SystemItemColors::CRITICAL_DARK,
        neutral: SystemItemColors::NEUTRAL_DARK,
        neutral_solid: SystemItemColors::NEUTRAL_SOLID_DARK,
    };

    pub const LIGHT: Self = Self {
        attention: SystemItemColors::ATTENTION_LIGHT,
        success: SystemItemColors::SUCCESS_LIGHT,
        caution: SystemItemColors::CAUTION_LIGHT,
        critical: SystemItemColors::CRITICAL_LIGHT,
        neutral: SystemItemColors::NEUTRAL_LIGHT,
        neutral_solid: SystemItemColors::NEUTRAL_SOLID_LIGHT,
    };
}
