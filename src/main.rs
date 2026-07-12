#![allow(dead_code)]
#![allow(unused)]

use macroquad::prelude::*;

use crate::{
    components::sidebar::Sidebar,
    core::geometry::Bounds,
    game::{WINDOW_HEIGHT, WINDOW_WIDTH, game::Game},
};

mod components;
mod core;
mod game;

#[macroquad::main(window_conf())]
async fn main() {
    let game = Game::new();

    loop {
        clear_background(WHITE);
        game.start();

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Layout".to_string(),
        window_width: game::WINDOW_WIDTH,
        window_height: game::WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}
