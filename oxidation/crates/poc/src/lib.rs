// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod parser;
mod utils;

use proc_macro::TokenStream;
use std::fs::File;
use std::io::Read;
use syn::{parse_macro_input, LitStr};

use crate::utils::quote_struct;

#[proc_macro]
pub fn parsec_schema(path: TokenStream) -> TokenStream {
    let path = parse_macro_input!(path as LitStr);
    let file_path = std::env::current_dir().unwrap().join(&path.value());
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let input: parser::Schema = syn::parse_str(&content).unwrap();
    let variants = input.0;

    TokenStream::from(match variants.len() {
        0 => panic!("No variant found"),
        1 => quote_struct(&variants[0]),
        _ => {
            unimplemented!()
        }
    })
}
