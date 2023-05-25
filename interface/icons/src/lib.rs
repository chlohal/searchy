pub struct Font {
    pub name: &'static str,
    pub bytes: &'static [u8]
}

pub type Icon = (&'static str, Font);

pub const FONT_BRANDS: Font = Font {
    name: "FontAwesome Brands",
    bytes: include_bytes!("..//fonts/fa-brands-400.ttf"),
};

pub const FONT_REGULAR: Font = Font {
    name: "FontAwesome Regular",
    bytes: include_bytes!("..//fonts/fa-regular-400.ttf"),
};

pub const FONT_SOLID: Font = Font {
    name: "FontAwesome Solid",
    bytes: include_bytes!("..//fonts/fa-solid-900.ttf"),
};


pub mod icons;