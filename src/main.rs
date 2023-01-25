mod graphics;
mod game_manager;
mod piece;
mod gameboard;
mod piece_provider;
mod piece_factory;

use console::Term;

use game_manager::GameManager;

fn main() {
    
    GameManager::start();
}
