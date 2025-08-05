use std::fmt::{Debug, Formatter};

use crate::systems::{render_system::RenderSystem, update_system::UpdateSystem, window_system::WindowSystem};

/// Systems are collections of related High Level Game Logic
/// Systems can have Nodes as children
/// These node than do all the concrete/low level stuff they want to do
pub trait System {}

impl Debug for dyn System {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "")
    } 
}

// todo: make this more generic so that new systems can be added by the game application it self
//      or systems can be modified to suit the game needs

/// Base Systems/Plugins of the Engine
#[derive(Debug)]
pub struct BaseSystems {
    window_system: WindowSystem,
    update_system: UpdateSystem,
    render_system: RenderSystem,
}

impl BaseSystems {
    pub fn new() -> Self {
        Self { 
            window_system: WindowSystem::default(), 
            update_system: UpdateSystem, 
            render_system: RenderSystem 
        }
    }
    pub fn game_loop(&self) {
        self.window_system.update();

        self.update_system.update();

        self.render_system.update();   
    }
}