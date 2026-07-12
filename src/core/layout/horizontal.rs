use macroquad::prelude::*;

use crate::core::{
    drawable::Drawable, event::event::Event, geometry::Bounds, layout::traits::Layout,
};

pub struct HorizontalLayout {
    pub x: Option<f32>,
    pub y: Option<f32>,

    pub h: f32,
    pub w: f32,
    pub padding: f32,
    pub gap: f32,

    pub children: Vec<Box<dyn Drawable>>,
    pub background: Color,
}

impl HorizontalLayout {
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

    pub fn new_with_position(
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
            y: Some(x),
            w,
            h,
            padding,
            gap,
            children,
            background,
        }
    }
}

impl Layout for HorizontalLayout {
    fn arrange(&mut self) {
        let x = match self.x {
            None => 0.,
            Some(val) => val,
        };
        let mut x = x + self.padding;
        let y = match self.y {
            None => 0.,
            Some(val) => val,
        };

        let max_layout_x = x + self.w;
        let max_layout_y = y + self.h;

        for child in &mut self.children {
            let (child_width, _) = child.get_dimensions();
            let child_width = match child_width {
                None => break,
                Some(val) => val,
            };
            if x + child_width >= max_layout_x {
                break;
            }

            child.draw(x, y);
            x += (child_width + self.gap);
        }
    }
}

impl Drawable for HorizontalLayout {
    fn draw(&mut self, x: f32, y: f32) {
        self.x = Some(x);
        self.y = Some(y);
        draw_rectangle(x, y, self.w, self.h, self.background);
        self.arrange();
    }

    fn get_dimensions(&self) -> (Option<f32>, Option<f32>) {
        return (self.x, self.y);
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
