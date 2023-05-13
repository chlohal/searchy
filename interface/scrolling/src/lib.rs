use std::sync::Arc;

use actions::actions::Action;
use iced::{
    widget::{
        container, container::Appearance, mouse_area, scrollable, scrollable::Properties, text,
        vertical_space, Column, Scrollable,
    },
    Alignment, Background, Color, Element, Length, Theme,
};

static PAGE_SIZE: usize = 20;
static ENTRY_HEIGHT: f32 = 50.0;

use messages::Message;
use once_cell::sync::Lazy;

pub static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

pub fn results_scrollbox(
    results: &Vec<Arc<Action>>,
    scroll_top: f32,
    selected: &Option<Arc<Action>>,
) -> Scrollable<'static, Message> {
    scrollable_subset_from(results, scroll_top, selected)
        .height(Length::Fill)
        .vertical_scroll(Properties::new())
        .on_scroll(|offset| Message::Scroll(offset.y))
        .id(SCROLLABLE_ID.clone())
}

fn scrollable_subset_from(
    results: &Vec<Arc<Action>>,
    scroll_top: f32,
    selected: &Option<Arc<Action>>,
) -> iced::widget::Scrollable<'static, Message> {
    let start_index: usize = std::cmp::min(
        results.len() - PAGE_SIZE,
        (scroll_top * ((results.len() - PAGE_SIZE) as f32)) as usize,
    );

    let end_index: usize = start_index + PAGE_SIZE;

    let slice = &results[start_index..end_index];

    let before_space = vertical_space(Length::Fixed(ENTRY_HEIGHT * (start_index as f32)));
    let after_space = vertical_space(Length::Fixed(
        ENTRY_HEIGHT * ((results.len() - end_index) as f32),
    ));

    let elem_iter = Some(Into::<Element<Message>>::into(before_space))
        .into_iter()
        .chain(slice.iter().map(|x| {
            mouse_area({
                let c = container(text(x.to_string()))
                    .height(Length::Fixed(ENTRY_HEIGHT))
                    .width(Length::Fill)
                    .center_y();

                if selected
                    .clone()
                    .map(|y| (x.clone() == y))
                    .unwrap_or_default()
                {
                    c.style(
                        selected_entry
                            as for<'a> fn(&'a Theme) -> iced::widget::container::Appearance,
                    )
                } else {
                    c
                }
            })
            .on_press(Message::ClickOption(x.clone()))
            .into()
        }))
        .chain(Some(Into::<Element<Message>>::into(after_space)).into_iter());

    scrollable(
        Column::with_children(elem_iter.collect::<Vec<Element<Message>>>())
            .width(Length::Fill)
            .align_items(Alignment::Start),
    )
}

fn selected_entry(_: &Theme) -> Appearance {
    Appearance {
        text_color: None,
        background: Some(Background::Color(Color::from_rgb8(0xff, 0x00, 0xaa))),
        border_radius: 0.0,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
    }
}
