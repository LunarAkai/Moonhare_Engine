use crate::{event::Event, events::window_events::WindowEvent};

#[derive(Debug)]
pub struct WindowCloseEvent {

}

impl Event for WindowCloseEvent {
    fn get_event_name() -> &'static str {
        todo!()
    }

    fn get_event_type() -> crate::event::EventType {
        todo!()
    }

    fn emit(){
    }
}

impl WindowEvent for WindowCloseEvent {

}