use iced::Length::Shrink;
use iced::widget::pane_grid::Target;
use iced::widget::rule::{horizontal, vertical};
use iced::widget::{
    Column, PaneGrid, button, column, container, grid, pane_grid, responsive, row, rule,
    scrollable, space, text, text_editor,
};
use iced::{Alignment, Background, Task, Theme};
use iced::{Element, Length::Fill};

use crate::Message;
use crate::component::button::listing_button;
use crate::component::style::base_bg_container_style;
use crate::component::widget::table::ResizableTable;
use crate::data::music::{AlbumId, Artist, ArtistNameAsId, Library, SongId};
use crate::query::scan_folders;

// ----Messages----

#[derive(Debug, Default, Clone)]
pub enum OnClickEffect {
    #[default]
    Undefined,
    ArtistFilter(InitialFilter),
}

#[derive(Debug, Clone)]
pub enum LibraryMessage {
    ColumnResized(usize, f32),
    PaneResized(pane_grid::ResizeEvent),
    PaneDragged(pane_grid::DragEvent),
    ButtonPressed(OnClickEffect),
    ScanRequested,
    ScanFinished(Result<String, String>),
}

// ----UI States Enums----

/// For distinction in the layout of library content
#[derive(Debug, Default)]
enum LayoutMode {
    #[default]
    GroupedLayout,
    _TableLayout, // not implemented yet
}

/// For distinction in the type of content the pane contains
#[derive(Debug)]
enum PaneType {
    Sidebar,
    TrackSelection,
}

/// Selected artist filter char in SidePane
#[derive(Debug, Default, Clone)]
pub enum InitialFilter {
    #[default]
    All,
    Initial(char),
}

impl InitialFilter {
    /// Tells if the artist pass through given filter
    fn matches(&self, artist: &ArtistNameAsId) -> bool {
        match self {
            InitialFilter::All => true,
            InitialFilter::Initial(c) => artist.0.chars().next() == Some(*c),
        }
    }
}

/// Selected artist in SidePane
#[derive(Debug, Default)]
pub enum ActiveArtist {
    #[default]
    AllArtists,
    AllFilteredArtists,
    ArtistWithID(ArtistNameAsId),
}

// ----UI States----

/// Represents the configuration and title of an individual pane within the pane grid.
#[derive(Debug)]
struct PaneState {
    pane_type: PaneType,
    title: String,
}

/// Stores state variables, headers, data, and column widths for the tracklist table.
#[derive(Debug)]
struct TableState {
    column_widths: Vec<f32>,
    headers: Vec<String>,
    data: Vec<Vec<String>>,
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
                vec![
                    "1".into(),
                    "Yes! I Am a Long Way From Home".into(),
                    "3:20".into(),
                ],
                vec![
                    "1".into(),
                    "Yes! I Am a Long Way From Home".into(),
                    "3:20".into(),
                ],
                vec![
                    "1".into(),
                    "Yes! I Am a Long Way From Home".into(),
                    "3:20".into(),
                ],
                vec![
                    "1".into(),
                    "Yes! I Am a Long Way From Home".into(),
                    "3:20".into(),
                ],
                vec![
                    "1".into(),
                    "Yes! I Am a Long Way From Home".into(),
                    "3:20".into(),
                ],
                vec![
                    "1".into(),
                    "Yes! I Am a Long Way From Home".into(),
                    "3:20".into(),
                ],
                vec![
                    "1".into(),
                    "Yes! I Am a Long Way From Home".into(),
                    "3:20".into(),
                ],
                vec![
                    "1".into(),
                    "Yes! I Am a Long Way From Home".into(),
                    "3:20".into(),
                ],
                vec![
                    "1".into(),
                    "Yes! I Am a Long Way From Home".into(),
                    "3:20".into(),
                ],
                vec![
                    "1".into(),
                    "Yes! I Am a Long Way From Home".into(),
                    "3:20".into(),
                ],
                vec![
                    "1".into(),
                    "Yes! I Am a Long Way From Home".into(),
                    "3:20".into(),
                ],
            ],
        }
    }
}

// Needs renaming later on
/// To store "newline" separated paths to scan for music
#[derive(Debug)]
struct TextboxState {
    text_editor_content: text_editor::Content,
}

/// State container for the main content pane, housing the tracklist table and albums list.
#[derive(Debug)]
struct MainPaneState {
    table_state: TableState,
    albums: Vec<AlbumId>,
}

/// State container for the sidebar filter pane, tracking artist filters and artists
#[derive(Debug)]
struct SidePaneState {
    artist_filter_char_list: Vec<char>,
    artist_filter_selected_char: Option<char>,
    artists: Vec<ArtistNameAsId>,
    selected_artist: ActiveArtist,
    path_textbox_state: TextboxState,
}

/// Now Playing information store
#[derive(Debug)]
struct NowPlayingState {
    song: SongId,
    album: AlbumId,
    album_artist: ArtistNameAsId,
}

/// The root state and view manager for the music library interface.
#[derive(Debug)]
pub struct LibraryView {
    view_layout: LayoutMode,
    pane_state: pane_grid::State<PaneState>,
    side_pane_state: SidePaneState,
    main_pane_state: MainPaneState,
    now_playing_state: Option<NowPlayingState>,
}

impl Default for LibraryView {
    fn default() -> Self {
        Self {
            view_layout: LayoutMode::GroupedLayout,
            pane_state: build_panes(),
            main_pane_state: MainPaneState {
                table_state: TableState::new(),
                albums: Vec::new(),
            },
            side_pane_state: SidePaneState {
                artist_filter_char_list: Vec::new(),
                artist_filter_selected_char: None,
                artists: Vec::new(),
                selected_artist: ActiveArtist::AllArtists,
                path_textbox_state: TextboxState {
                    text_editor_content: text_editor::Content::new(),
                },
            },
            now_playing_state: None,
        }
    }
}

/// Build initial panes for LibraryView
fn build_panes() -> pane_grid::State<PaneState> {
    let (mut pane_state, sidebar_pane_id) = pane_grid::State::new(PaneState {
        pane_type: PaneType::Sidebar,
        title: "Filter".into(),
    });

    if let Some((_main_pane_id, split)) = pane_state.split(
        pane_grid::Axis::Vertical,
        sidebar_pane_id, // TODO: put ids into PaneState
        PaneState {
            pane_type: PaneType::TrackSelection,
            title: "Library".into(),
        },
    ) {
        pane_state.resize(split, 0.2);
    }

    pane_state
}

impl LibraryView {
    pub fn view<'a>(&'a self, library: &'a Library) -> Element<'a, LibraryMessage> {
        match self.view_layout {
            LayoutMode::GroupedLayout => grouped_layout_view(self, library),
            LayoutMode::_TableLayout => space().into(),
        }
    }

    pub fn update(&mut self, message: LibraryMessage, library: &Library) -> Task<LibraryMessage> {
        match message {
            LibraryMessage::ColumnResized(index, new_width) => {
                if let Some(w) = self
                    .main_pane_state
                    .table_state
                    .column_widths
                    .get_mut(index)
                {
                    *w = new_width;
                }
                ().into()
            }

            LibraryMessage::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.pane_state.resize(split, ratio);
                ().into()
            }

            LibraryMessage::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                if let Target::Pane(other, _) = target {
                    self.pane_state.swap(pane, other);
                }
                // self.pane_state.swap(pane, target);
                ().into()
            }

            LibraryMessage::ScanRequested => {
                Task::perform(scan_folders(), LibraryMessage::ScanFinished)
            }

            LibraryMessage::ScanFinished(result) => match result {
                Ok(resp) => {
                    match resp.as_str() {
                        "Works" => {
                            println!("Works");
                        }
                        _ => {}
                    }
                    ().into()
                }
                Err(_) => ().into(),
            },

            LibraryMessage::PaneDragged(_) => ().into(),
            LibraryMessage::ButtonPressed(effect) => match effect {
                OnClickEffect::Undefined => {
                    self.side_pane_state.artists = match self.side_pane_state.selected_artist {
                        ActiveArtist::AllArtists => {
                            let mut artists: Vec<&Artist> = library.artists.values().collect();
                            artists.sort_by(|a, b| {
                                a._id.0.to_lowercase().cmp(&b._id.0.to_lowercase())
                            });

                            let artist_ids: Vec<ArtistNameAsId> =
                                artists.into_iter().map(|a| a._id.clone()).collect();
                            artist_ids
                        }

                        ActiveArtist::AllFilteredArtists | ActiveArtist::ArtistWithID(_) => {
                            let mut artists: Vec<&Artist> = library
                                .artists
                                .values()
                                .filter(|a| {
                                    a._id
                                        .0
                                        .chars()
                                        .next()
                                        .eq(&self.side_pane_state.artist_filter_selected_char)
                                })
                                .collect();
                            artists.sort_by(|a, b| {
                                a._id.0.to_lowercase().cmp(&b._id.0.to_lowercase())
                            });

                            let artist_ids: Vec<ArtistNameAsId> =
                                artists.into_iter().map(|a| a._id.clone()).collect();
                            artist_ids
                        }
                    };
                    ().into()
                }
                OnClickEffect::ArtistFilter(f) => match f {
                    InitialFilter::All => {
                        self.side_pane_state.artist_filter_selected_char = None;

                        let mut artists: Vec<&Artist> = library.artists.values().collect();
                        artists.sort_by(|a, b| a._id.0.to_lowercase().cmp(&b._id.0.to_lowercase()));

                        self.side_pane_state.artists =
                            artists.into_iter().map(|a| a._id.clone()).collect();
                        ().into()
                    }
                    InitialFilter::Initial(c) => {
                        self.side_pane_state.artist_filter_selected_char = Some(c);

                        let mut artists: Vec<&Artist> = library
                            .artists
                            .values()
                            .filter(|a| a._id.0.chars().next().eq(&Some(c)))
                            .collect();
                        artists.sort_by(|a, b| a._id.0.to_lowercase().cmp(&b._id.0.to_lowercase()));

                        self.side_pane_state.artists =
                            artists.into_iter().map(|a| a._id.clone()).collect();
                        ().into()
                    }
                },
            },
        }
    }

    pub fn add_fields_from_library(&mut self, library: &Library) {
        self.side_pane_state.artist_filter_char_list = library.artist_initial_char();

        let mut artists: Vec<&Artist> = library.artists.values().collect();
        artists.sort_by(|a, b| a._id.0.to_lowercase().cmp(&b._id.0.to_lowercase()));

        self.side_pane_state.artists = artists.into_iter().map(|a| a._id.clone()).collect();
    }
}

pub fn player_library<'a>(
    library_view: &'a LibraryView,
    library: &'a Library,
) -> Element<'a, Message> {
    column![
        container(column![
            LibraryView::view(&library_view, &library).map(Message::Library)
        ])
        .width(Fill)
        .height(Fill)
        .padding(4)
        .style(container::bordered_box),
    ]
    .into()
}

fn grouped_layout_view<'a>(
    library_view: &'a LibraryView,
    library: &'a Library,
) -> Element<'a, LibraryMessage> {
    // Panes
    PaneGrid::new(&library_view.pane_state, |_pane, state, _is_maximized| {
        let content = match state.pane_type {
            PaneType::TrackSelection => library_content_view(library_view),
            PaneType::Sidebar => filter_sidebar_view(library_view, library),
        };

        // let controls: Element<'_, LibraryMessage> = row![button("-"),].spacing(5).into();

        pane_grid::Content::new(content).title_bar(
            pane_grid::TitleBar::new(text(&state.title).width(Fill).align_x(Alignment::Center))
                .style(container::bordered_box)
                .padding(4),
        )
    })
    .on_resize(10, LibraryMessage::PaneResized)
    .on_drag(LibraryMessage::PaneDragged)
    .height(Fill)
    .min_size(250)
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

fn album_content_table(
    library_view: &LibraryView,
    effective_widths: Vec<f32>,
) -> Element<'_, LibraryMessage> {
    let headers: Vec<Element<LibraryMessage>> = library_view
        .main_pane_state
        .table_state
        .headers
        .iter()
        .map(|h| container(text(h).size(20)).padding(4).width(Fill).into())
        .collect();

    let rows: Vec<Vec<Element<LibraryMessage>>> = library_view
        .main_pane_state
        .table_state
        .data
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| container(text(cell).size(16)).padding(4).width(Fill).into())
                .collect()
        })
        .collect();

    let table = ResizableTable::new(effective_widths, LibraryMessage::ColumnResized)
        .headers(headers)
        .rows(rows)
        .min_width(60.0);

    container(table).into()
}

fn album_content_right_pane(library_view: &LibraryView) -> Element<'_, LibraryMessage> {
    responsive(move |size| {
        // Account for outer padding (4 left + 4 right = 8px)
        let available_width = (size.width - 8.0).max(0.0);

        let raw_widths = &library_view.main_pane_state.table_state.column_widths;
        let total_widths: f32 = raw_widths.iter().sum();

        // Dynamically compute column widths:
        // Stretch proportionally if extra space exists, otherwise keep base widths for scrolling
        let effective_widths: Vec<f32> = if available_width > total_widths && total_widths > 0.0 {
            let mut widths = raw_widths.clone();
            let extra_space = available_width - total_widths;

            if let Some(last) = widths.last_mut() {
                *last += extra_space;
            }

            widths
        } else {
            raw_widths.clone()
        };

        scrollable(
            container(row![
                column![album_content_table(library_view, effective_widths),].spacing(4),
            ])
            .style(base_bg_container_style)
            .padding(4),
        )
        .direction(scrollable::Direction::Horizontal(
            scrollable::Scrollbar::new(),
        ))
        .into()
    })
    .height(Shrink)
    .into()
}

fn filter_sidebar_view<'a>(
    library_view: &'a LibraryView,
    library: &'a Library,
) -> Element<'a, LibraryMessage> {
    // let initial_chars = library.artist_initial_char();
    // let char_buttons: Vec<Element<'_, LibraryMessage>> = initial_chars
    //     .into_iter()
    //     .map(|c| {
    //         button(text(c.to_string()))
    //             // .on_press(LibraryMessage::ButtonPressed(ButtonEffect::ArtistFilter(
    //             //     SelectedArtistFilterChar::AllArtists,
    //             // )))
    //             .on_press(LibraryMessage::ButtonPressed(ButtonEffect::ArtistFilter(
    //                 SelectedArtistFilterChar::CertainChar(c),
    //             )))
    //             .into()
    //     })
    //     .collect();

    let initial_chars = library.artist_initial_char();

    // Start the vector with the '*' button
    let mut char_buttons: Vec<Element<'_, LibraryMessage>> = vec![
        button(text('*'))
            .on_press(LibraryMessage::ButtonPressed(OnClickEffect::ArtistFilter(
                InitialFilter::All,
            )))
            .into(),
    ];

    // Extend it with the mapped initial characters
    char_buttons.extend(initial_chars.into_iter().map(|c| {
        button(text(c.to_string()))
            .on_press(LibraryMessage::ButtonPressed(OnClickEffect::ArtistFilter(
                InitialFilter::Initial(c),
            )))
            .into()
    }));

    let artist_buttons: Vec<Element<'_, LibraryMessage>> = library_view
        .side_pane_state
        .artists
        .iter()
        .map(|a| {
            listing_button(
                &a.0,
                LibraryMessage::ButtonPressed(OnClickEffect::Undefined),
            )
            .into()
        })
        .collect();

    column![
        scrollable(
            container(
                column![
                    container(grid(char_buttons).fluid(40).spacing(4)),
                    rule::horizontal(1),
                    Column::with_children(artist_buttons).spacing(4),
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
        )
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::new().spacing(5)
        ))
        .height(Fill),
        row![
            text("Search paths")
                .width(Fill)
                .height(Fill)
                .align_y(Alignment::Center),
            button("Scan for Music").on_press(LibraryMessage::ScanRequested),
        ]
        .width(Fill)
        .height(Shrink),
        text_editor(
            &library_view
                .side_pane_state
                .path_textbox_state
                .text_editor_content
        )
        .placeholder("Enter a path in each line")
        .height(100),
    ]
    .spacing(4)
    .into()
}
