// game/src/lib.rs
mod game;
mod ui;

// Pull the Game struct into the current scope so you can just type 'Game'
use crate::game::Game; 
use engine_app::app::App;
use winit::event_loop::EventLoop;

pub fn run() -> anyhow::Result<()> {
    // Initialize the logger
    env_logger::init();

    // Create the event loop
    let event_loop = EventLoop::with_user_event().build()?;

    // Create the game instance
    let game_instance = Game::new();

    // Create the application
    let mut app = App::new(game_instance);

    // Run the application with the event loop
    event_loop.run_app(&mut app)?;
    Ok(())
}

