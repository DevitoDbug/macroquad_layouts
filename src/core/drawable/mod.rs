use macroquad::math::bool;

use crate::core::event::event::Event;

pub trait Drawable {
    fn draw(&mut self, x: f32, y: f32);
    fn get_dimensions(&self) -> (f32, f32);
    fn handle_event(&mut self, e: &Event) -> Option<bool> {
        None
    }
}
