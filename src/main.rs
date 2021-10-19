use crate::size::Size;
use crate::tile::Tile;
use crate::turn::Turn;
use crate::{board::Board, evaluation::Evaluation};
use std::io::stdin;

mod board;
mod evaluation;
mod prune;
mod size;
mod tile;
mod trace;
mod turn;

/// The entry point of the application.
fn main() {
    println!("Do you want to have the bot print out every evaluating move which is incredibly slow and cumbersome? (Y/N)");

    let is_printing = of_input(&|s| match s.to_lowercase().as_str() {
        "y" => Some(true),
        "n" => Some(false),
        _ => None,
    });

    println!("Please specify the board width. This number cannot be higher than 4.");

    let width = of_input(&try_parse);

    println!("Please specify the board height. This number cannot be higher than 4.");

    let height = of_input(&try_parse);

    let mut board = Board::new(Size::new(width, height), None);

    const MAX_DEPTH: usize = 10;

    println!("How deep should the AI's search be? Enter nothing for default value. This number cannot exceed {}, or be 0.", MAX_DEPTH);

    let depth = of_input(&|s| if s.is_empty() { Some(MAX_DEPTH) } else { s.parse::<usize>().ok().and_then(|u| if u > MAX_DEPTH || u == 0 { None } else { Some(u) })});

    println!("Enter your move in format #.# where # is a digit. The first digit corresponds from left-to-right, the second digit corresponds from top-to-bottom. Both numbers are 0-indexed.");

    let mut index = 1;

    while board.evaluate().is_none() {
        println!("{}", board.current_position());

        let turn = of_input(&|s| {
            let mut s = s.chars();

            let first = s.nth(0).and_then(|x| to_number(x, board.width()));
            let second = s.nth(1).and_then(|y| to_number(y, board.height()));

            match first {
                Some(x) => match second {
                    Some(y) => Some((x, y)),
                    _ => None
                }
                _ => None
            }
        });

        let turn = Turn::new(turn.0, turn.1, Tile::O);

        if let Some(played_board) = board.play(turn) {
            board = played_board;
            
            if let Some(turn) = Evaluation::game(&mut board, Some(depth), is_printing).get(index) {
                index += 2;
                board = board.play(turn.clone()).unwrap();
            }
        } else {
            println!("This move is not legal, try again.");
        }
    }
    
    println!("{}", board.current_position());

    println!("{}", match board.evaluate().unwrap().result {
        0 => "Draw!",
        i if i > 0 => "Win!",
        _ => "Lose!"
    });

    user();
}

fn to_number(c: char, limit: usize) -> Option<usize> {
    let c = (c as u32 - '0' as u32) as usize;

    if c >= limit {
        None
    }
    else {
        Some(c)
    }
}

fn try_parse(input: String) -> Option<usize> {
    match input.parse::<usize>() {
        Ok(i) if (i <= 4 && i != 0) => Some(i),
        _ => None,
    }
}

/// Returns a string of what the user typed.
fn user() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}

/// Repeatedly forces the user to input a line until the converter returns some value.
fn of_input<T>(converter: &dyn Fn(String) -> Option<T>) -> T {
    loop {
        if let Some(out) = converter(user()) {
            break out;
        } else {
            println!("Unrecognized input, try again.");
        }
    }
}
