use std::env;
use std::fs;
use std::path::Path;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "setup" => run_setup(),
        "build" => {
            if args.len() < 3 {
                eprintln!("Error: Specify a source file (e.g., heml build index.heml)");
                return;
            }
            run_build(&args[2]);
        },
        _ => print_help(),
    }
}

fn run_setup() {
    println!("Status: Initializing HEML Project Structure...");

    let dirs = ["C:\\heml", "C:\\heml\\bin", "C:\\heml\\components", "src"];
    for dir in dirs {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir).expect("Failed to create directory");
            println!("Created: {}", dir);
        }
    }

    let welcome_comp = "C:\\heml\\components\\welcome.heml";
    let welcome_data = r#"<center>
    <font color="gold">
        <h1>*** WELCOME TO THE RETRO ZONE ***</h1>
    </font>
    <marquee>HEML Engine v0.1.0 - Bringing back the 90s style with Rust power!</marquee>
</center>"#;
    
    fs::write(welcome_comp, welcome_data).ok();
    println!("Final Status: Environment ready. Place your .heml files in the project root.");
}

fn run_build(source_file: &str) {
    println!("Status: Reviving legacy code in {}...", source_file);

    let content = fs::read_to_string(source_file).expect("Error reading file");
    let mut processed = content.clone();
    
    processed = processed.replace("<center>", "<div style=\"text-align: center;\">");
    processed = processed.replace("</center>", "</div>");

    let re_font_color = Regex::new(r#"<font color="([^"]+)">"#).unwrap();
    processed = re_font_color.replace_all(&processed, "<span style=\"color: $1;\">").into_owned();
    
    let re_font_size = Regex::new(r#"<font size="([^"]+)">"#).unwrap();
    processed = re_font_size.replace_all(&processed, |caps: &regex::Captures| {
        let size = match &caps[1] {
            "1" => "10px", "2" => "13px", "3" => "16px", "4" => "18px", "5" => "24px", "6" => "32px", "7" => "48px",
            _ => "16px",
        };
        format!("<span style=\"font-size: {};\">", size)
    }).into_owned();
    processed = processed.replace("</font>", "</span>");

    processed = processed.replace("<marquee>", "<div style=\"overflow: hidden; white-space: nowrap; animation: scroll 10s linear infinite;\">");
    processed = processed.replace("</marquee>", "</div>");

    processed = processed.replace("<blink>", "<span style=\"animation: blink 1s step-start infinite;\">");
    processed = processed.replace("</blink>", "</span>");

    processed = processed.replace("<strike>", "<span style=\"text-decoration: line-through;\">");
    processed = processed.replace("</strike>", "</span>");
    processed = processed.replace("<big>", "<span style=\"font-size: larger;\">");
    processed = processed.replace("</big>", "</span>");
    processed = processed.replace("<u>", "<span style=\"text-decoration: underline;\">");
    processed = processed.replace("</u>", "</span>");

    let runtime_css = r#"
<style>
@keyframes scroll { 0% { transform: translateX(100%); } 100% { transform: translateX(-100%); } }
@keyframes blink { 50% { opacity: 0; } }
</style>
"#;
    processed = format!("{}\n{}", processed, runtime_css);

    let re_include = Regex::new(r#"he-include="([^"]+)""#).unwrap();
    let final_content = re_include.replace_all(&processed, |caps: &regex::Captures| {
        let path = &caps[1];
        fs::read_to_string(path).unwrap_or_else(|_| format!("", path))
    }).into_owned();

    let output_file = source_file.replace(".heml", ".html");
    fs::write(&output_file, final_content).expect("Failed to write output");
    
    println!("Success: Legacy code translated to modern HTML in {}!", output_file);
}

fn print_help() {
    println!("HyperExtension Markup Language v0.1.0.");
    println!("Usage: heml [command] [file]");
    println!("Commands:");
    println!("  setup    Build project folders and core components");
    println!("  build    Convert .heml (with legacy tags) to .html");
}
