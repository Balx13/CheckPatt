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

use iced::{
    Element,
    Length,
    widget::{
        container,
        text
    }
};
use crate::app::App;
use crate::ui::messages::Message;

pub fn view(app: &App) -> Element<'_, Message> {
    container(text("Settings Page"))
        .width(Length::Fill)
        .height(Length::Fill)
        .center(Length::Fill)
        .into()
}