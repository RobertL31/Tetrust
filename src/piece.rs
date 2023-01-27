extern crate kiss3d;

use std::{hash::Hasher, hash::Hash};
use std::fmt::Debug;

use crate::{piece_factory::{PieceFactory, PieceType}};

use kiss3d::nalgebra::{Vector2};

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Cyan, 
    DarkBlue,
    Orange, 
    Yellow,
    Green,
    Purple, 
    Red
}


fn rotate_90_cw_around(point: &Vector2<isize>, pivot: Vector2<isize>) -> Vector2<isize> {
    // 90 degrees is a particluar case where sin = 1 and cos = 0
    Vector2::new(
        point.y - pivot.y + pivot.x,
        - (point.x - pivot.x) + pivot.y
    )
}

#[derive(Copy, Clone)]
pub struct Square {
    position: Vector2<isize>,
    color: Color
}

impl Square {

    pub fn new(position: Vector2<isize>, color: Color) -> Self {
        Square {
            position,
            color
        }
    }


    pub fn get_position(&self) -> Vector2<isize>{
        self.position
    }


    pub fn get_position_mut(&mut self) -> &mut Vector2<isize> {
        &mut self.position
    }


    pub fn set_position(&mut self, value: Vector2<isize>) {
        self.position = value;
    }


    pub fn get_color(&self) -> Color{
        self.color
    }


    pub fn get_rotated_position(&self, rotation_type: RotationType, pivot: Vector2<isize>) -> Vector2<isize> {
        match rotation_type {
            RotationType::AroundSquare => 
                rotate_90_cw_around(&self.position, pivot),
            RotationType::AroundPoint(point) => 
                rotate_90_cw_around(&self.position, point) + Vector2::new(1,0)
        }
    }


    pub fn translate(&mut self, translation: Vector2<isize>) {
        self.position = self.position + translation;
    }


    pub fn move_at(&mut self, position: Vector2<isize>) {
        self.position = position;
    }
}


impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.color))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RotationType {
    AroundSquare,
    AroundPoint(Vector2<isize>)
}

#[derive(Clone)]
pub struct Piece {
    squares: [Square; 4],
    rotation: RotationType,
    piece_type: PieceType
}

impl Piece {

    pub fn new(squares: [Square; 4], rotation: RotationType, piece_type: PieceType) -> Self {
        Piece {
            squares,
            rotation,
            piece_type
        }
    }


    pub fn get_squares(&self) -> &[Square; 4]{
        &self.squares
    }


    pub fn get_squares_mut(&mut self) -> &mut[Square; 4] {
        &mut self.squares
    }


    pub fn get_squares_owned(&self) -> [Square; 4]{
        self.squares
    }


    pub fn get_rotation_type(&self) -> RotationType {
        self.rotation
    }


    pub fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }


    pub fn translate(&mut self, translation: Vector2<isize>) {
        for square in self.squares.iter_mut() {
            square.translate(translation);
        }
        if let RotationType::AroundPoint(ref mut point) = self.rotation {
            *point += translation;
        }
    }


    pub fn move_at(&mut self, position: Vector2<isize>) {
        self.translate(position - self.squares[0].get_position())
    }


    pub fn rotate(&mut self) {
        let pivot = self.squares[0].get_position();
        for square in self.squares.iter_mut() {
            square.set_position(square.get_rotated_position(self.rotation, pivot))
        }
    }


    pub fn has_same_type(&self, other: &Piece) -> bool {
        self.piece_type == other.piece_type
    }


    pub fn to_array(mut self) -> [[bool; 4]; 2] {
        let mut piece_holder = [[false; 4]; 2];
        self.move_at(Vector2::new(1,0));
        for square in self.squares {
            let position = square.get_position();
            piece_holder[position.y as usize][position.x as usize] = true;
        }
        piece_holder
    }

}


impl Hash for Piece {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.piece_type.hash(state);
    }
}


impl From<PieceType> for Piece {
    fn from(piece_type: PieceType) -> Self {
        PieceFactory::get_piece(piece_type)
    }
}


impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.piece_type == other.piece_type
    }
}


impl Eq for Piece {}


#[cfg(test)]
mod test {
    use kiss3d::nalgebra::Vector2;

    use crate::piece_factory::PieceType;

    use super::{Square, Color, RotationType, Piece};
    
    #[test]
    fn rotation_of_square_around_itself_is_identity() {
        let sut = Square::new(
            Vector2::new(5, 5),
            Color::Cyan
        );
        let rotation_type = RotationType::AroundSquare;
        let pivot = Vector2::new(5, 5);

        let result = sut.get_rotated_position(rotation_type, pivot);

        assert_eq!(result, Vector2::new(5, 5));
    }

    #[test]
    fn rotation_of_square_90_around_point_is_correct(){
        let sut = Square::new(
            Vector2::new(4, 4),
            Color::Cyan
        );
        let rotation_type = RotationType::AroundPoint(Vector2::new(2, 2));
        let pivot = Vector2::new(2, 2);

        let result = sut.get_rotated_position(rotation_type, pivot);

        assert_eq!(result, Vector2::new(3, 0));
    }


    #[test]
    fn rotation_of_square_360_around_point_is_identity(){
        let sut = Square::new(
            Vector2::new(4321, 1234),
            Color::Cyan
        );
        let rotation_type = RotationType::AroundPoint(Vector2::new(2, 2));
        let dummy_pivot = Vector2::new(123, 321);

        let mut result = sut.get_rotated_position(rotation_type, dummy_pivot);
        for _ in 0..3 {
            result = Square::new(
                result,
                Color::Cyan
            ).get_rotated_position(rotation_type, dummy_pivot);
        }

        assert_eq!(result, sut.get_position());
    }


    #[test]
    fn rotation_of_square_90_around_square_is_correct(){
        let sut = Square::new(
            Vector2::new(4, 4),
            Color::Cyan
        );
        let rotation_type = RotationType::AroundSquare;
        let pivot = Vector2::new(2, 2);

        let result = sut.get_rotated_position(rotation_type, pivot);

        assert_eq!(result, Vector2::new(4, 0));
    }


    #[test]
    fn rotation_of_360_around_square_is_identity(){
        let sut = Square::new(
            Vector2::new(4321, 1234),
            Color::Cyan
        );
        let rotation_type = RotationType::AroundSquare;
        let dummy_pivot = Vector2::new(123, 321);

        let mut result = sut.get_rotated_position(rotation_type, dummy_pivot);
        for _ in 0..3 {
            result = Square::new(
                result,
                Color::Cyan
            ).get_rotated_position(rotation_type, dummy_pivot);
        }

        assert_eq!(result, sut.get_position());
    }

    #[test]
    fn rotating_a_piece_around_a_square_works() {
        let mut sut = Piece::from(PieceType::T);
        sut.move_at(Vector2::new(5,5));

        println!("{:?}", sut.get_squares().iter().map(|s| s.get_position()).collect::<Vec<_>>());
        match sut.rotation {
            RotationType::AroundSquare => println!("{}", sut.get_squares()[0].get_position()),
            RotationType::AroundPoint(point) => println!("{}", point)
        }
        
        let rotation_type = sut.get_rotation_type();
        let pivot = sut.get_squares()[0].get_position(); 
        let new_positions = sut.get_squares_mut()
            .iter()
            .map(|square| square.get_rotated_position(rotation_type, pivot))
            .collect::<Vec<_>>();

        assert_eq!(new_positions[0], Vector2::new(5, 5));
        assert_eq!(new_positions[1], Vector2::new(5, 6));
        assert_eq!(new_positions[2], Vector2::new(5, 4));
        assert_eq!(new_positions[3], Vector2::new(6, 5));
    }


    #[test]
    fn rotating_around_square_still_correct_after_piece_falling_twice() {
        let mut sut = Piece::from(PieceType::T);
        sut.move_at(Vector2::new(5,5));
        sut.translate(Vector2::new(0,-1));
        sut.translate(Vector2::new(0,-1));

        let rotation_type = sut.get_rotation_type();
        let pivot = sut.get_squares()[0].get_position(); 
        let new_positions = sut.get_squares_mut()
            .iter()
            .map(|square| square.get_rotated_position(rotation_type, pivot))
            .collect::<Vec<_>>();

        assert_eq!(new_positions[0], Vector2::new(5, 3));
        assert_eq!(new_positions[1], Vector2::new(5, 4));
        assert_eq!(new_positions[2], Vector2::new(5, 2));
        assert_eq!(new_positions[3], Vector2::new(6, 3));
    }
}