//! Base functionality for a Moonhare Game Engine Project
pub mod basic;

/// Only one Game may exist per project
pub struct Game {
    pub name: String
}

impl Default for Game {
    fn default() -> Self {
        Self { 
            name: default_game_name()
        }
    }
}

fn default_game_name() -> String {
    "Moonhare Game".to_owned()
}