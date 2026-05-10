use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

pub fn run_setup() {
    if Path::new("C:\\heml\\components\\header.heml").exists() {
        println!("HEML is already installed.");
        return;
    }

    println!("Initializing HEML setup...");

    let tasks = [
        ("folder", "C:\\heml"),
        ("folder", "C:\\heml\\bin"),
        ("folder", "C:\\heml\\components"),
        ("folder", "src"),
        ("file", "C:\\heml\\bin\\run.bat"),
        ("file", "C:\\heml\\components\\header.heml"),
        ("file", "C:\\heml\\components\\button.heml"),
        ("file", "C:\\heml\\components\\footer.heml"),
    ];

    let total = tasks.len();

    for (i, (kind, path)) in tasks.iter().enumerate() {
        let percent = ((i + 1) * 100) / total;
        let progress = (i + 1) * 20 / total;
        
        let bar = "#".repeat(progress);
        let spaces = " ".repeat(20 - progress);
        
        print!("\rExtracting {:<8} [{}{}] {:3}% - {}", 
               kind, bar, spaces, percent, path);
        io::stdout().flush().unwrap();

        if *kind == "folder" {
            if !Path::new(path).exists() {
                fs::create_dir_all(path).ok();
            }
        } else {
            fs::write(path, "").ok();
        }

        thread::sleep(Duration::from_millis(200));
    }

    println!("\nInstallation completed successfully.");
}
