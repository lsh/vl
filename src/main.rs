#![allow(unused_variables)]
#![allow(unused_must_use)]

use clap::Parser;
use vl::{gen::Marks, Vl};
use std::io::{self, Write};

pub fn main() {
    let cli = Vl::parse();
    let mut schema =
        String::from("{\"$schema\": \"https://vega.github.io/schema/vega-lite/v5.json\",");
    match cli.mark {
        Marks::Arc(arc) => {
            schema.push_str("\"mark\": \"arc\",");
        }
        Marks::Area(area) => {
            schema.push_str("\"mark\": \"area\",\"encoding\":");
            let s = area.encode();
            schema.push_str(&s);
        }
        Marks::Bar(bar) => {
            schema.push_str("\"mark\": \"bar\",\"encoding\":");
            let s = bar.encode();
            schema.push_str(&s);
        }
        Marks::Image(image) => {
            schema.push_str("\"mark\": \"image\",\"encoding\":");
            let s = image.encode();
            schema.push_str(&s);
        }
        Marks::Line(line) => {
            schema.push_str("\"mark\": \"line\",\"encoding\":");
            let s = line.encode();
            schema.push_str(&s);
        }
        Marks::Point(point) => {
            schema.push_str("\"mark\": \"point\",\"encoding\":");
            let s = point.encode();
            schema.push_str(&s);
        }
        Marks::Rect(rect) => {
            schema.push_str("\"mark\": \"rect\",\"encoding\":");
            let s = rect.encode();
            schema.push_str(&s);
        }
        Marks::Rule(rule) => {
            schema.push_str("\"mark\": \"rule\",\"encoding\":");
            let s = rule.encode();
            schema.push_str(&s);
        }
        Marks::Text(text) => {
            schema.push_str("\"mark\": \"text\",\"encoding\":");
            let s = text.encode();
            schema.push_str(&s);
        }
        Marks::Tick(tick) => {
            schema.push_str("\"mark\": \"tick\",\"encoding\":");
            let s = tick.encode();
            schema.push_str(&s);
        }
        Marks::Trail(trail) => {
            schema.push_str("\"mark\": \"trail\",\"encoding\":");
            let s = trail.encode();
            schema.push_str(&s);
        }
        Marks::Circle(circle) => {
            schema.push_str("\"mark\": \"circle\",\"encoding\":");
            let s = circle.encode();
            schema.push_str(&s);
        }
        Marks::Square(square) => {
            schema.push_str("\"mark\": \"square\",\"encoding\":");
            let s = square.encode();
            schema.push_str(&s);
        }
        Marks::Geoshape(geoshape) => {
            schema.push_str("\"mark\": \"geoshape\",\"encoding\":");
            let s = geoshape.encode();
            schema.push_str(&s);
        }
    };
    schema.push_str(",\"width\":");
    if let Some(width) = cli.width {
        schema.push_str(width.to_string().as_str());
    } else {
        schema.push_str(1600.to_string().as_str());
    }
    schema.push_str(",\"height\":");
    if let Some(height) = cli.height {
        schema.push_str(height.to_string().as_str());
    } else {
        schema.push_str(900.to_string().as_str());
    }
    schema.push_str(",\"data\": ");
    for line in io::stdin().lines() {
        if let Ok(line) = line {
            schema.push_str(&line);
        } else {
            panic!("Failed to read line!");
        }
    }
    let mut config = dirs::home_dir().expect("failed to get home dir");
    config.push(".config");
    config.push("vl");
    config.push("config.json");
    if config.exists() {
        let conf = std::fs::read_to_string(config).expect("failed to read config file");
        let f = format!(",\"config\": {conf}");
        schema.push_str(&f);
    }
    schema.push('}');
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(schema.as_bytes());
}
