use std::fs;
use regex::Regex;

pub fn run_build(source_file: &str) {
    // ANSI Colors: \x1b[36m is Cyan, \x1b[0m is Reset
    println!("\x1b[36m[SYSTEM] Starting Transpilation Process...\x1b[0m");

    let content = fs::read_to_string(source_file).expect("FATAL: Source file access denied.");
    let mut processed = content.clone();

    // Mapping Legacy Elements to Modern Standards
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
        let mut s = String::from("<span style=\"font-size: ");
        s.push_str(size);
        s.push_str(";\">");
        s
    }).into_owned();
    processed = processed.replace("</font>", "</span>");

    // Dynamic UI Components (Marquee/Blink)
    processed = processed.replace("<marquee>", "<div style=\"overflow: hidden; white-space: nowrap; animation: heml-scroll 10s linear infinite;\">");
    processed = processed.replace("</marquee>", "</div>");
    processed = processed.replace("<blink>", "<span style=\"animation: heml-blink 1s step-start infinite;\">");
    processed = processed.replace("</blink>", "</span>");

    // Static Assets Injection
    let runtime_css = "<style>\n@keyframes heml-scroll { 0% { transform: translateX(100%); } 100% { transform: translateX(-100%); } }\n@keyframes heml-blink { 50% { opacity: 0; } }\nbody { margin: 0; padding: 0; font-family: sans-serif; }\n</style>";
    processed.push_str(runtime_css);

    // Component Integration Logic
    let re_include = Regex::new(r#"he-include="([^"]+)""#).unwrap();
    let final_content = re_include.replace_all(&processed, |caps: &regex::Captures| {
        let path = &caps[1];
        match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => String::from("")
        }
    }).into_owned();

    let output_file = source_file.replace(".heml", ".html");
    fs::write(&output_file, final_content).expect("FATAL: Failed to write output buffer.");
    
    println!("\x1b[32m[SUCCESS] Deployment ready: {}\x1b[0m", output_file);
}
