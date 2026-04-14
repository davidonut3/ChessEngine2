pub mod utils;
pub mod parsing;
pub mod fen;
pub mod move_gen;

use crate::utils::*;
use crate::fen::Fen;

const COMMAND_HELP: &str = "
    help \t Get this info.
    quit \t Quit the application.
    fen \t Get access to the engine.

    Fen:

    pos default \t Get the default position
    pos [FEN notation] \t Get the given position
";

// This layered system may not be ideal, but for a different system we would only have to change this file, so its fine

fn main() {
    println!("Chess Engine Command Line Tool | Enter 'help' for help.");

    loop {
        let user_input = get_user_input().to_lowercase();

        if user_input == "help" {
            println!("{}", COMMAND_HELP);
            continue
        }

        if user_input == "fen" && !fen() { return }

        if user_input == "quit" { break }
    }
}

fn fen() -> bool {
    let mut fen;

    loop {
        let user_input = get_user_input();

        if user_input == "pos default" {
            fen = Fen::new();
            println!("Pos: {}", fen.to_string());
            continue
        }

        if user_input.find("pos") == Some(0) {
            let fen_str = user_input.split_at(3).1;
            let result = Fen::from_str(fen_str);

            match result {
                Ok(new_fen) => { fen = new_fen; println!("Position: {}", fen.to_string()) },
                Err(error) => println!("{}", error)
            }

            continue
        }

        if user_input == "return" { return true }

        if user_input == "quit" { return false }
    }
}