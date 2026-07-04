use macroquad::prelude::*;

use crate::core::{drawable::Drawable, geometry::Bounds};

struct Button {
    pub bounds: Bounds,
    pub background: Color,
}

impl Button {
    pub fn new(bounds: Bounds, background: Color) -> Self {
        Self { bounds, background }
    }
}

impl Drawable for Button {
    fn draw(&self, x: f32, y: f32) -> (f32, f32) {
        draw_rectangle(x, y, self.bounds.width, self.bounds.height, self.background);
        (self.bounds.get_max_width(), self.bounds.get_max_height())
    }
}
