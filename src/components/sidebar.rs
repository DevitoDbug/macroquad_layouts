use macroquad::prelude::*;

use crate::{
    core::{
        drawable::Drawable,
        event::event::Event,
        geometry::Bounds,
        layout::{
            horizontal::{self, HorizontalLayout},
            traits::Layout,
            vertical::VerticalLayout,
        },
        widgets::button::Button,
    },
    game::{BLOCK_SIZE, WINDOW_WIDTH},
};

pub struct Sidebar {
    pub background: Color,
    pub bounds: Bounds,
}

impl Sidebar {
    pub fn new(background: Color, bounds: Bounds) -> Self {
        Self { background, bounds }
    }

    pub fn render(&self, events: &Vec<Event>) {
        let button1 = Box::new(Button::new(
            BLOCK_SIZE as f32 * 2.,
            BLOCK_SIZE as f32 * 5.,
            GREEN,
        ));
        let button2 = Box::new(Button::new(
            BLOCK_SIZE as f32 * 2.,
            BLOCK_SIZE as f32 * 2.,
            GREEN,
        ));
        let button3 = Box::new(Button::new(
            BLOCK_SIZE as f32 * 2.,
            BLOCK_SIZE as f32 * 5.,
            ORANGE,
        ));
        let button4 = Box::new(Button::new(
            BLOCK_SIZE as f32 * 2.,
            BLOCK_SIZE as f32 * 2.,
            PURPLE,
        ));

        let mut hor_layout = Box::new(HorizontalLayout::new(
            self.bounds.height * 20. / 100.,
            self.bounds.width,
            1.,
            1.,
            vec![button4, button3],
            YELLOW,
        ));

        let mut vert_layout = VerticalLayout::new_with_pos(
            self.bounds.x,
            self.bounds.y,
            self.bounds.height,
            self.bounds.width,
            self.bounds.gap,
            self.bounds.padding,
            vec![button1, hor_layout, button2],
            self.background,
        );

        vert_layout.draw(self.bounds.x, self.bounds.y);
        for e in events {
            vert_layout.handle_event(e);
        }
    }
}
