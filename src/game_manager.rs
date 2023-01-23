use std::time::Duration;

use std::io::stdin;

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::gameboard::GameBoard;
use crate::traits::{Move, FallError};

const GLOBAL_SEED: u64 = 1;
struct GameManager;

impl GameManager {

    pub fn start(&self) {

        let rng = ChaCha8Rng::seed_from_u64(GLOBAL_SEED);

        let mut board = GameBoard::new(rng);
        
        loop {
            board.step();
        }
        
        
    }

    pub fn end(&self) {
        
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
            self.start();
        }
    }

}

