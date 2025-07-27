use glium::{backend::glutin::SimpleWindowBuilder, glutin::config::{ConfigTemplate, ConfigTemplateBuilder}, winit::window::WindowAttributes};

use crate::ENGINE_NAME;

/// General Config for [`WinitWindow`](crate::winit::winit_window::WinitWindow)
pub struct WindowConfig {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
    pub visble: bool,
    pub decorations: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self { 
            title: ENGINE_NAME, 
            width: 1280, 
            height: 720,
            visble: default_visibility(),
            decorations: default_decorations(),
        }
    }
}

fn default_visibility() -> bool {
    true
}

fn default_decorations() -> bool {
    true
}