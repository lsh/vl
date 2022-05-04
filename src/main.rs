#![feature(let_chains)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use clap::Parser;
use serde_json::Value;
use std::io::{self, Write};
use vl::{gen::Marks, Vl};

pub fn main() {
    let cli = Vl::parse();
    let mut key_values = vec![(
        "$schema".to_string(),
        "\"https://vega.github.io/schema/vega-lite/v5.json\"".to_string(),
    )];
    match cli.mark {
        Marks::Arc(arc) => {
            key_values.push(("mark".to_string(), "\"arc\"".to_string()));
            key_values.push(("encoding".to_string(), arc.encode()));
        }
        Marks::Area(area) => {
            key_values.push(("mark".to_string(), "\"area\"".to_string()));
        }
        Marks::Bar(bar) => {
            key_values.push(("mark".to_string(), "\"bar\"".to_string()));
            key_values.push(("encoding".to_string(), bar.encode()));
        }
        Marks::Image(image) => {
            key_values.push(("mark".to_string(), "\"image\"".to_string()));
            key_values.push(("encoding".to_string(), image.encode()));
        }
        Marks::Line(line) => {
            key_values.push(("mark".to_string(), "\"line\"".to_string()));
            key_values.push(("encoding".to_string(), line.encode()));
        }
        Marks::Point(point) => {
            key_values.push(("mark".to_string(), "\"point\"".to_string()));
            key_values.push(("encoding".to_string(), point.encode()));
        }
        Marks::Rect(rect) => {
            key_values.push(("mark".to_string(), "\"rect\"".to_string()));
            key_values.push(("encoding".to_string(), rect.encode()));
        }
        Marks::Rule(rule) => {
            key_values.push(("mark".to_string(), "\"rule\"".to_string()));
            key_values.push(("encoding".to_string(), rule.encode()));
        }
        Marks::Text(text) => {
            key_values.push(("mark".to_string(), "\"text\"".to_string()));
            key_values.push(("encoding".to_string(), text.encode()));
        }
        Marks::Tick(tick) => {
            key_values.push(("mark".to_string(), "\"tick\"".to_string()));
            key_values.push(("encoding".to_string(), tick.encode()));
        }
        Marks::Trail(trail) => {
            key_values.push(("mark".to_string(), "\"trail\"".to_string()));
            key_values.push(("encoding".to_string(), trail.encode()));
        }
        Marks::Circle(circle) => {
            key_values.push(("mark".to_string(), "\"circle\"".to_string()));
            key_values.push(("encoding".to_string(), circle.encode()));
        }
        Marks::Square(square) => {
            key_values.push(("mark".to_string(), "\"square\"".to_string()));
            key_values.push(("encoding".to_string(), square.encode()));
        }
        Marks::Geoshape(geoshape) => {
            key_values.push(("mark".to_string(), "\"geoshape\"".to_string()));
            key_values.push(("encoding".to_string(), geoshape.encode()));
        }
    };
    if let Some(width) = cli.width {
        key_values.push(("width".to_string(), width.to_string()));
    }

    if let Some(height) = cli.height {
        key_values.push(("height".to_string(), height.to_string()));
    }

    if let Some(name) = cli.name {
        key_values.push(("name".to_string(), format!("\"{name}\"")));
    }

    if let Some(description) = cli.description {
        key_values.push(("description".to_string(), format!("\"{description}\"")));
    }

    if let Some(transform) = cli.transform {
        todo!() // need to write an input parser
                // key_values.push(("transform".to_string(), format!("{transform}")));
    }

    if let Some(param) = cli.parameter {
        todo!() // need to write an input parser
                // key_values.push(("parameter".to_string(), format!("{param}")));
    }

    let data_val: Result<Value, _> = if let Some(data) = cli.data {
        if (data.starts_with("\"") && data.ends_with("\""))
            || (data.starts_with("'") && data.ends_with("'"))
        {
            serde_json::from_str(&data)
        } else if data.ends_with(".json") {
            Ok(Value::String(data))
        } else {
            Ok(Value::Null)
        }
    } else {
        let mut lines_buffer = Vec::new();
        for line in std::io::stdin().lines() {
            if let Ok(line) = line {
                lines_buffer.push(line);
            }
        }
        let lines = lines_buffer.join("");
        serde_json::from_str(&lines)
    };

    if let Ok(data_val) = data_val {
        let data = data_val.to_string();
        match data_val {
            Value::Array(_) => {
                key_values.push(("data".to_string(), format!("{{ \"values\": {data}}}")))
            }
            Value::String(_) => {
                key_values.push(("data".to_string(), format!("{{ \"url\": {data}}}")))
            }
            Value::Object(_) => key_values.push(("data".to_string(), data)),
            _ => { /* do nothing */ }
        };
    } else {
        // key_values.push(("data".to_string(), format!("{{ \"url\": \"{}\"}}", data_val)))
    }

    let mut config = dirs::home_dir().expect("failed to get home dir");
    config.push(".config");
    config.push("vl");
    config.push("config.json");
    if let Some(config) = cli.config {
        key_values.push(("config".to_string(), config));
    } else if config.exists() {
        let conf = std::fs::read_to_string(config).expect("failed to read config file");
        key_values.push(("config".to_string(), conf));
    }
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let schema = format!(
        "{{{}}}",
        key_values
            .iter()
            .map(|(key, value)| format!("\"{key}\":{value}"))
            .collect::<Vec<String>>()
            .join(",")
    );
    handle.write_all(schema.as_bytes());
}
