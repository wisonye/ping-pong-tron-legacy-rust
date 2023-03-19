mod ball;
mod config;
mod game;
mod player;
mod scoreboard;
mod table;

use game::Game;

fn main() {
    match Game::init() {
        Ok(mut game) => game.run(),
        Err(e) => println!("\n>>> [ Game init fail ]: {:?}", e),
    }
}
