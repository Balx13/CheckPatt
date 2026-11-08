/*
CheckPatt
Copyright (C) 2026  Balázs André

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TextStyle {
    pub size: u32,
    pub weight: Weight,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Typography {
    pub font: FontFamily,
    pub text: TextStyle,
    pub window_title: TextStyle,
    pub section_title: TextStyle,
    pub highlight: TextStyle,
    pub secondary_text: TextStyle,
    pub help_text: TextStyle,
    pub control_label: TextStyle,
    pub monospace_text: TextStyle,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum FontFamily {
    System,
    Custom(String),
}

#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub enum Weight {
    Normal,
    Medium,
    Semibold,
    Bold,
}

impl From<Weight> for iced::font::Weight {
    fn from(w: Weight) -> Self {
        match w {
            Weight::Normal => Self::Normal,
            Weight::Medium => Self::Medium,
            Weight::Semibold => Self::Semibold,
            Weight::Bold => Self::Bold,
        }
    }
}