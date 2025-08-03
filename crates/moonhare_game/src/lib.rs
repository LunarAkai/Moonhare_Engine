//! Base functionality for a Moonhare Game Engine Project

use std::sync::Arc;

use moonhare_log::*;
use moonhare_window::{glfw::{self, PWindow}, platforms::glfw_window::GLFWWindow, Window, WindowRenderContext};

pub mod basic;

/// Only one Game may exist per project
#[derive(Debug)]
pub struct Game {
    pub is_running: bool,
    pub name: String,
    pub context: WindowRenderContext,
    pub glfw_window: Option<GLFWWindow>
}

impl Default for Game {
    fn default() -> Self {
        Self { 
            is_running: true,
            name: default_game_name(),
            context: WindowRenderContext::OPENGLGLFW,
            glfw_window: None,
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game::default()
    }

    pub fn run(self) {
        info("Running Game...");
        let mut glfw_window_unwrapped: GLFWWindow = self.glfw_window.unwrap();
    
        while self.is_running {
            handle_window_event(&mut glfw_window_unwrapped);
            // update();
            // render();
        }
    }

    pub fn add_window(&mut self) {
        moonhare_log::info(format!("Adding window to {:?}", self));
        self.glfw_window =Some(Window::create(self.context).into());
    }
}

fn default_game_name() -> String {
    "Moonhare Game".to_owned()
}

/// Deals with GLFW Window Events (in `monhare_window`)
fn handle_window_event(glfw_window: &mut GLFWWindow) {
    glfw_window.glfw_window.glfw.poll_events();
    for (_, event) in glfw::flush_messages(&glfw_window.events) {
        GLFWWindow::handle_window_event(&mut glfw_window.glfw_window, event);
    }   
}
