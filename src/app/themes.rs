pub mod read;
pub mod hexacolors;
pub mod letters;

use hexacolors::HexaColor;
use letters::Letters;

pub enum Colors {
    Light,
    Dark,
    System
}

pub struct Theme {
    name: String,
    colors: ThemeColors,
    fonts: Letters
}

pub struct  ThemeColors {
    light: ColorPalette,
    dark: ColorPalette
}

pub struct ColorPalette {
    pub background: HexaColor,
    pub surface: HexaColor,
    pub surface_elevated: HexaColor,

    pub titlebar_background: HexaColor,
    pub sidebar_background: HexaColor,

    pub text_primary: HexaColor,
    pub text_secondary: HexaColor,
    pub text_disabled: HexaColor,

    pub accent: HexaColor,
    pub border: HexaColor,
    pub focus: HexaColor,

    pub success: HexaColor,
    pub warning: HexaColor,
    pub danger: HexaColor
}

pub fn rgb(num: f32) -> f32 {num / 255.0}


