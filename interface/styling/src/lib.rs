mod hex_color_macro;

pub static ITEM_PADDING: f32 = 6.0;

pub static COLOR_DARK_BG: [f32; 3] = hex_color!(0x681885);

pub static COLOR_HIGHLIGHT_BG: [f32; 3] = hex_color!(0xff00aa);

pub static ENTRY_HEIGHT: u32 = 50;

pub static MONO_FONT_REGULAR: iced::Font = iced::Font {
    family: iced::font::Family::Name("Fira Code"),
    monospaced: true,
    stretch: iced::font::Stretch::Normal,
    weight: iced::font::Weight::Normal,
};

pub static MONO_FONT_MEDIUM: iced::Font = iced::Font {
    family: iced::font::Family::Name("Fira Code"),
    monospaced: true,
    stretch: iced::font::Stretch::Normal,
    weight: iced::font::Weight::Medium,
};

pub static MONO_FONT_BOLD: iced::Font = iced::Font {
    family: iced::font::Family::Name("Fira Code"),
    monospaced: true,
    stretch: iced::font::Stretch::Normal,
    weight: iced::font::Weight::Bold,
};