use std::fs;
use regex::Regex;

pub fn run_build(source_file: &str) {
    let mut content = fs::read_to_string(source_file).expect("Error reading file");

    // --- QUICK MACROS ---
    if content.trim() == "!" {
        content = String::from("<!DOCTYPE html>\n<html>\n<head><title>HEML App</title></head>\n<body>\n\n</body>\n</html>");
    } else if content.trim() == "S" {
        content = String::from("<!DOCTYPE html>\n<html>\n<body>\n<center>\n<font color=\"cyan\"><h1>NEW PROJECT</h1></font>\n<marquee>Welcome to your new HEML site!</marquee>\n</center>\n</body>\n</html>");
    }

    let mut processed = content.clone();

    // Legacy Tags
    processed = processed.replace("<center>", "<div style=\"text-align: center;\">");
    processed = processed.replace("</center>", "</div>");

    let re_font_color = Regex::new(r#"<font color="([^"]+)">"#).unwrap();
    processed = re_font_color.replace_all(&processed, "<span style=\"color: $1;\">").into_owned();

    // Animations & CSS Runtime
    processed = processed.replace("<marquee>", "<div style=\"overflow: hidden; white-space: nowrap; animation: heml-scroll 10s linear infinite;\">");
    processed = processed.replace("</marquee>", "</div>");
    processed = processed.replace("<blink>", "<span style=\"animation: heml-blink 1s step-start infinite;\">");
    processed = processed.replace("</blink>", "</span>");

    let runtime_css = "<style>\n@keyframes heml-scroll { 0% { transform: translateX(100%); } 100% { transform: translateX(-100%); } }\n@keyframes heml-blink { 50% { opacity: 0; } }\nbody { margin: 0; padding: 20px; font-family: sans-serif; }\n</style>";
    processed.push_str(runtime_css);

    let output_file = source_file.replace(".heml", ".html");
    fs::write(&output_file, processed).expect("Error writing output");
    
    println!("\x1b[32m[BUILD] {} updated.\x1b[0m", output_file);
}
