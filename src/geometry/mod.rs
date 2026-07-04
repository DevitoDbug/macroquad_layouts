pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    pub padding: f32,
    pub gap: f32,
}

impl Bounds {
    pub fn new(x: f32, y: f32, width: f32, height: f32, padding: f32, gap: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            padding,
            gap,
        }
    }
}
