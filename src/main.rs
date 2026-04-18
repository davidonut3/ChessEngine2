pub mod utils;
pub mod parsing;
pub mod fen;
pub mod move_gen;
pub mod fish;
pub mod tests;
pub mod games;
pub mod magics;

use crate::magics::BISHOP_MASKS;
use crate::magics::ROOK_MASKS;
use crate::parsing::*;
use crate::utils::*;
use crate::fen::Fen;

const COMMAND_HELP: &str = "
    help \t\t\t Get this info.
    quit \t\t\t Quit the application.
    fen \t\t\t Get access to the engine.

    Fen:

    pos default \t\t Get the default position.
    pos [FEN notation] \t\t Get the given position.
    move [LAN notation] \t Apply the given move to the current position.
    string \t\t\t Show the current position in FEN notation.
    board \t\t\t Show the board with info.
";

pub fn startup_information() {
    println!("Chess Engine Command Line Tool | Enter 'help' for help.");

    if !is_x86_feature_detected!("bmi2") {
        println!("BMI2 not detected. This means we cannot use certain instructions.");
        println!("In particular, move generation will not work (properly?).");
    }
}


// This layered system may not be ideal, but for a different system we would only have to change this file, so its fine

fn main() {
    startup_information();

    let mut total: usize = 0;
    for mask in ROOK_MASKS {
        let offset = mask.count_ones();
        total += 2_usize.pow(offset);
        println!("{},", total);
    }

    for mask in BISHOP_MASKS {
        let offset = mask.count_ones();
        total += 2_usize.pow(offset);
        println!("{},", total);
    }

    main_loop();
}

fn main_loop() {
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
    let mut fen = Fen::new();

    loop {
        let user_input = get_user_input();

        if user_input == "pos default" {
            fen = Fen::new();
            fen.print_board();
            continue
        }

        if user_input.find("pos") == Some(0) {
            let fen_str = user_input.split_at(4).1;
            let result = Fen::from_str(fen_str);

            match result {
                Ok(new_fen) => { fen = new_fen; fen.print_board(); },
                Err(error) => println!("{}", error)
            }

            continue
        }

        if user_input.find("move") == Some(0) {
            let lan = user_input.split_at(5).1;
            let result = parsing::lan_to_move(lan);

            match result {
                Ok(move1) => { fen.make_move(move1); fen.print_board(); },
                Err(error) => println!("{}", error)
            }
        }

        if user_input == "string" { println!("{}", fen.to_string()) }

        if user_input == "board" { fen.print_board() }

        if user_input == "return" { return true }

        if user_input == "quit" { return false }
    }
}