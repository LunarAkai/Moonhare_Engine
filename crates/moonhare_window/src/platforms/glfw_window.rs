use std::sync::Arc;

use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};
use gtk4::gdk::Key;
use moonhare_event::{event::{self, Event}, events::window_events::window_close_event::WindowCloseEvent};

use crate::{window_config, MoonhareWindow};

#[derive(Debug)]
pub struct GLFWWindow {
    glfw_window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    glfw: Glfw,
    is_running: bool,
}

const APP_ID: &str = "de.lunarakai.moonhare_engine";

impl GLFWWindow {
    fn new() -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        let config = window_config::WindowConfig::default();
        let (mut window, events) = glfw.create_window(
            config.width, 
            config.height, 
            format!("{} GLFW", config.title).as_str(), 
            glfw::WindowMode::Windowed)
            .unwrap();

        window.set_key_polling(true);
        window.make_current();

        Self {
            glfw_window: window,
            events: events,
            glfw: glfw,
            is_running: true
        }
    }

    fn run_window(&mut self) {
        while !self.glfw_window.should_close() {
            self.glfw.poll_events();
            for(_, event) in glfw::flush_messages(&self.events) {
                Self::handle_window_event(&mut self.glfw_window, event);
            }
        }
    }

    fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Close => {
                WindowCloseEvent::emit();
            }
            _ => {},
        }
    }
    
}

impl MoonhareWindow for GLFWWindow {  
    fn init() {
        let mut window = GLFWWindow::new();
        GLFWWindow::run_window(&mut window);
    }


    fn on_update() {
        
    }

    fn shutdown() {
        // todo: emit WindowCloseEvent
        
    }
}

