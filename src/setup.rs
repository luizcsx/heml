use std::fs;
use std::path::Path;

pub fn run_setup() {
    println!("Status: Initializing HEML Infrastructure...");

    let dirs = ["C:\\heml", "C:\\heml\\bin", "C:\\heml\\components"];
    for dir in dirs {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir).expect("Failed to create directory");
            println!("Folder created: {}", dir);
        }
    }

    let welcome_comp = "C:\\heml\\components\\welcome.heml";
    let welcome_data = "<center><h1>HEML READY</h1></center>";
    
    fs::write(welcome_comp, welcome_data).ok();
    println!("Final Status: Environment is ready.");
}
