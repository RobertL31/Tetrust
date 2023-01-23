use kiss3d::nalgebra::Vector2;

use strum_macros::EnumIter;

use crate::piece::{Piece, Square, Color, RotationType};

const SPAWN_POINT: Vector2<isize> = Vector2::new(100, 100);

#[derive(EnumIter, Copy, Clone)]
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
            T => Self::get_T_piece(),
            LeftL => Self::get_LeftL_piece(),
            RightL => Self::get_RightL_piece(),
            LeftSkew => Self::get_LeftSkew_piece(),
            RightSkew => Self::get_RightSkew_piece(),
            Square => Self::get_Square_piece(),
            Straight => Self::get_Straight_piece()
        }
    }

    fn get_T_piece() -> Piece {
        let squares = [
            Square::new(
                Vector2::new(SPAWN_POINT.x - 1, SPAWN_POINT.y),
                Color::Purple
            ),
            Square::new(
                SPAWN_POINT,
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
            RotationType::AroundSquare(squares[1]),
            PieceType::T
        )
    }

    fn get_LeftL_piece() -> Piece {
        let squares = [
            Square::new(
                Vector2::new(SPAWN_POINT.x - 1, SPAWN_POINT.y),
                Color::DarkBlue
            ),
            Square::new(
                SPAWN_POINT,
                Color::DarkBlue
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(1, 0),
                Color::DarkBlue
            ),
            Square::new(
                Vector2::new(SPAWN_POINT.x - 1, SPAWN_POINT.y - 1),
                Color::DarkBlue
            )
        ];

        Piece::new(
            squares,
            RotationType::AroundSquare(squares[1]),
            PieceType::LeftL
        )
    }

    fn get_RightL_piece() -> Piece {
        let squares = [
            Square::new(
                Vector2::new(SPAWN_POINT.x - 1, SPAWN_POINT.y),
                Color::Orange
            ),
            Square::new(
                SPAWN_POINT,
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
            RotationType::AroundSquare(squares[1]),
            PieceType::RightL
        )
    }

    fn get_LeftSkew_piece() -> Piece {
        let squares = [
            Square::new(
                Vector2::new(SPAWN_POINT.x - 1, SPAWN_POINT.y),
                Color::Green
            ),
            Square::new(
                SPAWN_POINT,
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
            RotationType::AroundSquare(squares[1]),
            PieceType::LeftSkew
        )
    }

    fn get_RightSkew_piece() -> Piece {
        let squares = [
            Square::new(
                SPAWN_POINT,
                Color::Red
            ),
            Square::new(
                SPAWN_POINT + Vector2::new(0, 1),
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
            RotationType::AroundSquare(squares[1]),
            PieceType::RightSkew
        )
    }

    fn get_Square_piece() -> Piece {
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
            RotationType::AroundPoint(squares[2]),
            PieceType::Square
        )
    }

    fn get_Straight_piece() -> Piece {
        let squares = [
            Square::new(
                Vector2::new(SPAWN_POINT.x - 1, SPAWN_POINT.y),
                Color::Cyan
            ),
            Square::new(
                SPAWN_POINT,
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
            RotationType::AroundPoint(squares[1]),
            PieceType::Straight
        )
    }

}