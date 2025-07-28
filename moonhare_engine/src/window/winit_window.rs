
use glium::{backend::{glutin::SimpleWindowBuilder, Context}, glutin::{display::GetGlDisplay, prelude::GlContext, surface::WindowSurface}, winit::{self, dpi::LogicalSize, event_loop::{ActiveEventLoop, EventLoop}, raw_window_handle::HasDisplayHandle, window::{Window, WindowAttributes}}, Display};
use winit::application::ApplicationHandler;

use crate::window::window_config::WindowConfig;

use crate::ENGINE_NAME;

#[derive(Default)]
pub struct WinitWindow {
    pub window: Option<Window>,
}


impl ApplicationHandler for WinitWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
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


        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        todo!()
    }
}



