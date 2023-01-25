
use std::io::stdin;

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::gameboard::GameBoard;

pub const GLOBAL_SEED: u64 = 1;
pub struct GameManager;

impl GameManager {

    pub fn start() {

        let rng = ChaCha8Rng::seed_from_u64(GLOBAL_SEED);

        let mut board = GameBoard::new(rng);
        
        while board.keep_playing() {
            match board.try_fall() {
                Ok(_) => (),
                Err(_) => board.lock_current_piece()
            }
        }

        println!("{:?}", board);

        GameManager::end();
    }

    pub fn end() {
        
        let mut s=String::new();
        println!("Please enter some text: ");
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        
        if let Some('r') = s.chars().next_back() {
            GameManager::start();
        }
    }

}

