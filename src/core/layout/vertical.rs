use macroquad::prelude::*;

use crate::core::{drawable::Drawable, geometry::Bounds, layout::traits::Layout};

pub struct VerticalLayout {
    pub bounds: Bounds,
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
    pub fn new_component(
        height: f32,
        width: f32,
        gap: f32,
        padding: f32,
        children: Vec<Box<dyn Drawable>>,
        background: Color,
    ) -> Self {
        Self {
            bounds: Bounds {
                x: 0.,
                y: 0.,
                width,
                height,
                padding,
                gap,
            },
            children,
            background,
        }
    }
}

impl Layout for VerticalLayout {
    // Based on the bounds of the vertical layout should be able to
    // arrange the give components within  the specified bounds
    fn arrange(&mut self) {
        let x = self.bounds.x; // starting point x  
        let mut y = self.bounds.y + self.bounds.padding; // starting points y

        let max_layout_x = self.bounds.get_max_width();
        let max_layout_y = self.bounds.get_max_height();

        for child in &mut self.children {
            let (child_width, child_height) = child.draw(x, y);
            // X remains the same,
            // Y goes down that means ++
            // Check to see if we have enough space to continue with the iteration
            if y + child_height >= max_layout_y {
                break;
            }

            y += (child_height + self.bounds.gap);
        }
    }
}

impl Drawable for VerticalLayout {
    fn draw(&mut self, x: f32, y: f32) -> (f32, f32) {
        self.bounds.x = x;
        self.bounds.y = y;
        draw_rectangle(x, y, self.bounds.width, self.bounds.height, self.background);
        self.arrange();
        (self.bounds.x, self.bounds.y)
    }
}
