use std::{env, ffi::CString};

pub fn get_current_directory() -> String {
    match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(_) => String::from("Failed to get current directory"),
    }
}

pub fn change_directory(path: &str) {
    let path_as_cstring = CString::new(path).expect("Conversion to CString failed");
    
    if unsafe {libc::chdir(path_as_cstring.as_ptr())} != 0 {
        eprintln!("Failed to change directory");
    }
}