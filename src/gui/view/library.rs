use iced::Length::Shrink;
use iced::widget::pane_grid::Target;
use iced::widget::rule::{horizontal, vertical};
use iced::widget::{
    PaneGrid, button, column, container, grid, pane_grid, row, rule, scrollable, space, text,
};
use iced::{Alignment, Background, Theme};
use iced::{Element, Length::Fill};

use crate::component::button::listing_button;
use crate::component::style::base_bg_container_style;
use crate::component::table::ResizableTable;
use crate::{AppState, Message};

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
struct TableState {
    column_widths: Vec<f32>,
    headers: Vec<String>,
    data: Vec<Vec<String>>,
}

#[derive(Debug)]
pub struct LibraryView {
    view_layout: ViewLayout,
    filter_sidebar: FilterSidebar,
    pane_state: pane_grid::State<PaneState>,
    table_state: TableState,
}

#[derive(Debug, Clone)]
pub enum LibraryMessage {
    ColumnResized(usize, f32),
    PaneResized(pane_grid::ResizeEvent),
    PaneDragged(pane_grid::DragEvent),
    ButtonPressed,
}

pub fn player_library(app_state: &AppState) -> Element<'_, Message> {
    column![
        container(column![
            LibraryView::view(&app_state.library_view).map(Message::Library)
        ])
        .width(Fill)
        .height(Fill)
        .padding(4)
        .style(container::bordered_box),
    ]
    .into()
}

impl TableState {
    fn new() -> Self {
        Self {
            column_widths: vec![100.0, 500.0, 120.0],
            headers: vec![
                "Track No.".to_string(),
                "Title".to_string(),
                "Duration".to_string(),
            ],
            data: vec![
                vec!["1".into(), "Yes! I Am a Long Way From Home".into(), "3:20".into()],
                vec!["1".into(), "Yes! I Am a Long Way From Home".into(), "3:20".into()],
                vec!["1".into(), "Yes! I Am a Long Way From Home".into(), "3:20".into()],
                vec!["1".into(), "Yes! I Am a Long Way From Home".into(), "3:20".into()],
                vec!["1".into(), "Yes! I Am a Long Way From Home".into(), "3:20".into()],
                vec!["1".into(), "Yes! I Am a Long Way From Home".into(), "3:20".into()],
                vec!["1".into(), "Yes! I Am a Long Way From Home".into(), "3:20".into()],
                vec!["1".into(), "Yes! I Am a Long Way From Home".into(), "3:20".into()],
                vec!["1".into(), "Yes! I Am a Long Way From Home".into(), "3:20".into()],
                vec!["1".into(), "Yes! I Am a Long Way From Home".into(), "3:20".into()],
                vec!["1".into(), "Yes! I Am a Long Way From Home".into(), "3:20".into()],
            ],
        }
    }
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
            table_state: TableState::new(),
        }
    }
}

impl LibraryView {
    pub fn view(&self) -> Element<'_, LibraryMessage> {
        match self.view_layout {
            ViewLayout::GroupedLayout => grouped_layout_view(self),
            ViewLayout::TableLayout => space().into(),
        }
    }

    pub fn update(&mut self, message: LibraryMessage) {
        match message {
            LibraryMessage::ColumnResized(index, new_width) => {
                if let Some(w) = self.table_state.column_widths.get_mut(index) {
                    *w = new_width;
                }
            }
            LibraryMessage::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.pane_state.resize(split, ratio);
            }
            LibraryMessage::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                if let Target::Pane(other, _) = target {
                    self.pane_state.swap(pane, other);
                }
                // self.pane_state.swap(pane, target);
            }
            LibraryMessage::PaneDragged(_) => {}
            LibraryMessage::ButtonPressed => {}
        }
    }
}

fn grouped_layout_view(library_view: &LibraryView) -> Element<'_, LibraryMessage> {
    // Panes
    PaneGrid::new(&library_view.pane_state, |_pane, state, _is_maximized| {
        let content = match state.pane_type {
            PaneType::Main => library_content_view(library_view),
            PaneType::Sidebar => filter_sidebar_view(library_view),
        };

        // let controls: Element<'_, LibraryMessage> = row![button("-"),].spacing(5).into();

        pane_grid::Content::new(content).title_bar(
            pane_grid::TitleBar::new(
                text(match state.pane_type {
                    PaneType::Main => "Library",
                    PaneType::Sidebar => "Filters",
                })
                .width(Fill)
                .align_x(Alignment::Center),
            )
            .style(container::bordered_box)
            .padding(4),
        )
    })
    .on_resize(10, LibraryMessage::PaneResized)
    .on_drag(LibraryMessage::PaneDragged)
    .height(Fill)
    .spacing(4)
    .into()
}

fn library_content_view(library_view: &LibraryView) -> Element<'_, LibraryMessage> {
    scrollable(
        row![
            container(
                column![
                    container(text("Mogwai").size(18)).padding(6),
                    album_content(library_view),
                    horizontal(1),
                    album_content(library_view),
                    horizontal(1),
                    album_content(library_view),
                    horizontal(1),
                    container(text("Mogwai").size(18)).padding(6),
                    album_content(library_view),
                ]
                .spacing(4)
            )
            .width(Fill)
            .padding(0),
        ]
        .spacing(4),
    )
    .direction(scrollable::Direction::Vertical(scrollable::Scrollbar::new()))
    .into()
}

fn album_content(library_view: &LibraryView) -> Element<'_, LibraryMessage> {
    container(
        row![
            album_content_left_pane(library_view),
            vertical(1),
            album_content_right_pane(library_view),
        ]
        .spacing(4),
    )
    // .style(base_bg_container)
    .padding(4)
    .height(Shrink)
    .width(Fill)
    .into()
}

fn album_content_left_pane(_library_view: &LibraryView) -> Element<'static, LibraryMessage> {
    column![
        container(space())
            .style(base_bg_container_style)
            .width(200)
            .height(200),
        column![
            text("Young Team").size(18),
            text("Mogwai").size(14),
            text("1997").size(14),
            text("1:05:02").size(14),
        ],
    ]
    .spacing(8)
    .into()
}

fn album_content_table(library_view: &LibraryView) -> Element<'_, LibraryMessage> {
    let headers: Vec<Element<LibraryMessage>> = library_view
                .table_state
                .headers
                .iter()
                .map(|h| {
                    container(text(h).size(20))
                        .padding(4)
                        .width(Fill)
                        .into()
                })
                .collect();

    let rows: Vec<Vec<Element<LibraryMessage>>> = library_view
        .table_state
        .data
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| {
                    container(text(cell).size(16))
                        .padding(4)
                        .width(Fill)
                        .into()
                })
            .collect()
        })
    .collect();

    let table = ResizableTable::new(&library_view.table_state.column_widths, LibraryMessage::ColumnResized)
        .headers(headers)
        .rows(rows)
        .min_width(60.0);

    container(table)
        .into()
}

fn album_content_right_pane(library_view: &LibraryView) -> Element<'_, LibraryMessage> {
    scrollable(
        container(row![
            column![
                album_content_table(library_view),
            ]
            .spacing(4),
        ])
        .style(base_bg_container_style)
        .padding(4),
    )
    .direction(scrollable::Direction::Horizontal(
        scrollable::Scrollbar::new(),
    ))
    .into()
}

fn filter_sidebar_view(_library_view: &LibraryView) -> Element<'static, LibraryMessage> {
    container(
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
            ),
            rule::horizontal(1),
            column![
                listing_button("Mogwai", LibraryMessage::ButtonPressed),
                listing_button("Mogwai", LibraryMessage::ButtonPressed),
                listing_button("Mogwai", LibraryMessage::ButtonPressed),
                listing_button("Mogwai", LibraryMessage::ButtonPressed),
                listing_button("Mogwai", LibraryMessage::ButtonPressed),
                listing_button("Mogwai", LibraryMessage::ButtonPressed),
                listing_button("Mogwai", LibraryMessage::ButtonPressed),
                listing_button("Mogwai", LibraryMessage::ButtonPressed),
                listing_button("Mogwai", LibraryMessage::ButtonPressed),
            ]
            .spacing(4),
        ]
        .spacing(4),
    )
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
