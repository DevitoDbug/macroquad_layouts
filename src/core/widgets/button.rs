use macroquad::prelude::*;

use crate::core::{drawable::Drawable, event::event::Event, geometry::Bounds};

pub struct Button {
    pub x: Option<f32>,
    pub y: Option<f32>,

    pub h: f32,
    pub w: f32,

    pub background: Color,
}

impl Button {
    pub fn new(height: f32, width: f32, background: Color) -> Self {
        Self {
            h: height,
            w: width,
            background,
            // x and y can only be set during drawing
            x: None,
            y: None,
        }
    }

    fn clicked(&self) {
        println!("button: {:?} clicked ", self.background);
    }
}

impl Drawable for Button {
    fn draw(&mut self, x: f32, y: f32) {
        self.x = Some(x);
        self.y = Some(y);

        draw_rectangle(x, y, self.w, self.h, self.background);
    }

    fn get_dimensions(&self) -> (f32, f32) {
        return (self.w, self.h);
    }

    fn handle_event(&self, e: &Event) -> Option<bool> {
        if e.is_within_bounds(self.x, self.y, self.h, self.w) {
            self.clicked();
            return Some(true);
        }

        Some(false)
    }
}
