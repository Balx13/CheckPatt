

use iced::{widget::{
    container,
    row,
    text,
    button
}, Background, Color, Border, Element, Length};
use crate::app::{
    App,
    themes::rgb
};
use crate::ui::messages::Message;

fn background_shadow(status: button::Status) -> button::Style {
    let mut style = button::Style::default();
    if status == button::Status::Hovered {
        style.background = Some(Background::Color(Color::from_rgba(0.25, 0.25, 0.25, 0.35)));
        style.border = Border {
            radius: 5.0.into(),
            ..Border::default()
        };
    }
    style
}

fn menu_button<'a>(txt: String, height: f32, msg: Message) -> button::Button<'a, Message> {
    button(
        container(text(txt)).center_y(Length::Fill)
    ).style(|_theme, status| {background_shadow(status)})
        .height(height)
        .padding([0, 12])
        .on_press(msg)
}

pub fn view(_app: &App) -> Element<'_, Message> {
    let height: f32 = 20.0;
    container(
        row![
        menu_button("File".to_string(), height, Message::None),
        menu_button("View".to_string(), height, Message::None),
        menu_button("Help".to_string(), height, Message::None),
    ])
        .width(Length::Fill)
        .height(height)
        .style(|_theme| container::Style {
            background: Some(Background::Color(
                Color::from_rgb(rgb(240.0), rgb(240.0), rgb(240.0))
            )),
            ..container::Style::default()
        })
        .into()
}