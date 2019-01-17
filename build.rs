extern crate protoc_rust;

use protoc_rust::Customize;
use std::fs;
use std::fs::FileType;
use std::string::String;


const OUT_DIR: &'static str = "src/protos";

fn protos() -> Vec<String> {
    let path = "extern/s2client-proto/s2clientprotocol/";
    fs::read_dir(path)
        .unwrap()
        .filter_map(|e| {
            e.ok()
                .and_then(|f| f.file_name().to_str().map(|s| s.to_string()))
        })
        .filter(|s| s.ends_with(".proto"))
        .map(|s| path.to_owned() + &s)
        .collect()
}

fn mod_contents() -> String {

    let contents: String =
    fs::read_dir(OUT_DIR).expect("output dir").filter_map(|e| {
        e.ok().and_then(|f| f.file_name().to_str().map(|s| s.to_string()))
    }).filter(|s| s.ends_with(".rs") && s != "mod.rs").map(|s| {
        "pub mod ".to_string() + &s.replace(".rs", ";\n")

    }).collect();
    contents
}

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: OUT_DIR,
        input: protos()
            .iter()
            .map(String::as_ref)
            .collect::<Vec<&str>>()
            .as_ref(),
        includes: &["extern/s2client-proto"],
        customize: Customize {
            ..Default::default()
        },
    })
    .expect("protoc");

    fs::write(OUT_DIR.to_owned() + "/mod.rs", mod_contents());
}
