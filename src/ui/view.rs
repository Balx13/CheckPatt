use crate::app::App;
use crate::ui::{
    titlebar,
    messages::Message
};
use iced::{
    Element,
    Length,
    widget::{container, column, text}}
;

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        column![
            titlebar::view(self),
            container(text("Hello Iced"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center(Length::Fill),
        ]
            .into()
    }
}