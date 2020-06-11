use iced::{Element, text_input, text_input::TextInput, Column};
use crate::style;
use crate::custom_text_input::CustomTextInput;

#[derive(Debug, Clone)]
pub enum FormMessage<I: 'static+ Clone + Copy> {
    TextInputMessage(I, usize, FormTextInputMessage)
}

#[derive(Debug, Clone)]
pub enum FormTextInputMessage {
    InputChanged(String),
    OnTab(bool)
}

#[derive(Default)]
pub struct FormTextInput<I: 'static+ Clone+ Copy> {
    state: text_input::State,
    id: I,
    value: String,
    placeholder: String,
    on_change: Option<Box<dyn Fn(String)>>,
}

impl<I: 'static+ Clone+ Copy> FormTextInput<I> {
    pub fn new(id: I, value: String, placeholder: &str) -> Self {
        Self {
            state: Default::default(),
            id,
            value,
            placeholder: String::from(placeholder),
            on_change: None
        }
    }

    pub fn on_change<F: 'static + Fn(String)>(mut self, f: F) -> Self {
        self.on_change = Some(Box::new(f));
        self
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
                .into(), has_focus, FormTextInputMessage::OnTab
        ).into()
    }

    pub fn update(&mut self, msg: FormTextInputMessage) {
        match msg {
            FormTextInputMessage::InputChanged(text) => {
                if let Some(on_change) = &self.on_change {
                    on_change(text.clone())
                }
                self.value = text;
            },
            FormTextInputMessage::OnTab(_) => ()
        }
    }
}
#[derive(Default)]
pub struct Form<I: 'static + Clone+ Copy> {
    active: Option<usize>,
    inputs: Vec<Box<FormTextInput<I>>>,
}

impl<I: 'static+ Clone+ Copy> Form<I> {
    pub fn new() -> Self {
        Self {
            active: None,
            inputs: vec![]
        }
    }

    pub fn push(mut self, id: I, placeholder: &str, value: Option<String>) -> Self {
        self.inputs.push(Box::new(FormTextInput::new(id, value.unwrap_or(String::new()), placeholder)));
        self
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

    pub fn view(&mut self) -> Element<FormMessage<I>> {
        self.inputs.iter_mut().enumerate().map(|(idx, el)| {
            let id = el.id.clone();
            el.view().map(move |m| FormMessage::TextInputMessage(id, idx, m))
        }).fold(Column::new().spacing(5), |acc, el| {
            acc.push(el)
        }).into()
    }

    pub fn update(&mut self, msg: FormMessage<I>) {
        match msg {
            FormMessage::TextInputMessage(_id, idx, m) => {
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