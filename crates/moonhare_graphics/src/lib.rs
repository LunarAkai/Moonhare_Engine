//! Crate for providing an abstraction layer over different graphics APIs

use std::{cell::RefCell, rc::Rc};

use glium::{backend::Context, Surface};
use moonhare_window::glfw::{PWindow, Window};
pub mod shader;
pub mod backend;
pub use glium;


pub fn build_context(window: PWindow) -> Rc<Context>{
    let gl_window = Rc::new(RefCell::new(window));
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
