use std::{ops::{ControlFlow, DerefMut}, sync::Mutex};

use glium::{glutin::surface::WindowSurface, winit::{self, event::{self, WindowEvent}, event_loop::{self, EventLoop}, window::Window}};

use crate::{game, game_plugin::GamePlugin, winit::winit_window::WinitWindow, ENGINE_NAME};


pub struct Game {
    pub running: bool,
    pub game_plugin: Option<Box<dyn GamePlugin>>,
    pub window: Window,
    pub display: glium::Display<WindowSurface>,
    pub event_loop: EventLoop<()>
}

impl Game {
    pub fn new() -> Self {
        let _event_loop: EventLoop<()> = EventLoop::new().unwrap();

        let _window = WinitWindow::construct_window(&_event_loop);

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
        self.window.set_fullscreen(None);
        self.window.set_decorations(true);
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

    pub fn handle_events(&mut self, event: winit::event::Event<()>) {
        if let Some(ref mut game_plugin) = self.game_plugin {
            game_plugin.handle_events();
        }
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    
                },
                glium::winit::event::WindowEvent::Resized(window_size) => {
                   
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    
                },
                _ => (),
            }
            glium::winit::event::Event::AboutToWait => {

            },
            _ => (),
        }
    }   
}
