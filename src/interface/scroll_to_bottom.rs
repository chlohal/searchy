use std::marker::PhantomData;
use std::sync::Arc;

use druid::widget::{Axis, Controller, Scroll};
use druid::{Command, Data, Env, Event, EventCtx, LifeCycle, LifeCycleCtx, Selector, Widget};

use crate::actions::action_database::ActionDatabase;

use super::window::SearchingWindow;

pub(crate) struct ScrollsToBottom<T> {
    _phantom: PhantomData<T>,
}

impl ScrollsToBottom<SearchingWindow> {
    pub fn new() -> Self {
        ScrollsToBottom {
            _phantom: PhantomData,
        }
    }
}

pub const SCROLL_TO_BOTTOM: Selector = Selector::new("searchy.scroll-to-bottom");

impl<T: Data, W: Widget<T>> Controller<T, Scroll<T, W>> for ScrollsToBottom<SearchingWindow> {
    fn event(
        &mut self,
        child: &mut Scroll<T, W>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        env: &Env,
    ) {
        match event {
            Event::WindowConnected => {
                ctx.submit_command(SCROLL_TO_BOTTOM);
            }
            Event::Command(command) if command.is(SCROLL_TO_BOTTOM) => {
                child.scroll_to_on_axis(ctx, Axis::Vertical, child.child_size().height);
            }
            _ => {}
        }

        child.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        child: &mut druid::widget::Scroll<T, W>,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &T,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env);
    }
}
