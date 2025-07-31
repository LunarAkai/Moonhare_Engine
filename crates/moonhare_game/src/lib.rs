//! Base functionality for a Moonhare Game Engine Project

use moonhare_log::*;

use crate::basic::game_window::GameWindow;
pub mod basic;

/// Only one Game may exist per project
#[derive(Debug)]
pub struct Game {
    pub name: String,
    pub primary_window: Option<GameWindow>,
}

impl Default for Game {
    fn default() -> Self {
        Self { 
            name: default_game_name(),
            primary_window: None, 
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
        if self.primary_window.is_none() {
            moonhare_log::trace("Primary Window is none");
            self.primary_window = Some(GameWindow::create());
        }
    }
}

fn default_game_name() -> String {
    "Moonhare Game".to_owned()
}