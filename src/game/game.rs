use macroquad::prelude::*;

use crate::{
    components::sidebar::Sidebar,
    core::{
        event::{event::Event, event_type::EventType},
        geometry::Bounds,
    },
    game::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

pub struct Game {
    sidebar: Sidebar,
}

impl Game {
    pub fn new() -> Self {
        let game_panel_width = (WINDOW_WIDTH as f32 * 40. / 100.);
        let sidebar_width = (WINDOW_WIDTH as f32 * 60. / 100.);
        let sidebar = Sidebar::new(
            BROWN,
            Bounds {
                x: WINDOW_WIDTH as f32 - sidebar_width,
                y: 0.,
                width: sidebar_width,
                height: WINDOW_HEIGHT as f32,
                gap: 2.0,
                padding: 2.0,
            },
        );

        Self { sidebar }
    }

    pub async fn start(&mut self) {
        loop {
            clear_background(WHITE);

            let mut events: Vec<Event> = vec![];

            // Handle mouse events
            let mouse_event = self.listen_to_mouse_event();
            match mouse_event {
                None => (),
                Some(val) => events.push(val),
            }

            self.render_screen_layout(&events);
            next_frame().await
        }
    }

    fn render_screen_layout(&mut self, events: &Vec<Event>) {
        self.sidebar.render(events);
    }

    fn listen_to_mouse_event(&self) -> Option<Event> {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();

            let mouse_click_event = Event::new(EventType::Click, x, y);
            return Some(mouse_click_event);
        }

        None
    }
}
