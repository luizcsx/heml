use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("HEML CLI Engine v0.1.0");
        eprintln!("Usage: heml [command]\n");
        eprintln!("Commands:");
        eprintln!("  setup    Configure system directories and core files");
        eprintln!("  init     Initialize local project workspace");
        return;
    }

    match args[1].as_str() {
        "setup" => {
            println!("Status: Starting system-wide setup...");
            
            let base_path = "C:\\heml";
            let components_path = "C:\\heml\\components";

            // Create system directories
            if !Path::new(base_path).exists() {
                fs::create_dir_all(components_path).expect("Error: Failed to create system directories.");
                println!("Step 1/2: System directories created at {}", base_path);
            }

            // Install core component
            let core_file = format!("{}\\{}", base_path, "core.heml");
            fs::write(core_file, "\n<system>ready</system>")
                .expect("Error: Failed to install core components.");
            
            println!("Step 2/2: Core components installed successfully.");
            println!("Final Status: HEML is fully configured.");
        }
        "init" => {
            println!("Status: Initializing local project...");
            let config = r#"{
  "project": "heml_app",
  "version": "0.1.0"
}"#;
            fs::write("heml.config.json", config).expect("Error: Failed to create config file.");
            println!("Success: heml.config.json generated.");
        }
        _ => eprintln!("Error: Unknown command '{}'.", args[1]),
    }
}
