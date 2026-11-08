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
        Button,
        button
}};

use iced::widget::text;
use crate::{
    app::App,
    ui::messages::{Message, TabId}
};


pub fn view(app: &App) -> Element<'_, Message> {

    row![
        sidebar(&app),

        match app.active_tab {
            TabId::Home => home::view(&app),
            TabId::Settings => settings::view(&app),
        }
    ]
        .into()
}

/*
fn sidebar(app: &App) -> Element<'_, Message> {
    container(
        column![
                Sidebar::new(Message::TabSelected)
                    .push(TabId::Home, TabLabel::Text("Home".into()))
                    //.width(60)
                    .height(Length::Shrink)
                    .style(sidebar_style)
                    .set_active_tab(&app.active_tab),

                space().height(Length::Fill),

                button("Settings").on_press(Message::TabSelected(TabId::Settings)),
        ].height(Length::Fill)
    ).into()
}
*/
fn sidebar(app: &App) -> Element<'_, Message> {
    container(
        column![
            sidebar_button(&app.assets.icons.home, TabId::Home, &app),
            space().height(Length::Fill),
            sidebar_button(&app.assets.icons.settings, TabId::Settings, &app)
        ]
    )
        .style(|_theme| {
            container::Style {
                background: Some(Color::from_rgb8(230, 229, 223).into()),
                ..Default::default()
            } })
        .into()
}

fn sidebar_button<'a>(
    icon: &'a svg::Handle,
    tab: TabId,
    app: &App,

) -> Element<'a, Message> {

    let selected = app.active_tab == tab;

    row![
    container("")
        .width(2)
        .height(24)
        .style(move |_| container::Style {
            background: if selected {
                Some(Color::from_rgb8(50, 100, 220).into())
            } else {
                None
            },
            ..Default::default()
        }),

    button(
        container(svg(icon.clone())).width(12).height(12)
            .center(22)
    )
    .on_press(Message::TabSelected(tab))
    .style(move |_theme, status|{

            let background = match (selected, status) {
                (true, button::Status::Active) |
                (true, button::Status::Hovered)|
                (false, button::Status::Hovered)=> {
                    Some(Color::from_rgb8(210, 210, 210).into())
                }
                _ => None,
            };

            let text_color = if selected {Color::WHITE} else {Color::BLACK};

            button::Style {
                background,
                text_color,
                border: iced::Border {
                    radius: 16.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
    ]
        .spacing(4)
        .into()
}