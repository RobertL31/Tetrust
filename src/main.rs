mod graphics;
mod game_manager;
mod piece;
mod gameboard;
mod piece_provider;
mod piece_factory;
mod setup;

use game_manager::GameManager;

fn main(){
    
    let config = match setup::get_config() {
        Ok(conf) => conf,
        Err(err) => {
            println!("{:?}", err.to_string());
            return;
        }
    };

    GameManager::start(config);
}
