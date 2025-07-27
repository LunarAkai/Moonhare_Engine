use std::sync::{Arc, Mutex};

use glium::{glutin::surface::WindowSurface, winit::{application::ApplicationHandler, event::{Event, WindowEvent}, event_loop::{self, EventLoop}, window::{self, Window, WindowAttributes}}, Display};

use crate::game_plugin::GamePlugin;

pub struct Game {
    pub running: bool,
    pub game_plugin: Option<Box<dyn GamePlugin>>,
    pub window: Option<Window>,
    pub display: Option<glium::Display<WindowSurface>>,
    pub event_loop: EventLoop<()>
}

impl Game {
    pub fn new() -> Self {
        Self { 
            running: true,
            game_plugin: None,
            window: None,
            display: None,
            event_loop: init_event_loop(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn GamePlugin>) {
        self.game_plugin = Some(plugin);
    }

    pub fn init(&mut self) {
        if let Some(ref mut game_plugin) = self.game_plugin {
            game_plugin.init();
        } else {
            //todo!("Default Impl init")
        }
        
        let (_window, _display) = return_window(&self.event_loop);
        self.window = Some(_window);
        self.display = Some(_display);
    }   

    pub fn update(&mut self) {
        if let Some(ref mut game_plugin) = self.game_plugin {
            game_plugin.update();
        } else {
            //todo!("Default Impl update")
        }
    }

    pub fn render(&mut self) {
        if let Some(ref display) = self.display{
            let mut target = display.draw();

            if let Some(ref mut game_plugin) = self.game_plugin {
                game_plugin.render(&mut target);
            } else {
                //todo!("Default Impl render")
            }
            
            let _ = &target.finish().unwrap();
        }    
    }

    pub fn cleanup(&mut self) {
        if let Some(ref mut game_plugin) = self.game_plugin {
            game_plugin.cleanup();
        } else {
            //todo!("Default Impl cleanup")
        }
    }

    pub fn run(&mut self) {
        self.init();

        while self.running {
            self.update();
            self.render();
        }

        self.cleanup();
    }

    pub fn get_display(&self) -> &glium::Display<WindowSurface>{
        self.display.as_ref().unwrap()
    }

    pub fn get_window(self) -> Option<Window> {
        return self.window;
    }
}


impl ApplicationHandler for Game {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap())
    }

    fn window_event(
            &mut self,
            event_loop: &event_loop::ActiveEventLoop,
            window_id: window::WindowId,
            event: WindowEvent,
        ) {    
        
        match event {

            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Resized(window_size) => {
                self.display.as_ref().unwrap().resize(window_size.into());
            },

            WindowEvent::RedrawRequested => { 
                self.window.as_ref().unwrap().request_redraw();
            },
            _ => (),
        }

        }

}


fn init_event_loop() -> EventLoop<()> {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    return event_loop;
}

fn return_window(event_loop: &EventLoop<()>) -> (Window, Display<WindowSurface>) {
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title(crate::ENGINE_NAME)
        .build(event_loop);
      
    return (_window, display);
}
