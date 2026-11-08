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

use iced::Color;

/// Egy szín, ami a témafájlban `"#RRGGBB"` (vagy `"#RRGGBBAA"`) szövegként van,
/// betöltés után `iced::Color`.
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize)]
#[serde(try_from = "String")]
pub struct HexColor(Color);

impl HexColor {
    pub fn color(self) -> Color {
        self.0
    }
}

impl From<HexColor> for Color {
    fn from(c: HexColor) -> Color {
        c.0
    }
}

impl TryFrom<String> for HexColor {
    type Error = String;

    fn try_from(s: String) -> Result<Self, String> {
        let hex = s.trim().trim_start_matches('#');
        if !hex.is_ascii() || (hex.len() != 6 && hex.len() != 8) {
            return Err(format!("hibás színkód: {s:?} (várt: #RRGGBB vagy #RRGGBBAA)"));
        }
        let byte = |i: usize| {
            u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| format!("hibás színkód: {s:?}"))
        };
        let (r, g, b) = (byte(0)?, byte(2)?, byte(4)?);
        let a = if hex.len() == 8 { byte(6)? } else { 255 };
        Ok(Self(Color::from_rgba8(r, g, b, a as f32 / 255.0)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Result<HexColor, String> {
        HexColor::try_from(s.to_string())
    }

    #[test]
    fn parses_with_and_without_hash() {
        assert_eq!(parse("#FF8000").unwrap(), parse("ff8000").unwrap());
        assert_eq!(parse("#FF8000").unwrap().color(), Color::from_rgb8(255, 128, 0));
    }

    #[test]
    fn parses_alpha() {
        let c = parse("#00000080").unwrap().color();
        assert!((c.a - 128.0 / 255.0).abs() < 1e-6);
    }

    #[test]
    fn rejects_garbage_without_panicking() {
        assert!(parse("#12345").is_err());
        assert!(parse("#GGGGGG").is_err());
        assert!(parse("árvíz").is_err());
        assert!(parse("").is_err());
        assert!(parse("megszentségteleníthetetlenségeskedéseitekért").is_err());
        assert!(parse("a á b c").is_err());
        assert!(parse("Ⰺ⬪➬⽮♝⻝☱┃⫬⟶⺳⋢").is_err());
    }
}