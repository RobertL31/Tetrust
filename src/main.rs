use game_manager::GameManager;

mod game_manager;
mod piece;
mod gameboard;
mod piece_provider;
mod piece_factory;

fn main() {
    
    GameManager::start();
}
