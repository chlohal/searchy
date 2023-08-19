mod hex_color_macro;

pub static ITEM_PADDING: f32 = 6.0;

pub static COLOR_DARK_BG: [f32; 3] = hex_color!(0x681885);

pub static COLOR_HIGHLIGHT_BG: [f32; 3] = hex_color!(0xff00aa);

pub static ENTRY_HEIGHT: u32 = 50;

pub static FONT: iced::Font = iced::Font::External {
    name: "Fira Code",
    bytes: include_bytes!("../../../assets/fonts/FiraCode-Regular.ttf"),
};