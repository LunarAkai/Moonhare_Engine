use std::{ops::{ControlFlow, DerefMut}, sync::Mutex};

use glium::{glutin::{api::egl::surface::Surface, surface::WindowSurface}, winit::{self, event::{self, WindowEvent}, event_loop::{self, EventLoop}, window::Window}, Display};
use winit::application::ApplicationHandler;

use crate::{core::game, game_plugin::GamePlugin, window::winit_window::WinitWindow, ENGINE_NAME};


pub struct Game {
    pub running: bool,
    pub game_plugin: Option<Box<dyn GamePlugin>>,
    pub window: WinitWindow,
}

impl Game {
    pub fn new() -> Self {
        let _event_loop: EventLoop<()> = EventLoop::new().unwrap();

        let mut game_window = WinitWindow::default();
        _event_loop.run_app(&mut game_window);

        Game { 
            running: true, 
            game_plugin: None, 
            window: game_window,
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
        //let mut target = display.draw();

        //if let Some(ref mut game_plugin) = self.game_plugin {
        //    game_plugin.render(&mut target);
        //}
            
        //target.finish().unwrap();   
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
}
