use crate::utils::*;
use crate::parsing::*;

#[derive(Debug, Clone)]
pub enum Prom { NoProm, Queen, Bishop, Knight, Rook, }

impl Prom {
    pub fn from_str(str: &str) -> Self {
        match str {
            "q" => Self::Queen,
            "b" => Self::Bishop,
            "n" => Self::Knight,
            "r" => Self::Rook,
            _ => Self::NoProm
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Queen => "q".to_string(),
            Self::Bishop => "b".to_string(),
            Self::Knight => "n".to_string(),
            Self::Rook => "r".to_string(),
            Self::NoProm => "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Move { pub move1: u16 }

impl Move {
    pub fn new(from: u64, to: u64) -> Self {
        let from_move: u16 = from.trailing_zeros() as u16;
        let to_move: u16 = (to.trailing_zeros() << 6) as u16;
        let move1 = from_move | to_move;

        Self { move1 }
    }

    pub fn new_with_prom(from: u64, to: u64, prom: Prom) -> Self {
        let from_move: u16 = from.trailing_zeros() as u16;
        let to_move: u16 = (to.trailing_zeros() << 6) as u16;
        
        let prom_move: u16 = match prom {
            Prom::Queen => 1u16 << 12,
            Prom::Bishop => 1u16 << 13,
            Prom::Knight => 1u16 << 14,
            Prom::Rook => 1u16 << 15,
            Prom::NoProm => 0,
        };

        let move1 = from_move | to_move | prom_move;

        Self { move1 }
    }

    pub fn empty() -> Self {
        Self { move1: 0u16 }
    }

    pub fn get_to(&self) -> u64 {
        let shift = (self.move1 & 0b0000111111000000) >> 6;
        1u64 << shift
    }

    pub fn get_from(&self) -> u64 {
        let shift = self.move1 & 0b0000000000111111;
        1u64 << shift
    }

    pub fn get_prom(&self) -> Prom {
        let prom_move = (self.move1 & 0b1111000000000000) >> 12;

        match prom_move {
            0b1 => Prom::Queen,
            0b10 => Prom::Bishop,
            0b100 => Prom::Knight,
            0b1000 => Prom::Rook,
            _ => Prom::NoProm,
        }
    }

    pub fn from_str(lan: &str) -> Result<Self, String> {
        let lower_lan = lan.to_lowercase();

        if lan.len() != 4 && lan.len() != 5 { return Err("Error: Move must have four or five characters".to_string()) }

        let from_square = &lower_lan[0..2];
        let to_square = &lower_lan[2..4];

        if !is_valid_lan(from_square) || !is_valid_lan(to_square) { return Err("Error: Move must have valid squares".to_string()) }

        let from = lan_to_bitboard(from_square);
        let to = lan_to_bitboard(to_square);

        if lan.len() == 4 { return Ok(Self::new(from, to)) }

        let allowed_proms = ["q", "b", "n", "r"];
        let prom_str = &lower_lan[4..5];

        if !allowed_proms.contains(&prom_str) { return Err("Error: Promotion flag must be 'q', 'b', 'n' or 'r'".to_string()) }

        let prom = Prom::from_str(prom_str);

        Ok(Self::new_with_prom(from, to, prom))
    }

    pub fn to_string(&self) -> String {
        let from = self.get_from();
        let to = self.get_to();
        let prom = self.get_prom();

        bitboard_to_lan(from) + &bitboard_to_lan(to) + &prom.to_string()
    }

}

pub type MoveArray = [Move; MAX_MOVES];

#[derive(Debug, Clone)]
pub struct Moves { pub array: MoveArray, pub size: usize }

impl Moves {
    pub fn new(array: MoveArray, size: usize) -> Self {
        Self { array, size }
    }

    pub fn empty() -> Self {
        let array = [Move::empty(); MAX_MOVES];
        let size = 0;

        Self { array, size }
    }

    pub fn to_string(&self) -> String {
        let mut result = "".to_string();

        for i in 0..self.size {
            let move1 = self.array[i];
            result += &move1.to_string();
            result += ", ";
        }

        result
    }

    pub fn add(&mut self, move1: Move) {
        self.array[self.size] = move1;
        self.size += 1;
    }
}

#[derive(Debug, Clone)]
pub struct PerftResult { pub moves: Moves, pub counts: [usize; MAX_MOVES], pub total: usize }

impl PerftResult {
    pub fn new(moves: Moves, counts: [usize; MAX_MOVES], total: usize) -> Self {
        Self { moves, counts, total }
    }

    pub fn empty() -> Self {
        let moves = Moves::empty();
        let counts = [0; MAX_MOVES];
        let total = 0;

        Self { moves, counts, total }
    }

    pub fn to_string(&self) -> String {
        let mut result = "".to_string();

        for i in 0..self.moves.size {
            let move1 = self.moves.array[i];
            let count = self.counts[i];
            result += &format!("{}: {}\n", move1.to_string(), count);
        }
        result += "Total: ";
        result += &self.total.to_string();
        
        result
    }
}