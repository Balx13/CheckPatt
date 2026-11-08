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
use std::fs;
use std::path::PathBuf;
use iced::widget::svg;
use crate::app::themes::Theme;
use crate::app::assets::Icons;
use super::ThemeSettings;

pub fn themes(prefix_path: &PathBuf) -> Result<Vec<Theme>, Box<dyn std::error::Error>> {
    let path = prefix_path.join("themes.ron").to_str().unwrap().to_string();
    let content = std::fs::read_to_string(path)?;
    let themes: Vec<Theme> = ron::from_str(&content)?;
    Ok(themes)
}

pub fn themes_settings(prefix_path: &PathBuf) -> Result<ThemeSettings, Box<dyn std::error::Error>> {
    let path = prefix_path.join("theme_settings.toml").to_str().unwrap().to_string();
    let content: ThemeSettings = toml::from_str(std::fs::read_to_string(path)?.as_str()).unwrap();
    Ok(content)
}

pub fn installtoml() -> Result<bool, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("install.toml")?;
    let config: toml::Table = content.parse()?;
    Ok(config["portable-mode"].as_bool().unwrap())
}

pub fn assets(prefix_path: &PathBuf, asset_file: &str) -> svg::Handle {
    let path = prefix_path.join(asset_file).to_str().unwrap().to_string();
    svg::Handle::from_path(path)

}