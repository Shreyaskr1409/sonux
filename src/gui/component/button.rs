use iced::{
    Alignment, Element, Length::{self, Fill},
    widget::{self, button, container, text},
};

use crate::{Message, component::style::listing_button_style};

pub fn centered_button(txt: &str, w: f32, h: f32, text_size: f32) -> Element<'static, Message> {
    widget::button(
        container(text(String::from(txt)).size(text_size))
            .width(Length::Fixed(w))
            .height(Length::Fixed(h))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .into()
}

pub fn listing_button<MessageType: Clone + 'static>(txt: &str, message: MessageType) -> Element<'static, MessageType> {
    widget::button(
        text(String::from(txt))
    )
    .on_press(message)
    .width(Fill)
    .padding(4)
    .style(listing_button_style)
    .into()
}

pub fn song_listing_button<MessageType: Clone + 'static>(txt: &str, message: MessageType) -> Element<'static, MessageType> {
    widget::button(
        text(String::from(txt)).size(14)
    )
    .on_press(message)
    .width(Fill)
    .padding(4)
    .style(button::text)
    .into()
}
