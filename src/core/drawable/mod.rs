pub trait Drawable {
    fn draw(&self, x: f32, y: f32) -> (f32, f32); // Returns the max for the x and y respectively
}
