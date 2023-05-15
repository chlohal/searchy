use iced::{
    widget::{container::Appearance, container, text_input, Container},
    Length, Theme, Background, Color,
};
use interface_scrolling::ENTRY_HEIGHT;
use messages::Message;
use once_cell::sync::Lazy;
use styling::{COLOR_DARK_BG, ITEM_PADDING};

pub static SEARCHBOX_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub fn searchbox(content: &String) -> Container<'static, Message> {
    container(
        text_input("Search", content)
            .on_input(Message::Search)
            .on_submit(Message::LaunchSelected)
            .width(Length::Fill)
            .id(SEARCHBOX_ID.clone()),
    )
    .height(Length::Fixed(ENTRY_HEIGHT as f32))
    .width(Length::Fill)
    .padding(ITEM_PADDING)
    .align_y(iced::alignment::Vertical::Center)
    .align_x(iced::alignment::Horizontal::Center)
    .style(|_: &Theme| -> Appearance {
        Appearance {
            text_color: None,
            background: Some(Background::Color(COLOR_DARK_BG.into())),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    } as for<'a> fn(&'a Theme) -> iced::widget::container::Appearance
    )
}
