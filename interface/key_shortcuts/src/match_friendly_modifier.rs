use iced_core::keyboard::Modifiers;

pub const NONE: u8 = 0;

pub const CTRL: u8 = 0b1000;
pub const WIN: u8 = 0b0100;
pub const ALT: u8 = 0b0010;
pub const SHIFT: u8 = 0b0001;

pub const CTRL_SHIFT: u8 = CTRL | SHIFT;

pub fn match_friendly_modifier(modifiers: Modifiers) -> u8 {
    let mut r: u8 = 0;

    if modifiers.control() {
        r |= CTRL;
    }

    if modifiers.logo() {
        r |= WIN;
    }

    if modifiers.alt() {
        r |= ALT;
    }

    if modifiers.shift() {
        r |= SHIFT;
    }

    r
}