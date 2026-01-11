mod component;
mod font;

use iced::{
    Element, Font,
    Length::Fill,
    Task, Theme,
    widget::{column, container, rule, text},
};

use crate::font::setup_fonts;

#[derive(Debug, Default)]
pub struct AppState {}

#[derive(Debug, Clone)]
pub enum Message {
    Default,
    Minimize,
    Maximize,
    Exit,
}

pub fn new_app_state() -> AppState {
    AppState {}
}

pub fn view(app_state: &AppState) -> Element<'_, Message> {
    column![app_state.header(), app_state.content(), app_state.footer()]
        .spacing(4)
        .padding(4)
        .into()
}

impl AppState {
    fn header(&self) -> Element<'_, Message> {
        container(
            column![
                component::title_bar::title_bar(),
                rule::horizontal(1),
                component::player_header::player_header()
            ]
            .spacing(4),
        )
        .width(Fill)
        .padding(4)
        .style(container::bordered_box)
        .into()
    }

    fn content(&self) -> Element<'_, Message> {
        container(text("Listen-Listen").size(24))
            .width(Fill)
            .height(Fill)
            .padding(4)
            .style(container::bordered_box)
            .into()
    }

    fn footer(&self) -> Element<'_, Message> {
        container(text("Listen-Listen").size(24))
            .width(Fill)
            .padding(4)
            .style(container::bordered_box)
            .into()
    }
}

pub fn update(_app_state: &mut AppState, message: Message) -> Task<Message> {
    match message {
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
