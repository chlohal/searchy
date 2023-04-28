use std::marker::PhantomData;
use std::sync::Arc;

use druid::widget::{Axis, Controller, Scroll};
use druid::{Command, Data, Env, Event, EventCtx, LifeCycle, LifeCycleCtx, Selector, Widget, WidgetExt};

use super::window::SearchingWindow;

pub(crate) struct DynamicallySelected<T> {
    _phantom: PhantomData<T>,
}

impl DynamicallySelected<SearchingWindow> {
    pub fn new() -> Self {
        DynamicallySelected {
            _phantom: PhantomData,
        }
    }
}

pub const SCROLL_TO_BOTTOM: Selector = Selector::new("searchy.scroll-to-bottom");

impl<T: Data, W: Widget<T>> Controller<T, W> for DynamicallySelected<T> {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        env: &Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &T,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env);
    }
}
