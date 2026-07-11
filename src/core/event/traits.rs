use crate::core::event::event::Event;

pub trait EventListener {
    //TODO: Currently this is all i think the event handler should look like
    fn handle_event(e: Event) -> bool;
}
