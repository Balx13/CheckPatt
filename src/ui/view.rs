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


use crate::app::App;
use crate::ui::{
    titlebar,
    sidebar,
    messages::Message
};
use iced::{
    Element,
    Length,
    Background,
    widget::{container, column, text}}
;

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        container(
        column![
            titlebar::view(&self),
            sidebar::view(&self),
        ])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(self.palette().background.into())),
                text_color: Some(self.palette().letters.into()),
                ..Default::default()
            })
            .into()
    }
}