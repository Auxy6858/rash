use std::env;

pub fn get_current_directory() -> String {
    match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(_) => String::from("Failed to get current directory"),
    }
}
