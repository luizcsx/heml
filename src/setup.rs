use std::fs;
use std::path::Path;

pub fn run_setup() {
    println!("\x1b[33m[SETUP] Provisioning System Infrastructure...\x1b[0m");

    let dirs = ["C:\\heml", "C:\\heml\\bin", "C:\\heml\\components"];
    for dir in dirs {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir).ok();
            println!("\x1b[32m[OK] Created Directory: {}\x1b[0m", dir);
        }
    }

    let welcome = "C:\\heml\\components\\welcome.heml";
    fs::write(welcome, "<center><h1>HEML KERNEL v0.1.0 ACTIVE</h1></center>").ok();
    println!("\x1b[32m[READY] Environment fully initialized.\x1b[0m");
}
