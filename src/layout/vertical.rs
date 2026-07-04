use macroquad::prelude::*;

use crate::{
    drawable::Drawable,
    geometry::{self, Bounds},
    layout::traits::Layout,
};

pub struct VerticalLayout {
    pub bounds: geometry::Bounds,
    pub children: Vec<Box<dyn Drawable>>,
    pub background: Color,
}

impl VerticalLayout {
    pub fn new(bounds: Bounds, children: Vec<Box<dyn Drawable>>, background: Color) -> Self {
        Self {
            bounds,
            children,
            background,
        }
    }

    fn get_max_height(&self) -> f32 {
        self.bounds.y + self.bounds.height
    }

    fn get_max_width(&self) -> f32 {
        self.bounds.x + self.bounds.width
    }
}

impl Layout for VerticalLayout {
    // Based on the bounds of the vertical layout should be able to
    // arrange the give components within  the specified bounds
    fn arrange(&self) {
        let x = self.bounds.x; // starting point x  
        let mut y = self.bounds.y + self.bounds.padding; // starting points y

        let max_layout_x = self.get_max_width();
        let max_layout_y = self.get_max_height();

        for child in &self.children {
            let (max_x, max_y) = child.draw(x, y);
            // X remains the same,
            // Y goes down that means ++
            // Check to see if we have enough space to continue with the iteration
            if y + max_y >= max_layout_y {
                break;
            }

            y += (max_y + self.bounds.gap);
        }
    }
}

impl Drawable for VerticalLayout {
    fn draw(&self, x: f32, y: f32) -> (f32, f32) {
        draw_rectangle(x, y, self.bounds.width, self.bounds.height, self.background);
        (self.get_max_width(), self.get_max_height())
    }
}
