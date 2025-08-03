use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::Window};

use crate::MoonhareWindow;


#[derive(Debug, Default)]
pub struct WinitWindow {
    window: Option<Arc<Window>>,
}

impl WinitWindow {
    pub fn new() -> Self {
        Self { 
            window: None, 
        }
    }
    
}

const APP_ID: &str = "de.lunarakai.moonhare_engine";

impl ApplicationHandler for WinitWindow {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(Arc::new(event_loop.create_window(Window::default_attributes()).unwrap()));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
 match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}



impl MoonhareWindow for WinitWindow {  
    type WindowResult = WinitWindow; 
    fn init() -> Self::WindowResult {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let mut app = WinitWindow::default();
        let _ = event_loop.run_app(&mut app);

        // Need to find a better way because this will not return until the window is closed
        app
    }


    fn on_update() {
        
    }

    fn shutdown() {
        // todo: emit WindowCloseEvent
        
    }
}

