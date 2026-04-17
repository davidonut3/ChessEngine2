// use crate::fish::ask_the_fish;
use crate::fen::Fen;
use crate::games::get_games;

/// Test if fen from string and fen to string are inverses
pub fn test_fen_string_conversion() {
    let games = get_games();

    for game in games {
        let fen = Fen::from_str(&game).unwrap();
        let fen_str = fen.to_string();

        assert_eq!(game, fen_str)
    }
}