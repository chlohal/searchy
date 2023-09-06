pub struct Font {
    pub name: &'static str,
    pub bold: bool
}

pub type Icon = (&'static str, Font);

pub const FONT_BRANDS_TTF_BYTES: &[u8] = include_bytes!("../../../assets/fonts/fa-brands-400.ttf");
pub const FONT_REGULAR_TTF_BYTES: &[u8] = include_bytes!("../../../assets/fonts/fa-regular-400.ttf");
pub const FONT_SOLID_TTF_BYTES: &[u8] = include_bytes!("../../../assets/fonts/fa-solid-900.ttf");


pub const FONT_BRANDS: Font = Font {
    name: "Font Awesome 6 Brands Regular",
    bold: false
};

pub const FONT_REGULAR: Font = Font {
    name: "Font Awesome 6 Free Regular",
    bold: false
};

pub const FONT_SOLID: Font = Font {
    name: "Font Awesome 6 Free Solid",
    bold: true
};


pub mod icons;