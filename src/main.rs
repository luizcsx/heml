mod compiler;
mod setup;

use std::env;
use std::sync::mpsc::channel;
use std::time::Duration;
use notify::{Watcher, RecursiveMode, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("HEML Engine v0.2.0\nCommands: setup, build [file], watch [file]");
        return;
    }

    match args[1].as_str() {
        "setup" => setup::run_setup(),
        "build" => {
            if args.len() >= 3 { compiler::run_build(&args[2]); }
        },
        "watch" => {
            if args.len() >= 3 { run_watch(&args[2]); }
        },
        _ => println!("Unknown command."),
    }
}

fn run_watch(file: &str) {
    println!("[WATCH] Monitoring {}... Press Ctrl+C to stop.", file);
    
    let (tx, rx) = channel();
    
    // Configuração correta do Watcher para a versão 6.1.1
    let mut watcher = notify::RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher.watch(std::path::Path::new("."), RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(_) => {
                compiler::run_build(file);
            },
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}
