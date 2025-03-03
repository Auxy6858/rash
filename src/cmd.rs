use crate::VERSION_NUMBER;
use std::io::{self, Write};

// For now we will pretend that paths have \ as the path seperator
// even for unix/unix-like systems
pub fn cmd_mode() {
    println!("Entering CMD mode.");
    
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
    }
}