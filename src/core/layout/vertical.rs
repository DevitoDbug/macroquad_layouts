use macroquad::prelude::*;

use crate::core::{
    drawable::Drawable, event::event::Event, geometry::Bounds, layout::traits::Layout,
};

pub struct VerticalLayout {
    pub x: Option<f32>,
    pub y: Option<f32>,

    pub h: f32,
    pub w: f32,
    pub padding: f32,
    pub gap: f32,

    pub children: Vec<Box<dyn Drawable>>,
    pub background: Color,
}

impl VerticalLayout {
    pub fn new(
        h: f32,
        w: f32,
        gap: f32,
        padding: f32,
        children: Vec<Box<dyn Drawable>>,
        background: Color,
    ) -> Self {
        Self {
            x: None,
            y: None,
            w,
            h,
            padding,
            gap,
            children,
            background,
        }
    }

    pub fn new_with_pos(
        x: f32,
        y: f32,
        h: f32,
        w: f32,
        gap: f32,
        padding: f32,
        children: Vec<Box<dyn Drawable>>,
        background: Color,
    ) -> Self {
        Self {
            x: Some(x),
            y: Some(y),
            h,
            w,
            gap,
            padding,
            children,
            background,
        }
    }
}

impl Layout for VerticalLayout {
    // Based on the bounds of the vertical layout should be able to
    // arrange the give components within  the specified bounds
    fn arrange(&mut self) {
        let x = match self.x {
            None => 0.,
            Some(val) => val,
        };
        let y = match self.y {
            None => 0.,
            Some(val) => val,
        };
        let mut y = y + self.padding;
        let max_layout_x = x + self.w;
        let max_layout_y = y + self.h;

        for child in &mut self.children {
            let (_, child_height) = child.get_dimensions();
            // X remains the same,
            // Y goes down that means ++
            // Check to see if we have enough space to continue with the iteration
            if y + child_height >= max_layout_y {
                break;
            }

            child.draw(x, y);
            y += (child_height + self.gap);
        }
    }
}

impl Drawable for VerticalLayout {
    fn draw(&mut self, x: f32, y: f32) {
        self.x = Some(x);
        self.y = Some(y);
        draw_rectangle(x, y, self.w, self.h, self.background);
        self.arrange();
    }

    fn get_dimensions(&self) -> (f32, f32) {
        return (self.w, self.h);
    }

    fn handle_event(&self, e: &Event) -> Option<bool> {
        for child in self.children.iter() {
            let has_handled_event = match child.handle_event(&e) {
                None => false,
                Some(val) => val,
            };

            if has_handled_event {
                return Some(true);
            }
        }
        Some(false)
    }
}
