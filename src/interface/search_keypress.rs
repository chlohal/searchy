use std::marker::PhantomData;
use std::sync::Arc;

use druid::commands::CLOSE_ALL_WINDOWS;
use druid::keyboard_types::Key;
use druid::widget::Controller;
use druid::{Env, Event, EventCtx, LifeCycle, LifeCycleCtx, Widget};

use crate::actions::action_database::ActionDatabase;

use super::window::SearchingWindow;

pub(crate) struct SearchKeypress<T> {
    actions: Arc<ActionDatabase>,
    _phantom: PhantomData<T>,
}

impl SearchKeypress<SearchingWindow> {
    pub fn new(actions: Arc<ActionDatabase>) -> Self {
        SearchKeypress {
            actions,
            _phantom: PhantomData,
        }
    }
}

impl<W: Widget<SearchingWindow>> Controller<SearchingWindow, W>
    for SearchKeypress<SearchingWindow>
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut SearchingWindow,
        env: &Env,
    ) {
        match event {
            Event::KeyDown(_keys) => match _keys.key {
                Key::Enter => {
                    data.run_selected();
                    ctx.submit_command(CLOSE_ALL_WINDOWS);
                }
                Key::ArrowDown => {
                    if data.selected_index < data.results.len() - 1 {
                        data.selected_index += 1;
                    }
                }
                Key::ArrowUp => {
                    if data.selected_index > 0 {
                        data.selected_index -= 1;
                    }
                }
                _ => {
                    data.results = Arc::new(self.actions.get_action_results(&data.search_query));
                    data.selected_index = data.results.len() - 1;

                    ctx.submit_command(super::scroll_to_bottom::SCROLL_TO_BOTTOM);
                }
            },
            _ => {}
        }

        child.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &SearchingWindow,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env);
    }
}
