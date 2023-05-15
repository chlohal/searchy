#[macro_export]
macro_rules! hex_color {
    ($hex:expr) => {
        {
            let hex = $hex as u32;
            let r = (hex & 0xff0000) >> 16;
            let g = (hex & 0xff00) >> 8;
            let b = (hex & 0xff);

            [(r as f32) / 256.0, (g as f32) / 256.0, (b as f32) / 256.0]
        }
    };
}