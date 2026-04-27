pub mod utils;
pub mod parsing;
pub mod fen;
pub mod movegen;
pub mod fish;
pub mod tests;
pub mod games;
pub mod moves;

use std::num::ParseIntError;

use crate::movegen::*;
use crate::moves::Move;
use crate::tests::*;
use crate::utils::*;
use crate::fen::Fen;

const COMMAND_HELP: &str = "
    help \t\t\t Get this info.
    quit \t\t\t Quit the application.
    fen \t\t\t Get access to the engine.

    Fen:

    pos default \t\t Get the default position.
    pos [FEN] \t\t\t Get the given position.
    move [LAN] \t\t\t Apply the given move to the current position.
    string \t\t\t Show the current position in FEN notation.
    board \t\t\t Show the board with info.
    moves \t\t\t Get the legal moves in the current position.
    perft [depth] \t\t Perform perft at current position with given depth.
    perft [FEN] [depth] \t Perform perft at given position with given depth.
";

pub fn startup() {

    // We check if the BMI2 instructions are available, since these are used in move generation
    if !is_x86_feature_detected!("bmi2") || !test_bmi2() {
        println!("Warning: BMI2 not detected. This means we cannot use certain CPU instructions.");
        println!("This means that some operations will be slower, leading to suboptimal results.");
    }

    init_attack_table();

    println!("\nChess Engine Command Line Tool | Enter 'help' for help.");
}


// This layered system may not be ideal, but for a different system we would only have to change this file, so its fine

fn main() {
    startup();

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

        if user_input.find("move ") == Some(0) {
            let lan = user_input.split_at(5).1;
            let result = Move::from_str(lan);

            match result {
                Ok(move1) => { fen.make_move(move1); fen.print_board(); },
                Err(error) => println!("{}", error)
            }
        }

        if user_input == "moves" { 

            let moves = fen.get_pseudo_legal_moves();
            println!("{}", moves.to_string())

        }

        if user_input.find("perft") == Some(0) {
            let perft_str: Vec<&str> = user_input.trim().split_whitespace().collect();
            if perft_str.len() != 2 && perft_str.len() != 3 {
                println!("Error: Perft requires 1 or 2 arguments");
                continue
            }

            let depth_index = perft_str.len() - 1;
            let depth_result: Result<usize, ParseIntError> = perft_str[depth_index].parse();

            if depth_result.is_err() {
                println!("Error: Depth must be a positive integer");
                continue
            }

            let depth = depth_result.unwrap();

            let new_fen: Fen;
            if perft_str.len() == 2 {
                new_fen = fen.clone()
            } else {
                let result = Fen::from_str(perft_str[1]);

                match result {
                    Ok(ok_fen) => { new_fen = ok_fen },
                    Err(error) => { println!("{}", error); continue }
                }
            }

            let result = perft(depth, new_fen);
            println!("{}", result.to_string())


        }

        if user_input == "string" { println!("{}", fen.to_string()) }

        if user_input == "board" { fen.print_board() }

        if user_input == "return" { return true }

        if user_input == "quit" { return false }
    }
}