use std::sync::Arc;

use ::actions::{Action, Action::Application, Action::ShellCommand};
use iced::{widget::{Row, text, row, Text, container}, Font, Length};
use icons::Icon;
use messages::Message;
use styling::{MONO_FONT_MEDIUM, MONO_FONT_BOLD, MONO_FONT_REGULAR};




pub fn make_entry(action: &Arc<Action>) -> Row<'static, Message> {
    let label = text(action.primary_name()).width(Length::FillPortion(2))
        .font(MONO_FONT_BOLD);

    let icon = iced_icon(match **action {
        Action::Application(_) => &icons::icons::WINDOW_MAXIMIZE_REGULAR,
        Action::ShellCommand(_) => &icons::icons::TERMINAL,
    });

    let detail = container(detail(action)); 

    row!(icon, label, detail).spacing(10)
}

fn detail(action: &Arc<Action>) -> Row<'static, Message> {
    match &**action {
        Application(_) => {
            row!()
        },
        ShellCommand(command) => {
            let tail = text(command.to_string_lossy())
                .font(MONO_FONT_MEDIUM);

            row!(tail)
        },
    }
}


fn iced_icon(icon: &Icon) -> Text<'static> {
    text(icon.0)
        .font(Font::External { name: icon.1.name, bytes: icon.1.bytes })
        .width(20)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
}