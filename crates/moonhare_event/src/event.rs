pub enum EventType {
    None,
    WindowClose, WindowResize,
    KeyPressed, KeyReleased,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

/// Base Class for Engine Events
pub trait Event {
    fn get_event_name() -> &'static str;
    fn get_event_type() -> EventType;
}