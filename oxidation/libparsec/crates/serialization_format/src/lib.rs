// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod parser;

use proc_macro::TokenStream;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use syn::{parse_macro_input, LitStr};

fn path_from_str(path: &str) -> PathBuf {
    let manifest_dir_path = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR should be set")
        .parse::<PathBuf>()
        .expect("CARGO_MANIFEST_DIR must be a valid path");
    manifest_dir_path.join(path)
}

fn content_from_file(path: &PathBuf) -> String {
    let file = File::open(path).expect("Cannot open the json file");
    let buf = BufReader::new(file);
    let mut content = String::new();
    for (i, line) in buf.lines().enumerate() {
        let line = line.unwrap_or_else(|_| panic!("Non-Utf-8 character found in line {i}"));
        let line = match line.split_once("//") {
            Some((line, _)) => line,
            None => &line,
        };
        content.push_str(line)
    }
    content
}

fn content_from_dir(path: &PathBuf) -> String {
    let dir = std::fs::read_dir(path).expect("Cannot read the directory");
    dir.filter_map(|entry| entry.ok())
        .map(|entry| content_from_file(&entry.path()))
        .collect::<Vec<_>>()
        .join(",")
}

#[proc_macro]
pub fn parsec_data(path: TokenStream) -> TokenStream {
    let path = parse_macro_input!(path as LitStr).value();
    let content = content_from_file(&path_from_str(&path));

    let data: parser::Data = miniserde::json::from_str(&content).expect("Data is not valid");
    TokenStream::from(data.quote())
}

#[proc_macro]
pub fn parsec_cmds(path: TokenStream) -> TokenStream {
    let path = parse_macro_input!(path as LitStr).value();
    let path = path_from_str(&path);
    let content = content_from_dir(&path);
    let dir_name = path
        .file_name()
        .expect("Couldn't convert the directory name to String")
        .to_str()
        .expect("Directory name contains non-utf-8 character");
    let content = format!(r#"{{"family": "{dir_name}", "cmds": [{content}]}}"#);

    let cmds: parser::Cmds = miniserde::json::from_str(&content).expect("Protocol is not valid");
    TokenStream::from(cmds.quote())
}
