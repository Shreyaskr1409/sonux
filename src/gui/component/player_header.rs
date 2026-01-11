use iced::{
    Alignment, Background, Element, Length, Padding, Theme,
    widget::{Space, column, container, row, text},
};

use crate::{Message, component::button::custom_button};

pub fn player_header() -> Element<'static, Message> {
    row![
        container(Space::new().height(150).width(150)).style(container::rounded_box),
        column![
            row![
                column![
                    column![
                        text("Postpostpartum").size(16),
                        text("Injury Reserve").size(12),
                        text("By the Time I Get to Phoenix").size(12),
                    ]
                    .spacing(4),
                    Space::new().height(Length::Fill),
                    row![
                        custom_button("\u{23EE}", 25.0, 25.0, 20.0),
                        custom_button("\u{25B6}", 25.0, 25.0, 16.0),
                        custom_button("\u{23ED}", 25.0, 25.0, 20.0),
                    ]
                    .spacing(4),
                ],
                Space::new().width(Length::Fill),
                column![
                    Space::new().height(Length::Fill),
                    row![
                        text("#9").size(64),
                        container(text("/11").size(16)).padding(Padding::new(0.0).bottom(17)),
                    ]
                    .align_y(Alignment::End),
                    Space::new().height(Length::Fill),
                ]
                .align_x(Alignment::End),
            ],
            container(Space::new().height(40).width(Length::Fill)).style(|theme: &Theme| {
                let _palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(_palette.background.base.color)),
                    ..container::rounded_box(theme)
                }
            }),
        ]
        .spacing(4),
    ]
    .spacing(4)
    .align_y(Alignment::Center)
    .height(Length::Shrink)
    .into()
}
