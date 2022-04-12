// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod parser;
mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields};

use parser::Schema;
use utils::{quote_attrs, quote_fields, quote_variants};

#[proc_macro_attribute]
pub fn parsec_schema(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as parser::Schema);

    TokenStream::from(match input {
        Schema::Struct(schema) => {
            let attrs = quote_attrs(&schema.attrs);
            let ident = schema.ident;
            let fields = quote_fields(schema.fields.iter());

            let _struct = match schema.fields {
                Fields::Unit => quote! {pub struct #ident;},
                Fields::Unnamed(_) => quote! {pub struct #ident (#fields)},
                Fields::Named(_) => quote! {pub struct #ident {#fields}},
            };

            quote! {
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
                #attrs
                #_struct
            }
        }
        Schema::Enum(schema) => {
            let attrs = quote_attrs(&schema.attrs);
            let ident = schema.ident;
            let variants = quote_variants(schema.variants.iter());

            quote! {
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
                #attrs
                pub enum #ident {
                    #variants
                }
            }
        }
    })
}
