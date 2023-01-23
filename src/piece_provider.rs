use rand_chacha::ChaCha8Rng;
use strum::IntoEnumIterator;
use rand::prelude::SliceRandom;

use crate::{piece::Piece, piece_factory::PieceType};

struct PieceBag {
    pieces: Vec<Piece>,
}

impl PieceBag {

    fn new(rng: &mut ChaCha8Rng) -> Self {
        let mut pieces = PieceType::iter().collect::<Vec<_>>();
        pieces.shuffle(rng);
        PieceBag {
            pieces: pieces.iter().map(|piece_type| Piece::from(*piece_type)).collect::<Vec<_>>()
        }
    }

    fn get_piece(&mut self) -> Piece {
        self.pieces.pop().unwrap()
    }

    fn get_bag_size(&self) -> usize {
        self.pieces.len()
    }
}

pub struct PieceProvider {
    current_bag: PieceBag,
    rng: ChaCha8Rng
}

impl PieceProvider {

    pub fn new(mut rng: ChaCha8Rng) -> Self {
        PieceProvider {
            current_bag: PieceBag::new(&mut rng),
            rng
        }
    }

    pub fn get_piece(&mut self) -> Piece {
        if self.current_bag.get_bag_size() == 0 {
            self.current_bag = PieceBag::new(&mut self.rng);
        }
        
        self.current_bag.get_piece()
    }
}