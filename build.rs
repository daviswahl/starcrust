extern crate protoc_rust;

use protoc_rust::Customize;
use std::fs;
use std::fs::FileType;
use std::string::String;

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

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
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
}
