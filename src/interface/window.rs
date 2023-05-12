use iced::futures::stream::BoxStream;
use iced::subscription::{Recipe, self};
use iced::widget::container::Appearance;
use iced::widget::scrollable::Properties;
use iced::widget::{
    column, container, mouse_area, scrollable, text, text_input, vertical_space, Column,
};
use iced::{
    executor, Alignment, Application, Background, Color, Command, Element, Length, Settings, Theme, window, Subscription, Event
};

use once_cell::sync::Lazy;
use std::hash::Hasher;
use std::sync::Arc;

use crate::actions::{action::Action, action_database::ActionDatabase};
use crate::ipc_communication::message::IpcMessage;

use super::unix_stream_sub::{self, unix_stream_subscription};

static PAGE_SIZE: usize = 20;
static ENTRY_HEIGHT: f32 = 50.0;
static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);
static SEARCHBOX_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub fn open_window(actions: Arc<ActionDatabase>) -> Result<(), iced::Error> {
    let mut settings = Settings::with_flags(actions);
    settings.window.decorations = false;
    settings.window.always_on_top = true;
    settings.window.visible = false;
    settings.id = Some("searchy".to_string());
    SearchingWindow::run(settings)
}

fn get_action_results(actions: &ActionDatabase, query: &String) -> Vec<Arc<Action>> {
    actions
        .get_action_results(query)
        .iter()
        .map(|x| x.clone())
        .collect()
}

pub struct SearchingWindow {
    pub search_query: String,
    pub selected: Option<Arc<Action>>,
    pub actions: Arc<ActionDatabase>,
    pub results: Vec<Arc<Action>>,
    pub scroll_top: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    LaunchSelected,
    ClickOption(Arc<Action>),
    SelectNext,
    SelectPrevious,
    Search(String),
    Scroll(f32),
    Ipc(IpcMessage)
}

impl Application for SearchingWindow {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = Arc<ActionDatabase>;
    type Theme = Theme;

    fn new(init_data: Self::Flags) -> (SearchingWindow, iced::Command<Message>) {
        let results = init_data.iter().cloned().collect::<Vec<Arc<Action>>>();
        (
            SearchingWindow {
                search_query: "".into(),
                selected: None,
                actions: init_data,
                results,
                scroll_top: 1.0,
            },
            Command::batch(vec![
                scrollable::snap_to(
                    SCROLLABLE_ID.clone(),
                    scrollable::RelativeOffset::END
                ),
                text_input::focus(SEARCHBOX_ID.clone())
            ]),
        )
    }

    fn title(&self) -> String {
        "Searchy".to_string()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        unix_stream_subscription()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        match message {
            Message::Search(query) => {
                self.results = get_action_results(&self.actions, &query);
                self.search_query = query;
                self.selected = self.results.last().cloned();

                self.scroll_top = 1.0;
                scrollable::snap_to(
                    SCROLLABLE_ID.clone(),
                    scrollable::RelativeOffset::END
                )
            }
            Message::ClickOption(action) => {
                self.selected = Some(action);
                Command::none()
            }
            Message::Scroll(y) => {
                self.scroll_top = y;
                Command::none()
            }
            Message::LaunchSelected => {
                self.run_selected();
                window::change_mode(window::Mode::Hidden)
            },
            Message::Ipc(ipc_message) => {
                match ipc_message {
                    IpcMessage::OpenWindow => window::change_mode(window::Mode::Windowed),
                    IpcMessage::CloseProgram => window::close(),
                    IpcMessage::Refresh => Command::none(),
                }
            }
            _ => Command::none()
        }
        
    }

    fn view(&self) -> Element<Self::Message> {
        let searchbox = text_input("Search...", &self.search_query)
            .on_input(|content| Message::Search(content))
            .on_submit(Message::LaunchSelected)
            .width(Length::Fill)
            .id(SEARCHBOX_ID.clone());

        let scrollbox = scrollable_subset_from(&self.results, self.scroll_top, &self.selected)
            .height(Length::Fill)
            .vertical_scroll(Properties::new())
            .on_scroll(|offset| Message::Scroll(offset.y))
            .id(SCROLLABLE_ID.clone());

        column!(scrollbox, searchbox)
            .align_items(Alignment::Center)
            .into()
    }
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

                if selected.clone().map(|y| (x.clone() == y)).unwrap_or_default() {
                    c.style(selected_entry as for<'a> fn(&'a Theme) -> iced::widget::container::Appearance)
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

impl SearchingWindow {
    pub fn get_selected(&self) -> Option<Arc<Action>> {
        return self.selected.clone();
    }
    pub fn run_selected(&self) -> () {
        let Some(to_run) = &self.get_selected() else { return };

        match to_run.run() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error launching: {}", e)
            }
        }
    }
}
