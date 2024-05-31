#[allow(unused_imports)]
use std::{
    env,
    io::{self, Write},
    path::{Path, PathBuf},
    process,
};
use std::process::Command;

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
            unknown => {
                let path_var = env::var("PATH").unwrap_or_default();
                let paths: Vec<&str> = path_var.split(':').collect();
                let mut found = false;
                let args = &tokens[1..];
                for path in paths {
                    let mut full_path = Path::new(path).join(unknown);
                    full_path.set_extension("");
                    if full_path.exists() {
                        Command::new(full_path)
                        .args(args)
                        .status()
                        .expect("failed to execute process");
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("{}: command not found", tokens[0]);
                }
            }
        }
    }
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}