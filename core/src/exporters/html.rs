use crate::types::{Project, SvgDocument, ElementConfig, Effect};

pub fn inject_filters_into_svg(svg_content: &str, elementos: &[ElementConfig], filters: &str) -> String {
    let mut svg = svg_content.to_string();

    if let Some(pos) = svg.find("</svg>") {
        let defs = format!("\n<defs>\n{}\n</defs>\n", filters);
        svg.insert_str(pos, &defs);
    }

    for elem in elementos {
        if let Some(effect) = &elem.animacao {
            if matches!(effect.tipo.as_str(), "organica" | "blur") {
                let filter_id = match effect.tipo.as_str() {
                    "organica" => format!("org-{}", elem.id),
                    "blur"     => format!("blr-{}", elem.id),
                    _          => format!("fx-{}", elem.id),
                };
                let pattern = format!("id=\"{}\"", elem.id);
                let replacement = format!("id=\"{}\" filter=\"url(#{})\"", elem.id, filter_id);
                if svg.contains(&pattern) {
                    svg = svg.replacen(&pattern, &replacement, 1);
                }
            }
        }
    }

    svg
}

pub fn export_html(
    svg_content: &str,
    _doc: &SvgDocument,
    project: &Project,
    filters: &str,
    runtime_js: &str,
) -> String {
    let svg_final = inject_filters_into_svg(svg_content, &project.elementos, filters);
    let config_json = serde_json::to_string(&project.elementos).unwrap_or_default();

    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"pt-BR\">\n");
    html.push_str("<head>\n");
    html.push_str("  <meta charset=\"UTF-8\">\n");
    html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str(&format!("  <title>{}</title>\n", project.nome));
    html.push_str("  <style>\n");
    html.push_str("    * { box-sizing: border-box; margin: 0; padding: 0; }\n");
    html.push_str("    body { display: flex; align-items: center; justify-content: center; min-height: 100vh; background: #f5f5f0; font-family: sans-serif; }\n");
    html.push_str("    .container { max-width: 800px; width: 100%; padding: 2rem; }\n");
    html.push_str("    h1 { font-size: 1.2rem; font-weight: 500; color: #333; margin-bottom: 1rem; text-align: center; }\n");
    html.push_str("    svg { width: 100%; height: auto; }\n");
    html.push_str("  </style>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str("  <div class=\"container\">\n");
    html.push_str(&format!("    <h1>{}</h1>\n", project.nome));
    html.push_str(&format!("    {}\n", svg_final));
    html.push_str(&format!("    <script id=\"edusvg-config\" type=\"application/json\">{}</script>\n", config_json));
    html.push_str(&format!("    <script>{}</script>\n", runtime_js));
    html.push_str("  </div>\n");
    html.push_str("</body>\n");
    html.push_str("</html>");
    html
}
