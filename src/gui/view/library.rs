use iced::widget::{PaneGrid, button, column, container, grid, pane_grid, row, text};
use iced::{Background, Theme};
use iced::{Element, Length::Fill};

use crate::Message;
use crate::{component::empty_element::empty_element};

#[derive(Debug, Default)]
pub enum ViewLayout {
    #[default]
    GroupedLayout,
    TableLayout,
}

#[derive(Debug, Default)]
struct FilterSidebar {
    width: u32,
}

#[derive(Debug)]
enum PaneType {
    Sidebar,
    Main,
}

#[derive(Debug)]
struct PaneState {
    pane_type: PaneType,
    title: String,
}

#[derive(Debug)]
pub struct LibraryView {
    view_layout: ViewLayout,
    filter_sidebar: FilterSidebar,
    pane_state: pane_grid::State<PaneState>,
}

#[derive(Debug, Clone)]
pub enum LibraryMessage {
    ResizeSidebar,
}

impl Default for LibraryView {
    fn default() -> Self {
        let (mut pane_state, sidebar_pane_id) = pane_grid::State::new(PaneState {
            pane_type: PaneType::Sidebar,
            title: "Library Filter Sidebar".into(),
        });

        if let Some((_main_pane_id, split)) = pane_state.split(
            pane_grid::Axis::Vertical,
            sidebar_pane_id, // TODO: put ids into PaneState
            PaneState {
                pane_type: PaneType::Main,
                title: "Library Content".into(),
            },
        ) {
            pane_state.resize(split, 0.28);
        }

        Self {
            view_layout: ViewLayout::GroupedLayout,
            filter_sidebar: FilterSidebar { width: 300 },
            pane_state: pane_state,
        }
    }
}

impl LibraryView {
    pub fn view(&self) -> Element<'_, Message> {
        match self.view_layout {
            ViewLayout::GroupedLayout => grouped_layout(self),
            ViewLayout::TableLayout => empty_element(),
        }
    }

    pub fn update(&mut self, message: LibraryMessage) {
        match message {
            LibraryMessage::ResizeSidebar => {}
        }
    }
}

fn grouped_layout(library_view: &LibraryView) -> Element<'_, Message> {
    // Panes
    PaneGrid::new(&library_view.pane_state, |_pane, state, _is_maximized| {
        let content = match state.pane_type {
            PaneType::Main => library_content_view(library_view),
            PaneType::Sidebar => filter_sidebar_view(library_view),
        };
        pane_grid::Content::new(content)
    })
    .spacing(4)
    .into()
}

fn library_content_view(_library_view: &LibraryView) -> Element<'static, Message> {
    row![
        container(text("Hello world"))
            .width(Fill)
            .height(Fill)
            .padding(4),
    ]
    .spacing(4)
    .into()
}

fn filter_sidebar_view(_library_view: &LibraryView) -> Element<'static, Message> {
    column![
        container(
            grid([
                button("A").into(),
                button("B").into(),
                button("C").into(),
                button("D").into(),
                button("E").into(),
                button("F").into(),
                button("A").into(),
                button("B").into(),
                button("C").into(),
                button("D").into(),
                button("E").into(),
                button("F").into(),
                button("A").into(),
                button("B").into(),
                button("C").into(),
                button("D").into(),
                button("E").into(),
                button("F").into(),
            ])
            .fluid(40)
            .spacing(4)
        )
        .style(|theme: &Theme| {
            let _palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(_palette.background.base.color)),
                ..container::rounded_box(theme)
            }
        })
        .padding(4)
        .height(Fill)
    ]
    .into()
}
