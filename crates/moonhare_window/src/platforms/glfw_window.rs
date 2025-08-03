use std::sync::Arc;

use glfw::{Context, Glfw, GlfwReceiver, PWindow, Window, WindowEvent};
use moonhare_event::{event::Event, events::window_events::window_close_event::WindowCloseEvent};

use crate::{window_config, MoonhareWindow};

#[derive(Debug)]
pub struct GLFWWindow {
    pub glfw_window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
    pub glfw: Glfw,
    pub is_running: bool,
}

const APP_ID: &str = "de.lunarakai.moonhare_engine";

impl GLFWWindow {
    fn new() -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        let config = window_config::WindowConfig::default();
        let (mut window, events) = glfw.create_window(
            config.width, 
            config.height, 
            format!("{} GLFW {}", config.title, glfw::get_version_string()).as_str(), 
            glfw::WindowMode::Windowed)
            .unwrap();

        window.set_key_polling(true);
        window.make_current();

        Self {
            glfw_window: window.try_into().unwrap(),
            events: events,
            glfw: glfw,
            is_running: true
        }
    }


    pub fn handle_window_event(_window: &mut glfw::Window, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Close => {
                WindowCloseEvent::emit();
            }
            _ => {},
        }
    }
}

impl MoonhareWindow for GLFWWindow {  
    fn init() -> GLFWWindow {
        let window = GLFWWindow::new();
        window
    }


    fn on_update() {
        
    }

    fn shutdown() {
        // todo: emit WindowCloseEvent
        
    }
}

