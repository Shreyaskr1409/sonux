use iced::widget::pane_grid::Target;
use iced::widget::{PaneGrid, button, column, container, grid, pane_grid, row, rule, text};
use iced::{Alignment, Background, Theme};
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
    PaneResized(pane_grid::ResizeEvent),
    PaneDragged(pane_grid::DragEvent),
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
    pub fn view(&self) -> Element<'_, LibraryMessage> {
        match self.view_layout {
            ViewLayout::GroupedLayout => grouped_layout(self),
            ViewLayout::TableLayout => empty_element(),
        }
    }

    pub fn update(&mut self, message: LibraryMessage) {
        match message {
            LibraryMessage::PaneResized(pane_grid::ResizeEvent {split, ratio}) => {
                self.pane_state.resize(split, ratio);
            }
            LibraryMessage::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                if let Target::Pane(other, _) = target {
                    self.pane_state.swap(pane, other);
                }
                // self.pane_state.swap(pane, target);
            }
            LibraryMessage::PaneDragged(_) => {}
        }
    }
}

fn grouped_layout(library_view: &LibraryView) -> Element<'_, LibraryMessage> {
    // Panes
    PaneGrid::new(&library_view.pane_state, |_pane, state, _is_maximized| {
        let content = match state.pane_type {
            PaneType::Main => library_content_view(library_view),
            PaneType::Sidebar => filter_sidebar_view(library_view),
        };

        let controls: Element<'_, LibraryMessage> = row![
            button("-"),
        ].spacing(5).into();

        pane_grid::Content::new(content).title_bar(
            pane_grid::TitleBar::new(text(match state.pane_type {
                PaneType::Main => "Library",
                PaneType::Sidebar => "Filters",
            }).width(Fill).align_x(Alignment::Center))
            .style(container::bordered_box)
            .padding(4)
        )
    })
    .on_resize(10, LibraryMessage::PaneResized)
        .on_drag(LibraryMessage::PaneDragged)
        .height(Fill)
        .spacing(4)
        .into()
}

fn library_content_view(_library_view: &LibraryView) -> Element<'static, LibraryMessage> {
    row![
        container(text("Hello world"))
            .width(Fill)
            .height(Fill)
            .padding(4),
    ]
    .spacing(4)
    .into()
}

fn filter_sidebar_view(_library_view: &LibraryView) -> Element<'static, LibraryMessage> {
    container(column![
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
        ),

        rule::horizontal(1),

        column![
            container("Mogwai").width(Fill).padding(4).style(container::bordered_box),
            container("Mogwai").width(Fill).padding(4).style(container::bordered_box),
            container("Mogwai").width(Fill).padding(4).style(container::bordered_box),
            container("Mogwai").width(Fill).padding(4).style(container::bordered_box),
            container("Mogwai").width(Fill).padding(4).style(container::bordered_box),
            container("Mogwai").width(Fill).padding(4).style(container::bordered_box),
            container("Mogwai").width(Fill).padding(4).style(container::bordered_box),
            container("Mogwai").width(Fill).padding(4).style(container::bordered_box),
            container("Mogwai").width(Fill).padding(4).style(container::bordered_box),
        ].spacing(4),
    ].spacing(4))
    .padding(4)
    .style(|theme: &Theme| {
        let _palette = theme.extended_palette();
        container::Style {
            background: Some(Background::Color(_palette.background.base.color)),
            ..container::rounded_box(theme)
        }
    })
    .height(Fill)
    .into()
}
