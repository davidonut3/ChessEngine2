use crate::utils::*;
use crate::parsing;

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
}