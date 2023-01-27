
use std::{io::stdin, thread, time::{Duration, Instant}, sync::mpsc::{self}};

use console::Term;
use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha8Rng;

use crate::{gameboard::{GameBoard, MovementDirection, Action}, graphics::{AsciiVisualizer, Visualizer}};

pub const FALL_TIME: f32 = 1.0 / 4.0;
pub const GLOBAL_SEED: u64 = 16;
pub const FPS: u32 = 60;
pub const SLEEP_TIME: f32 = 1.0/FPS as f32;
pub const LOCK_DELAY: f32 = 2.0 * FALL_TIME;
pub struct GameManager;

impl GameManager {

    pub fn start() {

        let seed = rand::thread_rng().gen::<u64>();
        let stdout = Term::buffered_stdout();
        let rng = ChaCha8Rng::seed_from_u64(seed);
        let mut level = 1;
        let mut board = GameBoard::new(rng, level);

        let (to_main, from_thread) = mpsc::channel::<Action>();
        let keyboard_listener = thread::spawn(move || {
            loop {
                if let Ok(character) = stdout.read_char() {
                    let _ = match character.to_ascii_lowercase() {
                        'z' => to_main.send(Action::Move(MovementDirection::Top)),
                        'q' => to_main.send(Action::Move(MovementDirection::Left)),
                        's' => to_main.send(Action::Move(MovementDirection::Bottom)),
                        'd' => to_main.send(Action::Move(MovementDirection::Right)),
                        'm' => to_main.send(Action::Rotate),
                        'h' => to_main.send(Action::Hold),
                        _ => Ok(())
                    };
                }
            }
        });
        

        AsciiVisualizer::display(&board);
        
        let mut last_fall = 0.0;
        let mut lock_timer = 0.0;
        let mut start = Instant::now();
        while board.keep_playing() {

            let _ = match from_thread.try_recv() {
                Ok(Action::Move(movement)) => {
                    let _ = match board.try_move(movement) {
                        Ok(_) => {
                            match movement {
                                MovementDirection::Top => lock_timer = LOCK_DELAY,
                                _ => lock_timer = 0.0
                            }
                        }
                        Err(_) => ()
                    };
                },
                Ok(Action::Rotate) => {
                    let _ = match board.try_rotate(){
                        Ok(_) => lock_timer = 0.0,
                        Err(_) => ()
                    };
                },
                Ok(Action::Hold) => {
                    let _ = match board.try_swap(){
                        Ok(_) => lock_timer = 0.0,
                        Err(_) => ()
                    };
                },
                Err(_) => ()
            };
            
            let time_delta = start.elapsed().as_secs_f32();
            last_fall += time_delta;
            lock_timer += time_delta;
            start = Instant::now();
            if last_fall >= FALL_TIME {
                match board.try_fall() {
                    Ok(_) => lock_timer = 0.0,
                    Err(_) => {
                        if lock_timer >= LOCK_DELAY {
                            board.lock_current_piece();
                        }
                    }
                }
                last_fall = 0.0;
            }

            AsciiVisualizer::display(&board);
        }

        keyboard_listener.join().unwrap();

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


