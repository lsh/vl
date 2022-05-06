use clap::Parser;
pub mod gen;
use gen::Marks;

#[derive(Parser, Debug)]
pub struct Vl {
    #[clap(subcommand)]
    pub mark: Marks,

    #[clap(short = 'w', long = "width")]
    pub width: Option<usize>,
    #[clap(short = 'h', long = "height")]
    pub height: Option<usize>,
    #[clap(short = 'n', long = "name")]
    pub name: Option<String>,
    #[clap(short = 'd', long = "description")]
    pub description: Option<String>,
    #[clap(short = 't', long = "transform")]
    pub transform: Option<String>,
    #[clap(short = 'p', long = "parameter")]
    pub parameter: Option<String>,

    #[clap(short = 'c', long = "config")]
    pub config: Option<String>,

    #[clap(long = "bare")]
    pub bare: bool,

    pub data: Option<String>,
}

impl Marks {
    pub fn mark(&self) -> (String, String) {
        match self {
            Marks::Arc(_) => ("mark".to_string(), "\"arc\"".to_string()),
            Marks::Area(_) => ("mark".to_string(), "\"area\"".to_string()),
            Marks::Bar(_) => ("mark".to_string(), "\"bar\"".to_string()),
            Marks::Image(_) => ("mark".to_string(), "\"image\"".to_string()),
            Marks::Line(_) => ("mark".to_string(), "\"line\"".to_string()),
            Marks::Point(_) => ("mark".to_string(), "\"point\"".to_string()),
            Marks::Rect(_) => ("mark".to_string(), "\"rect\"".to_string()),
            Marks::Rule(_) => ("mark".to_string(), "\"rule\"".to_string()),
            Marks::Text(_) => ("mark".to_string(), "\"text\"".to_string()),
            Marks::Tick(_) => ("mark".to_string(), "\"tick\"".to_string()),
            Marks::Trail(_) => ("mark".to_string(), "\"trail\"".to_string()),
            Marks::Circle(_) => ("mark".to_string(), "\"circle\"".to_string()),
            Marks::Square(_) => ("mark".to_string(), "\"square\"".to_string()),
            Marks::Geoshape(_) => ("mark".to_string(), "\"geoshape\"".to_string()),
        }
    }

    pub fn encode(&self) -> (String, String) {
        match self {
            Marks::Arc(arc) => ("encoding".to_string(), arc.encode()),
            Marks::Area(area) => ("encoding".to_string(), area.encode()),
            Marks::Bar(bar) => ("encoding".to_string(), bar.encode()),
            Marks::Image(image) => ("encoding".to_string(), image.encode()),
            Marks::Line(line) => ("encoding".to_string(), line.encode()),
            Marks::Point(point) => ("encoding".to_string(), point.encode()),
            Marks::Rect(rect) => ("encoding".to_string(), rect.encode()),
            Marks::Rule(rule) => ("encoding".to_string(), rule.encode()),
            Marks::Text(text) => ("encoding".to_string(), text.encode()),
            Marks::Tick(tick) => ("encoding".to_string(), tick.encode()),
            Marks::Trail(trail) => ("encoding".to_string(), trail.encode()),
            Marks::Circle(circle) => ("encoding".to_string(), circle.encode()),
            Marks::Square(square) => ("encoding".to_string(), square.encode()),
            Marks::Geoshape(geoshape) => ("encoding".to_string(), geoshape.encode()),
        }
    }
}

pub fn parse_encodings(s: &str) -> String {
    if s.starts_with('{') {
        return s.to_string();
    }
    s.split(',')
        .map(|prop| {
            let mut tokens = prop.split(':').fuse();
            let k = tokens.next();
            let v = tokens.next();
            match (k, v) {
                (Some(k), Some(v)) => {
                    let k = k.to_lowercase();
                    match k.as_str() {
                        "f" | "field" => format!("\"field\": \"{v}\""),
                        "a" | "aggregate" => format!("\"aggregate\": \"{v}\""),
                        "t" | "type" => match v {
                            "q" | "Q" => "\"type\": \"quantitative\"".to_string(),
                            "t" | "T" => "\"type\": \"temporal\"".to_string(),
                            "o" | "O" => "\"type\": \"ordinal\"".to_string(),
                            "n" | "N" => "\"type\": \"nominal\"".to_string(),
                            "g" | "G" => "\"type\": \"geojson\"".to_string(),
                            _ => format!("\"type\": \"{v}\""),
                        },
                        "b" | "bin" => format!("\"bin\": {v}"),
                        "u" | "timeUnit" => format!("\"timeUnit\": \"{v}\""),
                        _ => format!("\"{k}\": {v}"),
                    }
                }
                _ => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(",")
}

pub fn parse_transforms(s: &str) -> String {
    if s.starts_with('{') {
        return s.to_string();
    }
    s.to_string()
}

pub fn parse_params(s: &str) -> String {
    if s.starts_with('{') {
        return s.to_string();
    }
    s.to_string()
}
