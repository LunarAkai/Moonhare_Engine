//! Provides functionality to create either a vulkan or opengl window
pub mod window_config;
pub mod platforms;

#[derive(Debug, Clone, Copy)]
pub enum WindowRenderContext {
    VULKANGTK, // TODO
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
    /// creates a gtk4 window while spaning a new thread that the window runs on.
    /// here: gtk sends engine events when _things happen_ with the window that other engine parts can interact with
    #[cfg(target_os = "linux")]
    pub fn create(context: WindowRenderContext) {
        match context {
            WindowRenderContext::VULKANGTK => {
                todo!()
            },
            WindowRenderContext::OPENGLGTK => {
                use std::thread;

                use gtk4::gio::prelude::ApplicationExtManual;

                use crate::platforms::gtk_window::GTKWindow;
                
                thread::spawn(|| {
                    moonhare_log::info("Created Window thread");
                    let application = GTKWindow::init();
                    application.get_application().run();
                });
            },
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn create() {
        todo!("moonhare engine only supports linux for now")
    }
}