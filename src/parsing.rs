use crate::utils::*;

/// Checks if given lan is valid
pub fn is_valid_lan(lan: &str) -> bool {
    if lan.len() != 2 { return false }

    let file_str = lan.chars().nth(0).unwrap();
    let rank_str = lan.chars().nth(1).unwrap();

    let file = lan_to_file(file_str);
    let rank = lan_to_rank(rank_str);

    file != usize::MAX && rank != usize::MAX
}

/// Maps file letter to file index, starting from the h file
pub fn lan_to_file(letter: char) -> usize {
    match letter {
        'a' => 7,
        'b' => 6,
        'c' => 5,
        'd' => 4,
        'e' => 3,
        'f' => 2,
        'g' => 1,
        'h' => 0,
        _ => usize::MAX,
    }
}

/// Maps file index to file letter, starting from the h file
pub fn file_to_lan(file: usize) -> String {
    let letter = match file {
        7 => "a",
        6 => "b",
        5 => "c",
        4 => "d",
        3 => "e",
        2 => "f",
        1 => "g",
        0 => "h",
        _ => "-",
    };

    letter.to_string()
}

/// Maps rank letter to rank index, starting from the 1 rank
pub fn lan_to_rank(letter: char) -> usize {
    match letter {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => usize::MAX,
    }
}

/// Maps rank index to rank letter, starting from the 1 rank
pub fn rank_to_lan(rank: usize) -> String {
    let letter = match rank {
        0 => "1",
        1 => "2",
        2 => "3",
        3 => "4",
        4 => "5",
        5 => "6",
        6 => "7",
        7 => "8",
        _ => "-",
    };

    letter.to_string()
}

/// Converts bitboard to LAN notation
pub fn bitboard_to_lan(bitboard: u64) -> String {
    let pos = bitboard.trailing_zeros() as usize;
    let file = pos % 8;
    let rank = pos / 8;

    file_to_lan(file) + &rank_to_lan(rank)
}

/// Converts LAN notation to bitboard
pub fn lan_to_bitboard(lan: &str) -> u64 {
    let file_str = lan.chars().nth(0).unwrap();
    let rank_str = lan.chars().nth(1).unwrap();

    let file = lan_to_file(file_str);
    let rank = lan_to_rank(rank_str);

    1u64 << (rank * 8) + file
}

/// Maps piece index to its char
pub fn piece_to_char(piece: usize) -> char {
    match piece {
        PAWN_W => 'P', PAWN_B => 'p',
        KING_W => 'K', KING_B => 'k',
        QUEEN_W => 'Q', QUEEN_B => 'q',
        BISHOP_W => 'B', BISHOP_B => 'b',
        KNIGHT_W => 'N', KNIGHT_B => 'n',
        ROOK_W => 'R', ROOK_B => 'r',
        _ => '-'
    }
}

/// Maps char to its piece index
pub fn char_to_piece(char: char) -> usize {
    match char {
        'P' => PAWN_W, 'p' => PAWN_B,
        'K' => KING_W, 'k' => KING_B,
        'Q' => QUEEN_W, 'q' => QUEEN_B,
        'B' => BISHOP_W, 'b' => BISHOP_B,
        'N' => KNIGHT_W, 'n' => KNIGHT_B,
        'R' => ROOK_W, 'r' => ROOK_B,
        _ => usize::MAX
    }
}

/// Converts piece position data into FEN notation
pub fn board_to_fen_notation(array: FenArray) -> String {
    let mut result = String::new();

    for rank in 0..8 {
        let mut empty = 0;

        for file in 0..8 {
            let pos = FIRST >> (rank * 8 + file);

            // We find the index for which the array is not empty at pos, then map this to its char
            let mut piece_char: Option<char> = None;
            for i in 0..PIECE_TYPES {
                if array[i] & pos != 0 {
                    piece_char = Some(piece_to_char(i));
                }
            }

            // If we find a piece, we write the number of empty spots before, then the piece char,
            // if we find no piece, we increase the empty spot counter
            if let Some(char)= piece_char {
                if empty > 0 {
                    result.push_str(&empty.to_string());
                    empty = 0;
                }
                result.push(char);
            } else {
                empty += 1;
            }
        }

        if empty > 0 {
            result.push_str(&empty.to_string());
        }

        if rank != 7 {
            result.push('/');
        }
    }

    result
}

/// Converts FEN notation into piece position data
pub fn fen_notation_to_board(board: &str) -> Result<FenArray, String> {
    let mut pieces: FenArray = [0; FEN_ARRAY_SIZE];

    let rows: Vec<&str> = board.split('/').collect();
    if rows.len() != 8 { return Err("Error: Board must have 8 rows".to_string())}

    for (rank, chars) in rows.iter().enumerate() {
        let mut file: usize = 0;

        for piece in chars.chars() {
            if piece.is_digit(10) {
                file += piece.to_digit(10).unwrap() as usize;
            } else {

                let allowed_pieces = ['P', 'p', 'K', 'k', 'Q', 'q', 'B', 'b', 'N', 'n', 'R', 'r'];
                if !allowed_pieces.contains(&piece) { return Err("Error: Piece must be one of 'P', 'p', 'K', 'k', 'Q', 'q', 'B', 'b', 'N', 'n', 'R', 'r'".to_string()) }

                let pos: u64 = FIRST >> (rank * 8 + file);
                let index = char_to_piece(piece);
                pieces[index] |= pos;
                file += 1
            }
        }

        if file != 8 { return Err("Error: Each row of the board must have 8 entries".to_string())}
    }

    pieces[WHITE] = get_white_pieces(&pieces);
    pieces[BLACK] = get_black_pieces(&pieces);

    Ok(pieces)
}


pub fn halfmove_to_info(halfmove: u64) -> u64 {
    halfmove << 8
}

pub fn info_to_halfmove(info: u64) -> u64 {
    (info & HALFMOVE_FLAG) >> 8
}

pub fn fullmove_to_info(fullmove: u64) -> u64 {
    fullmove << 24
}

pub fn info_to_fullmove(info: u64) -> u64 {
    (info & FULLMOVE_FLAG) >> 24
}

pub fn enpassant_to_info(enpassant: u64) -> u64 {
    enpassant.trailing_zeros() as u64
}

pub fn info_to_enpassant(info: u64) -> u64 {
    let shift = info & ENPASSANT_FLAG;

    if shift >= 64 { EMPTY } else { 1u64 << shift }
}

/// Converts FEN notation into fen array
pub fn string_to_fen(fen_str: &str) -> Result<FenArray, String> {
    let split: Vec<&str> = fen_str.trim().split_whitespace().collect();
    if split.len() != 6 { return Err("Error: FEN does not have 6 fields".to_string()) }

    let mut fen: FenArray;
    let result = fen_notation_to_board(split[0]);
    match result {
        Ok(fen_array) => fen = fen_array,
        Err(error) => return Err(error)
    }

    let turn_str: &str = split[1];
    if turn_str != "w" && turn_str != "b" { return Err("Error: Turn must be 'w' or 'b'".to_string()) }
    let turn: u64 = if turn_str == "w" { TURN_FLAG } else { EMPTY };

    let castle_str: &str = split[2];
    let allowed_castle = ["-", "K", "Q", "k", "q", "KQ", "Kk", "Kq", "Qk", "Qq", "kq", "KQk", "KQq", "Kkq", "Qkq", "KQkq"];
    if !allowed_castle.contains(&castle_str) { return Err("Error: Castle rights must be '-', or a subset of 'KQkq'".to_string()) }

    let mut castle: u64 = EMPTY;
    
    if castle_str.contains("K") { castle |= WHITE_KINGSIDE_RIGHTS }
    if castle_str.contains("Q") { castle |= WHITE_QUEENSIDE_RIGHTS }
    if castle_str.contains("k") { castle |= BLACK_KINGSIDE_RIGHTS }
    if castle_str.contains("q") { castle |= BLACK_QUEENSIDE_RIGHTS }

    let enpassant_str: &str = split[3];
    if enpassant_str != "-" && !is_valid_lan(enpassant_str) { return Err("Error: Enpassant must be '-' or a square on the board".to_string()) }
    let enpassant: u64 = if enpassant_str == "-" { NO_ENPASSANT_FLAG } else { enpassant_to_info(lan_to_bitboard(enpassant_str)) };

    let halfmove_str = split[4];
    for char in halfmove_str.chars() {
        if !char.is_digit(10) { return Err("Error: Halfmove must be a number".to_string()) }
    }
    let halfmove_int: u64 = halfmove_str.parse().unwrap();
    if halfmove_int > 65535 { return Err("Error: Halfmove must be less than 65535".to_string())}
    let halfmove = halfmove_to_info(halfmove_int);

    let fullmove_str = split[5];
    for char in fullmove_str.chars() {
        if !char.is_digit(10) { return Err("Error: Fullmove must be a number".to_string()) }
    }
    let fullmove_int: u64 = fullmove_str.parse().unwrap();
    if fullmove_int > 65535 { return Err("Error: Fullmove must be less than 65535".to_string())}
    let fullmove = fullmove_to_info(fullmove_int);

    let info = turn | castle | enpassant | halfmove | fullmove;
    fen[INFO] = info;

    Ok(fen)
}

/// Converts fen array into FEN notation
pub fn fen_to_string(fen: FenArray) -> String {
    let board = board_to_fen_notation(fen);
    let info = fen[INFO];

    let turn = if info & TURN_FLAG == 0 { "b" } else { "w" };

    let mut castle = "".to_string();

    if info & WHITE_KINGSIDE_RIGHTS != 0    { castle += "K" }
    if info & WHITE_QUEENSIDE_RIGHTS != 0   { castle += "Q" }
    if info & BLACK_KINGSIDE_RIGHTS != 0    { castle += "k" }
    if info & BLACK_QUEENSIDE_RIGHTS != 0   { castle += "q" }

    if castle == "" { castle = "-".to_string() }

    let enpassant = if info & ENPASSANT_FLAG >= 64 { "-".to_string() } else { bitboard_to_lan(info_to_enpassant(info)) };
    let halfmove = info_to_halfmove(info).to_string();
    let fullmove = info_to_fullmove(info).to_string();

    format!("{} {} {} {} {} {}", board, turn, castle, enpassant, halfmove, fullmove)
}
