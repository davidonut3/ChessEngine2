use std::process::{Command, Stdio};
use std::io::{Write, BufRead, BufReader};

pub const BESTMOVE: &str = "bestmove";
pub const NODES: &str = "Nodes";
pub const CHECKERS: &str = "Checkers";

pub const DEFAULT_NODES: u64 = 1000000;

pub fn fish_eval_in_centipawns(fen_str: &str, nodes_option: Option<u64>) -> i32 {
    let nodes = match nodes_option {
        Some(nodes) => nodes,
        None => DEFAULT_NODES
    };

    let pos_cmd: String = format!("position fen {}", fen_str);
    let node_cmd: String = format!("go nodes {}", nodes.to_string());

    let commands: Vec<&str> = vec![&pos_cmd, &node_cmd];
    let lines: Vec<String> = ask_the_fish(commands, BESTMOVE);

    let line = &lines[lines.len() - 2];
    let split: Vec<&str> = line.trim().split_whitespace().collect();
    let centipawns: i32 = split[9].parse().unwrap();

    centipawns
}

pub fn ask_the_fish(commands: Vec<&str>, stop_at: &str) -> Vec<String> {
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
        if line.starts_with(stop_at) {
            output.push(line);
            break;
        }
        output.push(line);
    }

    output
}