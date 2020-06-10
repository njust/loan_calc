use iced::{Element, text_input, text_input::TextInput, Column};
use crate::style;
#[derive(Debug, Clone)]
pub enum FormMessage {
    TextInputMessage(usize, FormTextInputMessage)
}

#[derive(Debug, Clone)]
pub enum FormTextInputMessage {
    InputChanged(String)
}

#[derive(Default)]
pub struct FormTextInput {
    state: text_input::State,
    value: String,
    placeholder: String,
}

impl FormTextInput {
    pub fn new(placeholder: &str) -> Self {
        Self {
            placeholder: String::from(placeholder),
            ..Self::default()
        }
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.state = if focus {
            text_input::State::focused()
        }else {
            text_input::State::new()
        }
    }

    pub fn view(&mut self) -> Element<FormTextInputMessage> {
        TextInput::new(
            &mut self.state,
            &self.placeholder,
            &self.value,
            FormTextInputMessage::InputChanged)
            .style(style::FormTextInputStyle{})
            .into()
    }

    pub fn update(&mut self, msg: FormTextInputMessage) {
        match msg {
            FormTextInputMessage::InputChanged(text) => self.value = text
        }
    }
}
#[derive(Default)]
pub struct Form {
    inputs: Vec<Box<FormTextInput>>,
}

impl Form {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, placeholder: &str) {
        if let Some(prev) = self.inputs.last_mut() {
            prev.set_focus(false);
        }
        self.inputs.push(Box::new(FormTextInput::new(placeholder)));
        if let Some(current) = self.inputs.last_mut() {
            current.set_focus(true);
        }
    }

    pub fn view(&mut self) -> Element<FormMessage> {
        self.inputs.iter_mut().enumerate().map(
            |(idx, el)| el.view()
                .map(move |m| FormMessage::TextInputMessage(idx, m)))
            .fold(Column::new(), |acc, el| {
            acc.push(el)
        }).into()
    }

    pub fn update(&mut self, msg: FormMessage) {
        match msg {
            FormMessage::TextInputMessage(idx, m) => {
                if let Some(el) = self.inputs.get_mut(idx) {
                    el.update(m)
                }
            }
        }
    }
}