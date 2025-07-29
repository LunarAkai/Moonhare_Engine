use crate::{event::Event,};

struct KeyPressedEvent{}

const KEY_PRESSED_EVENT_NAME: &str = "KeyPressedEvent";

impl Event for KeyPressedEvent {
    fn get_event_name() -> &'static str {
        KEY_PRESSED_EVENT_NAME
    }

    fn get_event_type() -> crate::event::EventType {
        crate::event::EventType::KeyPressed
    }
}

impl super::KeyEvent for KeyPressedEvent {

}