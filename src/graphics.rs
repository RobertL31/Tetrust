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
        let left = "\u{27E6}";
        let right = "\u{27E7}";
        let string = format!("{}{}", left, right);
        match square.get_color() {
            Color::Cyan => string.bright_blue().on_cyan(),
            Color::DarkBlue => string.blue().on_bright_blue(),
            Color::Orange => string.truecolor(255, 128, 0).on_bright_red(),
            Color::Yellow => string.yellow().on_bright_yellow(),
            Color::Green => string.green().on_bright_green(),
            Color::Purple => string.purple().on_bright_purple(),
            Color::Red => string.red().on_bright_red(),
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
        let header = "\n\r";
        let left_shift = "\t\t\t\t";
        let left_border = "\u{23B9}".bold();
        let right_border = "\u{23B8}".bold();
        let bottom_border = "\u{203E}".bold();

        building_text.push_str(header);
        for i in (0..BOARD_HEIGHT).rev() {
            building_text.push_str(left_shift);
            building_text.push_str(&left_border.to_string());

            for j in 0..BOARD_WIDTH {
                let value = square_board[j][i];
                let string = match value {
                    Some(square) => AsciiVisualizer::colored_ascii_of(square).bold(),
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
        
        let score_string = format!("  Your score : {}\n\r", board.get_score()).bold();
        let level_string = format!("  Level : {}\n\r", board.get_level()).bold();
        let lines_cleared_string = format!("  Lines cleared : {}\n\r", board.get_lines_cleared()).bold();
        building_text.push_str(left_shift);
        building_text.push_str(&score_string);
        building_text.push_str(left_shift);
        building_text.push_str(&level_string);
        building_text.push_str(left_shift);
        building_text.push_str(&lines_cleared_string);

        println!("{}", building_text);
    }
}