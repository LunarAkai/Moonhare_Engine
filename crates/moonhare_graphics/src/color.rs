pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Color {
    /// Color Values from 0 - 255
    /// For example: 255, 255, 255 -> White
    pub fn color_from_rgb(red: i32, green: i32, blue: i32) -> Self {
        // 255 -> 1.0
        // 0 -> 0.0
        let r: f32 = red as f32 / 255.0;
        let g: f32 = green as f32 / 255.0;
        let b: f32 = blue as f32 / 255.0;
        Self {
            red: r,
            green: g,
            blue: b,
            alpha: 1.0,
        }
    }

    pub fn color_from_rgba(red: i32, green: i32, blue: i32, alpha: f32) -> Self {
        // 255 -> 1.0
        // 0 -> 0.0
        let r: f32 = red as f32 / 255.0;
        let g: f32 = green as f32 / 255.0;
        let b: f32 = blue as f32 / 255.0;
        let a: f32 = alpha;
        Self {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        }
    }
}
