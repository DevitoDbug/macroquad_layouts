use macroquad::prelude::*;

pub struct Sidebar {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub background: Color,
}

impl Sidebar {
    pub fn new(x: f32, y: f32, width: f32, height: f32, background: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            background,
        }
    }

    pub fn render(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, self.background);
    }
}
