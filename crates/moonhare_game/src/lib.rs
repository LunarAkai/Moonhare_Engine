//! Base functionality for a Moonhare Game Engine Project

use moonhare_log::*;
use moonhare_window::{Window, WindowRenderContext};

pub mod basic;

/// Only one Game may exist per project
#[derive(Debug)]
pub struct Game {
    pub is_running: bool,
    pub name: String,
    pub context: WindowRenderContext,
}

impl Default for Game {
    fn default() -> Self {
        Self { 
            is_running: true,
            name: default_game_name(),
            context: WindowRenderContext::OPENGLGLFW
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game::default()
    }

    pub fn run(&self) {
        info("Running Game...");
        while self.is_running {
            
        }
    }

    pub fn add_window(&mut self) {
        moonhare_log::info(format!("Adding window to {:?}", self));
        Window::create(self.context);
    }
}

fn default_game_name() -> String {
    "Moonhare Game".to_owned()
}