#[derive(Debug)]
pub struct WindowSystem {
    context: moonhare_window::WindowRenderContext,
    glfw_window: Option<moonhare_window::platforms::glfw_window::GLFWWindow>,
}

impl Default for WindowSystem {
    fn default() -> Self {
        Self {
            context: moonhare_window::WindowRenderContext::OPENGLGLFW,
            glfw_window: None,
        }
    }
}

impl WindowSystem {
    pub(crate) fn update(&self) {}
}
