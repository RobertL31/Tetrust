
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::fmt::{Debug, Write};

use kiss3d::nalgebra::Vector2;

use rand_chacha::ChaCha8Rng;


use crate::{piece::{Square, Piece}, piece_provider::PieceProvider, piece_factory::{PieceType, SPAWN_POINT}};

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 22;

const PLAY_POINT: Vector2<isize> = Vector2::new(4, 20);

const SOFT_DROP_SCORE: u32 = 1;
const HARD_DROP_SCORE: u32 = 2;

const PIECE_QUEUE_SIZE: usize = 5;

const SINGLE_LINE_POINTS: u32 = 100;
const DOUBLE_LINE_POINTS: u32 = 300;
const TRIPLE_LINE_POINTS: u32 = 500;
const TETRIS_POINTS: u32 = 800;

pub struct GameBoard {
    square_board: [[Option<Square>; BOARD_HEIGHT]; BOARD_WIDTH],
    space_board: [[bool; BOARD_HEIGHT]; BOARD_WIDTH],
    piece_provider: PieceProvider,
    current_piece: Piece,
    held_piece: Option<Piece>,
    can_swap: bool,
    next_pieces: VecDeque<Piece>,
    score: u32,
    level: u32,
    lines_cleared: u32,
    game_over: bool
}

#[derive(Copy, Clone)]
pub enum Direction {
    Right,
    Top,
    Left,
    Bottom
}


enum Rotation {
    None, 
    Regular, 
    Right,
    Left
}

unsafe impl Send for Direction {}
unsafe impl Sync for Direction {}


pub enum Action {
    Move(Direction),
    Rotate,
    Hold
}

unsafe impl Send for Action {}
unsafe impl Sync for Action {}

pub struct FallError;
pub struct RotateError;
pub struct MoveError;
pub struct SwapError;


impl GameBoard {

    pub fn new(rng: ChaCha8Rng, level: u32) -> Self {
        let mut piece_provider = PieceProvider::new(rng);
        let mut current_piece = piece_provider.get_piece();
        current_piece.move_at(PLAY_POINT);
        let mut next_pieces: VecDeque<Piece> = VecDeque::new();
        for _ in 0..PIECE_QUEUE_SIZE {
            next_pieces.push_back(piece_provider.get_piece());
        }

        GameBoard {
            square_board: [[None; BOARD_HEIGHT]; BOARD_WIDTH],
            space_board: [[false; BOARD_HEIGHT]; BOARD_WIDTH],
            piece_provider,
            current_piece,
            held_piece: None,
            can_swap: true,
            next_pieces,
            score: 0,
            level,
            lines_cleared: 0,
            game_over: false
        }
    }


    pub fn get_current_piece(&self) -> &Piece {
        &self.current_piece
    }


    pub fn get_square_board(&self) -> [[Option<Square>; BOARD_HEIGHT]; BOARD_WIDTH] {
        self.square_board.clone()
    }


    pub fn get_score(&self) -> u32 {
        self.score
    }


    pub fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared
    }


    pub fn get_level(&self) -> u32 {
        self.level
    }


    pub fn get_held_piece(&self) -> &Option<Piece> {
        &self.held_piece
    }


    pub fn get_next_piece(&self) -> &Piece {
        &self.next_pieces.front().unwrap()
    }


    pub fn set_level(&mut self, value: u32) {
        self.level = value;
    }


    pub fn is_free(&self, position: Vector2<isize>) -> bool {

        if position.x < 0 || position.x >= BOARD_WIDTH as isize {
            return false;
        }

        if position.y < 0 || position.y >= BOARD_HEIGHT as isize {
            return false;
        }

        !self.space_board[position.x as usize][position.y as usize]
    }


    fn can_spawn(&self) -> bool {
        self.current_piece.get_squares().iter().all(|square| {
            self.is_free(square.get_position())
        })
    }


    fn can_fall(&self) -> bool{
        self.current_piece.get_squares().iter().all(|square|{
            let position = square.get_position();
            self.is_free(Vector2::new(position[0], position[1]-1))
        })
    }

    //TODO: implement proper wall-kick logic
    fn can_rotate(&self) -> Rotation {
        if self.current_piece.get_piece_type() == PieceType::Square {
            return Rotation::None;
        }

        //TODO: Move rotated position from square to piece (no sens to rotate a square alone)
        let rotated_positions = self.current_piece.get_squares().iter().map(|square| {
            let rotation_type = self.current_piece.get_rotation_type();
            let pivot = self.current_piece.get_squares()[0].get_position();
            square.get_rotated_position(rotation_type, pivot)
        }).collect::<Vec<_>>();

        let can_rotate = rotated_positions.iter().all(|position| {
            self.is_free(*position)
        });

        if can_rotate {
            return Rotation::Regular;
        }

        let can_rotate_right = rotated_positions.iter().all(|position| {
            self.is_free(*position + Vector2::new(1,0))
        });

        if can_rotate_right {
            return Rotation::Right;
        }

        let can_rotate_right = rotated_positions.iter().all(|position| {
            self.is_free(*position + Vector2::new(-1,0))
        });

        if can_rotate_right {
            return Rotation::Left;
        }
       
        Rotation::None
    }


    fn can_move(&self, direction: Direction) -> bool {
        match direction {
            Direction::Left => 
                self.current_piece.get_squares().iter().all(|square| {
                    let position = square.get_position();
                    self.is_free(Vector2::new(position[0]-1, position[1]))
                }),
            Direction::Right => 
                self.current_piece.get_squares().iter().all(|square| {
                    let position = square.get_position();
                    self.is_free(Vector2::new(position[0]+1, position[1]))
                }),
            Direction::Bottom => self.can_fall(),
            Direction::Top => self.can_fall()
        }
    }


    pub fn keep_playing(&self) -> bool {
        !self.game_over
    }


    fn initialize_piece_position(&mut self) {
        self.current_piece.translate(PLAY_POINT - SPAWN_POINT)
    }


    fn draw(&mut self){
        self.current_piece = self.next_pieces.pop_front().unwrap();
        self.next_pieces.push_back(self.piece_provider.get_piece());

        self.initialize_piece_position();
        self.game_over = !self.can_spawn();
    }


    fn get_first_free_line_from(&mut self, index: usize) -> usize {
        let mut result = BOARD_HEIGHT-1;
        for i in index..BOARD_HEIGHT {
            if self.space_board.iter().all(|column| !column[i]) {
                result = i;
                break;
            }
        }
        result
    }


    fn clear_line(&mut self, destroy_index: usize){
        let limit = self.get_first_free_line_from(destroy_index);
        for i in 0..BOARD_WIDTH {
            for line_index in destroy_index..limit {
                self.space_board[i][line_index] = self.space_board[i][line_index+1];
                self.square_board[i][line_index] = self.square_board[i][line_index+1];
            }
        }

        self.lines_cleared += 1;
    }


    // Usage of BTreeSet allows the collection to be sorted for destruction
    //TODO: Optimize to destroy multiple lines at once
    fn check_complete_line(&mut self, line_indexes: BTreeSet<usize>) {
        let mut cleared = 0;
        for line_index in line_indexes.iter().rev() {
            if self.space_board.iter().all(|column| column[*line_index]) {
                self.clear_line(*line_index);
                cleared += 1;
            }
        }

        

        self.score += match cleared {
            1 => SINGLE_LINE_POINTS * self.level,
            2 => DOUBLE_LINE_POINTS * self.level,
            3 => TRIPLE_LINE_POINTS * self.level,
            4 => TETRIS_POINTS * self.level,
            _ => 0
        }
    }


    pub fn lock_current_piece(&mut self) {
        let mut modified_lines: BTreeSet<usize> = BTreeSet::new();
        for square in self.current_piece.get_squares_owned() {
            let position = square.get_position();
            self.space_board[position.x as usize][position.y as usize] = true;
            self.square_board[position.x as usize][position.y as usize] = Some(square);
            modified_lines.insert(position.y as usize);
        }

        self.check_complete_line(modified_lines);
        self.draw();
        self.can_swap = true;
    }


    fn swap_held_piece(&mut self) {
        self.current_piece.to_initial_rotation();
        match self.held_piece {
            Some(_) => {
                self.current_piece.move_at(PLAY_POINT);
                std::mem::swap(&mut self.current_piece, self.held_piece.as_mut().unwrap())
            }
            None => {
                //TODO: properly code that crap
                // Just help_piece = some(piece) and gg ?
                self.current_piece.move_at(PLAY_POINT);
                let dummy_piece = Piece::from(PieceType::Square);
                self.held_piece = Some(dummy_piece);
                std::mem::swap(&mut self.current_piece, self.held_piece.as_mut().unwrap());
                self.draw();
            }
        }
    }


    pub fn try_swap(&mut self) -> Result<(), SwapError> {
        self.can_swap.then(|| {
            self.swap_held_piece();
            self.can_swap = false;
        }).ok_or(SwapError)
    }


    pub fn try_fall(&mut self) -> Result<(), FallError> {
        self.can_fall().then(|| {
            self.current_piece.translate(Vector2::new(0,-1));
        }).ok_or(FallError)
    }

    //TODO: properly implement wall-kick
    pub fn try_rotate(&mut self) -> Result<(), RotateError> {
        let _ = match self.can_rotate() {
            Rotation::Regular => self.current_piece.rotate(),
            Rotation::Right => {
                self.current_piece.translate(Vector2::new(1,0));
                self.current_piece.rotate();
            },
            Rotation::Left => {
                self.current_piece.translate(Vector2::new(-1,0));
                self.current_piece.rotate();
            },
            Rotation::None => return Err(RotateError)
        };
        Ok(())
    }


    fn move_at(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.current_piece.translate(Vector2::new(-1, 0)),
            Direction::Right => self.current_piece.translate(Vector2::new(1, 0)),
            Direction::Bottom => {
                self.current_piece.translate(Vector2::new(0, -1));
                self.score += SOFT_DROP_SCORE;
            },
            Direction::Top => {
                while let Ok(_) = self.try_fall() {
                    self.score += HARD_DROP_SCORE;
                }
            }
        }
    }


    pub fn try_move(&mut self, direction: Direction) -> Result<(), MoveError> {
        self.can_move(direction).then(|| {
            self.move_at(direction);
        }).ok_or(MoveError)
    }

}


impl Debug for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..BOARD_HEIGHT).rev() {
            for j in 0..BOARD_WIDTH {
                let value = self.square_board[j][i];
                let string = match value {
                    Some(square) => format!("{:?}", square).chars().next().unwrap(),
                    None => 'X'
                };
                f.write_str(&format!("{} ", string))?;
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')?;
        Ok(())
    }
}



#[cfg(test)]
mod test {
    
    
}