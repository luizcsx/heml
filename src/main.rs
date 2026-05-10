use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

pub fn run_setup() {
    if Path::new("C:\\heml\\components\\header.heml").exists() {
        println!("\x1b[32m[INFO] HEML Environment is already installed and up to date.\x1b[0m");
        return;
    }

    println!("\x1b[36m[SYSTEM] Initializing Full Environment Deployment\x1b[0m\n");

    let installation_queue = [
        ("Directory", "C:\\heml", ""),
        ("Directory", "C:\\heml\\bin", ""),
        ("Directory", "C:\\heml\\components", ""),
        ("Source Folder", "src", ""),
        ("Helper", "C:\\heml\\bin\\run.bat", "@echo off\nheml build %1"),
        ("Component", "C:\\heml\\components\\header.heml", "<center><font color=\"cyan\" size=\"6\"><b>MY HEML SITE</b></font><hr></center>"),
        ("Component", "C:\\heml\\components\\button.heml", "<button style=\"background: #808080; border: 2px outset #ffffff; padding: 5px 10px; cursor: pointer;\">CLICK HERE</button>"),
        ("Component", "C:\\heml\\components\\footer.heml", "<hr><center><font size=\"2\">Built with HEML Engine v0.2.0</font></center>"),
        ("Theme", "C:\\heml\\components\\retro_bg.heml", "<style>body { background-color: #000080; color: white; }</style>"),
    ];

    let total = installation_queue.len();

    for (i, (kind, path, content)) in installation_queue.iter().enumerate() {
        let percent = ((i + 1) * 100) / total;
        let progress = (i + 1) * 20 / total;
        
        let bar = "#".repeat(progress);
        let spaces = " ".repeat(20 - progress);
        
        print!("\r\x1b[2K\x1b[33mExtracting {:<12}\x1b[0m [{}{}] {:3}% -> {}", 
               kind, bar, spaces, percent, path);
        io::stdout().flush().unwrap();

        if *kind == "Directory" || *kind == "Source Folder" {
            if !Path::new(path).exists() {
                fs::create_dir_all(path).ok();
            }
        } else {
            fs::write(path, content).ok();
        }

        thread::sleep(Duration::from_millis(350));
    }

    println!("\n\n\x1b[32m[SUCCESS] Environment is fully populated.\x1b[0m");
}
