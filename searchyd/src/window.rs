use iced::{
    executor,
    widget::{column, scrollable, text_input},
    window, Alignment, Application, Command, Element, Settings, Theme,
};

use interface_scrolling::{results_scrollbox, SCROLLABLE_ID};
use interface_searchbox::{searchbox, SEARCHBOX_ID};
use messages::Message;
use std::sync::Arc;

use actions::actions::{action::Action, action_database::ActionDatabase};
use ipc_communication::message::IpcMessage;

use unix_stream_subscription::unix_stream_subscription;
use iced_keyboard_capture::keyboard_capture;

pub fn open_window(actions: Arc<ActionDatabase>) -> Result<(), iced::Error> {
    let mut settings = Settings::with_flags(actions);
    settings.window.decorations = false;
    settings.window.always_on_top = true;
    settings.window.visible = true;
    settings.exit_on_close_request = false;
    settings.id = Some("searchy".to_string());
    SearchingWindow::run(settings)
}

fn get_action_results(actions: &ActionDatabase, query: &str) -> Vec<Arc<Action>> {
    actions.get_action_results(query).to_vec()
}

pub struct SearchingWindow {
    pub search_query: String,
    pub selected: Option<Arc<Action>>,
    pub actions: Arc<ActionDatabase>,
    pub results: Vec<Arc<Action>>,
    pub scroll_top: f32,
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
            Command::none(),
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
                scrollable::snap_to(SCROLLABLE_ID.clone(), scrollable::RelativeOffset::END)
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
                self.update(Message::HideWindow)
            }
            Message::Ipc(ipc_message) => match ipc_message {
                IpcMessage::OpenWindow => Command::batch(vec![
                    window::gain_focus(),
                    scrollable::snap_to(SCROLLABLE_ID.clone(), scrollable::RelativeOffset::END),
                    text_input::focus(SEARCHBOX_ID.clone()),
                    window::change_mode(window::Mode::Windowed),
                ]),
                IpcMessage::CloseProgram => window::close(),
                IpcMessage::Refresh => Command::none(),
            },
            Message::HideWindow => {
                self.reset_state();
                window::change_mode(window::Mode::Hidden)
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let searchbox = searchbox(&self.search_query);

        let scrollbox = results_scrollbox(&self.results, self.scroll_top, &self.selected);

        let key_eventer = keyboard_capture().on_key_event(key_shortcuts::handle_key_event);

        let page = column!(scrollbox, searchbox, key_eventer).align_items(Alignment::Center);

        page.into()
    }
}

impl SearchingWindow {
    pub fn run_selected(&self) {
        let Some(to_run) = &self.selected else { return };

        match to_run.run() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error launching: {}", e)
            }
        }
    }

    pub fn reset_state(&mut self) {
        self.search_query = "".to_string();
        self.results = get_action_results(&self.actions, &self.search_query);
        self.selected = None;
        self.scroll_top = 1.0;
    }
}
