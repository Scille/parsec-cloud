// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub(crate) mod config;
pub(crate) mod protocol;
pub(crate) mod shared;
pub(crate) mod ty;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Context;
use itertools::Itertools;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, LitStr};

use config::MacroConfig;
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

pub(crate) fn content_from_dir<T>(path: &PathBuf) -> anyhow::Result<Vec<T>>
where
    T: for<'a> serde::de::Deserialize<'a>,
{
    let dir = std::fs::read_dir(path)
        .map_err(anyhow::Error::new)
        .context(format!("Reading directory `{}`", path.to_string_lossy()))?;

    let (contents, errors): (Vec<_>, Vec<_>) = dir
        .filter_map(|entry| entry.ok())
        .map(|entry| content_from_file::<T>(&entry.path()))
        .partition_result();

    anyhow::ensure!(
        errors.is_empty(),
        "Failed to serialize directory:\n{}",
        errors.iter().map(|e| e.to_string()).join(",\n")
    );
    Ok(contents)
}

pub(crate) fn content_from_file<T>(path: &PathBuf) -> anyhow::Result<T>
where
    T: for<'a> serde::de::Deserialize<'a>,
{
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

pub(crate) fn content_from_str<T>(content: &str, origin: &str) -> anyhow::Result<T>
where
    T: for<'a> serde::de::Deserialize<'a>,
{
    serde_json::from_str(content)
        .map_err(anyhow::Error::new)
        .context(format!("current file `{origin}`"))
}

/// Procedural macro that take a json file path.
/// The json file can contain comment.
#[proc_macro]
pub fn parsec_data(path: TokenStream) -> TokenStream {
    let config = parse_macro_input!(path as MacroConfig);
    let path = path_from_str(&config.raw_path);
    let collection = if path.is_dir() {
        let data: Vec<ty::parser::VersionedType> =
            content_from_dir(&path).expect("Failed to parse data");
        ty::intermediate::TypeCollection::from(data)
    } else {
        let data: ty::parser::VersionedType =
            content_from_file(&path).expect("Failed to parse data");
        ty::intermediate::TypeCollection::from(data)
    };
    let res = collection.quote(&config.crates_override);
    match res {
        Ok(mods) => quote::quote! {
            #(#mods)*
        }
        .into_token_stream()
        .into(),
        Err(e) => {
            panic!(
                "Got error when quoting `{}`:\n{}\nConfig: {:?}",
                config.raw_path, e, config
            );
        }
    }
}
