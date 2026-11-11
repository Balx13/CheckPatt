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

mod home;
mod settings;

use iced::{
    Element, Length, Color, Background,
    widget::{
        row,
        container,
        column,
        space,
        svg,
        button
}};

use crate::{
    app::App,
    ui::messages::{Message, TabId}
};

#[derive(Clone, Copy)]
struct SidebarStyle {
    background: Color,
    indicator_width: f32,
    indicator_height: f32,
    icon_size: f32,
    button_size: f32,
    button_spacing: f32,
    button_radius: f32,
    selected: Color,
    hover: Color,
    selected_text: Color,
    text: Color,
}

impl SidebarStyle {
    fn from_app(app: &App) -> Self {
        let palette = app.palette();
        let ui = &app.theme().ui;
        Self {
            background: palette.sidebar_background.into(),
            indicator_width: ui.sidebar_indicator_width,
            indicator_height: ui.sidebar_indicator_height,
            icon_size: ui.sidebar_icon_size,
            button_size: ui.sidebar_button_size,
            button_spacing: ui.sidebar_button_spacing,
            button_radius: ui.sidebar_button_radius,
            selected: palette.sidebar_selected.into(),
            hover: palette.sidebar_button_hover.into(),
            selected_text: palette.sidebar_selected_text.into(),
            text: palette.sidebar_text.into(),
        }
    }
}


pub fn view(app: &App) -> Element<'_, Message> {

    row![
        sidebar(&app),

        match app.active_tab {
            TabId::Home => home::view(&app),
            TabId::Settings => settings::view(&app),
        }
    ]
        .height(Length::Fill)
        .into()
}

fn sidebar(app: &App) -> Element<'_, Message> {
    let style = SidebarStyle::from_app(app);
    container(
        column![
            sidebar_button(&app.assets.icons.home, TabId::Home, &app, style),
            space().height(Length::Fill),
            sidebar_button(&app.assets.icons.settings, TabId::Settings, &app, style)
        ]
            .height(Length::Fill)
    )
        .height(Length::Fill)
        .style(move |_theme| {
            container::Style {
                background: Some(Background::Color(style.background)),
                ..Default::default()
            } })
        .into()
}

fn sidebar_button<'a>(
    icon: &'a svg::Handle,
    tab: TabId,
    app: &App,
    style: SidebarStyle,
) -> Element<'a, Message> {

    let selected = app.active_tab == tab;

    row![
    container("")
        .width(style.indicator_width)
        .height(style.indicator_height)
        .style(move |_| container::Style {
            background: if selected {
                Some(style.selected.into())
            } else {
                None
            },
            ..Default::default()
        }),

    button(
        container(svg(icon.clone()).style(move |_theme, _status| svg::Style {
            color: Some(style.text),
        }))
            .width(style.icon_size)
            .height(style.icon_size)
            .center(style.button_size)
    )
    .on_press(Message::TabSelected(tab))
    .style(move |_theme, status|{

            let background = match (selected, status) {
                (true, button::Status::Active) |
                (true, button::Status::Hovered)|
                (false, button::Status::Hovered)=> {
                    Some(style.hover.into())
                }
                _ => None,
            };

            let text_color = if selected { style.selected_text } else { style.text };

            button::Style {
                background,
                text_color,
                border: iced::Border {
                    radius: style.button_radius.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
    ]
        .spacing(style.button_spacing)
        .into()
}
