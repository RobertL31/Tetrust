
use crate::gameboard::GameBoard;

pub struct FallError;
pub struct RotateError;

pub trait Move {
    fn try_fall(&mut self, board: &GameBoard) -> Result<(), FallError>;
    fn try_rotate(&mut self, board: &GameBoard) -> Result<(), RotateError>;
}