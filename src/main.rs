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
                eprintln!("Erro: Especifique o arquivo (ex: heml build index.heml)");
                return;
            }
            run_build(&args[2]);
        },
        _ => print_help(),
    }
}

fn run_setup() {
    println!("Status: Inicializando Infraestrutura HEML...");

    // 1. Criando o esqueleto de pastas
    let dirs = ["C:\\heml", "C:\\heml\\bin", "C:\\heml\\components", "src"];
    for dir in dirs {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir).expect("Falha ao criar diretório");
            println!("Pasta criada: {}", dir);
        }
    }

    // 2. Gerando Componente de Boas-vindas (Exemplo do poder do motor)
    let welcome_comp = "C:\\heml\\components\\welcome.heml";
    let welcome_data = r#"<center>
    <font color="cyan" size="6">
        <h1>HEML SYSTEM READY</h1>
    </font>
    <marquee>Sua nova jornada na web retrô começou! Desenvolvido por Luiz Miguel.</marquee>
</center>"#;
    
    fs::write(welcome_comp, welcome_data).ok();
    println!("Final Status: Ambiente configurado com sucesso.");
}

fn run_build(source_file: &str) {
    println!("Status: Traduzindo código em {}...", source_file);

    let content = fs::read_to_string(source_file).expect("Erro ao ler o arquivo fonte");
    let mut processed = content.clone();

    // --- FASE 1: O TRADUTOR DE NOSTALGIA (LEGACY TAGS) ---
    
    // <center> -> Modern Div
    processed = processed.replace("<center>", "<div style=\"text-align: center;\">");
    processed = processed.replace("</center>", "</div>");

    // <font color="..."> -> Span Color
    let re_font_color = Regex::new(r#"<font color="([^"]+)">"#).unwrap();
    processed = re_font_color.replace_all(&processed, "<span style=\"color: $1;\">").into_owned();
    
    // <font size="..."> -> Escala Retro (1-7) para Pixels
    let re_font_size = Regex::new(r#"<font size="([^"]+)">"#).unwrap();
    processed = re_font_size.replace_all(&processed, |caps: &regex::Captures| {
        let size = match &caps[1] {
            "1" => "10px", "2" => "13px", "3" => "16px", "4" => "18px", "5" => "24px", "6" => "32px", "7" => "48px",
            _ => "16px",
        };
        format!("<span style=\"font-size: {};\">", size)
    }).into_owned();
    processed = processed.replace("</font>", "</span>");

    // <marquee> e <blink> -> CSS Runtime Animations
    processed = processed.replace("<marquee>", "<div style=\"overflow: hidden; white-space: nowrap; animation: heml-scroll 10s linear infinite;\">");
    processed = processed.replace("</marquee>", "</div>");
    processed = processed.replace("<blink>", "<span style=\"animation: heml-blink 1s step-start infinite;\">");
    processed = processed.replace("</blink>", "</span>");

    // Tags Diversas (u, strike, big)
    processed = processed.replace("<u>", "<span style=\"text-decoration: underline;\">");
    processed = processed.replace("</u>", "</span>");
    processed = processed.replace("<strike>", "<span style=\"text-decoration: line-through;\">");
    processed = processed.replace("</strike>", "</span>");
    processed = processed.replace("<big>", "<span style=\"font-size: larger;\">");
    processed = processed.replace("</big>", "</span>");

    // --- FASE 2: O ARQUITETO AUTOMÁTICO (GERADOR DE DIVS) ---
    // Aqui injetamos o CSS necessário para as animações funcionarem sem o usuário escrever CSS
    let runtime_css = r#"
<style>
@keyframes heml-scroll { 0% { transform: translateX(100%); } 100% { transform: translateX(-100%); } }
@keyframes heml-blink { 50% { opacity: 0; } }
body { margin: 0; padding: 0; } /* Reset básico automático */
</style>
"#;
    processed = format!("{}\n{}", processed, runtime_css);

    // --- FASE 3: O PLAGIADOR DE COMPONENTES (INCLUDES) ---
    let re_include = Regex::new(r#"he-include="([^"]+)""#).unwrap();
    let final_content = re_include.replace_all(&processed, |caps: &regex::Captures| {
        let path = &caps[1];
        // O fix das chaves {} que o GitHub avisou:
        fs::read_to_string(path).unwrap_or_else(|_| format!("", path))
    }).into_owned();

    // Salvando o resultado final
    let output_file = source_file.replace(".heml", ".html");
    fs::write(&output_file, final_content).expect("Erro ao salvar arquivo de saída");
    
    println!("Sucesso: O passado foi revivido em {}!", output_file);
}

fn print_help() {
    println!("--- HEML ENGINE v0.1.0 (The Legacy Protector) ---");
    println!("Uso: heml [comando] [arquivo]");
    println!("\nComandos:");
    println!("  setup    Cria as pastas do sistema e componentes base");
    println!("  build    Compila .heml (com tags antigas) para .html moderno");
}
