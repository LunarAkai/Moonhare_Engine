use crate::{event::{Event, EventType}, events::key_events::KeyEvent};

#[derive(Debug)]
struct KeyPressedEvent{}

const KEY_PRESSED_EVENT_NAME: &str = "KeyPressedEvent";

impl Event for KeyPressedEvent {
    fn get_event_name() -> &'static str {
        KEY_PRESSED_EVENT_NAME
    }

    fn get_event_type() -> EventType {
        EventType::KeyPressed
    }
    
    fn emit() {
        
    }
}

impl super::KeyEvent for KeyPressedEvent {

}