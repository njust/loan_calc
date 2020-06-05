use iced::widget::button::Style;
use iced::{Color, button, Text, Background};
use crate::util::icon;

pub struct Icons {}

impl Icons {
    pub fn edit_icon() -> Text {
        icon('c')
    }

    pub fn add_icon() -> Text {
        icon('b')
    }

    pub fn leave_icon() -> Text {
        icon('a')
    }

    pub fn delete_icon() -> Text {
        icon('d')
    }
    pub fn back_icon() -> Text {
        icon('g')
    }
}

struct Colors {}
impl Colors {
    fn blue() -> Color {
        Color::from_rgba8(0,190, 255, 0.7)
    }
}

pub struct ButtonStyle {
    pub active: bool,
}

impl iced::button::StyleSheet for ButtonStyle {
    fn active(&self) -> Style {
        let background = if self.active {
            Some(iced::Background::Color(Colors::blue()))
        }else {
            None
        };
        iced::button::Style {
            background,
            border_radius: 6,
            ..iced::button::Style::default()
        }
    }
}

pub struct IconButtonStyle {}
impl button::StyleSheet for IconButtonStyle {
    fn active(&self) -> Style {
        button::Style::default()
    }

    fn hovered(&self) -> Style {
        Style {
            text_color: Colors::blue(),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> Style {
        button::Style::default()
    }

    fn disabled(&self) -> Style {
        button::Style::default()
    }
}

pub struct ListButtonStyle {}
impl button::StyleSheet for ListButtonStyle {
    fn active(&self) -> Style {
        button::Style::default()
    }

    fn hovered(&self) -> Style {
        button::Style {
            background: Some(Background::Color(Colors::blue())),
            border_radius: 6,
            ..button::Style::default()
        }
    }
}
