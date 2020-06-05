use iced::{Font, Text, Length, HorizontalAlignment};

pub const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/app.ttf"),
};

pub fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}

