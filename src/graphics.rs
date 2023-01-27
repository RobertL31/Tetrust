use colored::{Colorize, ColoredString};

use crate::{gameboard::{GameBoard, BOARD_HEIGHT, BOARD_WIDTH}, piece::{Square, Color, Piece}};

const SQUARE_LEFT_STR: char = '\u{27E6}';
const SQUARE_RIGHT_STR: char = '\u{27E7}';

pub trait Visualizer {
    fn display(board: &GameBoard);
}

pub struct AsciiVisualizer;

impl AsciiVisualizer {

    pub fn colored_ascii_square_from(color: Color) -> ColoredString {
        
        let string = format!("{}{}", SQUARE_LEFT_STR, SQUARE_RIGHT_STR);
        match color {
            Color::Cyan => string.bright_blue().on_cyan(),
            Color::DarkBlue => string.blue().on_bright_blue(),
            Color::Orange => string.truecolor(255, 128, 0).on_bright_red(),
            Color::Yellow => string.yellow().on_bright_yellow(),
            Color::Green => string.green().on_bright_green(),
            Color::Purple => string.purple().on_bright_purple(),
            Color::Red => string.red().on_bright_red(),
        }
    }


    pub fn colored_ascii_of_piece(piece: Option<Piece>, cursor_position: usize) -> String {
        if let None = piece {
            return "None".white().to_string();
        }

        let piece = piece.unwrap();
        let color = piece.get_squares()[0].get_color();
        let array = piece.to_array();
        let mut building_string = String::new();
        for line in array.iter().rev() {
            for value in line {
                let value_string = match value {
                    true => AsciiVisualizer::colored_ascii_square_from(color),
                    false => "  ".white()
                };
                building_string.push_str(&value_string.to_string());
            }
            let align = std::iter::repeat(" ").take(cursor_position).collect::<String>();
            building_string.push_str("\n\r");
            building_string.push_str(&align);
        }
        building_string
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
        let header = "\n\r";
        let left_shift = "                ";
        let left_border = "\u{23B9}".bold();
        let right_border = "\u{23B8}".bold();
        let bottom_border = "\u{203E}".bold();

        building_text.push_str(header);

        let score_string = format!("  Your score : {}\n\r", board.get_score()).bold();
        let level_string = format!("  Level : {}\n\r", board.get_level()).bold();
        let lines_cleared_string = format!("  Lines cleared : {}\n\r", board.get_lines_cleared()).bold();
        building_text.push_str(&score_string);
        building_text.push_str(&level_string);
        building_text.push_str(&lines_cleared_string);

        building_text.push_str("\n\r");

        let next_string = "The next piece is :  ";
        let next_piece: Piece = board.get_next_piece().clone();
        let cursor_position = next_string.len() + left_shift.len();
        let next_piece_string = AsciiVisualizer::colored_ascii_of_piece(Some(next_piece), cursor_position);
        building_text.push_str(left_shift);
        building_text.push_str(next_string);
        building_text.push_str(&next_piece_string);
        building_text.push_str("\n\r");

        for i in (0..BOARD_HEIGHT).rev() {
            building_text.push_str(left_shift);
            building_text.push_str(&left_border.to_string());

            for j in 0..BOARD_WIDTH {
                let value = square_board[j][i];
                let string = match value {
                    Some(square) => AsciiVisualizer::colored_ascii_square_from(square.get_color()).bold(),
                    None => String::from("\u{00B7}\u{00B7}").white()
                };
                
                building_text.push_str(&string.to_string());
            }
            building_text.push_str(&right_border.to_string());
            building_text.push_str("\n\r")
        }

        building_text.push_str(left_shift);
        building_text.push_str(" ");
        for _ in 0..BOARD_WIDTH {
            building_text.push_str(&bottom_border.to_string());
            building_text.push_str(&bottom_border.to_string());
        }
        building_text.push_str("\n\r");

        let held_string = "The held piece is :";
        let held_piece = board.get_held_piece().clone();
        let cursor_position = held_string.len() + left_shift.len();
        let held_piece_string = AsciiVisualizer::colored_ascii_of_piece(held_piece, cursor_position);
        building_text.push_str(left_shift);
        building_text.push_str(held_string);
        building_text.push_str(&held_piece_string);
        building_text.push_str("\n\r");

        println!("{}", building_text);
    }
}