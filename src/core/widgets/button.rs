use macroquad::prelude::*;

use crate::core::{drawable::Drawable, event::event::Event, geometry::Bounds};

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

    fn clicked(&self) {
        print!("button: {:?} clicked ", self.background);
    }
}

impl Drawable for Button {
    fn draw(&mut self, x: f32, y: f32) -> (f32, f32) {
        self.x = Some(x);
        self.y = Some(y);

        draw_rectangle(x, y, self.width, self.height, self.background);
        (self.width, self.height)
    }

    fn handle_event(&self, e: &Event) -> Option<bool> {
        if e.is_within_bounds(self.x, self.y, self.height, self.width) {
            self.clicked();
            return Some(true);
        }

        Some(false)
    }
}
