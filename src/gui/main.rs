mod component;
mod data;
mod query;
mod view;

use iced::{Element, Font, Task, Theme, widget::column};
use rusqlite::Connection;

use crate::{
    component::style::setup_fonts,
    data::music::{Library, Metadata},
    query::{get_metadata, init},
    view::{
        library::{LibraryMessage, LibraryView, player_library},
        player_footer::player_footer,
        player_header::player_header,
    },
};

#[derive(Debug, Clone)]
pub enum ErrMessage {
    ErrDbInitFailed,
    ErrMetadataFetchingFailed,
}

#[derive(Debug, Clone)]
pub enum Message {
    Library(LibraryMessage),
    Error(ErrMessage),
    Default,
    Minimize,
    Maximize,
    Exit,
}

#[derive(Debug, Default)]
pub struct AppState {
    library_view: LibraryView,
    library: Library,
}

pub fn new_app_state() -> (AppState, Task<Message>) {
    let lib = Library::new();
    let mut state = AppState {
        library_view: LibraryView::default(),
        library: lib,
    };

    let conn: Connection = match init("sonux.sqlite.db") {
        Err(e) => {
            eprintln!("Failed to initialize database: {e}");
            return (
                state,
                Task::done(Message::Error(ErrMessage::ErrDbInitFailed)),
            );
        }
        Ok(c) => c,
    };

    let metadata_list: Vec<Metadata> = match get_metadata(conn) {
        Err(e) => {
            eprintln!("Error while fetching metadata: {e}");
            return (
                state,
                Task::done(Message::Error(ErrMessage::ErrMetadataFetchingFailed)),
            );
        }
        Ok(m) => m,
    };

    Library::populate_fields(&mut state.library, &metadata_list);

    state.library_view.add_fields_from_library(&state.library);

    (state, Task::none())
}

pub fn view(app_state: &AppState) -> Element<'_, Message> {
    column![app_state.header(), app_state.content(), app_state.footer()]
        .spacing(4)
        .padding(4)
        .into()
}

impl AppState {
    fn header(&self) -> Element<'_, Message> {
        player_header()
    }

    fn content(&self) -> Element<'_, Message> {
        player_library(&self.library_view, &self.library)
    }

    fn footer(&self) -> Element<'_, Message> {
        player_footer()
    }
}

pub fn update(app_state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Library(msg) => app_state
            .library_view
            .update(msg, &app_state.library)
            .map(Message::Library),

        Message::Default => {
            println!("Do nothing");
            ().into()
        }

        Message::Minimize => {
            println!("minimize trigger");
            ().into()
        }

        Message::Maximize => {
            println!("maximize trigger");
            ().into()
        }

        Message::Error(e) => match e {
            ErrMessage::ErrDbInitFailed => {
                println!("Handler ran for ErrDbInitFailed");
                ().into()
            }
            ErrMessage::ErrMetadataFetchingFailed => {
                println!("Handler ran for ErrMetadataFetchingFailed");
                ().into()
            }
        },

        Message::Exit => iced::exit(),
    }
}

pub fn theme(_app_state: &AppState) -> Theme {
    Theme::KanagawaDragon
}

fn main() {
    let font_families = setup_fonts();
    if let Err(e) = iced::application(new_app_state, update, view)
        .theme(theme)
        .default_font(Font::with_name(font_families.default_font_family))
        .run()
    {
        eprintln!("Application exited due to error: {e}");
    }
}
