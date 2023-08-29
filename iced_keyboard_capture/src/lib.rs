//! A container for capturing keyboard events.

use std::marker::PhantomData;

use iced_core::{
    event, keyboard,
    layout::{self, Layout},
    mouse, widget, Clipboard, Element, Event, Length, Rectangle, Renderer, Shell, Size,
    Widget, renderer
};

pub fn keyboard_capture<'a, M, R>() -> KeyboardCapture<'a, M, R>
where
    R: Renderer,
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

impl<'a, Message, R> Widget<Message, R> for KeyboardCapture<'a, Message, R>
where
    Message: Clone,
    R: Renderer,
{
    fn width(&self) -> Length {
        Length::Fixed(0.0)
    }

    fn height(&self) -> Length {
        Length::Fixed(0.0)
    }

    fn layout(&self, _renderer: &R, _limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::ZERO)
    }

    fn on_event(
        &mut self,
        _state: &mut widget::Tree,
        event: Event,
        _layout: Layout<'_>,
        _cursor_position: mouse::Cursor,
        _renderer: &R,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let Event::Keyboard(k) = event else { return event::Status::Ignored };

        let Some(on_key_event) = &self.on_key_event else { return event::Status::Ignored };

        if let Some(message) = on_key_event(k) {
            shell.publish(message);
            return event::Status::Captured;
        }
        event::Status::Ignored
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        _renderer: &mut R,
        _theme: &<R as Renderer>::Theme,
        _style: &renderer::Style,
        _layout: Layout<'_>,
        _cursor_position: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
    }
}

impl<'a, Message, R> From<KeyboardCapture<'a, Message, R>> for Element<'a, Message, R>
where
    Message: 'a + Clone,
    R: 'a + Renderer,
{
    fn from(area: KeyboardCapture<'a, Message, R>) -> Element<'a, Message, R> {
        Element::new(area)
    }
}
