use crate::ui::app::App;
use crate::ui::messages::Message;
use iced::{
    Element,
    Length,
    widget::{container, text}}
;

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        container(text("Hello Iced"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .into()


    }
}