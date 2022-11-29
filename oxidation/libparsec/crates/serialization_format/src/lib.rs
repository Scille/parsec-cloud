// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod old_parser;
pub(crate) mod protocol;
pub(crate) mod shared;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Context;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, LitStr};

use protocol::{intermediate, parser};

/// Procedural macro that take a directory or a file path.
///
/// If the `path` is a directory it **MUST** only contains json files.
///
/// If the `path` is a file it **MUST** be a json file.
#[proc_macro]
pub fn parsec_protocol(path: TokenStream) -> TokenStream {
    let pathname = parse_macro_input!(path as LitStr).value();
    let path = path_from_str(&pathname);
    let family = path
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .expect("Invalid filename");
    let collection = if path.is_dir() {
        let content = content_from_dir(&path).expect("Failed to get content from directory");
        parser::ProtocolCollection::with_protocols(family, content)
    } else {
        let content = content_from_file(&path).expect("Failed to get content from file");
        parser::ProtocolCollection::with_protocol(family, content)
    };
    let collection = intermediate::ProtocolCollection::from(collection);
    collection
        .quote()
        .expect("Got errors while quote the file/folder")
        .into_token_stream()
        .into()
}

pub(crate) fn path_from_str(path: &str) -> PathBuf {
    let manifest_dir_path: PathBuf = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR must be set")
        .parse()
        .expect("CARGO_MANIFEST_DIR must be a valid path");
    manifest_dir_path.join(path)
}

pub(crate) fn content_from_dir(path: &PathBuf) -> anyhow::Result<Vec<parser::Protocol>> {
    let dir = std::fs::read_dir(path)
        .map_err(anyhow::Error::new)
        .context(format!("Reading directory `{}`", path.to_string_lossy()))?;
    dir.filter_map(|entry| entry.ok())
        .map(|entry| content_from_file(&entry.path()))
        .collect()
}

pub(crate) fn content_from_file(path: &PathBuf) -> anyhow::Result<parser::Protocol> {
    let filename = path.to_string_lossy();
    let file = File::open(path)
        .map_err(anyhow::Error::new)
        .context("Opening file")?;
    let buf = BufReader::new(file);
    let mut content = String::new();
    for (i, line) in buf.lines().enumerate() {
        let line =
            line.unwrap_or_else(|_| panic!("{}:{} Non UTF-8 characters found", &filename, i));
        let line = line
            .split_once("//")
            .map(|(before, _after)| before)
            .unwrap_or(&line);
        content.push_str(line);
        content.push('\n');
    }
    content_from_str(&content, &filename)
}

pub(crate) fn content_from_str(
    content: &str,
    origin: &str,
) -> Result<parser::Protocol, anyhow::Error> {
    serde_json::from_str(content)
        .map_err(anyhow::Error::new)
        .context(format!("current file `{origin}`"))
}

#[proc_macro]
pub fn parsec_data(path: TokenStream) -> TokenStream {
    let path = parse_macro_input!(path as LitStr).value();
    let content = type_data::content_from_file(&path_from_str(&path));

    let data: old_parser::Data = miniserde::json::from_str(&content).expect("Data is not valid");
    TokenStream::from(data.quote())
}

mod type_data {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    pub fn content_from_file(path: &PathBuf) -> String {
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
}
