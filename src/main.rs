use std::io::{self, Write};
use cmd::cmd_mode;
use libc::{fork, execvp, waitpid, WUNTRACED};

mod cmd;

pub const VERSION_NUMBER: &str = "1.0";

fn main() {
    loop {
        print!("rash - {}> " , VERSION_NUMBER);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { 
            println!("\nError reading input");
            break;
        }

        let trimmed_input = input.trim();

        match trimmed_input {
            "cmd" => cmd_mode(),
            "exit" => break,
            "help" => println!("Available commands: exit, help, cd"),
            "hello" => println!("Hi! :D"),
            "whereami" => {
                let current_path = std::env::current_dir();
                println!("You are currently in the directory {}", current_path.display())
            },
            _ => println!("{} : Command not found.", trimmed_input)
        }
    }
}

