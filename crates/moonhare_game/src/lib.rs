//! Base functionality for a Moonhare Game Engine Project

use std::{cell::{RefCell, RefMut}, rc::Rc, sync::Arc};

use moonhare_graphics::glium::{backend::Context, glutin::api::egl::context, winit::event_loop};
use moonhare_log::*;
use moonhare_window::{glfw::PWindow, platforms::glfw_window::GLFWWindow};
pub mod basic;

/// Only one Game may exist per project
#[derive(Debug)]
pub struct Game {
    pub is_running: bool,
    pub name: String,
    pub context: moonhare_window::WindowRenderContext,
    pub glfw_window: Option<moonhare_window::platforms::glfw_window::GLFWWindow>
}

impl Default for Game {
    fn default() -> Self {
        Self { 
            is_running: true,
            name: default_game_name(),
            context: moonhare_window::WindowRenderContext::OPENGLGLFW,
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
        let mut glfw_window_unwrapped = self.glfw_window;
        let mut context: std::rc::Rc<moonhare_graphics::glium::backend::Context>;

        context = moonhare_graphics::build_context(glfw_window_unwrapped.clone().unwrap().glfw_window);
        
        
        let mut value = glfw_window_unwrapped;
        while self.is_running {
            // can't move glfwwindow cause i can't implement clone, or idk
            handle_window_event(value.as_mut().unwrap());
            render(context.clone());

            // update();
        }
    }

    pub fn add_window(&mut self) {
        moonhare_log::info(format!("Adding window to {:?}", self));
        self.glfw_window =Some(moonhare_window::Window::create(self.context).into());
    }
}

fn default_game_name() -> String {
    "Moonhare Game".to_owned()
}

/// Deals with GLFW Window Events (in `monhare_window`)
fn handle_window_event(mut glfw_window: &GLFWWindow) {
    glfw_window.glfw_window.borrow_mut().glfw.poll_events();
    for (_, event) in moonhare_window::glfw::flush_messages(&glfw_window.events.borrow()) {
        moonhare_window::platforms::glfw_window::GLFWWindow::handle_window_event(&glfw_window, event);
    }   
}

fn render(context: Rc<Context>) {
 
    let mut target = moonhare_graphics::glium::Frame::new(context.clone(), context.get_framebuffer_dimensions());
    moonhare_graphics::glium::Surface::clear_color(&mut target, 0.0, 0.0, 1.0, 1.0);
    target.finish().unwrap();
}
