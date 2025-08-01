//! Provides functionality to create either a vulkan or opengl window

use crate::opengl_window::create_open_gl_window;
pub mod window_config;
pub mod opengl_window;

#[derive(Debug, Clone, Copy)]
pub enum WindowRenderContext {
    VULKAN, // TODO
    OPENGL,
}

#[derive(Debug, Clone, Copy)]
pub struct MoonhareWindow {
    render_context: WindowRenderContext
}

impl MoonhareWindow {
    pub fn define_context(context: WindowRenderContext) -> Self {
        Self {
            render_context: context
        }
    }

    pub fn create_window_from_context(self) {
        match self.render_context {
            WindowRenderContext::VULKAN => {
                todo!("Vulkan not implemented yet")
            },
            WindowRenderContext::OPENGL => {
                create_open_gl_window();
            }
        }
    }
}