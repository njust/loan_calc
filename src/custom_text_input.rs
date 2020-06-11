use iced_native::{layout, Element, Hasher, Layout, Length, MouseCursor, Point, Widget, Event, Clipboard, input::keyboard, input};
use iced_wgpu::{Defaults, Primitive, Renderer};

pub struct CustomTextInput<'s, Message>
where Message: Clone {
    element: iced_native::Element<'s, Message, Renderer>,
    has_focus: bool,
    on_tab: Box<dyn Fn(bool) -> Message>,
}

impl<'s, Message> CustomTextInput<'s, Message>
where Message: Clone {
    pub fn new<F: 'static + Fn(bool) -> Message >(element: iced_native::Element<'s, Message, Renderer>, has_focus: bool, on_tab: F) -> Self{
        Self { element, has_focus, on_tab: Box::new(on_tab) }
    }
}

impl<'s, Message> Widget<Message, Renderer> for CustomTextInput<'s, Message>
where Message: Clone {
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.element.layout(renderer, limits)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> (Primitive, MouseCursor) {
        self.element.draw(renderer, defaults, layout, cursor_position)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.element.hash_layout(state)
    }

    fn on_event(&mut self, event: Event, layout: Layout<'_>, cursor_position: Point, messages: &mut Vec<Message>, renderer: &Renderer, clipboard: Option<&dyn Clipboard>) {
        match event {
            Event::Keyboard(keyboard::Event::Input {state, key_code, modifiers}) => {
                if key_code == keyboard::KeyCode::Tab && state == input::ButtonState::Released && self.has_focus {
                    let msg = (self.on_tab)(modifiers.shift);
                    messages.push(msg);
                }
            }
            _ => ()
        }
        self.element.on_event(event, layout, cursor_position, messages, renderer, clipboard)
    }
}

impl<'a, Message> Into<Element<'a, Message, Renderer>> for CustomTextInput<'a, Message>
where Message : 'a + Clone
{
    fn into(self) -> Element<'a, Message, Renderer> {
        Element::new(self)
    }
}