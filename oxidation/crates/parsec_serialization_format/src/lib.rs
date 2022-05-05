// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod parser;

use proc_macro::TokenStream;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use syn::{parse_macro_input, LitStr};

fn content(path: String) -> String {
    let manifest_dir_path = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR should be set")
        .parse::<PathBuf>()
        .expect("CARGO_MANIFEST_DIR must be a valid path");
    let file_path = manifest_dir_path.join(&path);
    let file = File::open(file_path).unwrap_or_else(|e| panic!("{e}"));
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

#[proc_macro]
pub fn parsec_cmds(path: TokenStream) -> TokenStream {
    let path = parse_macro_input!(path as LitStr).value();
    let content = content(path);

    let cmds: parser::Cmds =
        miniserde::json::from_str(&content).unwrap_or_else(|_| panic!("Protocol is not valid"));
    TokenStream::from(cmds.quote())
}

#[proc_macro]
pub fn parsec_data(path: TokenStream) -> TokenStream {
    let path = parse_macro_input!(path as LitStr).value();
    let content = content(path);

    let data: parser::Data =
        miniserde::json::from_str(&content).unwrap_or_else(|_| panic!("Data is not valid"));
    TokenStream::from(data.quote())
}
