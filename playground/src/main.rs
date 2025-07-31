use moonhare_engine::{game::Game, log};


fn main() {
    let _ = log::configere_logger();
    log::info("test");
    
    let mut game = Game::new();
    game.add_window();
    
    log::info(format!("Game: {:?}", game));
    
    // Enters Loop
    game.run();
}