use std::fs;
use regex::Regex;

pub fn run_build(source_file: &str) {
    let raw_content = fs::read_to_string(source_file).expect("Error: File not found");
    let input = raw_content.trim();

    // Shortcuts Logic
    let content = if input == "!" {
        String::from("<!DOCTYPE html>\n<html>\n<head><title>HEML App</title></head>\n<body>\n\n</body>\n</html>")
    } else if input == "S" {
        String::from("<!DOCTYPE html>\n<html>\n<body>\n<center>\n<font color=\"cyan\"><h1>NEW PROJECT</h1></font>\n<marquee>Welcome to HEML!</marquee>\n</center>\n</body>\n</html>")
    } else {
        raw_content
    };

    let mut processed = content.clone();

    // Legacy Tags
    processed = processed.replace("<center>", "<div style=\"text-align: center;\">");
    processed = processed.replace("</center>", "</div>");

    let re_font = Regex::new(r#"<font color="([^"]+)">"#).unwrap();
    processed = re_font.replace_all(&processed, "<span style=\"color: $1;\">").into_owned();
    processed = processed.replace("</font>", "</span>");

    // Animation Injection
    let runtime_css = "<style>\n@keyframes heml-scroll { 0% { transform: translateX(100%); } 100% { transform: translateX(-100%); } }\nbody { margin: 0; font-family: sans-serif; }\n</style>";
    processed.push_str(runtime_css);

    let output_file = source_file.replace(".heml", ".html");
    fs::write(&output_file, processed).expect("Error writing output");
    
    println!("[BUILD] Generated: {}", output_file);
}
