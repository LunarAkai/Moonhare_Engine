use std::fs::File;

use moonhare_engine::{
    game::{Game, basic::world::World, nodes::window::Window},
    log,
};

fn main() {
    let _ = log::configere_logger();
    log::info("test");

    let mut game = Game::new();
    let mut world = World::new();

    let window = Window::default();
    world.add_node(Box::new(window));
    game.add_world(world.clone());
    log::info(format!("{:?}", game.get_worlds()));
}
