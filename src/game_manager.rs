
use std::{io::stdin, thread, time::Duration};

use console::Term;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::{gameboard::{GameBoard, MovementDirection}, graphics::{AsciiVisualizer, Visualizer}};

pub const FALL_TIME: f32 = 1.0 / 4.0;
pub const GLOBAL_SEED: u64 = 1;
pub const FPS: u32 = 30;
pub const SLEEP_TIME: f32 = 1.0/FPS as f32;
pub struct GameManager;

impl GameManager {

    pub fn start() {

        let stdout = Term::buffered_stdout();

        let rng = ChaCha8Rng::seed_from_u64(GLOBAL_SEED);

        let mut board = GameBoard::new(rng);
        
        AsciiVisualizer::display(&board);
        
        let mut last_fall = 0.0;
        while board.keep_playing() {

            if let Ok(character) = stdout.read_char() {
                let _ = match character {
                    'Z' => Ok(()),
                    'Q' => board.try_move(MovementDirection::Left),
                    'S' => Ok(()),
                    'D' => board.try_move(MovementDirection::Right),
                    _ => Ok(())
                };
            }
            
            if last_fall >= FALL_TIME as f32 {
                match board.try_fall() {
                    Ok(_) => (),
                    Err(_) => board.lock_current_piece()
                }
                AsciiVisualizer::display(&board);

                last_fall = 0.0;
            }

            thread::sleep(Duration::from_secs_f32(SLEEP_TIME));
            last_fall += SLEEP_TIME;
        }

        

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

