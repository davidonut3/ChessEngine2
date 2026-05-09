use std::time::{Instant, Duration};

use rand::Rng;

use crate::fen::Fen;
use crate::fish::fish_perft;
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

pub fn get_edge_cases() -> Vec<Fen> {
    
    // See notes for the most relevant list
    let strings = vec![
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
    "8/8/8/KpP4r/8/8/8/7k w - b6 0 1",
    "8/8/8/1pP1K2r/8/8/8/7k w - b6 0 1",
    "8/8/8/r1PpP2K/8/8/8/7k w - d6 0 1",
    "4b3/8/8/2Pp3K/8/8/8/7k w - d6 0 1",
    "4b3/8/8/K1Pp1NQ1/8/8/8/7k w - d6 0 1",
    "1kb4q/6p1/3p2P1/r2Pp1K1/r7/8/8/8 w - e6 0 2",
    "k7/8/8/r2KpP2/8/8/8/8 w - e6 0 1",
    "k7/8/8/r2KpP1r/8/8/8/8 w - e6 0 1",
    "k7/8/8/r1bKpP1r/8/8/8/8 w - e6 0 1",
    "8/8/K7/1pP5/8/8/4b3/7k w - b6 0 1",
    "8/5b2/8/2PpP3/8/8/K7/7k w - d6 0 1",
    "6b1/8/4q3/3pP3/8/1K6/8/7k w - d6 0 1",
    "8/8/7K/6pP/8/8/8/2q4k w - g6 0 1",
    "K7/3B4/8/8/5Pp1/7k/8/8 b - f3 0 1",
    "8/7q/kn4R1/5K2/8/8/8/8 w - - 0 1",
    "2r3k1/8/4r3/8/2B5/2K5/8/8 w - - 0 1",
    "4k3/8/8/8/8/8/3r1r2/4K3 w - - 0 1",
    "4k3/8/8/8/8/8/4r3/4K3 w - - 0 1",
    "4k3/8/8/8/8/2b5/3B4/4K3 w - - 0 1",
    "4k3/8/8/8/8/2b5/3N4/4K3 w - - 0 1",
    "4k3/8/8/8/8/8/3P4/4rK2 w - - 0 1",
    "4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1",
    "4k3/8/8/3pP3/8/8/4r3/4K3 w - d6 0 1",
    "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
    "r3k2r/8/8/8/8/8/8/R2QK2R w KQkq - 0 1",
    "r3k2r/8/8/8/8/8/5r2/R3K2R w KQkq - 0 1",
    "r3k2r/8/8/8/8/8/4r3/R3K2R w KQkq - 0 1",
    "4k3/P7/8/8/8/8/8/4K3 w - - 0 1",
    "4k3/1p6/P7/8/8/8/8/4K3 w - - 0 1",
    "4k3/8/8/8/8/8/3R4/4K2r w - - 0 1",
    "7k/5Q2/6K1/8/8/8/8/8 b - - 0 1",
    "7k/6Q1/6K1/8/8/8/8/8 b - - 0 1",
    "4k3/8/8/8/2b5/8/3R4/4K2r w - - 0 1",
    "4k3/8/8/8/8/2r5/3B4/4K3 w - - 0 1",
    ];

    let mut result = Vec::new();

    for string in strings {
        let fen = Fen::from_str(string).unwrap();
        result.push(fen);
    }

    result
}

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

pub fn perft(depth: usize, fen: &Fen) -> PerftResult {
    let mut result = PerftResult::empty();
    result.moves = fen.get_moves();

    for i in 0..result.moves.size {
        let mut new_fen = fen.clone();
        let move1 = result.moves.array[i];
        new_fen.make_move(move1);
        let count = recursive_perft(depth, &new_fen);
        result.counts[i] = count;
        result.total += count;
    }

    result
}

pub fn recursive_perft(depth: usize, fen: &Fen) -> usize {
    let moves = fen.get_moves();

    if depth == 1 { return 1 }

    if depth == 2 { return moves.size }

    let mut total = 0;
    for i in 0..moves.size {
        let mut new_fen = fen.clone();
        let move1 = moves.array[i];
        new_fen.make_move(move1);

        total += recursive_perft(depth - 1, &new_fen);
    }

    total
}

pub fn compare_perft_results(depth: usize, fen: &Fen) {
    let our_result = perft(depth, &fen);
    let fish_result = fish_perft(depth, &fen);

    // For each fish move, we get search this move in our moves
    // If it is present, we compare counts, if these are equal we continue
    // If the counts are not equal, we continue searching in this move
    // If for each fish move, the counts are correct, we look for moves made by us that the fish did not generate

    for i in 0..fish_result.moves.size {
        let move1 = fish_result.moves.array[i];
        let count = fish_result.counts[i];

        let index_option = our_result.moves.array.iter().position(|m| *m == move1);

        if let Some(index) = index_option {

            let our_count = our_result.counts[index];

            if our_count != count {
                println!("In position {}, move {} has {} submoves, but we have {} submoves", fen.to_string(), move1.to_string(), count, our_count);
                let mut new_fen = fen.clone();
                new_fen.make_move(move1);
                compare_perft_results(depth - 1, &new_fen);
                return
            }

        } else {
            println!("In position {}, fish generates move {}, but we don't", fen.to_string(), move1.to_string());
            return
        }

    }

    for i in 0..our_result.moves.size {
        let move1 = our_result.moves.array[i];

        if !fish_result.moves.array.contains(&move1) {
            println!("In position {}, we generate move {}, but the fish doesn't", fen.to_string(), move1.to_string());
            return
        }
    }

    println!("Comparison at {} depth {} succesful!", fen.to_string(), depth)
}

pub fn check_perft_edge_cases() {
    let fens = get_edge_cases();
    for fen in fens {
        compare_perft_results(5, &fen);
    }

    println!("\nEdge case test completed")
}

pub fn move_gen_perft(count: usize) {
    let games = get_games();
    let mut fens = Vec::new();

    for i in 0..count {
        let index = i % games.len();
        fens.push(Fen::from_str(&games[index]).unwrap());
    }

    fens[0].get_moves();

    let mut durations: Vec<Duration> = Vec::with_capacity(count);

    for i in 0..count {
        let time: Instant = Instant::now();
        fens[i].get_moves();
        durations.push(time.elapsed());
    }

    let mut total_nanos: u128 = 0;
    let mut min: Duration = durations[0];
    let mut max: Duration = durations[0];

    let mut worst_fen: String = fens[0].to_string();
    let mut best_fen: String = fens[0].to_string();

    for i in 0..count {
        let duration: Duration = durations[i];
        total_nanos += duration.as_nanos();

        if duration < min {
            min = duration;
            best_fen = fens[i].to_string();
        }

        if duration > max {
            max = duration;
            worst_fen = fens[i].to_string();
        }
    }

    durations.sort_unstable();

    let ignore: usize = (count as f32 * 0.1) as usize;
    let smart_durations: &[Duration] = &durations[ignore..count-ignore];
    let smart_count: usize = smart_durations.len();

    let mut smart_total_nanos: u128 = 0;
    for i in 0..smart_count {
        let duration: Duration = smart_durations[i];
        smart_total_nanos += duration.as_nanos();
    }

    let avg: Duration = Duration::from_nanos((total_nanos / count as u128) as u64);
    let smart_avg: Duration = Duration::from_nanos((smart_total_nanos / smart_count as u128) as u64);

    println!("Min duration {:?} at {}", min, best_fen);
    println!("Max duration {:?} at {}", max, worst_fen);
    println!("Average duration {:?}", avg);
    println!("Middle 80% average {:?}", smart_avg);
}

pub fn moves_per_second() {
    let fen = Fen::new();

    let time: Instant = Instant::now();
    let count = perft(7, &fen).total;
    let duration = time.elapsed();

    let duration_seconds = duration.as_secs_f32();
    let nodes_per_second = count as f32 / duration_seconds;
    let million_per_second = nodes_per_second / 1000000.0;

    println!("Getting {} moves took {} seconds, which is {}M nodes per second", count, duration_seconds, million_per_second);
}
