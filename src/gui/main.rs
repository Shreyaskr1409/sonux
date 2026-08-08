mod component;
mod data;
mod query;
mod view;

use iced::{Element, Font, Task, Theme, widget::column};

use crate::{
    component::style::setup_fonts,
    view::{
        library::{LibraryMessage, LibraryView, player_library},
        player_footer::player_footer,
        player_header::player_header,
    },
};

#[derive(Debug, Default)]
pub struct AppState {
    library_view: LibraryView,
}

#[derive(Debug, Clone)]
pub enum Message {
    Library(LibraryMessage),
    Default,
    Minimize,
    Maximize,
    Exit,
}

pub fn new_app_state() -> AppState {
    AppState {
        library_view: LibraryView::default(),
    }
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
        player_library(&self)
    }

    fn footer(&self) -> Element<'_, Message> {
        player_footer()
    }
}

pub fn update(app_state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Library(msg) => {
            app_state.library_view.update(msg);
            ().into()
        }
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
        Message::Exit => iced::exit(),
    }
}

pub fn theme(_app_state: &AppState) -> Theme {
    Theme::KanagawaDragon
}

fn main() -> iced::Result {
    let font_families = setup_fonts();
    iced::application(new_app_state, update, view)
        .theme(theme)
        .default_font(Font::with_name(font_families.default_font_family))
        .run()
}
