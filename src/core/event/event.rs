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

    //TODO: Check currently assumes that all users of this method are rects
    // Future versions should support other shapes supported by macroquad
    pub fn is_within_bounds(&self, x: f32, y: f32, h: f32, w: f32) -> bool {
        if self.event_x > x && self.event_x < (x + w) {
            if self.event_y > y && self.event_y < (y + h) {
                return true;
            }
        }

        false
    }
}
