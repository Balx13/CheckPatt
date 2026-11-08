use iced::{
    widget::{button, container, row, text},
    Background, Border, Color, Element, Length,
};

use crate::app::App;
use crate::ui::messages::Message;
use crate::app::themes::color;
use iced_aw::{Menu, menu_items, menu_bar, menu};

const HEIGHT: f32 = 20.0;
const MENU_HEIGHT: f32 = HEIGHT / 2.0;

fn hover_style(status: button::Status) -> button::Style {
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

fn menu_button(label: &str, msg: Message, color: Color) -> button::Button<'_, Message> {
    button(
        container(text(label)
            .style(move |_| text::Style { color: Some(color), }))
            .center_y(Length::Fill)
    )
        .style(|_theme, status| hover_style(status))
        .height(HEIGHT)
        .padding([0, 12])
        .on_press(msg)
}



pub fn view(app: &App) ->Element<'_, Message> {
    let background = app.palette().titlebar_background.into();
    let letter_color = app.palette().letters.into();

    container(menu_bar!(
        (menu_button("File", Message::None, letter_color)),

        (
            menu_button("View", Message::None, letter_color),
            menu!(
                (menu_button("Mode", Message::None, letter_color))
            ).max_width(100.0).close_on_item_click(false).offset(MENU_HEIGHT)
        ),

        (menu_button("Help", Message::None, letter_color))
    ) // Menu_bar settings
        .close_on_item_click_global(true)
        .close_on_background_click_global(true)
        .safe_bounds_margin(f32::MAX)
    ) // Container settings
        .width(Length::Fill)
        .height(HEIGHT)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(background)),
            ..container::Style::default()
        })
        .into()
}

pub fn vview(app: &App) -> Element<'_, Message> {
    let background: Color = app.palette().titlebar_background.into();
    let button_color = app.palette().letters.into();

    container(row![
        menu_button("File", Message::None, button_color),
        menu_button("View", Message::None, button_color),
        menu_button("Help", Message::None, button_color),
    ])
        .width(Length::Fill)
        .height(HEIGHT)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(background)),
            ..container::Style::default()
        })
        .into()
}