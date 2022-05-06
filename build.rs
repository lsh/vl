#![allow(unused_must_use)]

use convert_case::{Case, Casing};
use serde_json::Value;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

pub fn main() {
    println!(
        "cargo:rerun-if-changed=build.rs
    cargo:rerun-if-changed=v5.json",
    );
    let file = File::open("v5.json").expect("Failed to find schema.");
    let reader = BufReader::new(file);
    let dest_path = Path::new(&"src").join("gen.rs");
    let mut f = std::fs::File::create(dest_path).expect("Failed to create destination file.");

    let vdata: Value = serde_json::from_reader(reader).expect("Failed to parse schema.");
    let data = vdata.as_object().unwrap();
    let defs = data["definitions"].as_object().unwrap();
    let marks = defs["Mark"].as_object().unwrap();
    let marks = marks["enum"].as_array().unwrap();
    let config = defs["Config"].as_object().unwrap();
    let config = config["properties"].as_object().unwrap();
    writeln!(
        f,
        "use clap::{{Subcommand, Args}}; use crate::parse_encodings;"
    );
    let mut enumstr = String::from("#[derive(Subcommand, Debug)] pub enum Marks {");
    let mut configs: HashSet<&str> = HashSet::new();
    for mark in marks {
        if let Value::String(mark) = mark {
            let cf = config[mark.as_str()].as_object().unwrap();
            let config_ref = cf["$ref"].as_str().unwrap();
            let config_name = config_ref.split('/').last().unwrap();
            if !configs.contains(config_name) {
                let conf = vdata.pointer(&config_ref[1..]).unwrap();
                let conf = conf.as_object().unwrap();
                let conf = conf["properties"].as_object().unwrap();
                writeln!(f, "#[derive(Args, Debug)] pub struct {config_name} {{");
                let mut impl_str = format!("impl {config_name} {{ pub fn encode(&self) -> String {{ let mut encodings: Vec<String> = Vec::new();");
                for (prop, _desc) in conf {
                    let snake_prop = prop.to_case(Case::Snake);
                    let kebab_prop = prop.to_case(Case::Kebab);
                    writeln!(
                        f,
                        "#[clap(long=\"{kebab_prop}\")] pub {snake_prop}: Option<String>,"
                    );
                    let prop_test = format!(
                        "if let Some(encoding) = &self.{snake_prop} {{ encodings.push(format!(\"\\\"{prop}\\\": {{{{{{}}}}}}\", parse_encodings(encoding))); }}\n\n"
                    );
                    impl_str.push_str(&prop_test);
                }
                writeln!(f, "}}");
                impl_str.push_str("format!(\"{{{}}}\", encodings.join(\", \")) }}");
                writeln!(f, "{impl_str}");
                configs.insert(config_name);
            };
            let camel_mark = mark.to_case(Case::UpperCamel);
            writeln!(
                f,
                "#[derive(Args, Debug)] pub struct {camel_mark} {{ #[clap(flatten)] config: {config_name} }}"
            );
            writeln!(
                f,
                "impl {camel_mark} {{pub fn encode(&self) -> String{{ self.config.encode() }}}}"
            );
            let s = format!("{camel_mark}({camel_mark}),\n");
            enumstr.push_str(&s);
        }
    }
    enumstr.push('}');
    writeln!(f, "{enumstr}");
}
