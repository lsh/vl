#![feature(let_chains)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use clap::Parser;
use serde_json::Value;
use std::io::{self, Write};
use vl::{parse_params, parse_transforms, Vl};

pub fn main() {
    let cli = Vl::parse();
    let mut key_values = vec![(
        "$schema".to_string(),
        "\"https://vega.github.io/schema/vega-lite/v5.json\"".to_string(),
    )];

    key_values.push(cli.mark.mark());
    key_values.push(cli.mark.encode());

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
        key_values.push(("transform".to_string(), parse_transforms(&transform)));
    }

    if let Some(param) = cli.parameter {
        key_values.push(("parameter".to_string(), parse_params(&param)));
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
        if atty::is(atty::Stream::Stdin) {
            Ok(Value::Null)
        } else {
            let mut lines_buffer = Vec::new();
            for line in std::io::stdin().lines() {
                if let Ok(line) = line {
                    lines_buffer.push(line);
                }
            }
            let lines = lines_buffer.join("");
            serde_json::from_str(&lines)
        }
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
    }

    let mut config = dirs::home_dir().expect("failed to get home dir");
    config.push(".config");
    config.push("vl");
    config.push("config.json");
    if let Some(config) = cli.config {
        key_values.push(("config".to_string(), config));
    } else if config.exists() && !cli.bare {
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
