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
    pub root_ui: Box<dyn Drawable>,
    pub background: Color,

    pub bounds: Bounds,
}

impl Sidebar {
    pub fn new(background: Color, bounds: Bounds) -> Self {
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
            bounds.height * 20. / 100.,
            bounds.width,
            1.,
            1.,
            vec![button4, button3],
            YELLOW,
        ));

        let mut vert_layout = Box::new(VerticalLayout::new_with_pos(
            bounds.x,
            bounds.y,
            bounds.height,
            bounds.width,
            bounds.gap,
            bounds.padding,
            vec![button1, hor_layout, button2],
            background,
        ));

        Self {
            root_ui: vert_layout,
            background,
            bounds,
        }
    }

    pub fn render(&mut self, events: &Vec<Event>) {
        self.root_ui.draw(self.bounds.x, self.bounds.y);
        for e in events {
            self.root_ui.handle_event(e);
        }
    }
}
