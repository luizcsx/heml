mod compiler;
mod setup;

use std::env;
use std::path::Path;
use std::time::Duration;
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_hud();
        return;
    }

    match args[1].as_str() {
        "setup" => setup::run_setup(),
        "build" => {
            if args.len() < 3 { return; }
            compiler::run_build(&args[2]);
        },
        "watch" => {
            if args.len() < 3 { return; }
            run_watch(&args[2]);
        },
        _ => print_hud(),
    }
}

fn run_watch(file: &str) {
    println!("\x1b[33m[WATCH] Monitoring {}... (Press Ctrl+C to stop)\x1b[0m", file);
    
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(500)).unwrap();
    
    watcher.watch(Path::new("."), RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(_) => {
                compiler::run_build(file);
            },
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

fn print_hud() {
    println!("\x1b[36m");
    println!(r#"
    -•- HyperExtension Markup Language -•-
             version v0.2.0.
    "#);
    println!("\x1b[0m  COMMANDS:");
    println!("  setup           Initialize folders");
    println!("  build [file]    Single compilation");
    println!("  watch [file]    Real-time auto-build");
}
