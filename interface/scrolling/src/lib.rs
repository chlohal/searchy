use std::sync::Arc;

use action_entry::make_entry;
use actions::Action;
use iced::{
    widget::{
        container, container::Appearance, mouse_area, scrollable, scrollable::Properties,
        vertical_space, Column, Scrollable,
    },
    Alignment, Background, Color, Command, Element, Length, Theme, BorderRadius,
};

pub static PAGE_SIZE: usize = 5;

static VISIBLE_PAGE_SIZE: usize = PAGE_SIZE * 3;
static ENTRY_HEIGHT_F: f32 = ENTRY_HEIGHT as f32;

use messages::{Message, SearchResultMessage};
use once_cell::sync::Lazy;
use styling::{COLOR_HIGHLIGHT_BG, ITEM_PADDING};
pub use styling::ENTRY_HEIGHT;

pub static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

pub fn scroll_to_view(index: usize, total_count: usize) -> Command<Message> {
    let index_pc = ((index + PAGE_SIZE) as f32) / (total_count as f32);

    scrollable::snap_to(
        SCROLLABLE_ID.clone(),
        scrollable::RelativeOffset {
            x: 0.0,
            y: index_pc,
        },
    )
}

pub fn results_scrollbox(
    results: &Vec<Arc<Action>>,
    scroll_top: f32,
    selected: &Option<Arc<Action>>,
) -> Scrollable<'static, Message> {
    scrollable_subset_from(results, scroll_top, selected)
        .height(Length::Fill)
        .direction(scrollable::Direction::Vertical(Properties::new()))
        .on_scroll(|offset| Message::ResultMessage(SearchResultMessage::Scroll(offset.relative_offset().y)))
        .id(SCROLLABLE_ID.clone())
}

fn scrollable_subset_from(
    results: &Vec<Arc<Action>>,
    scroll_top: f32,
    selected: &Option<Arc<Action>>,
) -> iced::widget::Scrollable<'static, Message> {
    let result_count = results.len();

    let start_index: usize = if result_count < VISIBLE_PAGE_SIZE {
        0
    } else {
        std::cmp::min(
            result_count - VISIBLE_PAGE_SIZE,
            (scroll_top * ((result_count - VISIBLE_PAGE_SIZE) as f32)) as usize,
        )
    };

    let end_index: usize = std::cmp::min(result_count, start_index + VISIBLE_PAGE_SIZE);

    let slice = &results[start_index..end_index];

    let before_space = vertical_space(Length::Fixed(ENTRY_HEIGHT_F * (start_index as f32)));
    let after_space = vertical_space(Length::Fixed(
        ENTRY_HEIGHT_F * ((results.len() - end_index) as f32),
    ));

    let elem_iter = Some(Into::<Element<Message>>::into(before_space))
        .into_iter()
        .chain(slice.iter().map(|x| {
            mouse_area({
                let c = container(make_entry(x))
                    .height(Length::Fixed(ENTRY_HEIGHT_F))
                    .width(Length::Fill)
                    .center_y()
                    .padding(ITEM_PADDING);

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
            .on_press(Message::ResultMessage(SearchResultMessage::ClickOption(x.clone())))
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
        background: Some(Background::Color(COLOR_HIGHLIGHT_BG.into())),
        border_radius: BorderRadius::from(0.0),
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
    }
}
