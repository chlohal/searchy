use iced::{
    executor,
    widget::{column, scrollable, text_input},
    window::{self, Position}, Alignment, Application, Command, Element, Settings, Theme,
};

use interface_scrolling::{results_scrollbox, SCROLLABLE_ID, scroll_to_view, ENTRY_HEIGHT, PAGE_SIZE};
use interface_searchbox::{searchbox, SEARCHBOX_ID};
use messages::Message;
use std::{sync::Arc, time::Instant};

use actions::actions::{action::Action, action_database::ActionDatabase};
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
    settings.window.size = (1920 * 5 / 12, ENTRY_HEIGHT * ( PAGE_SIZE as u32 + 1 ) );
    SearchingWindow::run(settings)
}

fn get_action_results(actions: &ActionDatabase, query: &str) -> Vec<Arc<Action>> {
    actions.get_action_results(query).to_vec()
}

pub struct SearchingWindow {
    pub last_search: Instant,
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
                last_search: Instant::now(),
                search_query: "".into(),
                selected: None,
                actions: init_data,
                results,
                scroll_top: 0.0,
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
            Message::Search(query) => {
                if Instant::now().duration_since(self.last_search).as_millis() >= MS_BETWEEN_SEARCHES {
                    self.results = get_action_results(&self.actions, &query);
                    self.search_query = query;
                    self.selected = self.results.first().cloned();

                    self.scroll_top = 0.0;
                    scrollable::snap_to(SCROLLABLE_ID.clone(), scrollable::RelativeOffset::START)
                } else {
                    Command::none()
                }
            }
            Message::ClickOption(action) => {
                self.selected = Some(action);

                //refocus searchbox
                text_input::focus(SEARCHBOX_ID.clone())
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
                    scrollable::snap_to(SCROLLABLE_ID.clone(), scrollable::RelativeOffset::START),
                    text_input::focus(SEARCHBOX_ID.clone()),
                    window::change_mode(window::Mode::Windowed),
                ]),
                IpcMessage::CloseProgram => window::close(),
                IpcMessage::Refresh => Command::none(),
            },
            Message::SelectNext => {
                let Some(selected) = &self.selected else { return Command::none() };

                let Some(index) = self.results.iter().position(|x| x == selected) else { return Command::none() };

                let Some(new_selected) = self.results.get(index + 1) else { return Command::none() };

                self.selected = Some(new_selected.clone());

                //scroll_to_view(index + 1, self.results.len())
                Command::none()
            }
            Message::SelectPrevious => {
                let Some(selected) = &self.selected else { return Command::none() };

                let Some(index) = self.results.iter().position(|x| x == selected) else { return Command::none() };

                if index == 0 {
                    return Command::none();
                }

                let Some(new_selected) = self.results.get(index - 1) else { return Command::none() };

                self.selected = Some(new_selected.clone());

                //scroll_to_view(index - 1, self.results.len())
                Command::none()
            }    
            Message::HideWindow => {
                self.reset_state();
                window::change_mode(window::Mode::Hidden)
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let searchbox = searchbox(&self.search_query);

        let scrollbox = results_scrollbox(&self.results, self.scroll_top, &self.selected);

        let key_eventer = keyboard_capture().on_key_event(key_shortcuts::handle_key_event);

        let page = column!(key_eventer, searchbox, scrollbox).align_items(Alignment::Center);

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
        self.scroll_top = 0.0;
    }
}
