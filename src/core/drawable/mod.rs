use macroquad::math::bool;

use crate::core::event::event::Event;

pub trait Drawable {
    fn draw(&mut self, x: f32, y: f32) -> (f32, f32); // Returns the max for the x and y respectively
    fn handle_event(&self, e: &Event) -> Option<bool> {
        None
    }
}
