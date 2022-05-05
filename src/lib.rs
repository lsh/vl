#![feature(let_chains)]

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

    pub data: Option<String>,
}

pub fn parse_encodings(s: &str) -> String {
    s.split(',')
        .map(|prop| {
            let mut tokens = prop.split(':').fuse();
            let k = tokens.next();
            let v = tokens.next();
            if let Some(k) = k && let Some(v) = v  {
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
                    "b" | "bin"=> format!("\"bin\": {v}"),
                    "u" | "timeUnit" => format!("\"timeUnit\": \"{v}\""),
                    _ => String::new(),
                }
            } else {
                String::new()
            }
        })
        .collect::<Vec<String>>()
        .join(",")
}

