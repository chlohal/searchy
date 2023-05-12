//! A container for capturing mouse events.

use std::{marker::PhantomData};

use iced::{event, keyboard, Element, Event};

use iced_native::{Length, Size, Widget};



pub fn keyboard_capture<'a, Message, Renderer>() -> KeyboardCapture<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    KeyboardCapture::new()
}


pub struct KeyboardCapture<'a, Message, Renderer> {
    on_key_event: Option<Box<dyn Fn(keyboard::Event) -> Option<Message> + 'a>>,
    _p: PhantomData<Renderer>,
}

impl<'a, Message, Renderer> KeyboardCapture<'a, Message, Renderer> {
    pub fn on_key_event<F: Fn(keyboard::Event) -> Option<Message> + 'a>(mut self, evt: F) -> Self {
        self.on_key_event = Some(Box::new(evt));
        self
    }

    pub fn new() -> Self {
        KeyboardCapture::default()
    }
}

impl<'a, Message, Renderer> Default for KeyboardCapture<'a, Message, Renderer> {
    fn default() -> Self {
        KeyboardCapture {
            on_key_event: None,
            _p: PhantomData,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for KeyboardCapture<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced_native::Renderer,
{
    fn width(&self) -> Length {
        Length::Fixed(0.0)
    }

    fn height(&self) -> Length {
        Length::Fixed(0.0)
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        iced_native::layout::Node::new(Size::ZERO)
    }

    fn on_event(
        &mut self,
        _state: &mut iced_native::widget::Tree,
        event: Event,
        _layout: iced_native::Layout<'_>,
        _cursor_position: iced_native::Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut iced_native::Shell<'_, Message>,
    ) -> event::Status {
        let Event::Keyboard(k) = event else { return event::Status::Ignored };

        let Some(on_key_event) = &self.on_key_event else { return event::Status::Ignored };

        if let Some(message) = on_key_event(k) {
            shell.publish(message);
        }
        event::Status::Ignored
    }

    fn draw(
        &self,
        _state: &iced_native::widget::Tree,
        _renderer: &mut Renderer,
        _theme: &<Renderer as iced_native::Renderer>::Theme,
        _style: &iced_native::renderer::Style,
        _layout: iced_native::Layout<'_>,
        _cursor_position: iced_native::Point,
        _viewport: &iced_native::Rectangle,
    ) {
    }
}

impl<'a, Message, Renderer> From<KeyboardCapture<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    fn from(area: KeyboardCapture<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(area)
    }
}
