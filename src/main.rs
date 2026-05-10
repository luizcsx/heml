mod compiler;
mod setup;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_hud();
        return;
    }

    match args[1].as_str() {
        "setup" => setup::run_setup(),
        "build" => {
            if args.len() < 3 {
                println!("\x1b[31m[ERROR] Source parameter missing. Expected: heml build <filename>\x1b[0m");
                return;
            }
            compiler::run_build(&args[2]);
        },
        _ => print_hud(),
    }
}

fn print_hud() {
    // Printing a Clean, Professional ASCII Header
    println!("\x1b[36m"); // Cyan Color
    println!(r#"
    #################################################
    #                                               #
    #   HEML ENGINE | CORE SYSTEM v0.1.0            #
    #   High-Performance Legacy Transpiler          #
    #                                               #
    #################################################
    "#);
    println!("\x1b[0m"); // Reset Color
    println!("  OPERATIONAL COMMANDS:");
    println!("  \x1b[33msetup\x1b[0m           Initialize system environment");
    println!("  \x1b[33mbuild [file]\x1b[0m    Compile HEML source to HTML");
    println!("\n  Status: Awaiting Command...");
}
