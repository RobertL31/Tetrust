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


fn rotate_90_cw_around(point: &Vector2<isize>, pivot: &Vector2<isize>) -> Vector2<isize> {
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


    pub fn get_position(&self) -> &Vector2<isize>{
        &self.position
    }


    pub fn get_position_mut(&mut self) -> &mut Vector2<isize> {
        &mut self.position
    }


    pub fn set_position(&mut self, value: Vector2<isize>) {
        self.position = value;
    }


    pub fn get_color(&self) -> &Color{
        &self.color
    }


    pub fn get_rotated_position(&self, rotation_type: &RotationType) -> Vector2<isize> {
        match rotation_type {
            RotationType::AroundSquare(pivot) => 
                rotate_90_cw_around(&self.position, &pivot.position),
            RotationType::AroundPoint(pivot) => 
                rotate_90_cw_around(&self.position, &pivot.position) + Vector2::new(-1,0)
        }
    }


    pub fn translate(&mut self, translation: Vector2<isize>) {
        self.position = self.position + translation;
    }
}


impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.color))
    }
}

#[derive(Copy, Clone)]
pub enum RotationType {
    AroundSquare(Square),
    AroundPoint(Square)
}


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


    pub fn get_rotation_type(&self) -> RotationType{
        self.rotation
    }


    pub fn get_type(&self) -> PieceType {
        self.piece_type
    }


    pub fn translate(&mut self, translation: Vector2<isize>) {
        for square in self.squares.iter_mut() {
            square.translate(translation);
        }
    }

    pub fn has_same_type(&self, other: &Piece) -> bool {
        self.piece_type == other.piece_type
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

    use super::{Square, Color, RotationType};

    
    #[test]
    fn rotating_90_around_point_works(){
        let sut = Square::new(
            Vector2::new(4, 4),
            Color::Cyan
        );
        let pivot = RotationType::AroundPoint(Square::new(
            Vector2::new(2, 2),
            Color::Cyan
        ));

        let result = sut.get_rotated_position(&pivot);

        assert_eq!(result, Vector2::new(3, 0))
    }


    #[test]
    fn rotating_360_around_point_is_identity(){
        let sut = Square::new(
            Vector2::new(4321, 1234),
            Color::Cyan
        );
        let dummy_pivot = RotationType::AroundPoint(Square::new(
            Vector2::new(123, 321),
            Color::Cyan
        ));

        let mut result = sut.get_rotated_position(&dummy_pivot);
        for _ in 0..3 {
            result = Square::new(
                result,
                Color::Cyan
            ).get_rotated_position(&dummy_pivot);
        }

        assert_eq!(result, *sut.get_position())
    }


    #[test]
    fn rotating_90_around_square_works(){
        let sut = Square::new(
            Vector2::new(4, 4),
            Color::Cyan
        );
        let pivot = RotationType::AroundSquare(Square::new(
            Vector2::new(2, 2),
            Color::Cyan
        ));

        let result = sut.get_rotated_position(&pivot);

        assert_eq!(result, Vector2::new(4, 0))
    }


    #[test]
    fn rotating_360_around_square_is_identity(){
        let sut = Square::new(
            Vector2::new(4321, 1234),
            Color::Cyan
        );
        let dummy_pivot = RotationType::AroundSquare(Square::new(
            Vector2::new(123, 321),
            Color::Cyan
        ));

        let mut result = sut.get_rotated_position(&dummy_pivot);
        for _ in 0..3 {
            result = Square::new(
                result,
                Color::Cyan
            ).get_rotated_position(&dummy_pivot);
        }

        assert_eq!(result, *sut.get_position())
    }

}