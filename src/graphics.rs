use colored::{Colorize, ColoredString};

use crate::{gameboard::{GameBoard, BOARD_HEIGHT, BOARD_WIDTH}, piece::{Square, Color}};

pub trait Visualizer {
    fn display(board: &GameBoard);
}

pub struct AsciiVisualizer {
    text: String
}

impl AsciiVisualizer {

    pub fn colored_ascii_of(square: Square) -> ColoredString {
        match square.get_color() {
            Color::Cyan => "[]".cyan(),
            Color::DarkBlue => "[]".blue(),
            Color::Orange => "[]".truecolor(255, 128, 0),
            Color::Yellow => "[]".yellow(),
            Color::Green => "[]".green(),
            Color::Purple => "[]".purple(),
            Color::Red => "[]".red(),
        }
    }
}


impl Visualizer for AsciiVisualizer {

    fn display(board: &GameBoard){
        print!("\x1B[2J\x1B[1;1H");
        let mut square_board = board.get_square_board().clone();
        for square in board.get_current_piece().get_squares() {
            let position = square.get_position();
            square_board[position.x as usize][position.y as usize] = Some(square.clone());
        }

        let mut building_text = String::new();

        for i in (0..BOARD_HEIGHT).rev() {
            for j in 0..BOARD_WIDTH {
                let value = square_board[j][i];
                let string = match value {
                    Some(square) => AsciiVisualizer::colored_ascii_of(square),
                    None => String::from(" .").white()
                };
                building_text.push_str(&string.to_string())
            }
            building_text.push_str("\n")
        }

        println!("{}", building_text);
    }
}