
use glium::{backend::glutin::SimpleWindowBuilder, glutin::{display::GetGlDisplay, surface::WindowSurface}, winit::{self, dpi::LogicalSize, event_loop::{ActiveEventLoop, EventLoop}, raw_window_handle::HasDisplayHandle, window::{Window, WindowAttributes}}, Display};

use crate::winit::window_config::WindowConfig;

use crate::ENGINE_NAME;

pub struct WinitWindow {}

impl WinitWindow {
    /// constructs a new winit window
    pub fn construct_window(event_loop: &EventLoop<()>) -> (Window, Display<WindowSurface>) {
        let config = WindowConfig::default();
        
        let mut window_attributes = WindowAttributes::default();

        let logical_size = LogicalSize::new(config.width, config.height);
        window_attributes = window_attributes.with_inner_size(logical_size);
        window_attributes = window_attributes.with_max_inner_size(logical_size);

        window_attributes = window_attributes.with_title(config.title)
            .with_fullscreen(None);

        // Set Visible    
        window_attributes = window_attributes.with_visible(true);

        window_attributes = winit::platform::wayland::WindowAttributesExtWayland::with_name(window_attributes, ENGINE_NAME, "");
        
        SimpleWindowBuilder::new().set_window_builder(window_attributes).build(event_loop)
    }
}



