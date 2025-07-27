pub mod vertex;
pub mod game;
pub mod game_plugin;

use glium::{glutin::surface::WindowSurface, uniform, uniforms, winit::{event::{self, StartCause}, event_loop::{self, EventLoop, EventLoopBuilder}, window::{self, Window}}, Display, Surface};



const ENGINE_NAME: &str = "Moonhare Engine";

pub struct CPointer<T>(T);

impl<T> Drop for CPointer<T> {
    fn drop(&mut self) {
        println!("Dropping")
    }
}

// rescaling:   position *= factor;
// rotating:    new_position = vec2(pos.x * cos(angle) - pos.y * sin(angle), pos.x * sin(single) + pos.y * cos(angle));
// skewing:     position.x += position.y * factor;




