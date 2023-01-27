use kiss3d::nalgebra::Vector2;

use strum_macros::EnumIter;

use crate::piece::{Piece, Square, Color, RotationType};

pub const SPAWN_POINT: Vector2<isize> = Vector2::new(100, 100);

#[derive(EnumIter, Copy, Clone, PartialEq, Hash, Debug)]
pub enum PieceType {
    T,
    LeftL,
    RightL,
    LeftSkew,
    RightSkew,
    Square,
    Straight
}

pub struct PieceFactory;

impl PieceFactory {

    pub fn get_piece(piece_type: PieceType) -> Piece {
        match piece_type {
            PieceType::T => Self::get_t_piece(),
            PieceType::LeftL => Self::get_left_l_piece(),
            PieceType::RightL => Self::get_right_l_piece(),
            PieceType::LeftSkew => Self::get_left_skew_piece(),
            PieceType::RightSkew => Self::get_right_skew_piece(),
            PieceType::Square => Self::get_square_piece(),
            PieceType::Straight => Self::get_straight_piece()
        }
    }

    fn get_t_piece() -> Piece {
        let squares = [
            Square::new(
                SPAWN_POINT,
                Color::Purple
            ),
            Square::new(
                Vector2::new(SPAWN_POINT.x - 1, SPAWN_POINT.y),
                Color::Purple
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(1, 0),
                Color::Purple
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(0, 1),
                Color::Purple
            )
        ];

        Piece::new(
            squares,
            RotationType::AroundSquare,
            PieceType::T
        )
    }

    fn get_left_l_piece() -> Piece {
        let squares = [
            Square::new(
                SPAWN_POINT,
                Color::DarkBlue
            ),
            Square::new(
                Vector2::new(SPAWN_POINT.x - 1, SPAWN_POINT.y),
                Color::DarkBlue
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(1, 0),
                Color::DarkBlue
            ),
            Square::new(
                Vector2::new(SPAWN_POINT.x - 1, SPAWN_POINT.y + 1),
                Color::DarkBlue
            )
        ];

        Piece::new(
            squares,
            RotationType::AroundSquare,
            PieceType::LeftL
        )
    }

    fn get_right_l_piece() -> Piece {
        let squares = [
            Square::new(
                SPAWN_POINT,
                Color::Orange
            ),
            Square::new(
                Vector2::new(SPAWN_POINT.x - 1, SPAWN_POINT.y),
                Color::Orange
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(1, 0),
                Color::Orange
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(1, 1),
                Color::Orange
            )
        ];

        Piece::new(
            squares,
            RotationType::AroundSquare,
            PieceType::RightL
        )
    }

    fn get_left_skew_piece() -> Piece {
        let squares = [
            Square::new(
                SPAWN_POINT,
                Color::Green
            ),
            Square::new(
                Vector2::new(SPAWN_POINT.x - 1, SPAWN_POINT.y),
                Color::Green
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(0, 1),
                Color::Green
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(1, 1),
                Color::Green
            )
        ];

        Piece::new(
            squares,
            RotationType::AroundSquare,
            PieceType::LeftSkew
        )
    }

    fn get_right_skew_piece() -> Piece {
        let squares = [
            Square::new(
                SPAWN_POINT,
                Color::Red
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(1, 0),
                Color::Red
            ),
            Square::new(
                Vector2::new(SPAWN_POINT.x - 1, SPAWN_POINT.y + 1),
                Color::Red
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(0, 1),
                Color::Red
            )
        ];

        Piece::new(
            squares,
            RotationType::AroundSquare,
            PieceType::RightSkew
        )
    }

    fn get_square_piece() -> Piece {
        let squares = [
            Square::new(
                SPAWN_POINT,
                Color::Yellow
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(1, 0),
                Color::Yellow
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(0, 1),
                Color::Yellow
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(1, 1),
                Color::Yellow
            )
        ];

        Piece::new(
            squares,
            RotationType::AroundPoint(SPAWN_POINT),
            PieceType::Square
        )
    }

    fn get_straight_piece() -> Piece {
        let squares = [
            Square::new(
                SPAWN_POINT,
                Color::Cyan
            ),
            Square::new(
                Vector2::new(SPAWN_POINT.x - 1, SPAWN_POINT.y),
                Color::Cyan
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(1, 0),
                Color::Cyan
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(2, 0),
                Color::Cyan
            )
        ];

        Piece::new(
            squares,
            RotationType::AroundPoint(SPAWN_POINT),
            PieceType::Straight
        )
    }

}