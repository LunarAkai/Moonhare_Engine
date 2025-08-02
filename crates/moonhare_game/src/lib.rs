//! Base functionality for a Moonhare Game Engine Project

use moonhare_log::*;
use moonhare_window::{Window, WindowRenderContext};

pub mod basic;

/// Only one Game may exist per project
#[derive(Debug)]
pub struct Game {
    pub name: String,
    pub context: WindowRenderContext,
}

impl Default for Game {
    fn default() -> Self {
        Self { 
            name: default_game_name(),
            context: WindowRenderContext::OPENGLGTK
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game::default()
    }

    pub fn run(&self) {
        info("Running Game...");
        loop {
            
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