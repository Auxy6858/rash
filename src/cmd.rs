use crate::{utils, VERSION_NUMBER};
use std::io::{self, Write};
use std::collections::HashMap;
use std::env;

// For now we will pretend that paths have \ as the path seperator
// even for unix/unix-like systems
pub fn cmd_mode() {
    println!("Entering CMD mode.");
    let mut environment_variables: HashMap<String, String> = HashMap::new();
    environment_variables.insert(String::from("USERNAME"), whoami::username());

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
            if formatted_input == "set" {
                for (key, value) in &environment_variables {
                    println!("{key}={value}");
                }
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
                            println!("Variable to output {}", variable_to_output);
                            if environment_variables.contains_key(&variable_to_output) {
                                println!("{}", environment_variables.get(&variable_to_output).unwrap());
                            } else {
                                println!("{} is not defined.", variable_to_output)
                            }
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