pub mod window_close_event;

use crate::event::Event;

pub trait WindowEvent: Event {}
