use druid::{
    lens, theme,
    widget::{Flex, Label, List, Painter, TextBox},
    AppDelegate, AppLauncher, Color, Data, FontDescriptor, FontFamily, Insets, Lens, LensExt,
    PlatformError, RenderContext, Value, Widget, WidgetExt, WindowDesc,
};
use std::sync::Arc;

use crate::actions::{action::Action, action_database::ActionDatabase};

use super::{scroll_to_bottom::ScrollsToBottom, search_keypress::SearchKeypress};


pub fn open_window(actions: ActionDatabase) -> Result<(), PlatformError> {
    let initial_results = get_action_results(&actions, "".to_string());
    let window = build_window(Arc::new(actions));

    let searching = SearchingWindow::new(initial_results);

    let launcher = AppLauncher::with_window(
        WindowDesc::new(window)
            .set_always_on_top(true)
            .show_titlebar(false),
    )
    .delegate(LoggingDelegate {})
    .configure_env(|env, _state| {
        env.set(
            druid::theme::UI_FONT,
            FontDescriptor::new(FontFamily::MONOSPACE).with_size(16.0),
        );
        env.set(
            theme::SELECTED_TEXT_BACKGROUND_COLOR,
            Color::rgb8(0xB2, 0x3F, 0xDC),
        )
    });

    launcher.launch(searching)?;

    Ok(())
}

struct LoggingDelegate {}

impl<T: Data> AppDelegate<T> for LoggingDelegate {
    fn event(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        window_id: druid::WindowId,
        event: druid::Event,
        data: &mut T,
        env: &druid::Env,
    ) -> Option<druid::Event> {
        //println!("{:?}", event);
        Some(event)
    }

    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        target: druid::Target,
        cmd: &druid::Command,
        data: &mut T,
        env: &druid::Env,
    ) -> druid::Handled {
        //println!("{:?}", cmd);
        druid::Handled::No
    }

    fn window_added(
        &mut self,
        id: druid::WindowId,
        handle: druid::WindowHandle,
        data: &mut T,
        env: &druid::Env,
        ctx: &mut druid::DelegateCtx,
    ) {
    }

    fn window_removed(
        &mut self,
        id: druid::WindowId,
        data: &mut T,
        env: &druid::Env,
        ctx: &mut druid::DelegateCtx,
    ) {
    }
}

fn get_action_results(actions: &ActionDatabase, query: String) -> Vec<Arc<Action>> {
    actions
        .get_action_results(&query)
        .iter()
        .map(|x| x.clone())
        .collect()
}

#[derive(Clone, Data, Lens)]
pub struct SearchingWindow {
    pub search_query: String,
    pub selected_index: usize,
    pub results: Arc<Vec<Arc<Action>>>,
}
impl SearchingWindow {
    pub fn new(results: Vec<Arc<Action>>) -> SearchingWindow {
        SearchingWindow {
            search_query: "".into(),
            selected_index: results.len() - 1,
            results: Arc::new(results),
        }
    }
    pub fn get_selected(&self) -> Option<Arc<Action>> {
        return self.results.get(self.selected_index).cloned();
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

fn build_window(actions: Arc<ActionDatabase>) -> impl Widget<SearchingWindow> {
    let textbox = TextBox::new()
        .with_placeholder("Search")
        .lens(SearchingWindow::search_query)
        .controller(SearchKeypress::new(actions));

    Flex::column()
        .with_flex_child(
            List::new(build_search_item)
                .lens(lens::Identity.map(
                    |dat: &SearchingWindow| {
                        Arc::new(
                            dat.results
                                .clone()
                                .iter()
                                .map(|x| (dat.get_selected().contains(x), x.to_owned()))
                                .collect::<Vec<_>>(),
                        )
                    },
                    |dat: &mut SearchingWindow, x| {
                        let mut i: usize = 0;
                        dat.results = Arc::new(
                            x.iter()
                                .map(|s| {
                                    if s.0 {
                                        dat.selected_index = i;
                                    }
                                    i += 1;
                                    s.1.clone()
                                })
                                .collect(),
                        );
                    },
                ))
                .expand_width()
                .scroll()
                .vertical()
                .controller(ScrollsToBottom::new())
                .expand(),
            1.0,
        )
        .with_child(Flex::row().with_flex_child(textbox.expand_width(), 1.0))
}

fn build_search_item() -> impl Widget<(bool, Arc<Action>)> {
    let name = Label::dynamic(|action: &(bool, Arc<Action>), _| action.1.clone().to_string());

    Flex::row()
        .with_child(name)
        .expand_width()
        .background(Painter::new(|ctx, data: &(bool, Arc<Action>), env| {
            let bound = ctx.size().to_rect();
            if data.0 {
                ctx.fill(bound, &env.get(theme::SELECTED_TEXT_BACKGROUND_COLOR));
            } else {
                ctx.fill(bound, &env.get(theme::WINDOW_BACKGROUND_COLOR));
            }
        }))
        .on_click(|_, dat: &mut (bool, Arc<Action>), _| {
            dat.0 = true;
        })
}
