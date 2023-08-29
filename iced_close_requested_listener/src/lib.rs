//! A container for capturing mouse events.

use std::marker::PhantomData;

use iced_core::{
    event,
    layout::{self, Layout},
    mouse, widget, Clipboard, Element, Event, Length, Rectangle, Renderer, Shell, Size,
    Widget, renderer,
    window::Event::{CloseRequested, Unfocused}
};

pub fn close_requested_listener<'a, Message, Renderer>(
) -> CloseRequestedListener<'a, Message, Renderer> {
    CloseRequestedListener::new()
}

pub struct CloseRequestedListener<'a, Message, Renderer> {
    on_close: Option<Box<dyn Fn() -> Option<Message> + 'a>>,
    _p: PhantomData<Renderer>,
}

impl<'a, Message, Renderer> CloseRequestedListener<'a, Message, Renderer> {
    pub fn on_close<F: Fn() -> Option<Message> + 'a>(mut self, evt: F) -> Self {
        self.on_close = Some(Box::new(evt));
        self
    }

    pub fn new() -> Self {
        CloseRequestedListener::default()
    }
}

impl<'a, Message, Renderer> Default for CloseRequestedListener<'a, Message, Renderer> {
    fn default() -> Self {
        CloseRequestedListener {
            on_close: None,
            _p: PhantomData,
        }
    }
}

impl<'a, Message, R> Widget<Message, R>
    for CloseRequestedListener<'a, Message, R>
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

    fn layout(
        &self,
        _renderer: &R,
        _limits: &layout::Limits,
    ) -> layout::Node {
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
        if let Event::Window(CloseRequested | Unfocused) = event {
            let Some(on_close) = &self.on_close else { return event::Status::Ignored };

            if let Some(message) = on_close() {
                shell.publish(message);
            }
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

impl<'a, Message, R> From<CloseRequestedListener<'a, Message, R>>
    for Element<'a, Message, R>
where
    Message: 'a + Clone,
    R: 'a + Renderer,
{
    fn from(area: CloseRequestedListener<'a, Message, R>) -> Element<'a, Message, R> {
        Element::new(area)
    }
}
