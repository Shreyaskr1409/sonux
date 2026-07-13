use iced::{Element, Length::Fill, widget::{container, row, rule, text}};

use crate::{Message, component::empty_element::empty_element};

#[derive(Debug, Default)]
pub enum ViewLayout {
    #[default]
    GroupedLayout,
    TableLayout
}

#[derive(Debug, Default)]
struct FilterSidebar {
    width: u32,
}

#[derive(Debug)]
pub struct LibraryState {
    view_layout: ViewLayout,
    filter_sidebar: FilterSidebar
}

impl Default for LibraryState {
    fn default() -> Self {
        Self {
            view_layout: ViewLayout::GroupedLayout,
            filter_sidebar: FilterSidebar { width: 300 }
        }
    }
}

pub fn library_view(library_state: &LibraryState) -> Element<'static, Message> {
    match library_state.view_layout {
        ViewLayout::GroupedLayout => grouped_layout(library_state),
        ViewLayout::TableLayout => empty_element(),
    }
}

pub fn grouped_layout(library_state: &LibraryState) -> Element<'static, Message> {
    row![
        // todo: build a dynamic grid which would contain alphabets
        container("").width(library_state.filter_sidebar.width),
        rule::vertical(1),
        container(
            text("Hello world")
        )
            .width(Fill)
            .height(Fill)
            .padding(4)
    ].into()
}
