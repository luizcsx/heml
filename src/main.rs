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
                eprintln!("Error: Please provide a source file. Example: heml build index.heml");
                return;
            }
            run_build(&args[2]);
        },
        _ => print_help(),
    }
}

fn run_setup() {
    println!("Status: Running system-wide setup...");
    let base_path = "C:\\heml";
    let comp_path = "C:\\heml\\components";
    
    fs::create_dir_all(comp_path).ok();
    
    let core_file = format!("{}\\{}", base_path, "core.heml");
    fs::write(core_file, "").ok();
    
    println!("Final Status: HEML is fully configured.");
}

fn run_build(source_file: &str) {
    println!("Status: Building {}...", source_file);

    // 1. Lê o arquivo .heml original
    let mut content = fs::read_to_string(source_file)
        .expect("Error: Could not read source file.");

    // 2. Lógica para 'he-include' (Simplificada via Regex)
    // Procura por: he-include="caminho/do/arquivo.heml"
    let re_include = Regex::new(r#"he-include="([^"]+)""#).unwrap();
    
    let mut final_content = content.clone();

    for cap in re_include.captures_iter(&content) {
        let path = &cap[1];
        println!("  > Including component: {}", path);
        
        if let Ok(component_html) = fs::read_to_string(path) {
            // Substitui a tag pelo conteúdo real do componente
            let pattern = format!(r#"<div he-include="{}"></div>"#, path);
            final_content = final_content.replace(&pattern, &component_html);
        } else {
            eprintln!("  ! Warning: Component {} not found.", path);
        }
    }

    // 3. Gera o arquivo .html final
    let output_file = source_file.replace(".heml", ".html");
    fs::write(&output_file, final_content).expect("Error: Could not write output file.");
    
    println!("Success: {} generated.", output_file);
}

fn print_help() {
    println!("HEML Engine v0.1.0\nUsage: heml [command]\n\nCommands:\n  setup    System configuration\n  build    Compile .heml to .html");
}
