// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod data;
mod protocol;
mod types;
mod utils;

use proc_macro::TokenStream;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};
use syn::{parse_macro_input, LitStr};

fn path_from_str(path: &str) -> PathBuf {
    let manifest_dir_path: PathBuf = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR must be set")
        .parse()
        .expect("CARGO_MANIFEST_DIR must be a valid path");
    manifest_dir_path.join(path)
}

fn content_from_file(path: &PathBuf) -> String {
    let file = File::open(path).expect("Cannot open the json file");
    let buf = BufReader::new(file);
    let mut content = String::new();
    for (i, line) in buf.lines().enumerate() {
        let line = line.unwrap_or_else(|_| panic!("Non-Utf-8 character found in line {i}"));
        // Remove comments given JSON parser doesn't support them :(
        let line = match line.split_once("//") {
            Some((line, _)) => line,
            None => &line,
        };
        content.push_str(line)
    }
    content
}

fn retrieve_protocol_family_json_cmds(path: &Path) -> (String, Vec<protocol::JsonCmd>) {
    let family_name = path
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .expect("Invalid path, cannot determine protocol family name")
        .to_owned();

    let dir = std::fs::read_dir(path).expect("Cannot read directory");

    let mut json_cmds = vec![];
    for json_path in dir.filter_map(|entry| entry.ok()) {
        // Only keep .json/.json5 files
        let file_name = json_path.file_name();
        match std::path::Path::new(&file_name).extension() {
            Some(x) if x == "json5" || x == "json" => (),
            _ => continue,
        }

        let json_content = {
            let json_without_outer_struct = content_from_file(&json_path.path());
            // Hack around the fact Miniserde only supports struct as root ;-)
            format!("{{\"items\": {json_without_outer_struct} }}")
        };

        let json_cmd = match miniserde::json::from_str::<protocol::JsonCmd>(&json_content) {
            Ok(json_cmd) => json_cmd,
            Err(err) => {
                panic!("{:?}: JSON spec is not valid ({err:?})", json_path.path());
            }
        };
        json_cmds.push(json_cmd);
    }
    (family_name, json_cmds)
}

/// Generates protocol commands code from a *path* to a directory containing one
/// JSON file per protocol command.
#[proc_macro]
pub fn parsec_protocol_cmds_family(path: TokenStream) -> TokenStream {
    let path = parse_macro_input!(path as LitStr).value();
    let path = path_from_str(&path);
    let (family_name, json_cmds) = retrieve_protocol_family_json_cmds(&path);
    TokenStream::from(protocol::generate_protocol_cmds_family(
        json_cmds,
        &family_name,
    ))
}

/// Generates protocol commands bindings code from a *path* to a directory
/// containing one JSON file per protocol command.
#[cfg(feature = "python-bindings-support")]
#[proc_macro]
pub fn python_bindings_parsec_protocol_cmds_family(path: TokenStream) -> TokenStream {
    let path = parse_macro_input!(path as LitStr).value();
    let path = path_from_str(&path);
    let (family_name, json_cmds) = retrieve_protocol_family_json_cmds(&path);
    TokenStream::from(protocol::python_bindings::generate_protocol_cmds_family(
        json_cmds,
        &family_name,
    ))
}

/// Generates protocol commands code from JSON contents.
/// Useful for tests to avoid dealing with the filesystem.
#[proc_macro]
pub fn generate_protocol_cmds_family_from_contents(json_contents: TokenStream) -> TokenStream {
    let json_contents: Vec<String> = {
        let content = parse_macro_input!(json_contents as LitStr).value();
        // Consider empty line as a separator between json files
        content.split("\n\n").map(String::from).collect()
    };
    let family_name = "protocol";
    let mut json_cmds = vec![];
    for json_without_outer_struct in json_contents {
        // Hack around the fact Miniserde only supports struct as root ;-)
        let json_content = format!("{{\"items\": {json_without_outer_struct} }}");
        let json_cmd = miniserde::json::from_str::<protocol::JsonCmd>(&json_content)
            .expect("JSON spec is not valid");
        json_cmds.push(json_cmd);
    }
    TokenStream::from(protocol::generate_protocol_cmds_family(
        json_cmds,
        family_name,
    ))
}

/// Generates serialization code from a *path* to a single data type JSON schema.
#[proc_macro]
pub fn parsec_data(path: TokenStream) -> TokenStream {
    let path = parse_macro_input!(path as LitStr).value();
    let path = path_from_str(&path);
    let content = content_from_file(&path);
    let data: data::JsonData = miniserde::json::from_str(&content).expect("Data is not valid");
    TokenStream::from(data::generate_data(data))
}

/// Generates serialization code from the JSON contents of a single data type.
/// Useful for tests to avoid dealing with the filesystem.
#[proc_macro]
pub fn parsec_data_from_contents(json_contents: TokenStream) -> TokenStream {
    let content = parse_macro_input!(json_contents as LitStr).value();
    let data: data::JsonData = miniserde::json::from_str(&content).expect("Data is not valid");
    TokenStream::from(data::generate_data(data))
}
