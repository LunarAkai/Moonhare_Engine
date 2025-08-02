//! Base functionality for a Moonhare Game Engine Project

use moonhare_log::*;
use moonhare_window::Window;

pub mod basic;

/// Only one Game may exist per project
#[derive(Debug)]
pub struct Game {
    pub name: String,
}

impl Default for Game {
    fn default() -> Self {
        Self { 
            name: default_game_name(),
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
        Window::create();
        
    }
}

fn default_game_name() -> String {
    "Moonhare Game".to_owned()
}