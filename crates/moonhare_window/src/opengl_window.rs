use glium::{winit::dpi::{LogicalPosition, LogicalSize}, Surface};

use crate::window_config::WindowConfig;

pub fn create_open_gl_window() {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let (mut _window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let config = WindowConfig::default();
    _window.set_title(format!("{} OpenGL", &config.title).as_str());
    _window.set_decorations(config.decorations);
    _window.set_visible(config.visble);
    //_window.set_min_inner_size(Some(LogicalSize::new(config.width, config.height)));


    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.finish().unwrap();

    let _ = event_loop.run(move | event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit()
                },
                _ => (),
            }
            _ => (),
        }
    });
}

