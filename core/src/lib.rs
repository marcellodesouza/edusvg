pub mod types;
pub mod svg_parser;
pub mod effects;
pub mod exporters;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_svg_wasm(svg_content: &str) -> String {
    match svg_parser::parse_svg(svg_content) {
        Ok(doc) => serde_json::to_string(&doc).unwrap_or_default(),
        Err(e) => format!("{{\"error\": \"{}\"}}", e),
    }
}

#[wasm_bindgen]
pub fn list_ids_wasm(svg_content: &str) -> String {
    match svg_parser::parse_svg(svg_content) {
        Ok(doc) => {
            let ids = svg_parser::list_ids(&doc);
            serde_json::to_string(&ids).unwrap_or_default()
        },
        Err(e) => format!("{{\"error\": \"{}\"}}", e),
    }
}

#[wasm_bindgen]
pub fn generate_animation_wasm(element_id: &str, effect_json: &str) -> String {
    let effect: types::Effect = match serde_json::from_str(effect_json) {
        Ok(e) => e,
        Err(e) => return format!("{{\"error\": \"{}\"}}", e),
    };

    let elem = types::SvgElement {
        id: element_id.to_string(),
        tag: "g".to_string(),
        x: None, y: None,
        width: None, height: None,
        children: vec![],
    };

    effects::generate_animation(&elem, &effect)
}
