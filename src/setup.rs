use std::fs;
use std::path::Path;

pub fn run_setup() {
    let dirs = ["C:\\heml", "C:\\heml\\bin", "C:\\heml\\components"];
    for dir in dirs {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir).ok();
            println!("[SETUP] Created: {}", dir);
        }
    }
    println!("[SUCCESS] Environment ready.");
}
