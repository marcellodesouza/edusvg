use crate::types::{Effect, SvgElement};

pub fn generate_animation(element: &SvgElement, effect: &Effect) -> String {
    match effect.tipo.as_str() {
        "organica" => generate_organica(element, effect),
        "pulsar"   => generate_pulsar(element, effect),
        "deriva"   => generate_deriva(element, effect),
        "impulso"  => generate_impulso(element, effect),
        "blur"     => generate_blur(element, effect),
        "aparecer" => generate_aparecer(element, effect),
        _          => String::new(),
    }
}

fn generate_organica(elem: &SvgElement, e: &Effect) -> String {
    let seed = e.seed.unwrap_or(4);
    let scale = e.intensidade;
    let dur = e.velocidade;
    let freq_min = 0.015 + (e.intensidade * 0.001);
    let freq_max = freq_min + 0.008;
    let filter_id = format!("org-{}", elem.id);
    format!(
        "<filter id=\"{fid}\" x=\"-20%\" y=\"-20%\" width=\"140%\" height=\"140%\">\n  <feTurbulence type=\"fractalNoise\" baseFrequency=\"{fmin:.4}\" numOctaves=\"3\" seed=\"{seed}\" result=\"n\">\n    <animate attributeName=\"baseFrequency\" values=\"{fmin:.4};{fmax:.4};{fmin:.4}\" dur=\"{dur}s\" repeatCount=\"indefinite\"/>\n  </feTurbulence>\n  <feDisplacementMap in=\"SourceGraphic\" in2=\"n\" scale=\"{scale}\"/>\n</filter>",
        fid=filter_id, fmin=freq_min, fmax=freq_max, seed=seed, dur=dur, scale=scale
    )
}

fn generate_pulsar(elem: &SvgElement, e: &Effect) -> String {
    let dur = e.velocidade;
    let amp = e.intensidade * 0.5;
    let neg = -amp;
    format!(
        "<animate xlink:href=\"#{id}\" attributeName=\"rx\" values=\"0;{ap:.1};0;{am:.1};0\" dur=\"{dur}s\" repeatCount=\"indefinite\" additive=\"sum\"/>\n<animate xlink:href=\"#{id}\" attributeName=\"ry\" values=\"0;{am:.1};0;{ap:.1};0\" dur=\"{dur}s\" repeatCount=\"indefinite\" additive=\"sum\"/>",
        id=elem.id, ap=amp, am=neg, dur=dur
    )
}

fn generate_deriva(elem: &SvgElement, e: &Effect) -> String {
    let dur = e.velocidade;
    let a = e.intensidade;
    let b = a * 0.6;
    let c = -a * 0.4;
    let d = a * 0.8;
    format!(
        "<animateTransform xlink:href=\"#{id}\" attributeName=\"transform\" type=\"translate\" values=\"0,0;{a:.1},{b:.1};{c:.1},{d:.1};0,0\" dur=\"{dur}s\" repeatCount=\"indefinite\" additive=\"sum\"/>",
        id=elem.id, a=a, b=b, c=c, d=d, dur=dur
    )
}

fn generate_impulso(_elem: &SvgElement, e: &Effect) -> String {
    let dur = e.velocidade;
    format!(
        "<circle r=\"6\" fill=\"#FFE070\" opacity=\"0\">\n  <animateMotion dur=\"{dur}s\" repeatCount=\"indefinite\" path=\"\"/>\n  <animate attributeName=\"opacity\" values=\"0;1;1;0.8;0\" dur=\"{dur}s\" repeatCount=\"indefinite\"/>\n</circle>",
        dur=dur
    )
}

fn generate_blur(_elem: &SvgElement, e: &Effect) -> String {
    let dur = e.velocidade;
    let min = e.intensidade * 0.3;
    let max = e.intensidade;
    format!(
        "<feGaussianBlur stdDeviation=\"{min:.1}\">\n  <animate attributeName=\"stdDeviation\" values=\"{min:.1};{max:.1};{min:.1}\" dur=\"{dur}s\" repeatCount=\"indefinite\"/>\n</feGaussianBlur>",
        dur=dur, min=min, max=max
    )
}

fn generate_aparecer(_elem: &SvgElement, e: &Effect) -> String {
    let dur = e.velocidade;
    format!(
        "<animate attributeName=\"opacity\" values=\"0;1;1;0\" dur=\"{dur}s\" repeatCount=\"indefinite\"/>",
        dur=dur
    )
}
