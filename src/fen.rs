use crate::parsing::info_to_enpassant;
use crate::utils::*;
use crate::parsing;
use crate::movegen::*;
use crate::moves::*;

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
            "Board: {}\n\n\t8 | {}\n\t7 | {}\n\t6 | {}\n\t5 | {}\n\t4 | {}\n\t3 | {}\n\t2 | {}\n\t1 | {}\n\t  + ---------------\n\t    a b c d e f g h",
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
        let occupied = get_occupancy(&self.array);
        if self.white_to_move() { 
            self.is_square_attacked_white(self.array[KING_W], occupied)
        } else {
            self.is_square_attacked_black(self.array[KING_B], occupied)
        }
    }

    pub fn get_pseudo_legal_moves(&self) -> Moves {
        if self.white_to_move() {
            self.get_pseudo_legal_moves_white()
        } else {
            self.get_pseudo_legal_moves_black()
        }
    }

    pub fn get_moves(&self) -> Moves {
        if self.white_to_move() {
            self.get_moves_white()
        } else {
            self.get_moves_black()
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

        let all_pieces = get_occupancy(&self.array);

        let from = move1.get_from();
        let to = move1.get_to();
        let prom = move1.get_prom();

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

        let all_pieces = get_occupancy(&self.array);

        let from = move1.get_from();
        let to = move1.get_to();
        let prom = move1.get_prom();

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

    fn get_pseudo_legal_moves_white(&self) -> Moves {

        // For castling, we do not check if the squares between king and rook are attacked

        let mut moves = Moves::empty();

        let king = self.array[KING_W];
        let info = self.array[INFO];

        let opponents = get_black_pieces(&self.array);
        let team = get_white_pieces(&self.array);
        let occupied = opponents | team;

        let mut king_moves = get_king_moves(king) & !team;
        while king_moves != 0 {
            let index = king_moves.trailing_zeros() as usize;
            let to = 1u64 << index;
            let move1 = Move::new(king, to);
            moves.add(move1);

            king_moves ^= to;
        }

        let kingside = info & WHITE_KINGSIDE_RIGHTS != 0;
        let queenside = info & WHITE_QUEENSIDE_RIGHTS != 0;
        let kingside_free = occupied & WHITE_KINGSIDE_SQUARES == 0;
        let queenside_free = occupied & WHITE_QUEENSIDE_SQUARES == 0;

        if kingside && kingside_free {
            let move1 = Move::new(king, WHITE_KINGSIDE_MOVE_TO);
            moves.add(move1);
        }

        if queenside && queenside_free {
            let move1 = Move::new(king, WHITE_QUEENSIDE_MOVE_TO);
            moves.add(move1);
        }

        let mut pawns = self.array[PAWN_W];
        while pawns != 0 {
            let pawn_index = pawns.trailing_zeros() as usize;
            let pawn = 1u64 << pawn_index;

            let pawn_attacks = get_white_pawn_attacks(pawn) & (opponents | info_to_enpassant(info));
            let pawn_steps = get_white_pawn_steps(pawn, occupied);
            let mut pawn_moves = pawn_attacks | pawn_steps;

            let promotes = pawn & RANK_7 != 0;

            while pawn_moves != 0 {
                let index = pawn_moves.trailing_zeros() as usize;
                let to = 1u64 << index;

                if promotes {
                    let move1 = Move::new_with_prom(pawn, to, Prom::Queen);
                    let move2 = Move::new_with_prom(pawn, to, Prom::Bishop);
                    let move3 = Move::new_with_prom(pawn, to, Prom::Knight);
                    let move4 = Move::new_with_prom(pawn, to, Prom::Rook);

                    moves.add(move1);
                    moves.add(move2);
                    moves.add(move3);
                    moves.add(move4);

                } else {
                    let move1 = Move::new(pawn, to);
                    moves.add(move1);
                }

                pawn_moves ^= to;
            }

            pawns ^= pawn;
        }

        let mut knights = self.array[KNIGHT_W];
        while knights != 0 {
            let piece_index = knights.trailing_zeros() as usize;
            let piece = 1u64 << piece_index;

            let mut knight_moves = get_knight_moves(piece) & !team;
            while knight_moves != 0 {
                let index = knight_moves.trailing_zeros() as usize;
                let to = 1u64 << index;

                let move1 = Move::new(piece, to);
                moves.add(move1);

                knight_moves ^= to;
            }

            knights ^= piece;
        }

        let mut bishops = self.array[BISHOP_W];
        while bishops != 0 {
            let piece_index = bishops.trailing_zeros() as usize;
            let piece = 1u64 << piece_index;

            let mut bishop_moves = get_bishop_moves(piece, occupied) & !team;
            while bishop_moves != 0 {
                let index = bishop_moves.trailing_zeros() as usize;
                let to = 1u64 << index;

                let move1 = Move::new(piece, to);
                moves.add(move1);

                bishop_moves ^= to;
            }

            bishops ^= piece;
        }

        let mut rooks = self.array[ROOK_W];
        while rooks != 0 {
            let piece_index = rooks.trailing_zeros() as usize;
            let piece = 1u64 << piece_index;

            let mut rook_moves = get_rook_moves(piece, occupied) & !team;
            while rook_moves != 0 {
                let index = rook_moves.trailing_zeros() as usize;
                let to = 1u64 << index;

                let move1 = Move::new(piece, to);
                moves.add(move1);

                rook_moves ^= to;
            }

            rooks ^= piece;
        }

        let mut queens = self.array[QUEEN_W];
        while queens != 0 {
            let piece_index = queens.trailing_zeros() as usize;
            let piece = 1u64 << piece_index;

            let mut queen_moves = get_queen_moves(piece, occupied) & !team;
            while queen_moves != 0 {
                let index = queen_moves.trailing_zeros() as usize;
                let to = 1u64 << index;

                let move1 = Move::new(piece, to);
                moves.add(move1);

                queen_moves ^= to;
            }

            queens ^= piece;
        }

        moves
    }

    fn get_pseudo_legal_moves_black(&self) -> Moves {

        // For castling, we do not check if the squares between king and rook are attacked

        let mut moves = Moves::empty();

        let king = self.array[KING_B];
        let info = self.array[INFO];

        let opponents = get_white_pieces(&self.array);
        let team = get_black_pieces(&self.array);
        let occupied = opponents | team;

        let mut king_moves = get_king_moves(king) & !team;
        while king_moves != 0 {
            let index = king_moves.trailing_zeros() as usize;
            let to = 1u64 << index;
            let move1 = Move::new(king, to);

            moves.add(move1);

            king_moves ^= to;
        }

        let kingside = info & BLACK_KINGSIDE_RIGHTS != 0;
        let queenside = info & BLACK_QUEENSIDE_RIGHTS != 0;
        let kingside_free = occupied & BLACK_KINGSIDE_SQUARES == 0;
        let queenside_free = occupied & BLACK_QUEENSIDE_SQUARES == 0;

        if kingside && kingside_free {
            let move1 = Move::new(king, BLACK_KINGSIDE_MOVE_TO);
            moves.add(move1);
        }

        if queenside && queenside_free {
            let move1 = Move::new(king, BLACK_QUEENSIDE_MOVE_TO);
            moves.add(move1);
        }

        let mut pawns = self.array[PAWN_B];
        while pawns != 0 {
            let pawn_index = pawns.trailing_zeros() as usize;
            let pawn = 1u64 << pawn_index;

            let pawn_attacks = get_black_pawn_attacks(pawn)  & (opponents | info_to_enpassant(info));
            let pawn_steps = get_black_pawn_steps(pawn, occupied);
            let mut pawn_moves = pawn_attacks | pawn_steps;

            let promotes = pawn & RANK_2 != 0;

            while pawn_moves != 0 {
                let index = pawn_moves.trailing_zeros() as usize;
                let to = 1u64 << index;

                if promotes {
                    let move1 = Move::new_with_prom(pawn, to, Prom::Queen);
                    let move2 = Move::new_with_prom(pawn, to, Prom::Bishop);
                    let move3 = Move::new_with_prom(pawn, to, Prom::Knight);
                    let move4 = Move::new_with_prom(pawn, to, Prom::Rook);

                    moves.add(move1);
                    moves.add(move2);
                    moves.add(move3);
                    moves.add(move4);

                } else {
                    let move1 = Move::new(pawn, to);
                    moves.add(move1);
                }

                pawn_moves ^= to;
            }

            pawns ^= pawn;
        }

        let mut knights = self.array[KNIGHT_B];
        while knights != 0 {
            let piece_index = knights.trailing_zeros() as usize;
            let piece = 1u64 << piece_index;

            let mut knight_moves = get_knight_moves(piece) & !team;
            while knight_moves != 0 {
                let index = knight_moves.trailing_zeros() as usize;
                let to = 1u64 << index;

                let move1 = Move::new(piece, to);
                moves.add(move1);

                knight_moves ^= to;
            }

            knights ^= piece;
        }

        let mut bishops = self.array[BISHOP_B];
        while bishops != 0 {
            let piece_index = bishops.trailing_zeros() as usize;
            let piece = 1u64 << piece_index;

            let mut bishop_moves = get_bishop_moves(piece, occupied) & !team;
            while bishop_moves != 0 {
                let index = bishop_moves.trailing_zeros() as usize;
                let to = 1u64 << index;

                let move1 = Move::new(piece, to);
                moves.add(move1);

                bishop_moves ^= to;
            }

            bishops ^= piece;
        }

        let mut rooks = self.array[ROOK_B];
        while rooks != 0 {
            let piece_index = rooks.trailing_zeros() as usize;
            let piece = 1u64 << piece_index;

            let mut rook_moves = get_rook_moves(piece, occupied) & !team;
            while rook_moves != 0 {
                let index = rook_moves.trailing_zeros() as usize;
                let to = 1u64 << index;

                let move1 = Move::new(piece, to);
                moves.add(move1);

                rook_moves ^= to;
            }

            rooks ^= piece;
        }

        let mut queens = self.array[QUEEN_B];
        while queens != 0 {
            let piece_index = queens.trailing_zeros() as usize;
            let piece = 1u64 << piece_index;

            let mut queen_moves = get_queen_moves(piece, occupied) & !team;
            while queen_moves != 0 {
                let index = queen_moves.trailing_zeros() as usize;
                let to = 1u64 << index;

                let move1 = Move::new(piece, to);
                moves.add(move1);

                queen_moves ^= to;
            }

            queens ^= piece;
        }

        moves
    }

    fn get_moves_white(&self) -> Moves {

        /*
        Consider the following components of legal move generation:
        - Generate pseudo legal moves
        - Determine if king is in check:

            Compute attacks by opponent pieces, count number of checking pieces

            Sliders: 

        - If double check or more -> only king moves

            No castle, has to move away from attacks, not closer to enemy king, beware xray check

        - If single check -> resolve check

            No castle, move away or capture attacker, see above

        - Determine and resolve pins

            Compute pins by opponent sliders, restrict team pieces movement per pin

            Do we keep a list or do we just have 8 bitboards? Test to see which is faster

        - Enpassant:

            Two edge cases:
            - pin by two pieces: 8/8/8/KpP4r/8/8/8/7k w - - 0 1
            - pin with opponent: 8/8/K7/1pP5/8/8/4b3/7k w - - 0 1

            Is that all?

        - Castling: castling is only allowed if the king and squares in between are not being attacked (rook being attacked is OK).


        So, we start with computing enemy piece attacks, but not for enemy king. For knights and pawns, we can do the king as piece trick (in_check).

        If the number of check is 2 or more, we can only move king to non-attacked squares.
        Else, only one piece is attacking, so we can use one bitboard for this.

        OPTIMISATIONS:

        We could remove branching at the number of checks by &-ing a bitboard of 0's when in double or more check, and a bitboard of 1's when not in check
        Is this not just better?

        Also, switch get_ _moves functions to index based? (Or rather one function for index, one for square?)
        
        Is it more efficient to compute enemy attacks, or reverse attacks?
        We need this information for king moves and castling, reverse is probably better.

        */

        let mut moves = Moves::empty();

        let team = get_white_pieces(&self.array);
        let opps = get_black_pieces(&self.array);
        let occupied = team | opps;

        let king = self.array[KING_W];

        // The king cannot move to a square that has a team member
        let mut king_moves = get_king_moves(king) & !team;

        // We determine the positions of the pieces that check the king
        // We use the sliding moves later to compute the check mask

        let knight_checks = get_knight_moves(king) & self.array[KNIGHT_B];
        let pawn_checks = get_white_pawn_attacks(king) & self.array[PAWN_B];

        let king_rook_moves = get_rook_moves(king, occupied);
        let rook_checks = king_rook_moves & self.array[ROOK_B];

        let king_bishop_moves = get_bishop_moves(king, occupied);
        let bishop_checks = king_bishop_moves & self.array[BISHOP_B];

        let king_queen_moves = get_queen_moves(king, occupied);
        let queen_checks = king_queen_moves & self.array[QUEEN_B];

        let checking_pieces = knight_checks | pawn_checks | rook_checks | bishop_checks | queen_checks;
        let check_count: u32 = checking_pieces.count_ones();

        // We add the king moves to the move list
        let occupied_except_king = occupied & !king;
        while king_moves != 0 {
            let index = king_moves.trailing_zeros() as usize;
            let to = 1u64 << index;

            // We remove the king from the board, so that squares 'behind' the king are also checked, preventing backwards check evasion
            if !self.is_square_attacked_white(to, occupied_except_king) {
                let move1 = Move::new(king, to);
                moves.add(move1);
            }

            king_moves ^= to;
        }

        // In case of two or more checks, only the king can move, so we return only those moves
        if check_count > 1 { return moves }

        // From now on, we can assume that at most one piece is attacking the king
        // Do note that we have not added the castling moves at this point

        // We compute the check_mask, which tells the pieces where they need to move to resolve check
        // In case of a sliding check, it can be resolved by capturing the piece, or moving in between king and attacker
        // In case of a non-sliding check, it can only be resolved by capturing the piece
        // So, we start with checking_pieces, which has only one bit set, and add the 'in between' squares
        // We can put *_checks into the get_*_moves function, since it can have at most one bit set
        let mut check_mask = checking_pieces;

        if queen_checks != 0 {
            check_mask |= king_queen_moves & get_queen_moves(queen_checks, occupied)
        } else if rook_checks != 0 {
            check_mask |= king_rook_moves & get_rook_moves(rook_checks, occupied)
        } else if bishop_checks != 0 {
            check_mask |= king_bishop_moves & get_bishop_moves(bishop_checks, occupied)
        }

        // We will & the mask with the piece movements, so in case there is no check, we want this to do nothing
        if check_mask == 0 { check_mask = u64::MAX }

        // We compute the pins, we DO NOT consider enpassant edge cases here
        let pin_masks = [0u64; 8];
        let pin_count: usize = 0;

        // The following is to prevent the enpassant edge cases:
        //      (1) Pin by two pieces, e.g. 8/8/8/KpP4r/8/8/8/7k w - b6 0 13
        //      (2) Pin by opponent, e.g. 8/8/K7/1pP5/8/8/4b3/7k w - b6 0 1
        // We prevent these by setting ep = EMPTY if they occur
        let mut ep = info_to_enpassant(self.array[INFO]);

        // We start by preventing case (1), this first check eliminates virtually all positions
        if ep != 0 && king & RANK_5 != 0 {

            let ep_attacker = get_black_pawn_attacks(ep) & self.array[PAWN_W];

            // If there is zero or two pawns that can take the enpassant, this case does not occur
            if ep_attacker.count_ones() == 1 {

                // We compute the king moves, ignoring the two pieces, to check if there is a rook or a queen
                let ep_occupied = occupied & !(ep_attacker | (ep >> 8));
                let ep_pin_king_moves = get_rook_moves(king, ep_occupied) & RANK_5;
                let ep_pin_attacker = ep_pin_king_moves & (self.array[ROOK_B] | self.array[QUEEN_B]);

                // If there is zero attackers, this case does not occur
                // If there is two attackers, the king must be in check, so enpassant would not be allowed anyway
                if ep_pin_attacker == 1 {

                    // We compute the squares between the king and the attacker
                    // If there are two pieces in this mask, these must be the two pieces, in which case we do not allow enpassant
                    let ep_mask = get_rook_moves(ep_pin_attacker, ep_occupied) & ep_pin_king_moves;
                    if (ep_mask & occupied).count_ones() == 2 {
                        ep = EMPTY;
                    }
                }
            }
        }


        let pin_occupied = opps & king;

        // First we compute the pin masks, here we ignore 
        let rook_pins = get_rook_moves(king, opps);


        moves
    }

    fn get_moves_black(&self) -> Moves {

        let mut moves = Moves::empty();

        moves
    }

    #[inline(always)]
    fn is_square_attacked_white(&self, square: u64, occupied: u64) -> bool {
        // We let the square make the moves of each piece, to see if there is a piece that is attacking the square.
        // This is more efficient than calculating all the attacks, since there is only one square.
        // It is difficult to test is this is optimal, but it seems to be at least fast enough.

        let knight_attacks = get_knight_moves(square) & self.array[KNIGHT_B];
        let pawn_attacks = get_white_pawn_attacks(square) & self.array[PAWN_B];
        let king_attacks = get_king_moves(square) & self.array[KING_B];
        let rook_or_queen_attacks = get_rook_moves(square, occupied) & (self.array[ROOK_B] | self.array[QUEEN_B]);
        let bishop_or_queen_attacks = get_bishop_moves(square, occupied) & (self.array[BISHOP_B] | self.array[QUEEN_B]);

        knight_attacks | pawn_attacks | king_attacks | rook_or_queen_attacks | bishop_or_queen_attacks != 0
    }

    #[inline(always)]
    fn is_square_attacked_black(&self, square: u64, occupied: u64) -> bool {
        // We let the square make the moves of each piece, to see if there is a piece that is attacking the square.
        // This is more efficient than calculating all the attacks, since there is only one square.
        // It is difficult to test is this is optimal, but it seems to be at least fast enough.

        let knight_attacks = get_knight_moves(square) & self.array[KNIGHT_W];
        let pawn_attacks = get_black_pawn_attacks(square) & self.array[PAWN_W];
        let king_attacks = get_king_moves(square) & self.array[KING_W];
        let rook_or_queen_attacks = get_rook_moves(square, occupied) & (self.array[ROOK_W] | self.array[QUEEN_W]);
        let bishop_or_queen_attacks = get_bishop_moves(square, occupied) & (self.array[BISHOP_W] | self.array[QUEEN_W]);

        knight_attacks | pawn_attacks | king_attacks | rook_or_queen_attacks | bishop_or_queen_attacks != 0
    }
}