use crate::{utils, VERSION_NUMBER};
use std::io::{self, Write};
use std::collections::HashMap;

// For now we will pretend that paths have \ as the path seperator
// even for unix/unix-like systems
pub fn cmd_mode() {
    println!("Entering CMD mode.");
    let mut environment_variables: HashMap<String, String> = HashMap::new();
    
    
    if cfg!(target_os = "windows"){
        println!("Running on Windows, if a command is not implemented, rash will fallback to using cmd.exe");
    } else {
        println!("Not running on Windows, if a command is not implemented rash will panic and terminate with exit code 91")
    }
    loop {
        print!("rash - {} - cmd> ", VERSION_NUMBER);
        io::stdout().flush().expect("Failed to flush stdout");
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("\nError reading input. Exiting.");
            break;
        }
        let formatted_input = input.trim().to_lowercase();


        if formatted_input.starts_with("set") {
            let mut i = 2;
            let mut setting_variable_value: bool = false;
            let mut i = 2;
            let split_input: Vec<&str> = formatted_input.split('=').collect();

            if split_input.len() != 2 {
                println!("Invalid characters in variable name or value for {}", input);
            } else {
                environment_variables.insert(split_input.get(0).unwrap().to_string(), split_input.get(1).unwrap().to_string());
            }
        } else if formatted_input.starts_with("echo"){
            if !input.contains('%') {
                println!("{}", input.strip_prefix("echo").unwrap().trim());
            } else {
                for i in 4..input.chars().count() {
                    match input.chars().nth(i) {
                        Some('%') => {
                            let mut variable_to_output = String::new();
                            while input.chars().nth(i) != Some('%') {
                                variable_to_output += &input.chars().nth(i).unwrap().to_string();
                            }
                            println!("{}", environment_variables.get(&variable_to_output).unwrap())
                        },
                        _ => {}
                    }
                }
            }
        } else {
            match  formatted_input.as_str(){
                "exit" => {
                    println!("Exiting cmd mode");
                    break;
                },
                _ => println!("'{}' is not recognised as an internal or external command, operable program or batch file.", input.trim())
            }
        }
        
    }
}