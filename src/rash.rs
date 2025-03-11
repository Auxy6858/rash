use std::{io::{self, Write}, path::Path, process::{Command, Stdio}};
use crate::VERSION_NUMBER;
use crate::cmd::cmd_mode;
use crate::utils::{change_directory, get_current_directory};


#[cfg(target_os = "windows")]
use winapi::um::{processenv::GetStdHandle, wincon::{FillConsoleOutputAttribute, FillConsoleOutputCharacterA, GetConsoleScreenBufferInfo, SetConsoleCursorPosition, CONSOLE_SCREEN_BUFFER_INFO, COORD}};
#[cfg(target_os = "windows")]
use winapi::um::winnt::HANDLE;
#[cfg(target_os = "windows")]
use winapi::um::winbase::STD_OUTPUT_HANDLE;

pub fn start_rash() {
    let mut username = whoami::username();
    let mut devicename = whoami::devicename();
    loop {
        if !username.is_empty() & !devicename.is_empty() {
            print!("\x1b[35m{username}\x1b[37m@\x1b[33m{devicename} \x1b[37m");
        }
        print!("Rash {VERSION_NUMBER}\n");
        print!("{}> ", get_current_directory());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { 
            println!("\nError reading input");
            break;
        }

        let trimmed_input = input.trim();

        if trimmed_input.starts_with("cd") {
            if trimmed_input == "cd" {
                clear_screen();
                println!("Not implemented yet");
            } else if trimmed_input == "cd .." {
                let mut new_directory = get_current_directory();
                if !new_directory.contains("/") {
                    println!("Error going into parent directory");
                }
                for i in (0..=new_directory.chars().count()).rev() {
                    if new_directory.chars().nth(i) == Some('/') {
                        change_directory(&new_directory);
                    } else {
                        new_directory = new_directory - new_directory.chars().nth(i).unwrap().to_string();
                    }
                }
            }
            if !trimmed_input.contains('/') {
                let folder = trimmed_input.strip_prefix("cd").unwrap().trim();
                let path_as_string = get_current_directory() + "/" + folder;
                let path = Path::new(&path_as_string);
                if path.exists() && path.is_dir() {
                    change_directory(&path_as_string);
                } else {
                    println!("Invalid directory {}", path_as_string);
                }
            }
        } else if trimmed_input.starts_with("./") {
            let exetutable_name = trimmed_input.strip_prefix("./");
            let process = Command::new(trimmed_input)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("failed to start");

        } else if trimmed_input.starts_with("echo") {
            if !input.contains('%') {
                println!("{}", input.strip_prefix("echo").unwrap().trim());
            }
        } else {
            match trimmed_input {
                "cmd" => cmd_mode(),
                "exit" => break,
                "help" => println!("Available commands: exit, help, cd"),
                "hello" => println!("Hi! :D"),
                "version" => println!("\x1b[33mRash\x1b[37m version {VERSION_NUMBER}"),
                "clear" => clear_screen(),
                "whereami" => {
                    println!("Your current directory is '{}'", utils::get_current_directory());
                },
                _ => println!("{} : Command not found.", trimmed_input)
            }
        }

        
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
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
