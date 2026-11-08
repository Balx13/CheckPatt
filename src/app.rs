pub mod paths;
pub mod read;
pub mod themes;
mod assets;

use std::arch::asm;
use paths::AppPaths;
use themes::{ColorPalette, Mode, Theme};
use crate::ui::messages::{TabId, Message};

#[derive(Debug, serde::Deserialize)]
pub struct ThemeSettings {
    selected: usize,
    mode: Mode
}

pub struct App {
    pub paths: AppPaths,
    pub themes: Vec<Theme>,
    pub thm_settings: ThemeSettings,

    pub active_tab: TabId,
    pub assets: assets::Assets
}

impl App {
    pub fn build() -> Self {
        let paths = if read::installtoml().unwrap() {
            AppPaths::portable()
        } else {
            AppPaths::installed()
        };
        paths.init().unwrap();

        let themes = read::themes(&paths.datas).unwrap();
        assert!(!themes.is_empty());
        let thm_settings = read::themes_settings(&paths.datas).unwrap();
        let active_tab = TabId::Home;

        let assets = assets::Assets {
            icons: assets::Icons {
                home: read::assets(&paths.datas.join("assets"), "home.svg"),
                settings: read::assets(&paths.datas.join("assets"), "settings.svg"),
            }
        };

        Self {
            paths,
            themes,
            thm_settings,
            active_tab,
            assets
        }
    }

    pub fn theme(&self) -> &Theme {
        &self.themes[self.thm_settings.selected]
    }

    pub fn palette(&self) -> &ColorPalette {
        self.theme().palette(self.thm_settings.mode)
    }
}