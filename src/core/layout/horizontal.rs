use macroquad::prelude::*;

use crate::core::{drawable::Drawable, geometry::Bounds, layout::traits::Layout};

pub struct HorizontalLayout {
    pub bounds: Bounds,
    pub children: Vec<Box<dyn Drawable>>,
    pub background: Color,
}

impl HorizontalLayout {
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

impl Layout for HorizontalLayout {
    fn arrange(&mut self) {
        let mut x = self.bounds.x + self.bounds.padding;
        let y = self.bounds.y;

        let max_layout_x = self.bounds.get_max_width();
        let max_layout_y = self.bounds.get_max_height();

        for child in &mut self.children {
            let (child_width, child_height) = child.draw(x, y);
            if x + child_width >= max_layout_x {
                break;
            }

            x += (child_width + self.bounds.gap);
        }
    }
}

impl Drawable for HorizontalLayout {
    fn draw(&mut self, x: f32, y: f32) -> (f32, f32) {
        self.bounds.x = x;
        self.bounds.y = y;
        draw_rectangle(x, y, self.bounds.width, self.bounds.height, self.background);
        self.arrange();
        (self.bounds.width, self.bounds.height)
    }
}
