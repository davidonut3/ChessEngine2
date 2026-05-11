use crate::parsing::*;
use crate::utils::*;
use crate::movegen::*;
use crate::moves::*;

macro_rules! gen_pseudo_legal {
    (
        $name:ident,

        $king:ident,
        $pawn:ident,
        $knight:ident,
        $bishop:ident,
        $rook:ident,
        $queen:ident,

        $get_team:ident,
        $get_opps:ident,

        $kingside_rights:ident,
        $queenside_rights:ident,
        $kingside_squares:ident,
        $queenside_squares:ident,
        $kingside_to_index:ident,
        $queenside_to_index:ident,

        $pawn_attacks:ident,
        $pawn_steps:ident,
        $pawn_promotion:ident,

    ) => {
        fn $name(&self) -> Moves {
            // For castling, we do not check if the squares between king and rook are attacked

            let mut moves = Moves::empty();

            let king = self.array[$king];
            let king_index = king.trailing_zeros() as usize;
            let info = self.array[INFO];
            let ep = info_to_enpassant(info);

            let opponents = $get_opps(&self.array);
            let team = $get_team(&self.array);
            let occupied = opponents | team;

            let mut king_moves = KING_MOVES[king_index] & !team;
            while king_moves != 0 {
                let index = king_moves.trailing_zeros() as usize;
                let move1 = Move::new(king_index, index);
                moves.add(move1);

                king_moves &= king_moves.wrapping_sub(1);
            }

            let kingside = info & $kingside_rights != 0;
            let queenside = info & $queenside_rights != 0;
            let kingside_free = occupied & $kingside_squares == 0;
            let queenside_free = occupied & $queenside_squares == 0;

            if kingside && kingside_free {
                let move1 = Move::new(king_index, $kingside_to_index);
                moves.add(move1);
            }

            if queenside && queenside_free {
                let move1 = Move::new(king_index, $queenside_to_index);
                moves.add(move1);
            }

            let mut pawns = self.array[$pawn];
            while pawns != 0 {
                let pawn_index = pawns.trailing_zeros() as usize;

                let pawn_attacks = $pawn_attacks[pawn_index] & (opponents | ep);
                let pawn_steps = $pawn_steps(pawn_index, occupied);
                let mut pawn_moves = pawn_attacks | pawn_steps;

                while pawn_moves != 0 {
                    let index = pawn_moves.trailing_zeros() as usize;

                    if $pawn_promotion(pawn_index) {
                        let move1 = Move::new_with_prom(pawn_index, index, Prom::Queen);
                        let move2 = Move::new_with_prom(pawn_index, index, Prom::Bishop);
                        let move3 = Move::new_with_prom(pawn_index, index, Prom::Knight);
                        let move4 = Move::new_with_prom(pawn_index, index, Prom::Rook);

                        moves.add(move1);
                        moves.add(move2);
                        moves.add(move3);
                        moves.add(move4);

                    } else {
                        let move1 = Move::new(pawn_index, index);
                        moves.add(move1);
                    }

                    pawn_moves &= pawn_moves.wrapping_sub(1);
                }

                pawns &= pawns.wrapping_sub(1);
            }

            let mut knights = self.array[$knight];
            while knights != 0 {
                let piece_index = knights.trailing_zeros() as usize;

                let mut knight_moves = KNIGHT_MOVES[piece_index] & !team;
                while knight_moves != 0 {
                    let index = knight_moves.trailing_zeros() as usize;

                    let move1 = Move::new(piece_index, index);
                    moves.add(move1);

                    knight_moves &= knight_moves.wrapping_sub(1);
                }

                knights &= knights.wrapping_sub(1);
            }

            let mut bishops = self.array[$bishop];
            while bishops != 0 {
                let piece_index = bishops.trailing_zeros() as usize;

                let mut bishop_moves = get_bishop_moves(piece_index, occupied) & !team;
                while bishop_moves != 0 {
                    let index = bishop_moves.trailing_zeros() as usize;

                    let move1 = Move::new(piece_index, index);
                    moves.add(move1);

                    bishop_moves &= bishop_moves.wrapping_sub(1);
                }

                bishops &= bishops.wrapping_sub(1);
            }

            let mut rooks = self.array[$rook];
            while rooks != 0 {
                let piece_index = rooks.trailing_zeros() as usize;

                let mut rook_moves = get_rook_moves(piece_index, occupied) & !team;
                while rook_moves != 0 {
                    let index = rook_moves.trailing_zeros() as usize;

                    let move1 = Move::new(piece_index, index);
                    moves.add(move1);

                    rook_moves &= rook_moves.wrapping_sub(1);
                }

                rooks &= rooks.wrapping_sub(1);
            }

            let mut queens = self.array[$queen];
            while queens != 0 {
                let piece_index = queens.trailing_zeros() as usize;

                let mut queen_moves = get_queen_moves(piece_index, occupied) & !team;
                while queen_moves != 0 {
                    let index = queen_moves.trailing_zeros() as usize;

                    let move1 = Move::new(piece_index, index);
                    moves.add(move1);

                    queen_moves &= queen_moves.wrapping_sub(1);
                }

                queens &= queens.wrapping_sub(1);
            }

            moves
        }
    };
}

macro_rules! gen_legal {
    (
        $name:ident,

        $king:ident,
        $pawn:ident,
        $knight:ident,
        $bishop:ident,
        $rook:ident,
        $queen:ident,

        $opp_pawn:ident,
        $opp_knight:ident,
        $opp_bishop:ident,
        $opp_rook:ident,
        $opp_queen:ident,

        $get_team:ident,
        $get_opps:ident,

        $kingside_rights:ident,
        $queenside_rights:ident,
        $kingside_squares:ident,
        $queenside_squares:ident,
        $kingside_to_index:ident,
        $queenside_to_index:ident,
        $kingside_attack_1:ident,
        $kingside_attack_2:ident,
        $queenside_attack_1:ident,
        $queenside_attack_2:ident,

        $pawn_attacks:ident,
        $pawn_steps:ident,
        $pawn_promotion:ident,

        $opp_pawn_attacks:ident,
        $ep_rank:ident,
        $rank_backward:ident,

        $square_attacked:ident,
    ) => {
        fn $name(&self) -> Moves {
            let mut moves = Moves::empty();

            let team = $get_team(&self.array);
            let opps = $get_opps(&self.array);
            let occupied = team | opps;

            let king = self.array[$king];
            let king_index = king.trailing_zeros() as usize;

            let info = self.array[INFO];
            let mut ep = info_to_enpassant(info);
            let ep_index = ep.trailing_zeros() as usize;
            let ep_piece = $rank_backward(ep);

            // The king cannot move to a square that has a team member
            let mut king_moves = KING_MOVES[king_index] & !team;

            // We determine the positions of the pieces that check the king
            // We use the sliding moves later to compute the check mask
            // We see the queen as a rook or as a bishop whenever applicable

            let knight_checks = KNIGHT_MOVES[king_index] & self.array[$opp_knight];
            let pawn_checks = $pawn_attacks[king_index] & self.array[$opp_pawn];

            let king_rook_moves = get_rook_moves(king_index, occupied);
            let rook_checks = king_rook_moves & (self.array[$opp_rook] | self.array[$opp_queen]);

            let king_bishop_moves = get_bishop_moves(king_index, occupied);
            let bishop_checks = king_bishop_moves & (self.array[$opp_bishop] | self.array[$opp_queen]);

            let checking_pieces = knight_checks | pawn_checks | rook_checks | bishop_checks;
            let check_count: u32 = checking_pieces.count_ones();

            // We add the king moves to the move list
            let occupied_except_king = occupied & !king;
            while king_moves != 0 {
                let index = king_moves.trailing_zeros() as usize;

                // We remove the king from the board, so that squares 'behind' the king are also checked, preventing backwards check evasion
                if !self.$square_attacked(index, occupied_except_king) {
                    let move1 = Move::new(king_index, index);
                    moves.add(move1);
                }

                king_moves &= king_moves.wrapping_sub(1);
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

            if rook_checks != 0 {
                let check_index = rook_checks.trailing_zeros() as usize;
                check_mask |= king_rook_moves & get_rook_moves(check_index, occupied)
            } else if bishop_checks != 0 {
                let check_index = bishop_checks.trailing_zeros() as usize;
                check_mask |= king_bishop_moves & get_bishop_moves(check_index, occupied)
            }

            // We allow pawns to resolve checks by doing enpassant
            let mut pawn_check_mask = check_mask;
            if check_mask & self.array[$opp_pawn] & ep_piece != 0 {
                pawn_check_mask |= ep;
            }

            // We will & the mask with the piece movements, so in case there is no check, we want this to do nothing
            if check_mask == 0 { check_mask = u64::MAX; pawn_check_mask = u64::MAX }

            // We compute the pins, we DO NOT consider enpassant edge cases here
            // We use a lookup table to loop through pins instead of pieces
            // This should be faster since pins are somewhat rare
            // This method works since a piece can be pinned by at most one attacker
            let mut pin_array = [u64::MAX; 64];

            // We only consider the opponents and the king as blockers
            let pin_occupied = opps | king;

            let king_rook_pins = get_rook_moves(king_index, pin_occupied);
            let king_bishop_pins = get_bishop_moves(king_index, pin_occupied);

            let mut rook_pins = king_rook_pins & (self.array[$opp_rook] | self.array[$opp_queen]);
            let mut bishop_pins = king_bishop_pins & (self.array[$opp_bishop] | self.array[$opp_queen]);

            // We check if there is exactly one team member between the king and an opponent rook or queen
            // If so, we add this to the list of pin masks, to later check with the team members
            while rook_pins != 0 {
                let index = rook_pins.trailing_zeros() as usize;

                let mask = get_rook_moves(index, pin_occupied) & king_rook_pins;
                if (mask & team).count_ones() == 1 {
                    let pin_mask = mask | (1u64 << index);
                    let mut pinned = pin_mask;
                    while pinned != 0 {
                        let pin_index = pinned.trailing_zeros() as usize;
                        pin_array[pin_index] = pin_mask;

                        pinned &= pinned.wrapping_sub(1);
                    }
                }

                rook_pins &= rook_pins.wrapping_sub(1);
            }

            // We check if there is exactly one team member between the king and an opponent bishop or queen
            // If so, we add this to the list of pin masks, to later check with the team members
            while bishop_pins != 0 {
                let index = bishop_pins.trailing_zeros() as usize;

                let mask = get_bishop_moves(index, pin_occupied) & king_bishop_pins;
                if (mask & team).count_ones() == 1 {
                    let pin_mask = mask | (1u64 << index);
                    let mut pinned = pin_mask;
                    while pinned != 0 {
                        let pin_index = pinned.trailing_zeros() as usize;
                        pin_array[pin_index] = pin_mask;

                        pinned &= pinned.wrapping_sub(1);
                    }
                }

                bishop_pins &= bishop_pins.wrapping_sub(1);
            }

            // For castling, we assume that the king and rooks are in the correct positions if the rights are set
            let kingside_rights = info & $kingside_rights != 0;
            let queenside_rights = info & $queenside_rights != 0;

            let kingside_free = occupied & $kingside_squares == 0;
            let queenside_free = occupied & $queenside_squares == 0;

            // We check if castling kingside is allowed and if there are no pieces in between
            if check_count == 0 && kingside_rights && kingside_free {

                // Checking if squares are under attack is relatively expensive
                if !self.$square_attacked($kingside_attack_1, occupied) {
                    if !self.$square_attacked($kingside_attack_2, occupied) {

                        // In case the squares are free and not under attack, we allow castling
                        let move1 = Move::new(king_index, $kingside_to_index);
                        moves.add(move1);
                    }
                }
            }

            // We check if castling queenside is allowed and if there are no pieces in between
            if check_count == 0 && queenside_rights && queenside_free {

                // Checking if squares are under attack is relatively expensive
                // Checking if squares are under attack is relatively expensive
                if !self.$square_attacked($queenside_attack_1, occupied) {
                    if !self.$square_attacked($queenside_attack_2, occupied) {

                        // In case the squares are free and not under attack, we allow castling
                        let move1 = Move::new(king_index, $queenside_to_index);
                        moves.add(move1);
                    }
                }
            }

            // We are now ready to generate moves for the other pieces
            // We start with the pawns, since their behavior is the most complicated
            // First, we have to determine whether we can enpassant or not

            // The following is to prevent the enpassant edge cases:
            //      (1) Pin by two pieces, e.g. 8/8/8/KpP4r/8/8/8/7k w - b6 0 13
            //      (2) Pin by opponent, e.g. 8/8/K7/1pP5/8/8/4b3/7k w - b6 0 1
            // We prevent these by setting ep = EMPTY if they occur
            // We start with some checks that fail almost all of the time
            if ep != 0 {

                // Case (1): For this case to occur the king must be on the fifth rank
                if king & $ep_rank != 0 {
                    let ep_attacker = $opp_pawn_attacks[ep_index] & self.array[$pawn];

                    // If there is zero or two pawns that can take the enpassant, this case does not occur
                    if ep_attacker.count_ones() == 1 {

                        // We compute the king moves, ignoring the two pieces, to check if there is a rook or a queen
                        let ep_occupied = occupied & !(ep_attacker | ep_piece);
                        let ep_pin_king_moves = get_rook_moves(king_index, ep_occupied) & $ep_rank;
                        let ep_pin_attacker = ep_pin_king_moves & (self.array[$opp_rook] | self.array[$opp_queen]);

                        // If there is zero attackers, this case does not occur
                        // If there is two attackers, the king must be in check, so enpassant would not be allowed anyway
                        if ep_pin_attacker.count_ones() == 1 {

                            // We compute the squares between the king and the attacker
                            // If there are two pieces in this mask, these must be the two pieces, so we prevent enpassant
                            let ep_mask = get_rook_moves(ep_pin_attacker.trailing_zeros() as usize, ep_occupied) & ep_pin_king_moves;
                            if (ep_mask & occupied).count_ones() == 2 {
                                ep = EMPTY;
                            }
                        }
                    }

                // Case (2)
                } else {

                    let ep_piece_index = ep_piece.trailing_zeros() as usize;
                    let ep_diag = BISHOP_MOVES[ep_piece_index] | ep_piece;

                    // For this case to occur the king must be on the same diagonal as the enpassant piece
                    if king & ep_diag != 0 {

                        let ep_attacker = $opp_pawn_attacks[ep_index] & self.array[$pawn];

                        // If there is no piece that can take the enpassant, this case cannot occur.
                        if ep_attacker.count_ones() != 0 {

                            let ep_occupied = occupied & !ep_piece;
                            let ep_pin_king_moves = get_bishop_moves(king_index, ep_occupied) & ep_diag;
                            let ep_pin_attacker = ep_pin_king_moves & (self.array[$opp_bishop] | self.array[$opp_queen]);

                            // If there is zero attackers, this case does not occur
                            // If there is two attackers, the king must be in check, so enpassant would not be allowed anyway
                            if ep_pin_attacker.count_ones() == 1 {

                                // We compute the squares between the king and the attacker
                                // If there is one piece in this mask, this must be the enpassant piece, so we prevent enpassant
                                let ep_mask = get_bishop_moves(ep_pin_attacker.trailing_zeros() as usize, ep_occupied) & ep_pin_king_moves;
                                if (ep_mask & occupied).count_ones() == 1 {
                                    ep = EMPTY;
                                }
                            }
                        }
                    }
                }
            }

            // We generate the moves for the pawns
            let mut pawns = self.array[$pawn];
            while pawns != 0 {
                let piece_index = pawns.trailing_zeros() as usize;

                let pawn_attacks = $pawn_attacks[piece_index] & (opps | ep);
                let pawn_steps = $pawn_steps(piece_index, occupied);
                let mut pawn_moves = (pawn_attacks | pawn_steps) & pawn_check_mask & pin_array[piece_index];

                while pawn_moves != 0 {
                    let index = pawn_moves.trailing_zeros() as usize;

                    if $pawn_promotion(piece_index) {
                        let move1 = Move::new_with_prom(piece_index, index, Prom::Queen);
                        let move2 = Move::new_with_prom(piece_index, index, Prom::Bishop);
                        let move3 = Move::new_with_prom(piece_index, index, Prom::Knight);
                        let move4 = Move::new_with_prom(piece_index, index, Prom::Rook);

                        moves.add(move1);
                        moves.add(move2);
                        moves.add(move3);
                        moves.add(move4);

                    } else {
                        let move1 = Move::new(piece_index, index);
                        moves.add(move1);
                    }

                    pawn_moves &= pawn_moves.wrapping_sub(1);
                }

                pawns &= pawns.wrapping_sub(1);
            }

            // We generate the moves for the knights
            let mut knights = self.array[$knight];
            while knights != 0 {
                let piece_index = knights.trailing_zeros() as usize;

                let mut knight_moves = KNIGHT_MOVES[piece_index] & !team & check_mask & pin_array[piece_index];

                while knight_moves != 0 {
                    let index = knight_moves.trailing_zeros() as usize;

                    let move1 = Move::new(piece_index, index);
                    moves.add(move1);

                    knight_moves &= knight_moves.wrapping_sub(1);
                }

                knights &= knights.wrapping_sub(1);
            }

            // We generate the moves for the bishops
            let mut bishops = self.array[$bishop];
            while bishops != 0 {
                let piece_index = bishops.trailing_zeros() as usize;

                let mut bishop_moves = get_bishop_moves(piece_index, occupied) & !team & check_mask & pin_array[piece_index];
                
                while bishop_moves != 0 {
                    let index = bishop_moves.trailing_zeros() as usize;

                    let move1 = Move::new(piece_index, index);
                    moves.add(move1);

                    bishop_moves &= bishop_moves.wrapping_sub(1);
                }

                bishops &= bishops.wrapping_sub(1);
            }

            // We generate the moves for the rooks
            let mut rooks = self.array[$rook];
            while rooks != 0 {
                let piece_index = rooks.trailing_zeros() as usize;

                let mut rook_moves = get_rook_moves(piece_index, occupied) & !team & check_mask & pin_array[piece_index];

                while rook_moves != 0 {
                    let index = rook_moves.trailing_zeros() as usize;

                    let move1 = Move::new(piece_index, index);
                    moves.add(move1);

                    rook_moves &= rook_moves.wrapping_sub(1);
                }

                rooks &= rooks.wrapping_sub(1);
            }

            // We generate the moves for the queens
            let mut queens = self.array[$queen];
            while queens != 0 {
                let piece_index = queens.trailing_zeros() as usize;

                let mut queen_moves = get_queen_moves(piece_index, occupied) & !team & check_mask & pin_array[piece_index];

                while queen_moves != 0 {
                    let index = queen_moves.trailing_zeros() as usize;

                    let move1 = Move::new(piece_index, index);
                    moves.add(move1);

                    queen_moves &= queen_moves.wrapping_sub(1);
                }

                queens &= queens.wrapping_sub(1);
            }

            moves
        }
    };
}

#[derive(Debug, Clone)]
pub struct Fen {
    pub array: FenArray,
}

impl Fen {
    pub fn new() -> Self {
        Self { array: DEFAULT_FEN_ARRAY }
    }

    pub fn from_str(fen_str: &str) -> Result<Self, String> {
        let result = string_to_fen(fen_str);

        match result {
            Ok(array) => Ok(Self { array }),
            Err(error) => Err(error),
        }
    }

    pub fn to_string(&self) -> String {
        fen_to_string(self.array)
    }

    pub fn print_board(&self) {

        let mut fen_str = fen_to_string(self.array);
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
            let king_index = self.array[KING_W].trailing_zeros() as usize;
            self.is_square_attacked_white(king_index, occupied)
        } else {
            let king_index = self.array[KING_B].trailing_zeros() as usize;
            self.is_square_attacked_black(king_index, occupied)
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

        let enpassant = info_to_enpassant(info);
        let halfmove = info_to_halfmove(info);

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
            new_halfmove = halfmove_to_info(halfmove + 1);
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

        let enpassant = info_to_enpassant(info);
        let halfmove = info_to_halfmove(info);
        let fullmove = info_to_fullmove(info);

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
        let new_fullmove = fullmove_to_info(fullmove + 1);

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
            new_halfmove = halfmove_to_info(halfmove + 1);
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

    #[inline(always)]
    fn is_square_attacked_white(&self, index: usize, occupied: u64) -> bool {
        // We let the square make the moves of each piece, to see if there is a piece that is attacking the square.
        // This is more efficient than calculating all the attacks, since there is only one square.
        // It is difficult to test is this is optimal, but it seems to be at least fast enough.

        let knight_attacks = KNIGHT_MOVES[index] & self.array[KNIGHT_B];
        let pawn_attacks = WHITE_PAWN_ATTACKS[index] & self.array[PAWN_B];
        let king_attacks = KING_MOVES[index] & self.array[KING_B];
        let rook_or_queen_attacks = get_rook_moves(index, occupied) & (self.array[ROOK_B] | self.array[QUEEN_B]);
        let bishop_or_queen_attacks = get_bishop_moves(index, occupied) & (self.array[BISHOP_B] | self.array[QUEEN_B]);

        knight_attacks | pawn_attacks | king_attacks | rook_or_queen_attacks | bishop_or_queen_attacks != 0
    }

    #[inline(always)]
    fn is_square_attacked_black(&self, index: usize, occupied: u64) -> bool {
        // We let the square make the moves of each piece, to see if there is a piece that is attacking the square.
        // This is more efficient than calculating all the attacks, since there is only one square.
        // It is difficult to test is this is optimal, but it seems to be at least fast enough.

        let knight_attacks = KNIGHT_MOVES[index] & self.array[KNIGHT_W];
        let pawn_attacks = BLACK_PAWN_ATTACKS[index] & self.array[PAWN_W];
        let king_attacks = KING_MOVES[index] & self.array[KING_W];
        let rook_or_queen_attacks = get_rook_moves(index, occupied) & (self.array[ROOK_W] | self.array[QUEEN_W]);
        let bishop_or_queen_attacks = get_bishop_moves(index, occupied) & (self.array[BISHOP_W] | self.array[QUEEN_W]);

        knight_attacks | pawn_attacks | king_attacks | rook_or_queen_attacks | bishop_or_queen_attacks != 0
    }

    gen_pseudo_legal!(
        get_pseudo_legal_moves_white,

        KING_W,
        PAWN_W,
        KNIGHT_W,
        BISHOP_W,
        ROOK_W,
        QUEEN_W,

        get_white_pieces,
        get_black_pieces,

        WHITE_KINGSIDE_RIGHTS,
        WHITE_QUEENSIDE_RIGHTS,
        WHITE_KINGSIDE_SQUARES,
        WHITE_QUEENSIDE_SQUARES,
        WHITE_KINGSIDE_MOVE_TO_INDEX,
        WHITE_QUEENSIDE_MOVE_TO_INDEX,

        WHITE_PAWN_ATTACKS,
        get_white_pawn_steps,
        white_pawn_promotion,
    );

    gen_pseudo_legal!(
        get_pseudo_legal_moves_black,

        KING_B,
        PAWN_B,
        KNIGHT_B,
        BISHOP_B,
        ROOK_B,
        QUEEN_B,

        get_black_pieces,
        get_white_pieces,

        BLACK_KINGSIDE_RIGHTS,
        BLACK_QUEENSIDE_RIGHTS,
        BLACK_KINGSIDE_SQUARES,
        BLACK_QUEENSIDE_SQUARES,
        BLACK_KINGSIDE_MOVE_TO_INDEX,
        BLACK_QUEENSIDE_MOVE_TO_INDEX,

        BLACK_PAWN_ATTACKS,
        get_black_pawn_steps,
        black_pawn_promotion,
    );

    gen_legal!(
        get_moves_white,

        KING_W,
        PAWN_W,
        KNIGHT_W,
        BISHOP_W,
        ROOK_W,
        QUEEN_W,

        PAWN_B,
        KNIGHT_B,
        BISHOP_B,
        ROOK_B,
        QUEEN_B,

        get_white_pieces,
        get_black_pieces,

        WHITE_KINGSIDE_RIGHTS,
        WHITE_QUEENSIDE_RIGHTS,
        WHITE_KINGSIDE_SQUARES,
        WHITE_QUEENSIDE_SQUARES,
        WHITE_KINGSIDE_MOVE_TO_INDEX,
        WHITE_QUEENSIDE_MOVE_TO_INDEX,
        WHITE_KINGSIDE_ATTACK_1,
        WHITE_KINGSIDE_ATTACK_2,
        WHITE_QUEENSIDE_ATTACK_1,
        WHITE_QUEENSIDE_ATTACK_2,

        WHITE_PAWN_ATTACKS,
        get_white_pawn_steps,
        white_pawn_promotion,

        BLACK_PAWN_ATTACKS,
        RANK_5,
        rank_down,

        is_square_attacked_white,
    );

    gen_legal!(
        get_moves_black,

        KING_B,
        PAWN_B,
        KNIGHT_B,
        BISHOP_B,
        ROOK_B,
        QUEEN_B,

        PAWN_W,
        KNIGHT_W,
        BISHOP_W,
        ROOK_W,
        QUEEN_W,

        get_black_pieces,
        get_white_pieces,

        BLACK_KINGSIDE_RIGHTS,
        BLACK_QUEENSIDE_RIGHTS,
        BLACK_KINGSIDE_SQUARES,
        BLACK_QUEENSIDE_SQUARES,
        BLACK_KINGSIDE_MOVE_TO_INDEX,
        BLACK_QUEENSIDE_MOVE_TO_INDEX,
        BLACK_KINGSIDE_ATTACK_1,
        BLACK_KINGSIDE_ATTACK_2,
        BLACK_QUEENSIDE_ATTACK_1,
        BLACK_QUEENSIDE_ATTACK_2,

        BLACK_PAWN_ATTACKS,
        get_black_pawn_steps,
        black_pawn_promotion,

        WHITE_PAWN_ATTACKS,
        RANK_4,
        rank_up,

        is_square_attacked_black,
    );
}