use std::{env, io::{self, Write}};
use cmd::cmd_mode;
use winapi::um::{processenv::GetStdHandle, wincon::{FillConsoleOutputAttribute, FillConsoleOutputCharacterA, GetConsoleScreenBufferInfo, SetConsoleCursorPosition, CONSOLE_SCREEN_BUFFER_INFO, COORD}};
use winapi::um::winnt::HANDLE;
use winapi::um::winbase::STD_OUTPUT_HANDLE;

mod cmd;
mod utils;

pub const VERSION_NUMBER: &str = "1.0";

fn main() {
    let mut username = whoami::username();
    let mut devicename = whoami::devicename();
    loop {
        if !username.is_empty() & !devicename.is_empty() {
            print!("\x1b[35m{username}\x1b[37m@\x1b[33m{devicename} \x1b[37m");
        }
        print!("rash - {VERSION_NUMBER}> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { 
            println!("\nError reading input");
            break;
        }

        let trimmed_input = input.trim();

        if trimmed_input.starts_with("echo") {
            if !input.contains('%') {
                println!("{}", input.strip_prefix("echo").unwrap().trim());
            }
        } else {
            match trimmed_input {
                "cmd" => cmd_mode(),
                "exit" => break,
                "help" => println!("Available commands: exit, help, cd"),
                "hello" => println!("Hi! :D"),
                "version" => println!("{VERSION_NUMBER}"),
                "clear" => {
                    if cfg!(target_os = "windows") {
                        unsafe {
                            let handle: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);
                            if handle.is_null() {
                                eprintln!("Unable to get standard handle");
                            }
                            let mut screen_buffer_info: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();
                            if GetConsoleScreenBufferInfo(handle, &mut screen_buffer_info) == 0 {
                                eprintln!("Unable to get console screen buffer info");
                            }
                            let screen_size: u32 = screen_buffer_info.dwSize.X as u32 * screen_buffer_info.dwSize.Y as u32;
                            let mut chars_written: u32 = 0;
                            let origin: COORD = COORD {X: 0, Y: 0};
    
                            FillConsoleOutputCharacterA(handle, ' ' as i8, screen_size, origin, &mut chars_written);
                            FillConsoleOutputAttribute(handle, screen_buffer_info.wAttributes, screen_size, origin, &mut chars_written);
                            SetConsoleCursorPosition(handle, origin);
                        }
                    } else {
                        print!("\x1B[2J\x1B[1;1H");
                        io::stdout().flush().unwrap();
                    }
                }
                "whereami" => {
                    println!("Your current directory is '{}'", utils::get_current_directory());
                },
                _ => println!("{} : Command not found.", trimmed_input)
            }
        }

        
    }
}

