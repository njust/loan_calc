use iced::{Element, text_input, text_input::TextInput, Column};
use crate::style;
use crate::custom_text_input::CustomTextInput;

#[derive(Debug, Clone)]
pub enum FormMessage {
    TextInputMessage(usize, FormTextInputMessage)
}

#[derive(Debug, Clone)]
pub enum FormTextInputMessage {
    InputChanged(String),
    OnTab(bool)
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
        let has_focus = self.state.is_focused();
        CustomTextInput::new(
            TextInput::new(
                &mut self.state,
                &self.placeholder,
                &self.value,
                FormTextInputMessage::InputChanged)
                .style(style::FormTextInputStyle{}).into(), has_focus, FormTextInputMessage::OnTab
        ).into()
    }

    pub fn update(&mut self, msg: FormTextInputMessage) {
        match msg {
            FormTextInputMessage::InputChanged(text) => self.value = text,
            FormTextInputMessage::OnTab(_) => ()
        }
    }
}
#[derive(Default)]
pub struct Form {
    active: Option<usize>,
    inputs: Vec<Box<FormTextInput>>,
}

impl Form {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, placeholder: &str) {
        self.inputs.push(Box::new(FormTextInput::new(placeholder)));
    }

    pub fn select(&mut self, next: bool) {
        let idx = self.inputs.iter()
            .enumerate().find(|(idx, e) |e.state.is_focused())
            .map(|(idx, _e)| idx);

        if let Some(idx) = idx {
            let next_idx = if next {
              idx +1
            }else {
                if idx == 0 { 0 } else { idx -1 }
            };
            self.set_focus(idx, false);
            self.set_focus(next_idx, true);
        }else {
            self.set_focus(0, true);
        }
    }

    fn set_focus(&mut self, idx: usize, focus: bool) {
        if let Some(el) = self.inputs.get_mut(idx) {
            el.set_focus(focus);
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
                if let FormTextInputMessage::OnTab(shift) = &m {
                    self.select(!(*shift));
                }else {
                    if let Some(el) = self.inputs.get_mut(idx) {
                        el.update(m)
                    }
                }
            }
        }
    }
}