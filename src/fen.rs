use crate::utils::*;
use crate::parsing;
use crate::movegen::*;

#[derive(Debug, Clone)]
pub struct Fen {
    pub array: FenArray,
}

impl Fen {
    pub fn new() -> Self {
        Self { array: DEFAULT_FEN_ARRAY }
    }

    pub fn from_str(fen_str: &str) -> Result<Self, String> {
        let result = parsing::string_to_fen(fen_str);

        match result {
            Ok(array) => Ok(Self { array }),
            Err(error) => Err(error),
        }
    }

    pub fn to_string(&self) -> String {
        parsing::fen_to_string(self.array)
    }

    pub fn print_board(&self) {

        let mut fen_str = parsing::fen_to_string(self.array);
        let split = fen_str.find(" ").unwrap();
        let info = fen_str.split_off(split);

        let board: Vec<&str> = fen_str.split('/').collect();

        let mut rows = Vec::new();

        for (row, chars) in board.iter().enumerate() {
            rows.push("".to_string());

            for piece in chars.chars() {
                if piece.is_digit(10) {
                    for _ in 0..piece.to_digit(10).unwrap() { rows[row] += ". " }
                } else {
                    rows[row] += &piece.to_string();
                    rows[row] += " ";
                }
            }
        }

        println!(
            "Board: {}\n\t8 {}\n\t7 {}\n\t6 {}\n\t5 {}\n\t4 {}\n\t3 {}\n\t2 {}\n\t1 {}\n\t  a b c d e f g h",
            info, rows[0], rows[1], rows[2], rows[3], rows[4], rows[5], rows[6], rows[7]
        )
    }

    pub fn white_to_move(&self) -> bool {
        self.array[INFO] & TURN_FLAG != 0
    }
    
    pub fn make_move(&mut self, move1: Move) {
        if self.white_to_move() { 
            self.make_move_white(move1);
        } else {
            self.make_move_black(move1);
        }
    }

    pub fn in_check(&self) -> bool {
        if self.white_to_move() { 
            self.in_check_white()
        } else {
            self.in_check_black()
        }
    }

    fn make_move_white(&mut self, move1: Move) {

        // We assume that white is to move, and that the move and the position are legal
        // If the move or position is not legal, some operations may have unintended behavior

        let info = self.array[INFO];

        let enpassant = parsing::info_to_enpassant(info);
        let halfmove = parsing::info_to_halfmove(info);

        let new_enpassant: u64;
        let new_halfmove: u64;

        let white_kingside = info & WHITE_KINGSIDE_RIGHTS != 0;
        let white_queenside = info & WHITE_QUEENSIDE_RIGHTS != 0;
        let black_kingside = info & BLACK_KINGSIDE_RIGHTS != 0;
        let black_queenside = info & BLACK_QUEENSIDE_RIGHTS != 0;

        let all_pieces = get_white_pieces(&self.array) | get_black_pieces(&self.array);

        let from = parsing::move_to_from(move1);
        let to = parsing::move_to_to(move1);
        let prom = parsing::move_to_prom(move1);

        // We only need to increase the fullmove counter when black is to move
        // Also, since black is now to move, we leave turn flag blank in info
        let new_fullmove = info & FULLMOVE_FLAG;

        // We update the castle information
        let king_moved = self.array[KING_W] & from != 0;

        let new_white_kingside = if white_kingside && !king_moved && (from & WHITE_KINGSIDE_ROOK == 0) { WHITE_KINGSIDE_RIGHTS } else { EMPTY };
        let new_white_queenside = if white_queenside && !king_moved && (from & WHITE_QUEENSIDE_ROOK == 0) { WHITE_QUEENSIDE_RIGHTS } else { EMPTY };
        let new_black_kingside = if black_kingside && (to & BLACK_KINGSIDE_ROOK == 0) { BLACK_KINGSIDE_RIGHTS } else { EMPTY };
        let new_black_queenside = if black_queenside && (to & BLACK_QUEENSIDE_ROOK == 0) { BLACK_QUEENSIDE_RIGHTS } else { EMPTY };

        // We assign the new enpassant flag
        if (to & RANK_4 != 0) && (from & self.array[PAWN_W] & RANK_2 != 0) {
            new_enpassant = (from << 8).trailing_zeros() as u64;
        } else {
            new_enpassant = NO_ENPASSANT_FLAG;
        }

        // In case of capture or pawn movement, we increase halfmove, else we reset it
        if (to & all_pieces == 0) && (from & (self.array[PAWN_W] | self.array[PAWN_B]) == 0) {
            new_halfmove = parsing::halfmove_to_info(halfmove + 1);
        } else {
            new_halfmove = 0;
        }

        // If there is a castle, we move the respective rook
        if king_moved && white_kingside && (to == WHITE_KINGSIDE_MOVE_TO) {
            self.array[ROOK_W] ^= WHITE_KINGSIDE_ROOK_MASK;
        } else if king_moved && white_queenside && (to == WHITE_QUEENSIDE_MOVE_TO) {
            self.array[ROOK_W] ^= WHITE_QUEENSIDE_ROOK_MASK;
        }

        // If there is an enpassant, we remove the captured pawn
        if (enpassant == to) && (self.array[PAWN_W] & from != 0) {
            self.array[PAWN_B] ^= to >> 8;
        }

        // In case of a capture, we remove the captured piece
        for i in 0..PIECE_TYPES {
            if self.array[i] & to != 0 {
                self.array[i] ^= to;
                break;
            }
        }

        // We move the piece of this move to its new place
        for i in 0..PIECE_TYPES {
            if self.array[i] & from != 0 {
                self.array[i] ^= from | to;
                break;
            }
        }

        // In case of promotion we replace the pawn with the new piece
        match prom {
            Prom::Queen =>      { self.array[PAWN_W] ^= to; self.array[QUEEN_W] ^= to; },
            Prom::Bishop =>     { self.array[PAWN_W] ^= to; self.array[BISHOP_W] ^= to; },
            Prom::Knight =>     { self.array[PAWN_W] ^= to; self.array[KNIGHT_W] ^= to; },
            Prom::Rook =>       { self.array[PAWN_W] ^= to; self.array[ROOK_W] ^= to; },
            Prom::NoProm =>     {},
        }

        self.array[INFO] = new_enpassant | new_halfmove | new_fullmove | new_white_kingside | new_white_queenside | new_black_kingside | new_black_queenside

    }

    fn make_move_black(&mut self, move1: Move) {

        // We assume that black is to move, and that the move and the position are legal
        // If the move or position is not legal, some operations may have unintended behavior

        let info = self.array[INFO];

        let enpassant = parsing::info_to_enpassant(info);
        let halfmove = parsing::info_to_halfmove(info);
        let fullmove = parsing::info_to_fullmove(info);

        let new_enpassant: u64;
        let new_halfmove: u64;

        let white_kingside = info & WHITE_KINGSIDE_RIGHTS != 0;
        let white_queenside = info & WHITE_QUEENSIDE_RIGHTS != 0;
        let black_kingside = info & BLACK_KINGSIDE_RIGHTS != 0;
        let black_queenside = info & BLACK_QUEENSIDE_RIGHTS != 0;

        let all_pieces = get_white_pieces(&self.array) | get_black_pieces(&self.array);

        let from = parsing::move_to_from(move1);
        let to = parsing::move_to_to(move1);
        let prom = parsing::move_to_prom(move1);

        // When black is to move, we increase the fullmove counter by one
        let new_turn = TURN_FLAG;
        let new_fullmove = parsing::fullmove_to_info(fullmove + 1);

        // We update the castle information
        let king_moved = self.array[KING_B] & from != 0;

        let new_white_kingside = if white_kingside && (to & WHITE_KINGSIDE_ROOK == 0) { WHITE_KINGSIDE_RIGHTS } else { EMPTY };
        let new_white_queenside = if white_queenside && (to & WHITE_QUEENSIDE_ROOK == 0) { WHITE_QUEENSIDE_RIGHTS } else { EMPTY };
        let new_black_kingside = if black_kingside && !king_moved && (from & BLACK_KINGSIDE_ROOK == 0) { BLACK_KINGSIDE_RIGHTS } else { EMPTY };
        let new_black_queenside = if black_queenside && !king_moved && (from & BLACK_QUEENSIDE_ROOK == 0) { BLACK_QUEENSIDE_RIGHTS } else { EMPTY };

        // We assign the new enpassant flag
        if (to & RANK_5 != 0) && (from & self.array[PAWN_B] & RANK_7 != 0) {
            new_enpassant = (from >> 8).trailing_zeros() as u64;
        } else {
            new_enpassant = NO_ENPASSANT_FLAG;
        }

        // In case of capture or pawn movement, we increase halfmove, else we reset it
        if (to & all_pieces == 0) && (from & (self.array[PAWN_W] | self.array[PAWN_B]) == 0) {
            new_halfmove = parsing::halfmove_to_info(halfmove + 1);
        } else {
            new_halfmove = 0;
        }

        // If there is a castle, we move the respective rook
        if king_moved && black_kingside && (to == BLACK_KINGSIDE_MOVE_TO) {
            self.array[ROOK_B] ^= BLACK_KINGSIDE_ROOK_MASK;
        } else if king_moved && black_queenside && (to == BLACK_QUEENSIDE_MOVE_TO) {
            self.array[ROOK_B] ^= BLACK_QUEENSIDE_ROOK_MASK;
        }

        // If there is an enpassant, we remove the captured pawn
        if (enpassant == to) && (self.array[PAWN_B] & from != 0) {
            self.array[PAWN_W] ^= to << 8;
        }

        // In case of a capture, we remove the captured piece
        for i in 0..PIECE_TYPES {
            if self.array[i] & to != 0 {
                self.array[i] ^= to;
                break;
            }
        }

        // We move the piece of this move to its new place
        for i in 0..PIECE_TYPES {
            if self.array[i] & from != 0 {
                self.array[i] ^= from | to;
                break;
            }
        }

        // In case of promotion we replace the pawn with the new piece
        match prom {
            Prom::Queen =>      { self.array[PAWN_B] ^= to; self.array[QUEEN_B] ^= to; },
            Prom::Bishop =>     { self.array[PAWN_B] ^= to; self.array[BISHOP_B] ^= to; },
            Prom::Knight =>     { self.array[PAWN_B] ^= to; self.array[KNIGHT_B] ^= to; },
            Prom::Rook =>       { self.array[PAWN_B] ^= to; self.array[ROOK_B] ^= to; },
            Prom::NoProm =>     {},
        }

        self.array[INFO] = new_enpassant | new_turn | new_halfmove | new_fullmove | new_white_kingside | new_white_queenside | new_black_kingside | new_black_queenside


    }

    fn in_check_white(&self) -> bool {
        
        // We let the king make the moves of each piece, to see if there is a piece that is attacking the king.
        // This is more efficient than calculating all the moves, since there is only one king.
        // We do not have to care for the opponent king, enpassant and pseudo-legal moves that are not legal.
        
        let king = self.array[KING_W];

        // We check knights and pawns first, since they are simple lookups and do not need occupancy.
        if get_knight_moves(king) & self.array[KNIGHT_B] != 0 { return true }

        // We use white pawn attacks, since this is the inverse of black pawn attacks.
        if get_white_pawn_attacks(king) & self.array[PAWN_B] != 0 { return true }

        let occupied = get_occupancy(&self.array);

        if get_queen_moves(king, occupied) & self.array[QUEEN_B] != 0 { return true }
        if get_rook_moves(king, occupied) & self.array[ROOK_B] != 0 { return true }
        if get_bishop_moves(king, occupied) & self.array[BISHOP_B] != 0 { return true }

        false
    }

    fn in_check_black(&self) -> bool {

        // We let the king make the moves of each piece, to see if there is a piece that is attacking the king.
        // This is more efficient than calculating all the moves, since there is only one king.
        // We do not have to care for the opponent king, enpassant and pseudo-legal moves that are not legal.
        
        let king = self.array[KING_B];

        // We check knights and pawns first, since they are simple lookups and do not need occupancy.
        if get_knight_moves(king) & self.array[KNIGHT_W] != 0 { return true }

        // We use black pawn attacks, since this is the inverse of black pawn attacks.
        if get_black_pawn_attacks(king) & self.array[PAWN_W] != 0 { return true }

        let occupied = get_occupancy(&self.array);

        if get_queen_moves(king, occupied) & self.array[QUEEN_W] != 0 { return true }
        if get_rook_moves(king, occupied) & self.array[ROOK_W] != 0 { return true }
        if get_bishop_moves(king, occupied) & self.array[BISHOP_W] != 0 { return true }

        false
    }
}