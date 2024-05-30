#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};
use std::path::Path;
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
                let path_var = env::var("PATH").unwrap_or_default();
                let paths: Vec<&str> = path_var.split(':').collect();
                let mut found = false;
                for path in paths {
                    let mut full_path = Path::new(path).join(tokens[1]);
                    full_path.set_extension("");
                    if full_path.exists() {
                        println!("{} is {}", tokens[1], full_path.display());
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("{} not found", tokens[1]);
                }
            }
            _ => println!("{}: command not found", command),
        }
    }
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}