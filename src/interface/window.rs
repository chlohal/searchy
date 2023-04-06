use druid::{
    piet::Text,
    widget::{Axis, Flex, Label, List, TextBox},
    AppLauncher, Data, FontDescriptor, FontFamily, Lens, PlatformError, Value, Widget, WidgetExt,
    WindowDesc, AppDelegate,
};
use std::sync::Arc;

use crate::actions::{action::Action, action_database::ActionDatabase};

use super::{scroll_to_bottom::ScrollsToBottom, search_keypress::SearchKeypress};

pub fn open_window(actions: ActionDatabase) -> Result<(), PlatformError> {
    let initial_results = get_action_results(&actions, "".to_string());
    let window = build_window(Arc::new(actions));

    let searching = SearchingWindow {
        search_query: "".into(),
        results: Arc::new(initial_results),
    };

    AppLauncher::with_window(
        WindowDesc::new(window)
            .set_always_on_top(true)
            .show_titlebar(false),
    )
    .delegate(LoggingDelegate {})
    .configure_env(|env, _state| {
        env.set(
            druid::theme::UI_FONT,
            FontDescriptor::new(FontFamily::MONOSPACE)
            .with_size(16.0)
        );
    })
    .launch(searching)?;

    Ok(())
}

struct LoggingDelegate {}

impl <T: Data> AppDelegate<T> for LoggingDelegate {
    fn event(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        window_id: druid::WindowId,
        event: druid::Event,
        data: &mut T,
        env: &druid::Env,
    ) -> Option<druid::Event> {
        println!("{:?}", event);
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
        println!("{:?}", cmd);
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

    fn window_removed(&mut self, id: druid::WindowId, data: &mut T, env: &druid::Env, ctx: &mut druid::DelegateCtx) {}
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
    pub results: Arc<Vec<Arc<Action>>>,
}

fn build_window(actions: Arc<ActionDatabase>) -> impl Widget<SearchingWindow> {
    let textbox = TextBox::new()
        .with_placeholder("Search")
        .lens(SearchingWindow::search_query)
        .controller(SearchKeypress::new(actions));

    Flex::column()
        .with_flex_child(
            Flex::column()
                .with_child(List::new(build_search_item).lens(SearchingWindow::results))
                .scroll()
                .controller(ScrollsToBottom::new())
                .expand_height()
                .expand_width(),
            1.0,
        )
        .with_child(Flex::row().with_flex_child(textbox.expand_width(), 1.0))
}

fn build_search_item() -> impl Widget<Arc<Action>> {
    let name = Label::dynamic(|name: &Arc<Action>, _| name.clone().to_string());

    Flex::row().with_child(name)
}
