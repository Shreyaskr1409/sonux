use iced::{
    Background, Border, Theme,
    widget::{
        button::{self, Status},
        container,
    },
};

use iced::font;

pub struct FontConfig {
    pub default_font_family: &'static str,
}

pub const ZED_MONO_REGULAR_BYTES: &[u8] =
    include_bytes!("./../assets/fonts/ZedMono/ZedMonoNerdFont-Regular.ttf");

fn load_fonts() {
    _ = font::load(ZED_MONO_REGULAR_BYTES);
}

pub fn setup_fonts() -> FontConfig {
    load_fonts();
    FontConfig {
        default_font_family: "ZedMono NF",
    }
}

pub fn base_bg_container_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(Background::Color(palette.background.base.color)),
        ..container::rounded_box(theme)
    }
}

pub fn listing_button_style(theme: &Theme, status: Status) -> button::Style {
    let palette = theme.extended_palette();

    let base = button::Style {
        background: Some(palette.background.weakest.color.into()),
        text_color: palette.background.weakest.text,
        border: Border {
            width: 1.0,
            radius: 5.0.into(),
            color: palette.background.weak.color,
        },
        ..Default::default()
    };

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => button::Style {
            text_color: palette.background.base.text.scale_alpha(0.8),
            ..base
        },
        // Status::Disabled => base,
        Status::Disabled => button::Style {
            background: base
                .background
                .map(|background| background.scale_alpha(0.5)),
            text_color: palette.background.base.text.scale_alpha(0.5),
            ..base
        },
    }
}
