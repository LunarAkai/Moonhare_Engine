//! Base functionality for a Moonhare Game Engine Project

use std::{any::Any, rc::Rc};


use moonhare_graphics::{color::Color, glium::{backend::Context, glutin::api::egl::context}};
use moonhare_log::*;
use moonhare_window::{platforms::glfw_window::GLFWWindow};

use crate::systems::system::{BaseSystems, System};

pub mod systems;
pub mod basic;

/// Only one Game may exist per project
/* #[derive(Debug)]
pub struct Game {
    pub base_systems: BaseSystems,
    pub context: moonhare_window::WindowRenderContext,
    pub glfw_window: Option<moonhare_window::platforms::glfw_window::GLFWWindow>,
    pub is_running: bool,
    pub name: String,
} */

// when creating a game, you can add systems to it, which do _things_ 
// BaseSystems -> Window, Update, Render

// Hierachy:
//      [Game] -> <Systems> -> <Nodes> (-> <Nodes> -> ... )
//-------------
//  [ ] => only 1 --- < > => allow multiple
#[derive(Debug)]
pub struct Game {
    pub systems: Option<Vec<Box<dyn System>>>,
    pub is_running: bool
}


impl Default for Game {
    fn default() -> Self {
        Self { 
            systems: None,
            is_running: true
        }
    }
}

pub struct GraphicsHandler {
    pub context: Option<Rc<Context>>
}

impl Default for GraphicsHandler {
    fn default() -> Self {
        Self { context: None }
    }
}

impl Game {
    pub fn new() -> Self {
        Game::default()
    }

    pub fn add_base_systems(&mut self) {

    }

    pub fn add_system(&mut self, system: Box<dyn System>) {
        if self.systems.is_none() {
           let a: Vec<Box<dyn System>> = vec![];
           self.systems = Some(a);
        }
        &self.systems.as_mut().unwrap().push(system);
    }

    pub fn run(self) {
        info("Running Game...");
        //------------------------------
        //  Run Init on all Systems
        //------------------------------
        let glfw_window_unwrapped = self.glfw_window;
        let mut graphics_handler: GraphicsHandler = GraphicsHandler { ..Default::default() };
        let context: std::rc::Rc<moonhare_graphics::glium::backend::Context>;

        context = moonhare_graphics::build_context(glfw_window_unwrapped.clone().unwrap().glfw_window);
        
        graphics_handler.context = Some(context.clone());
        let mut value = glfw_window_unwrapped;
        while self.is_running {
            handle_window_event(value.as_mut().unwrap());
            render(context.clone());

            self.base_systems.game_loop();
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
fn handle_window_event(glfw_window: &GLFWWindow) {
    glfw_window.glfw_window.borrow_mut().glfw.poll_events();
    for (_, event) in moonhare_window::glfw::flush_messages(&glfw_window.events.borrow()) {
        moonhare_window::platforms::glfw_window::GLFWWindow::handle_window_event(&glfw_window, event);
    }   
}

fn render(context: Rc<Context>) {
    let target = moonhare_graphics::glium::Frame::new(context.clone(), context.get_framebuffer_dimensions());
    moonhare_graphics::draw_background_color(Color::color_from_rgb(255, 255, 255), target);
}
