use iced::{
    executor,
    widget::{column, scrollable, text_input},
    window::{self, Position},
    Alignment, Application, Command, Element, Settings, Theme,
};

use interface_scrolling::{ENTRY_HEIGHT, PAGE_SIZE, SCROLLABLE_ID};
use interface_searchbox::{searchbox, SEARCHBOX_ID};
use messages::Message;
use results::{results_view, ActionsSearch, SearchType};
use std::{collections::VecDeque, sync::Arc, time::Instant};

use actions::actions::{action_database::ActionDatabase, Action};
use ipc_communication::message::IpcMessage;

use iced_keyboard_capture::keyboard_capture;
use unix_stream_subscription::unix_stream_subscription;

static MS_BETWEEN_SEARCHES: u128 = 60;

pub fn open_window(actions: Arc<ActionDatabase>) -> Result<(), iced::Error> {
    let mut settings = Settings::with_flags(actions);
    settings.window.decorations = false;
    settings.window.always_on_top = true;
    settings.window.visible = false;
    settings.exit_on_close_request = false;
    settings.id = Some("searchy".to_string());
    settings.window.position = Position::Centered;
    settings.window.size = (1920 * 5 / 12, ENTRY_HEIGHT * (PAGE_SIZE as u32 + 1));
    SearchingWindow::run(settings)
}

pub struct SearchingWindow {
    pub last_search: Instant,
    pub search_query: String,
    pub do_type_stack: VecDeque<SearchType>,
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
                last_search: Instant::now(),
                search_query: "".into(),
                do_type_stack: {
                    let mut stack = VecDeque::new();
                    stack.push_back(SearchType::ApplicationLaunch(ActionsSearch {
                        selected: None,
                        actions: init_data,
                        results,
                        scroll_top: 0.0,
                    }));
                    stack
                },
            },
            Command::batch(vec![
                scrollable::snap_to(SCROLLABLE_ID.clone(), scrollable::RelativeOffset::START),
                text_input::focus(SEARCHBOX_ID.clone()),
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
            Message::GenericKey => text_input::focus(SEARCHBOX_ID.clone()),
            Message::HideWindow => {
                self.reset_state();
                window::change_mode(window::Mode::Hidden)
            }
            Message::Ipc(ipc_message) => match ipc_message {
                IpcMessage::OpenWindow => Command::batch(vec![
                    window::gain_focus(),
                    scrollable::snap_to(SCROLLABLE_ID.clone(), scrollable::RelativeOffset::START),
                    text_input::focus(SEARCHBOX_ID.clone()),
                    window::change_mode(window::Mode::Windowed),
                ]),
                IpcMessage::CloseProgram => window::close(),
                IpcMessage::Refresh => Command::none(),
                IpcMessage::AppSearch => {
                    self.reset_state();
                    Command::none()
                }
                IpcMessage::Javascript => {
                    self.do_type_stack
                        .push_back(SearchType::JavascriptRepl(Default::default()));
                    Command::none()
                }
            },
            Message::Search(query) => {
                self.search_query = query.clone();
                if Instant::now().duration_since(self.last_search).as_millis()
                    >= MS_BETWEEN_SEARCHES
                {
                    self.do_type_stack
                        .back_mut()
                        .unwrap()
                        .update(messages::SearchResultMessage::Search(query))
                } else {
                    Command::none()
                }
            }
            Message::ResultMessage(m) => self.do_type_stack.back_mut().unwrap().update(m),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let searchbox = searchbox(&self.search_query);

        let scrollbox = results_view(&self.do_type_stack.back().unwrap());

        let key_eventer = keyboard_capture().on_key_event(key_shortcuts::handle_key_event);

        let page = column!(key_eventer, searchbox, scrollbox).align_items(Alignment::Center);

        page.into()
    }
}

impl SearchingWindow {
    pub fn reset_state(&mut self) {
        self.search_query = "".to_string();

        //remove all doTypes except for the first one
        while self.do_type_stack.len() > 1 {
            self.do_type_stack.pop_back();
        }

        match self.do_type_stack.back_mut().unwrap() {
            SearchType::ApplicationLaunch(ref mut search) => {
                search.selected = None;
                search.results = search.actions.get_action_results("");
                search.scroll_top = 0.0;
            }
            _ => panic!("First doType must be an ApplicationLaunch"),
        }
    }
}
