#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let command = input.trim();
        let tokens = tokenize(command);

        match tokens[0] {
            "exit" => process::exit(tokens[1].parse::<i32>().unwrap()),
            "echo" => println!("{}", tokens[1..].join(" ")),
            "type" => if tokens[1] == "exit" || tokens[1] == "echo" || tokens[1] == "type" {
                println!("{} is a shell builtin", tokens[1])
            } else {
                println!("{} not found", tokens[1])
            }
            _ => println!("{}: command not found", command),
        }
    }
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}