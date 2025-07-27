use glium::{backend::glutin::SimpleWindowBuilder, glutin::surface::WindowSurface, winit::{event_loop::EventLoop, window::Window}, Surface};

use crate::{game_plugin::GamePlugin, ENGINE_NAME};

pub struct Game {
    pub running: bool,
    pub game_plugin: Option<Box<dyn GamePlugin>>,
    pub window: Window,
    pub display: glium::Display<WindowSurface>,
    pub event_loop: EventLoop<()>
}

impl Game {
    pub fn new() -> Self {
        let _event_loop = EventLoop::new().unwrap();
        let _window = SimpleWindowBuilder::new().with_title(ENGINE_NAME).build(&_event_loop);

        Game { 
            running: true, 
            game_plugin: None, 
            window: _window.0, 
            display: _window.1, 
            event_loop: _event_loop, 
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn GamePlugin>) {
        self.game_plugin = Some(plugin);
    }

    pub fn init(&mut self) {
        if let Some(ref mut game_plugin) = self.game_plugin {
            game_plugin.init();
        } else {
            panic!("Needs Game Plugin to run!");
        }
    }   

    pub fn update(&mut self) {
        if let Some(ref mut game_plugin) = self.game_plugin {
            game_plugin.update();
        }
    }

    pub fn render(&mut self) {
        let mut target = self.display.draw();

        if let Some(ref mut game_plugin) = self.game_plugin {
            game_plugin.render(&mut target);
        }
            
        target.finish().unwrap();   
    }

    pub fn cleanup(&mut self) {
        if let Some(ref mut game_plugin) = self.game_plugin {
            game_plugin.cleanup();
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.update();
            self.render();
        }

        self.cleanup();
    }

    pub fn get_display(&self) -> &glium::Display<WindowSurface>{
        &self.display
    }

    pub fn get_window(self) -> Window {
        self.window
    }
}


/* impl ApplicationHandler for Game {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {

    }

    fn window_event(
            &mut self,
            event_loop: &event_loop::ActiveEventLoop,
            window_id: window::WindowId,
            event: WindowEvent,
        ) {
        let _ = window_id;    
        
        

        match event {

            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Resized(window_size) => {
                if let Some(window) = &self.window {
                    if let Some(display) = &self.display {
                        display.resize(window_size.into());
                    }                    
                } 
            },

            WindowEvent::RedrawRequested => { 
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            },
            _ => (),
        }

    }
    fn about_to_wait(&mut self, _: &event_loop::ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

} */
