
use std::{io::stdin, thread, time::Instant, sync::mpsc::{self}};

use console::Term;
use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha8Rng;

use crate::{gameboard::{GameBoard, MovementDirection, Action}, graphics::{AsciiVisualizer, Visualizer}};

pub const FPS: u32 = 120;
pub const SLEEP_TIME: f32 = 1.0/FPS as f32;
pub const SPEED_FACTOR: f32 = 0.8;
pub const MIN_LOCK_DELAY: f32 = 0.4;
pub struct GameManager;

impl GameManager {

    pub fn start() {

        let mut fall_time = 1.0 / 4.0;
        let mut lock_delay = 1.5 * fall_time;
        let seed = rand::thread_rng().gen::<u64>();
        let stdout = Term::buffered_stdout();
        let rng = ChaCha8Rng::seed_from_u64(seed);
        let mut level = 1;
        let mut hold_lines_cleared = 0;
        let mut board = GameBoard::new(rng, level);

        let (to_main, from_thread) = mpsc::channel::<Action>();
        let keyboard_listener = thread::spawn(move || {
            loop {
                if let Ok(character) = stdout.read_char() {
                    let _ = match character {
                        ' ' => to_main.send(Action::Move(MovementDirection::Top)),
                        'q' => to_main.send(Action::Move(MovementDirection::Left)),
                        's' => to_main.send(Action::Move(MovementDirection::Bottom)),
                        'd' => to_main.send(Action::Move(MovementDirection::Right)),
                        'z' => to_main.send(Action::Rotate),
                        '\n' => to_main.send(Action::Hold),
                        _ => Ok(())
                    };
                }
            }
        });
        

        AsciiVisualizer::display(&board);
        
        let mut update = true;
        let mut last_fall = 0.0;
        let mut lock_timer = 0.0;
        let mut start = Instant::now();
        while board.keep_playing() {

            let _ = match from_thread.try_recv() {
                Ok(Action::Move(movement)) => {
                    let _ = match board.try_move(movement) {
                        Ok(_) => {
                            match movement {
                                MovementDirection::Top => {
                                    lock_timer = lock_delay;
                                    last_fall = fall_time;
                                }
                                _ => lock_timer = 0.0
                            }
                        }
                        Err(_) => ()
                    };
                    update = true;
                },
                Ok(Action::Rotate) => {
                    let _ = match board.try_rotate(){
                        Ok(_) => lock_timer = 0.0,
                        Err(_) => ()
                    };
                    update = true;
                },
                Ok(Action::Hold) => {
                    let _ = match board.try_swap(){
                        Ok(_) => lock_timer = 0.0,
                        Err(_) => ()
                    };
                    update = true;
                },
                Err(_) => ()
            };
            
            let time_delta = start.elapsed().as_secs_f32();
            last_fall += time_delta;
            lock_timer += time_delta;
            start = Instant::now();
            if last_fall >= fall_time {
                match board.try_fall() {
                    Ok(_) => {
                        lock_timer = 0.0;
                        let cleared = board.get_lines_cleared();
                        if cleared % 2 == 0 &&
                        hold_lines_cleared != cleared {
                            level += 1;
                            board.set_level(level);
                            fall_time *= SPEED_FACTOR;
                            lock_delay *= SPEED_FACTOR + (1 as f32 - SPEED_FACTOR) / 2.0;
                            lock_delay = f32::min(lock_delay, MIN_LOCK_DELAY);
                            hold_lines_cleared = cleared;   
                        }
                    },
                    Err(_) => {
                        if lock_timer >= lock_delay {
                            board.lock_current_piece();
                        }
                    }
                }
                last_fall = 0.0;
                update = true;
            }
            
            if update {
                AsciiVisualizer::display(&board);
                update = false;
            }
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


