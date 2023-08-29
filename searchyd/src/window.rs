use iced::{
    executor,
    widget::{column, scrollable, text_input},
    window::{self, Position},
    Alignment, Application, Command, Element, Settings, Theme,
};

use iced_close_requested_listener::close_requested_listener;
use interface_scrolling::{ENTRY_HEIGHT, PAGE_SIZE, SCROLLABLE_ID};
use interface_searchbox::{searchbox, SEARCHBOX_ID};
use messages::Message;
use results::{results_view, ActionsSearch, SearchType};
use std::{collections::VecDeque, sync::Arc, time::Instant};

use actions::{
    actions::{action_database::ActionDatabase, Action},
    varieties::path_executables::run_shell_command::run_shell_command,
};
use ipc_communication::message::IpcMessage;

use iced_keyboard_capture::keyboard_capture;
use unix_stream_subscription::unix_stream_subscription;

static MS_BETWEEN_SEARCHES: u128 = 60;

pub fn open_window(actions: Arc<ActionDatabase>) -> Result<(), iced::Error> {
    let mut settings = Settings::with_flags(actions);
    settings.window.decorations = false;
    settings.window.visible = true;
    settings.window.resizable = false;
    settings.window.platform_specific.application_id = "Searchy".into();
    settings.exit_on_close_request = false;
    settings.id = Some("searchy".to_string());
    settings.window.position = Position::Centered;
    settings.window.size = (1920 * 5 / 12, ENTRY_HEIGHT * (PAGE_SIZE as u32 + 1));
    SearchingWindow::run(settings)
}

pub struct SearchingWindow {
    pub loading: bool,
    pub last_open: Instant,
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
                loading: true,
                last_open: Instant::now(),
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
                iced::font::load(icons::FONT_BRANDS_TTF_BYTES).map(|r| { eprintln!("{:?}", r); Message::GenericKey }),
                iced::font::load(icons::FONT_REGULAR_TTF_BYTES).map(|_| Message::GenericKey),
                iced::font::load(icons::FONT_SOLID_TTF_BYTES).map(|_| Message::GenericKey),
                scrollable::snap_to(SCROLLABLE_ID.clone(), scrollable::RelativeOffset::START),
                text_input::focus(SEARCHBOX_ID.clone()),
                Command::perform(async { Ok(()) }, |_: Result<_, ()>| Message::Loaded),
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
        if let Message::Loaded = message { self.loading = false; }
        if self.loading { return Command::none(); }

        match message {
            Message::Loaded => Command::none(),
            Message::GenericKey => text_input::focus(SEARCHBOX_ID.clone()),
            Message::HideWindow => {
                self.reset_state();
                window::change_mode(window::Mode::Hidden)
            }
            Message::Ipc(ipc_message) => match ipc_message {
                IpcMessage::OpenWindow => {
                    self.last_open = Instant::now();

                    Command::batch(vec![
                        window::gain_focus(),
                        scrollable::snap_to(
                            SCROLLABLE_ID.clone(),
                            scrollable::RelativeOffset::START,
                        ),
                        text_input::focus(SEARCHBOX_ID.clone()),
                        window::change_mode(window::Mode::Windowed),
                    ])
                }
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
            Message::ExecuteTypeShell => {
                log_err(run_shell_command(self.search_query.clone(), false));
                Command::perform((|| async { Message::HideWindow })(), |x| x)
            }
            Message::ExecuteTypeTerminal => {
                log_err(run_shell_command(self.search_query.clone(), true));
                Command::perform((|| async { Message::HideWindow })(), |x| x)
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let searchbox = searchbox(&self.search_query);

        let scrollbox = results_view(&self.do_type_stack.back().unwrap());

        let close_listener = close_requested_listener::<Message, _>().on_close(|| {
            if Instant::now()
                .saturating_duration_since(self.last_open)
                .as_millis()
                > 1000
            {
                Some(Message::HideWindow)
            } else {
                None
            }
        });

        let key_eventer =
            keyboard_capture::<Message, _>().on_key_event(key_shortcuts::handle_key_event);

        let page = column!(close_listener, key_eventer, searchbox, scrollbox)
            .align_items(Alignment::Center);

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

fn log_err<T, E: std::fmt::Debug>(err: Result<T, E>) {
    if let Err(err) = err {
        eprintln!("{:?}", err);
    }
}
