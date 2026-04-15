use std::process::{Command, Stdio};
use std::io::{Write, BufRead, BufReader};

pub fn ask_the_fish(commands: Vec<&str>) -> Vec<String> {
    let mut child = Command::new("./stockfish17.exe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start Stockfish");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();

    let mut output = Vec::new();

    for command in commands {
        writeln!(stdin, "{command}").unwrap();
    }

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.starts_with("Nodes") || line.starts_with("Checkers") {
            output.push(line);
            break;
        }
        output.push(line);
    }

    output
}