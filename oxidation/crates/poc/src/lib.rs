// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod parser;

use proc_macro::TokenStream;
use std::fs::File;
use std::io::Read;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn parsec_schema(path: TokenStream) -> TokenStream {
    let path = parse_macro_input!(path as LitStr);
    let file_path = std::env::current_dir().unwrap().join(&path.value());
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let schema: parser::Schema =
        miniserde::json::from_str(&content).unwrap_or_else(|_| panic!("Schema is not valid"));
    TokenStream::from(schema.quote())
}

#[proc_macro]
pub fn parsec_cmds(path: TokenStream) -> TokenStream {
    let path = parse_macro_input!(path as LitStr);
    let file_path = std::env::current_dir().unwrap().join(&path.value());
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let cmds: parser::Cmds =
        miniserde::json::from_str(&content).unwrap_or_else(|_| panic!("Protocol is not valid"));
    TokenStream::from(cmds.quote())
}

#[proc_macro]
pub fn parsec_data(path: TokenStream) -> TokenStream {
    let path = parse_macro_input!(path as LitStr);
    let file_path = std::env::current_dir().unwrap().join(&path.value());
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let data: parser::Data =
        miniserde::json::from_str(&content).unwrap_or_else(|_| panic!("Data is not valid"));
    TokenStream::from(data.quote())
}
