use moonhare_window::{window_config, winit_window::{self, WinitWindow}};

use crate::{basic::node::Node, Game};

#[derive(Debug)]
pub struct GameWindow {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
    pub visble: bool,
    pub decorations: bool,
    pub winit_window: Option<WinitWindow>,
}
              
impl Node for GameWindow {
}

impl Default for GameWindow {
    fn default() -> Self {
        Self { 
            title: "window", 
            width: default_game_window_width(),
            height: default_game_window_height(), 
            visble: default_game_window_visibility(), 
            decorations: default_game_window_decorations(),
            winit_window: None,
        }
    }
}

impl GameWindow {
    pub fn create() -> Self {
        let mut window_config = window_config::WindowConfig::default();
        moonhare_log::info(format!("creating window with config {:?}", window_config));
        let mut window = Self::default();
        window_config.title = window.title.to_owned();
        window_config.width = window.width;
        window_config.height = window.height;
        window_config.visble = window.visble;
        window_config.decorations = window.decorations;
        
        let winit = winit_window::WinitWindow::new(window_config);
        
        window.winit_window = Some(winit);
        // todo: tell winit to create a window for us 
        moonhare_log::info(format!("created window {:?}", window));
        window
    }
}

fn default_game_window_title() -> String {
    "Moonhare Engine".to_owned()
}

fn default_game_window_width() -> u32 {
    1280
}

fn default_game_window_height() -> u32 {
    720
}

fn default_game_window_visibility() -> bool {
    true
}

fn default_game_window_decorations() -> bool {
    true
}