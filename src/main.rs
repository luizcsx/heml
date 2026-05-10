mod compiler;
mod setup;

use std::env;
use std::sync::mpsc::channel;
use notify::{Watcher, RecursiveMode, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "help" {
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
                println!("Error: Please specify a source file (e.g., heml build index.heml).");
            }
        },
        "watch" => {
            if args.len() >= 3 {
                run_watch(&args[2]);
            } else {
                println!("Error: Please specify a file to watch.");
            }
        },
        "uninstall" => {
            println!("To uninstall HEML, please delete the C:\\heml folder manually.");
        },
        "install" => {
            if args.len() >= 3 && args[2] == "--lvr" {
                println!("Checking for the latest version...");
            }
        },
        "--seqc2" => {
            println!("bah tss-ka bah-bah tss-k bop wika-wika bum tshhh bap bip bop-bop skrrr-pah");
        },
        _ => {
            println!("\nNOTICE: This command does not exist.");
            println!("Try using 'heml help' to display all available commands.\n");
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
    println!("██╗░░██╗███████╗███╗░░░███╗██╗░░░░░");
    println!("██║░░██║██╔════╝████╗░████║██║░░░░░");
    println!("███████║█████╗░░██╔████╔██║██║░░░░░");
    println!("██╔══██║██╔══╝░░██║╚██╔╝██║██║░░░░░");
    println!("██║░░██║███████╗██║░╚═╝░██║███████╗");
    println!("╚═╝░░╚═╝╚══════╝╚═╝░░░░░╚═╝╚══════╝");
    println!("-- HyperExtension Markup Language --\n");
    println!("Use the commands available below.");

    println!("\n--• FOR PROJECTS •--");
    println!("  heml build <file>  =  Compiles a .heml file into .html.");
    println!("  heml watch <file>  =  It monitors changes in real time and automatically recompiles them into .html.");

    println!("\n--• FOR HELP •--");
    println!("  heml help          =  Displays the bar with all available commands.");
    println!("  heml uninstall     =  Uninstall HEML from your computer.");
    println!("  heml install --lvr =  Install the latest version of HEML.");
    println!("\nFeel free to use it in your projects!");
}
