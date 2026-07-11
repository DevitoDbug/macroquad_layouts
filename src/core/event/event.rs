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
    pub fn is_within_bounds(&self, x: f32, y: f32) -> bool {
        if x > self.event_x && y > self.event_y {
            return true;
        }
        false
    }
}
