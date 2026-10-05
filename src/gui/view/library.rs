use std::time::Duration;

use iced::Alignment::Center;
use iced::Length::Shrink;
use iced::widget::rule::{horizontal, vertical};
use iced::widget::{
    Column, PaneGrid, button, column, container, grid, pane_grid, responsive, row, rule,
    scrollable, space, text, text_editor,
};
use iced::{Alignment, Background, Padding, Task, Theme};
use iced::{Element, Length::Fill};

use crate::Message;
use crate::component::button::listing_button;
use crate::component::style::base_bg_container_style;
use crate::component::widget::table::ResizableTable;
use crate::data::music::{AlbumId, AlbumSummary, Artist, ArtistNameAsId, Library, SongId};
use crate::query::scan_folders;

// ----Messages----

#[derive(Debug, Default, Clone)]
pub enum OnClickEffect {
    #[default]
    Undefined,
    AlbumFilter(ActiveAlbumFilter),
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

/// Selected artist in SidePane
#[derive(Debug, Default, Clone)]
pub enum ActiveAlbumFilter {
    #[default]
    AllFilteredArtists,
    ArtistWithID(ArtistNameAsId),
}

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
            InitialFilter::Initial(c) => {
                (artist.0.to_lowercase().chars().next() == Some(*c))
                    || (artist.0.to_uppercase().chars().next() == Some(*c))
            }
        }
    }
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
struct TracklistTableState {
    column_widths: Vec<f32>,
    headers: Vec<String>,
}

impl TracklistTableState {
    fn placeholder() -> Self {
        Self {
            column_widths: vec![100.0, 500.0, 120.0],
            headers: vec![
                "Track No.".to_string(),
                "Title".to_string(),
                "Duration".to_string(),
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
    table_state: TracklistTableState,
    albums: Vec<AlbumSummary>,
}

/// State container for the sidebar filter pane, tracking artist filters and artists
#[derive(Debug)]
struct SidePaneState {
    artist_filter_char_list: Vec<char>,
    active_artist_filter: InitialFilter,
    artists: Vec<ArtistNameAsId>,
    selected_artist: ActiveAlbumFilter,
    path_textbox_state: TextboxState,
}

/// Now Playing information store
#[derive(Debug)]
struct NowPlayingState {
    _song: SongId,
    _album: AlbumId,
    _album_artist: ArtistNameAsId,
}

/// The root state and view manager for the music library interface.
#[derive(Debug)]
pub struct LibraryView {
    view_layout: LayoutMode,
    pane_state: pane_grid::State<PaneState>,
    side_pane_state: SidePaneState,
    main_pane_state: MainPaneState,
    _now_playing_state: Option<NowPlayingState>,
}

impl Default for LibraryView {
    fn default() -> Self {
        Self {
            view_layout: LayoutMode::GroupedLayout,
            pane_state: build_panes(),
            main_pane_state: MainPaneState {
                table_state: TracklistTableState::placeholder(),
                albums: Vec::new(),
            },
            side_pane_state: SidePaneState {
                artist_filter_char_list: Vec::new(),
                active_artist_filter: InitialFilter::All,
                artists: Vec::new(),
                selected_artist: ActiveAlbumFilter::AllFilteredArtists,
                path_textbox_state: TextboxState {
                    text_editor_content: text_editor::Content::new(),
                },
            },
            _now_playing_state: None,
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
    pub fn on_column_resized(&mut self, index: usize, width: f32) -> Task<LibraryMessage> {
        if let Some(w) = self
            .main_pane_state
            .table_state
            .column_widths
            .get_mut(index)
        {
            *w = width;
        }
        ().into()
    }

    pub fn on_pane_resized(&mut self, event: pane_grid::ResizeEvent) -> Task<LibraryMessage> {
        self.pane_state.resize(event.split, event.ratio);
        ().into()
    }

    pub fn on_pane_dragged(&mut self, event: pane_grid::DragEvent) -> Task<LibraryMessage> {
        if let pane_grid::DragEvent::Dropped {
            pane,
            target: pane_grid::Target::Pane(other, _),
        } = event
        {
            self.pane_state.swap(pane, other);
        }
        ().into()
    }

    pub fn on_scan_finished(&mut self, result: Result<String, String>) -> Task<LibraryMessage> {
        match result {
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
        }
    }

    pub fn on_artist_filter_clicked(
        &mut self,
        library: &Library,
        f: InitialFilter,
    ) -> Task<LibraryMessage> {
        match f {
            InitialFilter::All => {
                self.side_pane_state.active_artist_filter = InitialFilter::All;

                let mut artists: Vec<&Artist> = library.artists.values().collect();
                artists.sort_by(|a, b| a.id.0.to_lowercase().cmp(&b.id.0.to_lowercase()));

                self.side_pane_state.artists = artists.into_iter().map(|a| a.id.clone()).collect();
            }
            InitialFilter::Initial(c) => {
                self.side_pane_state.active_artist_filter = InitialFilter::Initial(c);

                let mut artists: Vec<&Artist> = library
                    .artists
                    .values()
                    .filter(|a| InitialFilter::Initial(c).matches(&a.id))
                    .collect();
                artists.sort_by(|a, b| a.id.0.to_lowercase().cmp(&b.id.0.to_lowercase()));

                self.side_pane_state.artists = artists.into_iter().map(|a| a.id.clone()).collect();
            }
        };
        self.on_artist_clicked(library, ActiveAlbumFilter::AllFilteredArtists)
    }

    pub fn on_artist_clicked(
        &mut self,
        library: &Library,
        active_album_filter: ActiveAlbumFilter,
    ) -> Task<LibraryMessage> {
        self.side_pane_state.selected_artist = active_album_filter.clone();
        let mut filtered_albums: Vec<AlbumId> = match active_album_filter {
            ActiveAlbumFilter::ArtistWithID(id) => library.albums_for_artists(&[&id]),
            ActiveAlbumFilter::AllFilteredArtists => {
                let matching_artists: Vec<&ArtistNameAsId> =
                    self.side_pane_state.artists.iter().collect();
                library.albums_for_artists(&matching_artists)
            }
        };

        filtered_albums.sort_by(|a, b| {
            a.title
                .to_lowercase()
                .cmp(&b.title.to_lowercase())
                .then_with(|| {
                    a.album_artist
                        .0
                        .to_lowercase()
                        .cmp(&b.album_artist.0.to_lowercase())
                })
        });

        self.main_pane_state.albums = filtered_albums
            .iter()
            .filter_map(|id| library.album_summary(id))
            .collect();
        self.main_pane_state.albums.sort_by(|a, b| {
            a.album_artist
                .0
                .to_lowercase()
                .cmp(&b.album_artist.0.to_lowercase())
        });
        ().into()
    }

    pub fn update(&mut self, message: LibraryMessage, library: &Library) -> Task<LibraryMessage> {
        match message {
            LibraryMessage::ColumnResized(index, new_width) => {
                self.on_column_resized(index, new_width)
            }
            LibraryMessage::PaneResized(event) => self.on_pane_resized(event),
            LibraryMessage::PaneDragged(event) => self.on_pane_dragged(event),

            LibraryMessage::ScanRequested => {
                Task::perform(scan_folders(), LibraryMessage::ScanFinished)
            }
            LibraryMessage::ScanFinished(result) => self.on_scan_finished(result),

            LibraryMessage::ButtonPressed(effect) => match effect {
                OnClickEffect::Undefined => ().into(),
                OnClickEffect::AlbumFilter(f) => self.on_artist_clicked(library, f),
                OnClickEffect::ArtistFilter(f) => self.on_artist_filter_clicked(library, f),
            },
        }
    }

    pub fn add_fields_from_library(&mut self, library: &Library) {
        self.side_pane_state.artist_filter_char_list = library.artist_initial_char();

        let mut artists: Vec<&Artist> = library.artists.values().collect();
        artists.sort_by(|a, b| a.id.0.to_lowercase().cmp(&b.id.0.to_lowercase()));

        self.side_pane_state.artists = artists.into_iter().map(|a| a.id.clone()).collect();

        let _ = self.on_artist_clicked(library, ActiveAlbumFilter::AllFilteredArtists);
    }
}

// ----Track Selection Section----

impl LibraryView {
    pub fn view<'a>(&'a self, library: &'a Library) -> Element<'a, LibraryMessage> {
        match self.view_layout {
            LayoutMode::GroupedLayout => grouped_layout_view(self, library),
            LayoutMode::_TableLayout => space().into(),
        }
    }
}

/// Starting point of UI creation and includes the actual view for the library
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

/// Called by LibraryView::view() for tracklists for each album grouped together.
/// Responsible for initializing panes for Sidebar and TrackSelection
fn grouped_layout_view<'a>(
    library_view: &'a LibraryView,
    library: &'a Library,
) -> Element<'a, LibraryMessage> {
    PaneGrid::new(&library_view.pane_state, |_pane, state, _is_maximized| {
        let content = match state.pane_type {
            PaneType::TrackSelection => tracklist_selection_pane(library_view),
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

/// Albums are expected to arrive sorted by album artist, so each artist gets one heading
fn tracklist_selection_pane(library_view: &LibraryView) -> Element<'_, LibraryMessage> {
    let mut content = Column::new()
        .spacing(4)
        .padding(Padding::new(0.0).top(8).bottom(4));
    let mut current_artist: Option<&ArtistNameAsId> = None;

    for album in &library_view.main_pane_state.albums {
        if current_artist != Some(&album.album_artist) {
            content = content.push(container(text(&album.album_artist.0).size(18)).padding(2));
            current_artist = Some(&album.album_artist);
        }
        content = content.push(album_content(library_view, album));
        content = content.push(horizontal(1));
    }

    scrollable(container(content).width(Fill))
        .direction(scrollable::Direction::Vertical(scrollable::Scrollbar::new()))
        .into()
}

/// Called by tracklist_selection_pane(), contains content for each album
fn album_content<'a>(
    library_view: &'a LibraryView,
    album: &'a AlbumSummary,
) -> Element<'a, LibraryMessage> {
    container(
        row![
            album_content_left_bar(library_view, album),
            vertical(1),
            album_content_right_bar(library_view, album),
        ]
        .spacing(4),
    )
    .padding(4)
    .height(Shrink)
    .width(Fill)
    .into()
}

/// Formats as `m:ss`, or `h:mm:ss` for an hour or longer
fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let (h, m, s) = (secs / 3600, (secs / 60) % 60, secs % 60);
    if h > 0 {
        format!("{h}:{m:02}:{s:02}")
    } else {
        format!("{m}:{s:02}")
    }
}

const LEFT_BAR_CONTENT_WIDTH: f32 = 200.00;

/// Left bar would display basic album information
fn album_content_left_bar<'a>(
    _: &'a LibraryView,
    album: &'a AlbumSummary,
) -> Element<'a, LibraryMessage> {
    let mut info = column![
        text(&album._title).size(18).width(LEFT_BAR_CONTENT_WIDTH),
        text(&album.album_artist.0)
            .size(14)
            .width(LEFT_BAR_CONTENT_WIDTH),
    ];

    if let Some(year) = album._year {
        info = info.push(text(year.to_string()).size(14));
    } else {
        info = info.push(text("1998").size(14));
    }

    if let Some(duration) = album._total_duration {
        info = info.push(text(format_duration(duration)).size(14));
    } else {
        info = info.push(text("1:04:38").size(14));
    }

    column![
        container(space())
            .style(base_bg_container_style)
            .width(LEFT_BAR_CONTENT_WIDTH)
            .height(LEFT_BAR_CONTENT_WIDTH),
        info,
    ]
    .spacing(8)
    .into()
}

/// Component for tracklist table
fn tracklist_table<'a>(
    library_view: &'a LibraryView,
    album: &'a AlbumSummary,
    effective_widths: Vec<f32>,
) -> Element<'a, LibraryMessage> {
    let headers: Vec<Element<LibraryMessage>> = library_view
        .main_pane_state
        .table_state
        .headers
        .iter()
        .map(|h| container(text(h).size(20)).padding(4).width(Fill).into())
        .collect();

    let rows: Vec<Vec<Element<LibraryMessage>>> = album
        .tracks
        .iter()
        .map(|track| {
            let duration = track.duration.map_or("--".to_string(), format_duration);
            [track.track_no.to_string(), track.title.clone(), duration]
                .into_iter()
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

/// Right bar would contain tracklist table with responsive sizing
fn album_content_right_bar<'a>(
    library_view: &'a LibraryView,
    album: &'a AlbumSummary,
) -> Element<'a, LibraryMessage> {
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
                column![tracklist_table(library_view, album, effective_widths),].spacing(4),
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

// ----Sidebar----

fn filter_sidebar_view<'a>(
    library_view: &'a LibraryView,
    library: &'a Library,
) -> Element<'a, LibraryMessage> {
    let initial_chars = library.artist_initial_char();

    // Start the vector with the '*' button
    let mut char_buttons: Vec<Element<'_, LibraryMessage>> = vec![
        button(text('*').align_x(Center).align_y(Center))
            .on_press(LibraryMessage::ButtonPressed(OnClickEffect::ArtistFilter(
                InitialFilter::All,
            )))
            .into(),
    ];
    // Extend it with the mapped initial characters
    char_buttons.extend(initial_chars.into_iter().map(|c| {
        button(text(c.to_string()).align_x(Center).align_y(Center))
            .on_press(LibraryMessage::ButtonPressed(OnClickEffect::ArtistFilter(
                InitialFilter::Initial(c),
            )))
            .into()
    }));

    // Start the vector with the 'show all' button
    let mut artist_buttons: Vec<Element<'_, LibraryMessage>> = vec![
        listing_button(
            "(show all)",
            LibraryMessage::ButtonPressed(OnClickEffect::AlbumFilter(
                ActiveAlbumFilter::AllFilteredArtists,
            )),
        )
        .into(),
    ];
    // Extend it with the filtered artists
    artist_buttons.extend(library_view.side_pane_state.artists.iter().map(|a| {
        listing_button(
            &a.0,
            LibraryMessage::ButtonPressed(OnClickEffect::AlbumFilter(
                ActiveAlbumFilter::ArtistWithID(a.clone()),
            )),
        )
        .into()
    }));

    column![
        container(
            column![
                container(grid(char_buttons).fluid(28).spacing(4)),
                rule::horizontal(1),
                scrollable(Column::with_children(artist_buttons).spacing(4))
                    .direction(scrollable::Direction::Vertical(
                        scrollable::Scrollbar::new().spacing(5)
                    ))
                    .height(Fill),
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
        .height(Fill),
        text_editor(
            &library_view
                .side_pane_state
                .path_textbox_state
                .text_editor_content
        )
        .placeholder("Enter a path in each line")
        .height(100),
        row![
            text("Search paths")
                .width(Fill)
                .height(Fill)
                .align_y(Alignment::Center),
            button("Scan for Music").on_press(LibraryMessage::ScanRequested),
        ]
        .width(Fill)
        .height(Shrink),
    ]
    .spacing(4)
    .into()
}
