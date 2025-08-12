use std::{
    cell::RefCell,
    ffi::{CString, c_void},
    rc::Rc,
};

use glium::SwapBuffersError;
use moonhare_window::glfw::Context;

// adopted from the glium repo -> examples -> manual_creation.rs
#[derive(Clone)]
pub struct Backend {
    pub gl_window: Rc<RefCell<moonhare_window::glfw::PWindow>>,
}

unsafe impl glium::backend::Backend for Backend {
    fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
        self.gl_window.borrow_mut().swap_buffers();
        Ok(())
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        let symbol = CString::new(symbol).unwrap();
        self.gl_window
            .borrow_mut()
            .get_proc_address(&symbol.to_str().unwrap()) as *const _
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        let window = &self.gl_window.borrow();
        (window.get_size().0 as u32, window.get_size().1 as u32)
    }

    fn resize(&self, new_size: (u32, u32)) {
        let _ = &self
            .gl_window
            .borrow_mut()
            .set_size(new_size.0 as i32, new_size.1 as i32);
    }

    fn is_current(&self) -> bool {
        self.gl_window.borrow().is_current()
    }

    unsafe fn make_current(&self) {
        let mut gl_window = self.gl_window.borrow_mut();
        gl_window.make_current();
    }
}
