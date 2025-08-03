//! Provides functionality to create either a vulkan or opengl window
pub mod window_config;
pub mod platforms;
pub use glfw as glfw;

use crate::platforms::glfw_window::GLFWWindow;

#[derive(Debug, Clone, Copy)]
pub enum WindowRenderContext {
    OPENGLGLFW,
}

pub trait WindowResult {
}

pub trait MoonhareWindow {
    fn init() -> GLFWWindow;
    fn on_update();
    fn shutdown();
}

pub struct Window {
}


impl Window {
    #[cfg(target_os = "linux")]
    pub fn create(context: WindowRenderContext) -> GLFWWindow {
        match context {
            WindowRenderContext::OPENGLGLFW => {
                use crate::platforms::glfw_window::GLFWWindow;
                moonhare_log::info("Creating GLFW OpenGL Window");
                GLFWWindow::init()
            }
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn create() {
        todo!("moonhare engine only supports linux for now")
    }
}