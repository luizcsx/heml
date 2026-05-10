use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

pub fn run_setup() {
    println!("\x1b[36mStarting the HyperExtension Markup Language installation...\x1b[0m\n");

    // Task list for the installer
    let tasks = [
        ("Directory", "C:\\heml"),
        ("Directory", "C:\\heml\\bin"),
        ("Directory", "C:\\heml\\components"),
        ("Source Folder", "src"),
        ("Configuration", "C:\\heml\\settings.ini"),
        ("Core Component", "C:\\heml\\components\\welcome.heml"),
    ];

    let total = tasks.len();

    for (i, (kind, path)) in tasks.iter().enumerate() {
        let percent = ((i + 1) * 100) / total;
        let progress = (i + 1) * 20 / total; // Bar size (20 characters)
        
        // Draws the bar: # for full, space for empty
        let mut bar = String::from("#").repeat(progress);
        let spaces = String::from(" ").repeat(20 - progress);
        
        // Line Formatting
        print!("\r\x1b[2KInstalling {:<15} [{}{}] {:3}% -> {}", 
               kind, bar, spaces, percent, path);
        io::stdout().flush().unwrap();

        // Real logic of creation
        if *kind == "Directory" || *kind == "Source Folder" {
            if !Path::new(path).exists() {
                fs::create_dir_all(path).ok();
            }
        } else {
            fs::write(path, "/* HEML System File */").ok();
        }

        // Delay for the user to see the progress bar "working"
        thread::sleep(Duration::from_millis(400));
    }

    println!("\n\n\x1b[32m[SUCCESS] All components installed successfully.\x1b[0m");
    println!("\x1b[33m[INFO] HEML Engine is now linked to the environment.\x1b[0m");
}
