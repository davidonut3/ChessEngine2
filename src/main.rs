use std::io;

const HELP: &str = "help";
const QUIT: &str = "quit";
const FEN: &str = "fen";

const COMMAND_HELP: &str = "
    help \t Get this info.
    quit \t Quit the application.
    fen \t Get access to the engine.
";

fn main() {
    println!("Chess Engine Command Line Tool | Enter 'help' for help.");

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        user_input = user_input.trim().to_lowercase();

        if user_input == HELP {
            println!("{}", COMMAND_HELP)
        }

        if user_input == QUIT {
            break;
        }

        if user_input == FEN {
            continue;
        }
    }
}
