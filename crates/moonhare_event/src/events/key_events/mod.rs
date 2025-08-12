use crate::event::Event;

pub mod key_pressed_event;

pub trait KeyEvent: Event {
    fn get_key_code() {}
}
