//! A container for capturing mouse events.

use iced::{keyboard, Element, event, Event};

use iced_native::{Widget, Length};

pub struct KeyboardCapture<'a, Message, Renderer> {
    content: Element<'a, Message, Renderer>,
    on_key_event: Option<Box<dyn Fn(keyboard::Event) -> Message + 'a>>,
}

impl<'a, Message, Renderer> KeyboardCapture<'a, Message, Renderer> {
    pub fn on_key_event<F: Fn(keyboard::Event) -> Message + 'a>(mut self, evt: F) -> Self {
        self.on_key_event = Some(Box::new(evt));
        self
    }

    pub fn new(content: impl Into<Element<'a, Message, Renderer>>) -> Self {
        KeyboardCapture {
            content: content.into(),
            on_key_event: None
        }
    }

}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for KeyboardCapture<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced_native::Renderer
{
    fn width(&self) -> Length {
        self.content.as_widget().width()
    }

    fn height(&self) -> Length {
        self.content.as_widget().height()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.content.as_widget().layout(renderer, limits)
    }

    fn draw(
        &self,
        state: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_native::Renderer>::Theme,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_native::Point,
        viewport: &iced_native::Rectangle,
    ) {
        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            viewport,
        );
    }
}

impl<'a, Message, Renderer> From<KeyboardCapture<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    fn from(
        area: KeyboardCapture<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(area)
    }
}