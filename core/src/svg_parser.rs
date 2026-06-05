use roxmltree::Document;
use crate::types::{SvgDocument, SvgElement};

pub fn parse_svg(svg_content: &str) -> Result<SvgDocument, String> {
    let doc = Document::parse(svg_content)
        .map_err(|e| format!("Erro ao parsear SVG: {}", e))?;

    let root = doc.root_element();

    let width = root.attribute("width")
        .and_then(|v| parse_dimension(v))
        .unwrap_or(680.0);

    let height = root.attribute("height")
        .and_then(|v| parse_dimension(v))
        .unwrap_or(520.0);

    let viewbox = root.attribute("viewBox")
        .unwrap_or("0 0 680 520")
        .to_string();

    let elements = extract_elements(&root);

    Ok(SvgDocument { width, height, viewbox, elements })
}

fn parse_dimension(val: &str) -> Option<f64> {
    val.trim_end_matches("px")
       .trim_end_matches("pt")
       .trim_end_matches("mm")
       .parse::<f64>().ok()
}

fn extract_elements(node: &roxmltree::Node) -> Vec<SvgElement> {
    let mut elements = Vec::new();

    for child in node.children() {
        if !child.is_element() { continue; }

        let tag = child.tag_name().name().to_string();

        if matches!(tag.as_str(),
            "defs" | "style" | "metadata" |
            "sodipodi:namedview" | "namedview"
        ) { continue; }

        let id = child.attribute("id")
            .unwrap_or("")
            .to_string();

        if id.is_empty() && tag != "g" { continue; }

        let elem = SvgElement {
            id: id.clone(),
            tag: tag.clone(),
            x: child.attribute("x").and_then(|v| v.parse().ok()),
            y: child.attribute("y").and_then(|v| v.parse().ok()),
            width: child.attribute("width")
                .and_then(|v| parse_dimension(v)),
            height: child.attribute("height")
                .and_then(|v| parse_dimension(v)),
            children: if tag == "g" {
                extract_elements(&child)
            } else {
                vec![]
            },
        };

        elements.push(elem);
    }

    elements
}

pub fn list_ids(doc: &SvgDocument) -> Vec<String> {
    let mut ids = Vec::new();
    collect_ids(&doc.elements, &mut ids);
    ids
}

fn collect_ids(elements: &[SvgElement], ids: &mut Vec<String>) {
    for elem in elements {
        if !elem.id.is_empty() {
            ids.push(format!("{} ({})", elem.id, elem.tag));
        }
        collect_ids(&elem.children, ids);
    }
}
