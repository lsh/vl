#![allow(unused_must_use)]

use convert_case::{Case, Casing};
use serde_json::Value;
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
        "use clap::{{Subcommand, Args}};\nuse crate::parse_encodings;"
    );
    let mut enumstr = String::from("#[derive(Subcommand, Debug)]\npub enum Marks {");
    for mark in marks {
        if let Value::String(mark) = mark {
            let cf = config[mark.as_str()].as_object().unwrap();
            let config_ref = cf["$ref"].as_str().unwrap();
            let conf = vdata.pointer(&config_ref[1..]).unwrap();
            let conf = conf.as_object().unwrap();
            let conf = conf["properties"].as_object().unwrap();
            let camel_mark = mark.to_case(Case::UpperCamel);
            let mut impl_str = String::from("impl ");
            impl_str.push_str(&camel_mark);
            impl_str.push_str(" {\n\tpub fn encode(&self) -> String {\n\t\tlet mut encodings: Vec<String> = Vec::new();\n");
            writeln!(f, "#[derive(Debug, Args)]\npub struct {camel_mark} {{");
            for (prop, _desc) in conf {
                let snake_prop = prop.to_case(Case::Snake);
                let kebab_prop = prop.to_case(Case::Kebab);
                writeln!(
                    f,
                    "\t#[clap(long=\"{kebab_prop}\")]\n\tpub {snake_prop}: Option<String>,"
                );
                let prop_test = format!(
                    "\t\tif let Some(encoding) = &self.{snake_prop} {{\n\t\t\tencodings.push(format!(\"\\\"{prop}\\\": {{{{{{}}}}}}\", parse_encodings(encoding)));\n\t\t}}\n"
                );
                impl_str.push_str(&prop_test);
            }
            writeln!(f, "}}");
            let s = format!("{camel_mark}({camel_mark}),\n");
            impl_str.push_str("\t\tformat!(\"{{{}}}\", encodings.join(\", \"))\n\t}\n}");
            writeln!(f, "{impl_str}");
            enumstr.push_str(&s);
        }
    }
    enumstr.push('}');
    writeln!(f, "{enumstr}");
    // panic!();
}
