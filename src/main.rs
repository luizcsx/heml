mod compiler;
mod setup;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "setup" => setup::run_setup(),
        "build" => {
            if args.len() < 3 {
                eprintln!("Error: Missing filename. Usage: heml build index.heml");
                return;
            }
            compiler::run_build(&args[2]);
        },
        _ => print_help(),
    }
}

fn print_help() {
    println!("--- HEML ENGINE v0.1.0 ---");
    println!("Usage: heml [command] [file]");
    println!("\nCommands:");
    println!("  setup    Initialize project structure");
    println!("  build    Compile .heml to .html");
}
