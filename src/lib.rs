use iced::window;
use crate::ui::app::App;

mod core;
pub mod ui;
mod platform;

pub fn run() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .window(window::Settings {
            maximized: true,
            ..window::Settings::default()
        }).run()
}