use winit::{application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, event_loop::{self, ActiveEventLoop, EventLoop}, window::{Window, WindowAttributes}};

use crate::window_config::WindowConfig;

#[derive(Debug)]
pub struct WinitWindow {
    pub window: Window,
}

impl WinitWindow {
    pub fn new(config: WindowConfig) -> Self {
        moonhare_log::trace("Im inside the create window function in winit");
        let event_loop = EventLoop::new().unwrap();
        let mut window_attributes = WindowAttributes::default()
            .with_title(config.title);

        let logical_size = LogicalSize::new(config.width, config.height);
        window_attributes = window_attributes.with_inner_size(logical_size);
        window_attributes = window_attributes.with_max_inner_size(logical_size);

        // Set Visible    
        window_attributes = window_attributes.with_visible(config.visble);
        window_attributes = window_attributes.with_decorations(config.decorations);

        let window = match event_loop.create_window(window_attributes) {
            Ok(window) => window,
            Err(err) => {
                moonhare_log::error("Error creating window: {err}");
                return;
            },
        };

        moonhare_log::info(format!("Winit WIndow: {:?}", window));

        Self { window: window }
    }
}

impl ApplicationHandler for WinitWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                
            },
            _ => (),
        }
    }
}



