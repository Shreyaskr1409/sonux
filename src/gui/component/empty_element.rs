use iced::{Element, widget::column};

use crate::Message;

pub fn empty_element() -> Element<'static, Message> {
    column![].into()
}
