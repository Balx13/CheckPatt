pub mod color;
pub mod typography;

use color::HexColor;
use typography::Typography;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum Mode {
    Light,
    Dark,
    System,
}

impl Mode {
    pub fn is_dark(self) -> bool {
        match self {
            Mode::Light => false,
            Mode::Dark => true,
            // Hiba vagy "nem tudom" esetén világos.
            Mode::System => matches!(dark_light::detect(), Ok(dark_light::Mode::Dark)),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: ThemeColors,
    pub typography: Typography,
    pub ui: UiStyle,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UiStyle {
    pub titlebar_height: f32,
    pub menu_button_height: f32,
    pub menu_button_horizontal_padding: u16,
    pub submenu_max_width: f32,
    pub sidebar_indicator_width: f32,
    pub sidebar_indicator_height: f32,
    pub sidebar_icon_size: f32,
    pub sidebar_button_size: f32,
    pub sidebar_button_spacing: f32,
    pub titlebar_hover_radius: f32,
    pub sidebar_button_radius: f32,
}

impl Theme {
    pub fn palette(&self, mode: Mode) -> &ColorPalette {
        if mode.is_dark() {
            &self.colors.dark
        } else {
            &self.colors.light
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ThemeColors {
    pub light: ColorPalette,
    pub dark: ColorPalette,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ColorPalette {
    pub background: HexColor,
    pub surface: HexColor,
    pub surface_elevated: HexColor,
    pub letters: HexColor,

    pub titlebar_background: HexColor,
    pub sidebar_background: HexColor,
    pub hover: HexColor,
    pub titlebar_hover: HexColor,
    pub sidebar_selected: HexColor,
    pub sidebar_button_hover: HexColor,
    pub sidebar_selected_text: HexColor,
    pub sidebar_text: HexColor,

    pub text_primary: HexColor,
    pub text_secondary: HexColor,
    pub text_disabled: HexColor,

    pub accent: HexColor,
    pub border: HexColor,
    pub focus: HexColor,

    pub success: HexColor,
    pub warning: HexColor,
    pub danger: HexColor,
}
