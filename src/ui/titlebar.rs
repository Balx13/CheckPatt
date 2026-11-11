use iced::{
    widget::{button, container, text},
    Background, Border, Color, Element, Length,
};

use crate::app::App;
use crate::ui::messages::Message;
use iced_aw::{menu_bar, menu};

#[derive(Clone, Copy)]
struct TitlebarStyle {
    background: Color,
    text: Color,
    hover: Color,
    height: f32,
    menu_button_height: f32,
    menu_button_horizontal_padding: u16,
    submenu_max_width: f32,
    hover_radius: f32,
}

impl TitlebarStyle {
    fn from_app(app: &App) -> Self {
        let ui = &app.theme().ui;
        Self {
            background: app.palette().titlebar_background.into(),
            text: app.palette().letters.into(),
            hover: app.palette().titlebar_hover.into(),
            height: ui.titlebar_height,
            menu_button_height: ui.menu_button_height,
            menu_button_horizontal_padding: ui.menu_button_horizontal_padding,
            submenu_max_width: ui.submenu_max_width,
            hover_radius: ui.titlebar_hover_radius,
        }
    }

    fn menu_button<'a>(&self, label: &'a str, msg: Message) -> button::Button<'a, Message> {
        let style = *self;
        button(
            container(text(label)
                .style(move |_| text::Style { color: Some(style.text) }))
                .center_y(Length::Fill)
        )
            .style(move |_theme, status| {
                let mut button_style = button::Style::default();
                button_style.background = Some(Background::Color(style.background));
                button_style.text_color = style.text;
                if status == button::Status::Hovered {
                    button_style.background = Some(Background::Color(style.hover));
                    button_style.border = Border {
                        radius: style.hover_radius.into(),
                        ..Border::default()
                    };
                }
                button_style
            })
            .height(style.menu_button_height)
            .padding([0, style.menu_button_horizontal_padding])
            .on_press(msg)
    }
}

pub fn view(app: &App) -> Element<'_, Message> {
    let style = TitlebarStyle::from_app(app);

    container(menu_bar!(
        (style.menu_button("File", Message::None)),

        (
            style.menu_button("View", Message::None),
            menu!(
                (style.menu_button("Mode", Message::None))
            ).max_width(style.submenu_max_width).close_on_item_click(false).offset(style.menu_button_height / 2.0)
        ),

        (style.menu_button("Help", Message::None))
    )
        .style(move |_theme, _status| iced_aw::style::menu_bar::Style {
            bar_background: style.background.into(),
            menu_background: style.background.into(),
            ..Default::default()
        })
        .close_on_item_click_global(true)
        .close_on_background_click_global(true)
        .safe_bounds_margin(f32::MAX)
    )
        .width(Length::Fill)
        .height(style.height)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(style.background)),
            ..container::Style::default()
        })
        .into()
}
