use iced::window;
use app::App;

mod core;
mod ui;
mod platform;
mod app;

pub fn run() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .window(window::Settings {
            maximized: true,
            ..window::Settings::default()
        })
        .title("CheckPatt")
        .run()
}