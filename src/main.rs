mod compiler;
mod setup;

use std::env;
use std::sync::mpsc::channel;
use std::time::Duration;
use notify::{Watcher, RecursiveMode, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "setup" => {
            setup::run_setup();
        },
        "build" => {
            if args.len() >= 3 {
                compiler::run_build(&args[2]);
            } else {
                println!("Error: Please specify a source file.");
            }
        },
        "watch" => {
            if args.len() >= 3 {
                run_watch(&args[2]);
            } else {
                println!("Error: Please specify a file to watch.");
            }
        },
        "--seqc2" => {
            println!("bah tss-ka bah-bah tss-k bop wika-wika bum tshhh bap bip bop-bop skrrr-pah");
        },
        _ => {
            println!("Unknown command. Use 'heml' without arguments to see help.");
        },
    }
}

fn run_watch(file: &str) {
    println!("Monitoring {} for changes. Press Ctrl+C to stop.", file);
    
    let (tx, rx) = channel();
    
    let mut watcher = notify::RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher.watch(std::path::Path::new("."), RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(_) => {
                compiler::run_build(file);
            },
            Err(e) => println!("Monitor error: {:?}", e),
        }
    }
}

fn print_help() {
    println!("HyperExtension Markup Language v0.2.0");
    println!("\nUsage:");
    println!("  heml setup           Install the environment and components");
    println!("  heml build <file>    Compile a .heml file to .html");
    println!("  heml watch <file>    Monitor changes and recompile automatically");
    println!("\nOptions:");
    println!("  --seqc2              Enable beatbox mode");
}
