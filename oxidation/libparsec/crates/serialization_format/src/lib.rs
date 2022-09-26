// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub(crate) mod protocol;
#[cfg(test)]
mod tests;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, LitStr};

use protocol::{intermediate, parser};

/// Procedural macro that take a directory or a file path.
///
/// If the `path` is a directory it **MUST** only contain json files.
///
/// If the `path` is a file it **MUST** be a json file.
#[proc_macro]
pub fn parsec_protocol(path: TokenStream) -> TokenStream {
    let pathname = parse_macro_input!(path as LitStr).value();
    let path = path_from_str(&pathname);
    let collection = if path.is_dir() {
        let content = content_from_dir(&path)
            .unwrap_or_else(|_| panic!("Failed to get content from directory `{pathname}`",));
        parser::ProtocolCollection::with_protocols("foo", content)
    } else {
        let content = content_from_file(&path)
            .unwrap_or_else(|_| panic!("Failed to get content from file `{pathname}`"));
        parser::ProtocolCollection::with_protocol("foo", content)
    };
    let collection = intermediate::ProtocolCollection::from(collection);
    collection.quote().into_token_stream().into()
}

pub(crate) fn path_from_str(path: &str) -> PathBuf {
    let manifest_dir_path: PathBuf = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR should be set")
        .parse()
        .expect("CARGO_MANIFEST_DIR should be a valid path");
    manifest_dir_path.join(path)
}

pub(crate) fn content_from_dir(path: &PathBuf) -> anyhow::Result<Vec<parser::Protocol>> {
    let dir = std::fs::read_dir(path).expect("Cannot read the directory");
    dir.filter_map(|entry| entry.ok())
        .map(|entry| content_from_file(&entry.path()))
        .collect()
}

pub(crate) fn content_from_file(path: &PathBuf) -> anyhow::Result<parser::Protocol> {
    let filename = path.to_string_lossy();
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut content = String::new();
    for (i, line) in buf.lines().enumerate() {
        let line =
            line.unwrap_or_else(|_| panic!("{}:{} Non UTF-8 characters found", &filename, i));
        let line = line
            .split_once("//")
            .map(|(before, _after)| before)
            .unwrap_or(&line);
        content.push_str(line)
    }
    content_from_str(&content, &filename)
}

pub(crate) fn content_from_str(
    content: &str,
    origin: &str,
) -> Result<parser::Protocol, anyhow::Error> {
    serde_json::from_str(content)
        .map_err(|e| anyhow::Error::msg(e.to_string()).context(format!("current file `{origin}`")))
}

// #[proc_macro]
// pub fn parsec_data(path: TokenStream) -> TokenStream {
//     let path = parse_macro_input!(path as LitStr).value();
//     let content = content_from_file(&path_from_str(&path));

//     let data: parser::Data = miniserde::json::from_str(&content).expect("Data is not valid");
//     TokenStream::from(data.quote())
// }
