//! Base functionality for a Moonhare Game Engine Project

use std::{any::Any, rc::Rc};


use moonhare_graphics::{color::Color, glium::{backend::Context, glutin::api::egl::context}};
use moonhare_log::*;
use moonhare_window::{platforms::glfw_window::GLFWWindow};

use crate::{basic::world::{self, World}, systems::system::{BaseSystems, System}};

pub mod systems;
pub mod basic;
pub mod nodes;
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
#[derive(Debug, Clone)]
pub struct Game {
    pub world: Vec<World>,
    pub is_running: bool
}


impl Default for Game {
    fn default() -> Self {
        Self { 
            world: vec![],
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
    pub fn add_world(&mut self, world: World) {
        self.world.push(world);
    }

    pub fn get_worlds(self) -> Vec<World> {
        self.world
    }

    pub fn init(&self) {
        for world in &self.world {
            world.clone().init();
        }
    }

    pub fn run(&mut self) {
        let worlds = self.world.clone();
        while self.is_running {
            for world in &worlds {
                <World as Clone>::clone(&world).update();
            }
        }
    }

}


fn default_game_name() -> String {
    "Moonhare Game".to_owned()
}


