use crate::core::event::event_type::EventType;

pub struct Event {
    event_type: EventType,
    event_x: f32,
    event_y: f32,
}

impl Event {
    pub fn new(event_type: EventType, event_x: f32, event_y: f32) -> Self {
        Self {
            event_type,
            event_x,
            event_y,
        }
    }
}
