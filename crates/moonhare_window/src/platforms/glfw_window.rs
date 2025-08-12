use std::{cell::RefCell, rc::Rc, sync::Arc};

use glfw::{Context, Glfw, GlfwReceiver, PWindow, Window, WindowEvent};
use moonhare_event::{event::Event, events::window_events::window_close_event::WindowCloseEvent};

use crate::{MoonhareWindow, window_config};

#[derive(Debug)]
pub struct GLFWWindow {
    // Todo: learn more about rust smart pointers so i actually understand whats going on here, but hey it works for now
    pub glfw_window: Rc<RefCell<PWindow>>,
    pub events: Rc<RefCell<GlfwReceiver<(f64, WindowEvent)>>>,
    pub glfw: Glfw,
    pub is_running: bool,
}

const APP_ID: &str = "de.lunarakai.moonhare_engine";

impl Clone for GLFWWindow {
    fn clone(&self) -> Self {
        Self {
            glfw_window: self.glfw_window.clone(),
            events: self.events.clone(),
            glfw: self.glfw.clone(),
            is_running: self.is_running.clone(),
        }
    }
}

impl GLFWWindow {
    fn new() -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        let config = window_config::WindowConfig::default();
        let (mut window, events) = glfw
            .create_window(
                config.width,
                config.height,
                format!("{} GLFW {}", config.title, glfw::get_version_string()).as_str(),
                glfw::WindowMode::Windowed,
            )
            .unwrap();

        window.set_key_polling(true);
        window.make_current();

        Self {
            glfw_window: Rc::new(RefCell::new(window)),
            events: Rc::new(RefCell::new(events)),
            glfw: glfw,
            is_running: true,
        }
    }

    pub fn handle_window_event(&self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Close => {
                WindowCloseEvent::emit();
            }
            _ => {}
        }
    }
}

impl MoonhareWindow for GLFWWindow {
    fn init() -> GLFWWindow {
        let window = GLFWWindow::new();
        window
    }

    fn on_update() {}

    fn shutdown() {
        // todo: emit WindowCloseEvent
    }
}
