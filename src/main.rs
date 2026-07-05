#![allow(dead_code)]
#![allow(unused)]

use macroquad::prelude::*;

use crate::{
    components::sidebar::Sidebar,
    core::geometry::Bounds,
    game::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

mod components;
mod core;
mod game;

#[macroquad::main(window_conf())]
async fn main() {
    loop {
        clear_background(WHITE);
        render_screen();

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Layout".to_string(),
        window_width: game::WINDOW_WIDTH,
        window_height: game::WINDOW_HEIGHT,
        ..Default::default()
    }
}

fn render_screen() {
    let sidebar = Sidebar::new(
        BROWN,
        Bounds {
            x: WINDOW_WIDTH as f32 - (WINDOW_WIDTH as f32 * 50. / 100.),
            y: 0.,
            width: WINDOW_WIDTH as f32 * 60. / 100.,
            height: WINDOW_HEIGHT as f32,
            gap: 2.0,
            padding: 2.0,
        },
    );
    sidebar.render();
}
