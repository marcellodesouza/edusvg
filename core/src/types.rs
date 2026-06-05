use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SvgElement {
    pub id: String,
    pub tag: String,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub children: Vec<SvgElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SvgDocument {
    pub width: f64,
    pub height: f64,
    pub viewbox: String,
    pub elements: Vec<SvgElement>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Effect {
    pub tipo: String,
    pub intensidade: f64,
    pub velocidade: f64,
    pub seed: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interaction {
    pub tipo: String,
    pub zona_destino: Option<String>,
    pub tooltip: Option<String>,
    pub feedback_acerto: Option<String>,
    pub feedback_erro: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElementConfig {
    pub id: String,
    pub animacao: Option<Effect>,
    pub interacao: Option<Interaction>,
    pub visivel: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub nome: String,
    pub svg_path: String,
    pub elementos: Vec<ElementConfig>,
}
