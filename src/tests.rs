use std::time::Instant;

use rand::Rng;

// use crate::fish::ask_the_fish;
use crate::fen::Fen;
use crate::games::get_games;
use crate::utils::*;
use crate::movegen::*;
use crate::moves::*;

pub const SINGLE_BITS: BitboardTable = [
    0b0000000000000000000000000000000000000000000000000000000000000001,
    0b0000000000000000000000000000000000000000000000000000000000000010,
    0b0000000000000000000000000000000000000000000000000000000000000100,
    0b0000000000000000000000000000000000000000000000000000000000001000,
    0b0000000000000000000000000000000000000000000000000000000000010000,
    0b0000000000000000000000000000000000000000000000000000000000100000,
    0b0000000000000000000000000000000000000000000000000000000001000000,
    0b0000000000000000000000000000000000000000000000000000000010000000,
    0b0000000000000000000000000000000000000000000000000000000100000000,
    0b0000000000000000000000000000000000000000000000000000001000000000,
    0b0000000000000000000000000000000000000000000000000000010000000000,
    0b0000000000000000000000000000000000000000000000000000100000000000,
    0b0000000000000000000000000000000000000000000000000001000000000000,
    0b0000000000000000000000000000000000000000000000000010000000000000,
    0b0000000000000000000000000000000000000000000000000100000000000000,
    0b0000000000000000000000000000000000000000000000001000000000000000,
    0b0000000000000000000000000000000000000000000000010000000000000000,
    0b0000000000000000000000000000000000000000000000100000000000000000,
    0b0000000000000000000000000000000000000000000001000000000000000000,
    0b0000000000000000000000000000000000000000000010000000000000000000,
    0b0000000000000000000000000000000000000000000100000000000000000000,
    0b0000000000000000000000000000000000000000001000000000000000000000,
    0b0000000000000000000000000000000000000000010000000000000000000000,
    0b0000000000000000000000000000000000000000100000000000000000000000,
    0b0000000000000000000000000000000000000001000000000000000000000000,
    0b0000000000000000000000000000000000000010000000000000000000000000,
    0b0000000000000000000000000000000000000100000000000000000000000000,
    0b0000000000000000000000000000000000001000000000000000000000000000,
    0b0000000000000000000000000000000000010000000000000000000000000000,
    0b0000000000000000000000000000000000100000000000000000000000000000,
    0b0000000000000000000000000000000001000000000000000000000000000000,
    0b0000000000000000000000000000000010000000000000000000000000000000,
    0b0000000000000000000000000000000100000000000000000000000000000000,
    0b0000000000000000000000000000001000000000000000000000000000000000,
    0b0000000000000000000000000000010000000000000000000000000000000000,
    0b0000000000000000000000000000100000000000000000000000000000000000,
    0b0000000000000000000000000001000000000000000000000000000000000000,
    0b0000000000000000000000000010000000000000000000000000000000000000,
    0b0000000000000000000000000100000000000000000000000000000000000000,
    0b0000000000000000000000001000000000000000000000000000000000000000,
    0b0000000000000000000000010000000000000000000000000000000000000000,
    0b0000000000000000000000100000000000000000000000000000000000000000,
    0b0000000000000000000001000000000000000000000000000000000000000000,
    0b0000000000000000000010000000000000000000000000000000000000000000,
    0b0000000000000000000100000000000000000000000000000000000000000000,
    0b0000000000000000001000000000000000000000000000000000000000000000,
    0b0000000000000000010000000000000000000000000000000000000000000000,
    0b0000000000000000100000000000000000000000000000000000000000000000,
    0b0000000000000001000000000000000000000000000000000000000000000000,
    0b0000000000000010000000000000000000000000000000000000000000000000,
    0b0000000000000100000000000000000000000000000000000000000000000000,
    0b0000000000001000000000000000000000000000000000000000000000000000,
    0b0000000000010000000000000000000000000000000000000000000000000000,
    0b0000000000100000000000000000000000000000000000000000000000000000,
    0b0000000001000000000000000000000000000000000000000000000000000000,
    0b0000000010000000000000000000000000000000000000000000000000000000,
    0b0000000100000000000000000000000000000000000000000000000000000000,
    0b0000001000000000000000000000000000000000000000000000000000000000,
    0b0000010000000000000000000000000000000000000000000000000000000000,
    0b0000100000000000000000000000000000000000000000000000000000000000,
    0b0001000000000000000000000000000000000000000000000000000000000000,
    0b0010000000000000000000000000000000000000000000000000000000000000,
    0b0100000000000000000000000000000000000000000000000000000000000000,
    0b1000000000000000000000000000000000000000000000000000000000000000,
];

/// The test_bmi2 functions test if a function flagged with #[cfg(target_feature = "bmi2")] will work or not
#[cfg(target_feature = "bmi2")]
pub fn test_bmi2() -> bool {
    true
}

/// The test_bmi2 functions test if a function flagged with #[cfg(target_feature = "bmi2")] will work or not
#[cfg(not(target_feature = "bmi2"))]
pub fn test_bmi2() -> bool {
    false
}

/// Test if fen from string and fen to string are inverses
pub fn test_fen_string_conversion() {
    let games = get_games();

    for game in games {
        let fen = Fen::from_str(&game).unwrap();
        let fen_str = fen.to_string();

        assert_eq!(game, fen_str)
    }

    println!("Fens converted succesfully")
}

pub fn test_iterator_speed_1(test_count: u128) {
    let mut total_value: u64 = 0;
    let mut rng = rand::rng();

    let start = Instant::now();

    for _ in 0..test_count {
        let value = rng.next_u64();

        for i in 0..64 {
            let mask = 1u64 << i;
            
            if value & mask != 0 {
                total_value = total_value.wrapping_add(mask);
            }
        }
    }

    let total_time = start.elapsed().as_nanos();
    println!("Test 1: Total time {}, average time {}, total value {}", total_time, total_time / test_count, total_value);
}

pub fn test_iterator_speed_2(test_count: u128) {
    let mut total_value: u64 = 0;
    let mut rng = rand::rng();

    let start = Instant::now();

    for _ in 0..test_count {
        let value = rng.next_u64();
        let mut mask = 1u64;

        for _ in 0..64 {
            mask <<= 1;
            
            if value & mask != 0 {
                total_value = total_value.wrapping_add(mask);
            }
        }
    }

    let total_time = start.elapsed().as_nanos();
    println!("Test 2: Total time {}, average time {}, total value {}", total_time, total_time / test_count, total_value);
}

pub fn test_iterator_speed_3(test_count: u128) {
    let mut total_value: u64 = 0;
    let mut rng = rand::rng();

    let start = Instant::now();

    let mut single_bits = [0; 64];
    for i in 0..64 {
        single_bits[i] = 1u64 << i;
    }

    for _ in 0..test_count {
        let value = rng.next_u64();

        for i in 0..64 {
            let mask = single_bits[i];

            if value & mask != 0 {
                total_value = total_value.wrapping_add(mask);
            }
        }
    }

    let total_time = start.elapsed().as_nanos();
    println!("Test 3: Total time {}, average time {}, total value {}", total_time, total_time / test_count, total_value);
}

pub fn test_iterator_speed_4(test_count: u128) {
    let mut total_value: u64 = 0;
    let mut rng = rand::rng();

    let start = Instant::now();

    for _ in 0..test_count {
        let value = rng.next_u64();

        for i in 0..64 {
            let mask = SINGLE_BITS[i];

            if value & mask != 0 {
                total_value = total_value.wrapping_add(mask);
            }
        }
    }

    let total_time = start.elapsed().as_nanos();
    println!("Test 4: Total time {}, average time {}, total value {}", total_time, total_time / test_count, total_value);
}

pub fn test_iterator_speed() {
    let test_count = 100000000;

    /*
    Results on --release for test_count = 100000000:

    Test 1: Total time 2838224200, average time 28, total value 15998089892765419073
    Test 2: Total time 1513037200, average time 15, total value 1604341627243524144
    Test 3: Total time 1515410900, average time 15, total value 13052302783882448646
    Test 4: Total time 1518758200, average time 15, total value 15308147740626881783

    Order of tests does not matter, I checked.

    Conclusion: option 2 (mask <<= 1) is probably best.
    */

    test_iterator_speed_1(test_count);
    test_iterator_speed_2(test_count);
    test_iterator_speed_3(test_count);
    test_iterator_speed_4(test_count);
}

pub fn test_pext_correctness() {
    let games = get_games();
    let mut fens = Vec::new();

    for fen_str in games {
        fens.push(Fen::from_str(&fen_str).unwrap())
    }

    for fen in &fens {
        let white = get_white_pieces(&fen.array);
        let black = get_black_pieces(&fen.array);
        let occupied = white | black;

        let mut rooks = fen.array[ROOK_W];
        while rooks != 0 {
            let square = 1u64 << rooks.trailing_zeros();

            let attacks_ray = get_ray_rook_moves(square, occupied) & !white;
            let attacks_pext = get_pext_rook_moves(square, occupied) & !white;

            if attacks_ray != attacks_pext {
                println!("{}", fen.to_string());
                fen.print_board();
                crate::parsing::print_bitboard(square);
                println!();
                crate::parsing::print_bitboard(attacks_ray);
                println!();
                crate::parsing::print_bitboard(attacks_pext);
                println!();
                panic!();
            }

            rooks ^= square
        }
    }

    println!("Generated moves checked succesfully")
}

pub fn test_pext_speed() {
    let games = get_games();
    let mut fens = Vec::new();

    for fen_str in games {
        for _ in 0..100 {
            fens.push(Fen::from_str(&fen_str).unwrap())
        }
    }

    let mut total = EMPTY;

    let start = Instant::now();
    for fen in &fens {
        let white = get_white_pieces(&fen.array);
        let black = get_black_pieces(&fen.array);
        let occupied = white | black;

        let mut rooks = fen.array[ROOK_W];
        while rooks != 0 {
            let square = 1u64 << rooks.trailing_zeros();

            let attacks = get_pext_rook_moves(square, occupied) & !white;

            total = total.wrapping_add(attacks);

            rooks ^= square
        }
    }

    println!("Test pext: {}", start.elapsed().as_nanos());
}

pub fn test_ray_speed() {
    let games = get_games();
    let mut fens = Vec::new();

    for fen_str in games {
        for _ in 0..100 {
            fens.push(Fen::from_str(&fen_str).unwrap())
        }
    }

    let mut total = EMPTY;

    let start = Instant::now();
    for fen in &fens {
        let white = get_white_pieces(&fen.array);
        let black = get_black_pieces(&fen.array);
        let occupied = white | black;

        let mut rooks = fen.array[ROOK_W];
        while rooks != 0 {
            let square = 1u64 << rooks.trailing_zeros();

            let attacks = get_ray_rook_moves(square, occupied) & !white;

            total = total.wrapping_add(attacks);

            rooks ^= square
        }
    }

    println!("Test rays: {}", start.elapsed().as_nanos());
}

pub fn test_gen_speed() {
    let games = get_games();
    let mut fens = Vec::new();

    for fen_str in games {
        for _ in 0..100 {
            fens.push(Fen::from_str(&fen_str).unwrap())
        }
    }

    let mut total = EMPTY;

    let start = Instant::now();
    for fen in &fens {
        let white = get_white_pieces(&fen.array);
        let black = get_black_pieces(&fen.array);
        let occupied = white | black;

        let mut rooks = fen.array[ROOK_W];
        while rooks != 0 {
            let square = 1u64 << rooks.trailing_zeros();

            let attacks = get_rook_moves(square, occupied) & !white;

            total = total.wrapping_add(attacks);

            rooks ^= square
        }
    }

    println!("Test general: {}", start.elapsed().as_nanos());
}

pub fn test_pext_vs_ray_speed() {
    
    // We test ray twice since there seems to be a latency in the first test
    test_ray_speed();
    
    test_gen_speed();
    test_ray_speed();
    test_pext_speed();
}

pub fn perft(depth: usize, fen: Fen) -> PerftResult {
    let mut result = PerftResult::empty();
    result.moves = fen.get_pseudo_legal_moves();

    for i in 0..result.moves.size {
        let mut new_fen = fen.clone();
        let move1 = result.moves.array[i];
        new_fen.make_move(move1);
        let count = recursive_perft(depth, new_fen);
        result.counts[i] = count;
        result.total += count;
    }

    result
}

pub fn recursive_perft(depth: usize, fen: Fen) -> usize {
    let moves = fen.get_pseudo_legal_moves();

    if depth == 1 { return moves.size }

    let mut total = 0;
    for i in 0..moves.size {
        let mut new_fen = fen.clone();
        let move1 = moves.array[i];
        new_fen.make_move(move1);
        total += recursive_perft(depth - 1, new_fen);
    }

    total
}

