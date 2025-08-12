use crate::event::Event;

pub trait EventListener {
    fn on_event<T: Event>(event: T);
}
