//! Provides functionality to create either a vulkan or opengl window

use std::marker;

pub mod window_config;

pub mod platforms;

#[derive(Debug, Clone, Copy)]
pub enum WindowRenderContext {
    VULKAN, // TODO
    OPENGL,
    OPENGLGTK,
}

pub trait WindowResult {
}

pub trait MoonhareWindow {
    type WindowResult;
    fn init() -> Self::WindowResult;
    fn on_update();
    fn shutdown();
}

pub struct Window {

}

impl Window {
    /// creates a gtk4 window
    #[cfg(target_os = "linux")]
    pub fn create() {
        use gtk4::gio::prelude::ApplicationExtManual;

        use crate::platforms::gtk_window::GTKWindow;

        let application = GTKWindow::init();
        
        application.get_application().run();
    }

    #[cfg(not(target_os = "linux"))]
    pub fn create() {
        todo!("moonhare engine only supports linux for now")
    }
}