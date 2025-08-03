//! Provides functionality to create either a vulkan or opengl window
pub mod window_config;
pub mod platforms;

#[derive(Debug, Clone, Copy)]
pub enum WindowRenderContext {
    VULKANGTK, // TODO
    OPENGLGTK,
    OPENGLWINIT,
    OPENGLGLFW,
}

pub trait WindowResult {
}

pub trait MoonhareWindow {
    fn init();
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
                todo!()
            },
            WindowRenderContext::OPENGLWINIT => {
                use crate::platforms::winit_window::WinitWindow;

                moonhare_log::info("Creating Winit OpenGL Window");
                WinitWindow::init();
            },
            WindowRenderContext::OPENGLGLFW => {
                std::thread::spawn(|| {
                    use crate::platforms::glfw_window::GLFWWindow;
                    moonhare_log::info("Creating GLFW OpenGL Window");
                    GLFWWindow::init();
                });

            }
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn create() {
        todo!("moonhare engine only supports linux for now")
    }
}