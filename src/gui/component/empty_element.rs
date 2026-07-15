use iced::{Element, widget::column};

pub fn empty_element<MessageType: 'static>() -> Element<'static, MessageType> {
    column![].into()
}
