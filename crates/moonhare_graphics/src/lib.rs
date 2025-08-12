//! Crate for providing an abstraction layer over different graphics APIs

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use glium::{
    Frame, Surface,
    backend::{Context, Facade},
};
use lazy_static::lazy_static;
use moonhare_window::glfw::PWindow;
pub mod backend;
pub mod color;
pub mod shader;
pub mod vertices;
pub use glium;
use state::InitCell;

use crate::color::Color;

pub fn build_context(window: Rc<RefCell<PWindow>>) -> Rc<Context> {
    let gl_window: Rc<RefCell<PWindow>> = window;
    // now building the context

    let context = unsafe {
        // The first parameter is our backend.
        //
        // The second parameter tells glium whether or not it should regularly call `is_current`
        // on the backend to make sure that the OpenGL context is still the current one.
        //
        // It is recommended to pass `true`, but you can pass `false` if you are sure that no
        // other OpenGL context will be made current in this thread.
        let backend = backend::Backend {
            gl_window: gl_window,
        };
        glium::backend::Context::new(backend, true, Default::default())
    }
    .unwrap();
    context
}

pub fn draw_background_color(color: Color, mut target: Frame) {
    Surface::clear_color(&mut target, color.red, color.green, color.blue, color.alpha);
    target.finish().unwrap()
}
