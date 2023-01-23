use std::{collections::VecDeque};

use kiss3d::nalgebra::Vector2;

use rand_chacha::ChaCha8Rng;


use crate::{piece::{Square, Piece}, piece_provider::PieceProvider, piece_factory::{PieceFactory, PieceType}, traits::{Move, FallError}};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

const PIECE_QUEUE_SIZE: usize = 5;

pub struct GameBoard {
    square_board: [[Option<Square>; BOARD_HEIGHT]; BOARD_WIDTH],
    space_board: [[bool; BOARD_HEIGHT]; BOARD_WIDTH],
    piece_provider: PieceProvider,
    current_piece: Piece,
    held_piece: Option<Piece>,
    can_swap: bool,
    next_pieces: VecDeque<Piece>,
}

impl GameBoard {

    pub fn new(rng: ChaCha8Rng) -> Self {
        let mut piece_provider = PieceProvider::new(rng);
        let current_piece = piece_provider.get_piece();
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
            next_pieces
        }
    }

    pub fn get_current_piece(&self) -> &Piece {
        &self.current_piece
    }

    pub fn is_free(&self, position: Vector2<isize>) -> bool {

        if position.x < 0 || position.x >= BOARD_WIDTH as isize {
            return false;
        }

        if position.y < 0 || position.y >= BOARD_HEIGHT as isize {
            return false;
        }

        self.space_board[position.x as usize][position.y as usize]
    }

    pub fn can_spawn(&self) -> bool {
        self.current_piece.get_squares().iter().all(|square| {
            self.is_free(*square.get_position())
        })
    }

    pub fn can_fall(&self, piece: &Piece) -> bool{
        piece.get_squares().iter().all(|square|{
            let position = square.get_position();
            self.is_free(Vector2::new(position[0], position[1]-1))
        })
    }

    pub fn can_rotate(&self, piece: &Piece) -> bool {
        piece.get_squares().iter().all(|square| {
            self.is_free(square.get_rotated_position(piece.get_rotation_type()))
        })
    }

    fn draw(&mut self){
        self.current_piece = self.next_pieces.pop_front().unwrap();
        self.next_pieces.push_back(self.piece_provider.get_piece());
    }

    pub fn lock_current_piece(&mut self) {
        for square in self.current_piece.get_squares_owned() {
            let position = square.get_position();
            self.space_board[position.x as usize][position.y as usize] = true;
            self.square_board[position.x as usize][position.y as usize] = Some(square);
        }

        self.draw();
    }

    pub fn swap_held_piece(&mut self) {
        match self.held_piece {
            Some(_) => {
                std::mem::swap(&mut self.current_piece, self.held_piece.as_mut().unwrap())
            }
            None => {
                // Weird but only way found for the moment
                let dummy_piece = Piece::from(PieceType::Square);
                self.held_piece = Some(dummy_piece);
                std::mem::swap(&mut self.current_piece, self.held_piece.as_mut().unwrap());
                self.draw();
            }
        }
    }

    pub fn step(&mut self) {
        match self.current_piece.try_fall(&self) {
            Ok(_) => (),
            Err(_) => self.lock_current_piece()
        }
    }
}


#[cfg(test)]
mod test {
    
    #[test]
    fn hello() {
        assert!(true)
    }
}