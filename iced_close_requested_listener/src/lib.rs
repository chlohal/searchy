//! A container for capturing mouse events.

use std::marker::PhantomData;

use iced::{event, Element, Event};

use iced_native::{
    window::Event::{CloseRequested, Unfocused},
    Length, Size, Widget,
};

pub fn close_requested_listener<'a, Message, Renderer>(
) -> CloseRequestedListener<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
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

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for CloseRequestedListener<'a, Message, Renderer>
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

impl<'a, Message, Renderer> From<CloseRequestedListener<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    fn from(area: CloseRequestedListener<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(area)
    }
}
