use std::fs;
use regex::Regex;

pub fn run_build(source_file: &str) {
    println!("Status: Compiling {}...", source_file);

    let content = fs::read_to_string(source_file).expect("Error reading source file");
    let mut processed = content.clone();

    // Legacy Tags Translation
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

    // Auto-divs for animations
    processed = processed.replace("<marquee>", "<div style=\"overflow: hidden; white-space: nowrap; animation: heml-scroll 10s linear infinite;\">");
    processed = processed.replace("</marquee>", "</div>");
    processed = processed.replace("<blink>", "<span style=\"animation: heml-blink 1s step-start infinite;\">");
    processed = processed.replace("</blink>", "</span>");

    // Runtime CSS Injection
    let runtime_css = r#"<style>
@keyframes heml-scroll { 0% { transform: translateX(100%); } 100% { transform: translateX(-100%); } }
@keyframes heml-blink { 50% { opacity: 0; } }
body { margin: 0; padding: 0; }
</style>"#;
    processed = format!("{}\n{}", processed, runtime_css);

    // Component Includes
    let re_include = Regex::new(r#"he-include="([^"]+)""#).unwrap();
    let final_content = re_include.replace_all(&processed, |caps: &regex::Captures| {
        let path = &caps[1];
        fs::read_to_string(path).unwrap_or_else(|_| format!("", path))
    }).into_owned();

    let output_file = source_file.replace(".heml", ".html");
    fs::write(&output_file, final_content).expect("Error saving output file");
    
    println!("Success: {} generated!", output_file);
}
