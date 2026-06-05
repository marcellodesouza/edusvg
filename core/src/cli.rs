use clap::{Parser, Subcommand};
use std::fs;

mod types;
mod svg_parser;
mod effects;
mod exporters;

use types::Project;

#[derive(Parser)]
#[command(name = "edusvg")]
#[command(version = "0.1.0")]
#[command(about = "EduSVG — ferramenta de autoria e animacao educacional")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Le um SVG e lista todas as camadas com ID
    Parse {
        svg: String,
    },
    /// Exporta SVG + config JSON como HTML animado
    Export {
        svg: String,
        config: String,
        #[arg(short, long, default_value = "output.html")]
        output: String,
        #[arg(long, default_value = "")]
        runtime: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { svg } => {
            let content = fs::read_to_string(&svg)
                .unwrap_or_else(|_| {
                    eprintln!("Erro: arquivo '{}' nao encontrado.", svg);
                    std::process::exit(1);
                });

            match svg_parser::parse_svg(&content) {
                Ok(doc) => {
                    println!("\n EduSVG Parser");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!(" Arquivo:    {}", svg);
                    println!(" Dimensoes:  {} x {}", doc.width, doc.height);
                    println!(" ViewBox:    {}", doc.viewbox);
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

                    let ids = svg_parser::list_ids(&doc);

                    if ids.is_empty() {
                        println!("\n Nenhum elemento com ID encontrado.");
                        println!(" Dica: nomeie as camadas no Inkscape");
                    } else {
                        println!("\n Camadas encontradas ({}):\n", ids.len());
                        for id in &ids {
                            println!("   -> {}", id);
                        }
                    }
                    println!();
                },
                Err(e) => eprintln!("Erro: {}", e),
            }
        },

        Commands::Export { svg, config, output, runtime } => {
            let svg_content = fs::read_to_string(&svg)
                .unwrap_or_else(|_| {
                    eprintln!("Erro: SVG '{}' nao encontrado.", svg);
                    std::process::exit(1);
                });

            let config_content = fs::read_to_string(&config)
                .unwrap_or_else(|_| {
                    eprintln!("Erro: config '{}' nao encontrado.", config);
                    std::process::exit(1);
                });

            let runtime_js = if runtime.is_empty() {
                // Tenta encontrar o animator.js relativo ao binario
                let candidates = [
                    "../runtime/animator.js",
                    "runtime/animator.js",
                    "../../runtime/animator.js",
                ];
                candidates.iter()
                    .find_map(|p| fs::read_to_string(p).ok())
                    .unwrap_or_default()
            } else {
                fs::read_to_string(&runtime)
                    .unwrap_or_else(|_| {
                        eprintln!("Aviso: runtime '{}' nao encontrado.", runtime);
                        String::new()
                    })
            };

            let doc = svg_parser::parse_svg(&svg_content)
                .expect("Erro ao parsear SVG");

            let project: Project = serde_json::from_str(&config_content)
                .expect("Erro ao parsear JSON de configuracao");

            println!("\n EduSVG Export");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!(" SVG:     {}", svg);
            println!(" Config:  {}", config);
            println!(" Output:  {}", output);
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            let ids = svg_parser::list_ids(&doc);
            let mut filters = String::new();
            let mut aplicados = 0;

            for elem_config in &project.elementos {
                if let Some(effect) = &elem_config.animacao {
                    if ids.iter().any(|id| id.starts_with(&elem_config.id)) {
                        let fake_elem = types::SvgElement {
                            id: elem_config.id.clone(),
                            tag: "g".to_string(),
                            x: None, y: None,
                            width: None, height: None,
                            children: vec![],
                        };
                        let anim = effects::generate_animation(&fake_elem, effect);
                        if !anim.is_empty() {
                            filters.push_str(&anim);
                            filters.push('\n');
                            aplicados += 1;
                            println!(" ok {} — efeito: {}", elem_config.id, effect.tipo);
                        }
                    } else {
                        println!(" aviso '{}' nao encontrado no SVG", elem_config.id);
                    }
                }
            }

            let html = exporters::html::export_html(
                &svg_content, &doc, &project, &filters, &runtime_js
            );

            fs::write(&output, html)
                .expect("Erro ao salvar HTML");

            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!(" {} efeito(s) aplicado(s)", aplicados);
            println!(" Exportado: {}", output);
            println!();
        },
    }
}
