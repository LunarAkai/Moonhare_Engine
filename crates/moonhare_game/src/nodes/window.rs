use std::rc::Rc;

use moonhare_derives::Node;
use moonhare_graphics::{color::Color, glium::backend::Context};
use moonhare_window::{glfw::RenderContext, platforms::glfw_window::{self, GLFWWindow}};
use crate::nodes::node::Node;

#[derive(Node, Clone)]
pub struct Window {
    context: moonhare_window::WindowRenderContext,
    glfw_window: Option<moonhare_window::platforms::glfw_window::GLFWWindow>,
    render_context: Option<Rc<Context>>
}

impl Default for Window {
    fn default() -> Self {
        Self { 
            context: moonhare_window::WindowRenderContext::OPENGLGLFW, 
            glfw_window: None, 
            render_context: None 
        }
    }
}

impl Window {
    fn init(&mut self) -> Self {
        Self {
            context: moonhare_window::WindowRenderContext::OPENGLGLFW,
            glfw_window: Some(moonhare_window::Window::create(self.context)),
            render_context: Some(moonhare_graphics::build_context(self.glfw_window.clone().unwrap().glfw_window))
        }
    }

    fn update(&mut self) {
        handle_window_event(&self.glfw_window.as_mut().unwrap());
        render(self.render_context.clone().unwrap());
    }
}

fn handle_window_event(glfw_window: &GLFWWindow) {
    glfw_window.glfw_window.borrow_mut().glfw.poll_events();
    for (_, event) in moonhare_window::glfw::flush_messages(&glfw_window.events.borrow()) {
        moonhare_window::platforms::glfw_window::GLFWWindow::handle_window_event(&glfw_window, event);
    }   
}

fn render(context: Rc<Context>) {
    let target = moonhare_graphics::glium::Frame::new(context.clone(), context.get_framebuffer_dimensions());
    moonhare_graphics::draw_background_color(Color::color_from_rgb(255, 255, 255), target);
}