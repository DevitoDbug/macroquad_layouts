use macroquad::prelude::*;

use crate::core::{drawable::Drawable, geometry::Bounds};

pub struct Button {
    pub x: Option<f32>,
    pub y: Option<f32>,

    pub height: f32,
    pub width: f32,

    pub background: Color,
}

impl Button {
    pub fn new(height: f32, width: f32, background: Color) -> Self {
        Self {
            height,
            width,
            background,
            // x and y can only be set during drawing
            x: None,
            y: None,
        }
    }
}

impl Drawable for Button {
    fn draw(&mut self, x: f32, y: f32) -> (f32, f32) {
        self.x = Some(x);
        self.y = Some(y);

        draw_rectangle(x, y, self.width, self.height, self.background);
        (self.width, self.height)
    }
}
