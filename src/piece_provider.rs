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

    fn get_piece(&mut self) -> Option<Piece> {
        self.pieces.pop()
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
        
        self.current_bag.get_piece().unwrap()
    }
}


mod test {

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use crate::piece_factory::PieceType;

    use super::{PieceProvider, PieceBag};
    
    #[test]
    fn ten_first_pieces_are_the_same_for_same_seed(){
        let rng1 = ChaCha8Rng::seed_from_u64(1);
        let rng2 = rng1.clone();
        let mut provider1 = PieceProvider::new(rng1);
        let mut provider2 = PieceProvider::new(rng2);
        let mut result = true;

        for _ in 0..10 {
            let piece1 = provider1.get_piece();
            let piece2 = provider2.get_piece();
            result &= piece1 == piece2;
        }

        assert!(result);
    }


    use std::hash::Hash;
    use std::collections::HashSet;

    fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
    {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }

    #[test]
    fn every_piece_in_a_bag_is_different() {
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        let mut bag = PieceBag::new(&mut rng);
        let mut pieces = vec![];
        while let Some(piece) = bag.get_piece(){
            pieces.push(piece);
        }

        let result = has_unique_elements(pieces);

        assert!(result);
    }
}