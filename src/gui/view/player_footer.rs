use iced::{
    Element,
    Length::Fill,
    widget::{container, text},
};

use crate::Message;

pub fn player_footer() -> Element<'static, Message> {
    container(text("Listen-Listen").size(24))
        .width(Fill)
        .padding(4)
        .style(container::bordered_box)
        .into()
}
