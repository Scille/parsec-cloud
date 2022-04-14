// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod parser;
mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields};

use parser::Schema;
use utils::{extract_extra_specs, quote_attrs, quote_fields, quote_variants};

#[proc_macro_attribute]
pub fn parsec_schema(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as parser::Schema);

    TokenStream::from(match input {
        Schema::Struct(schema) => {
            let (_attrs, _) = quote_attrs(schema.attrs);
            let ident = schema.ident;
            let (_fields, _specs) = quote_fields(schema.fields.iter());

            let _struct = match schema.fields {
                Fields::Unit => quote! {pub struct #ident;},
                Fields::Unnamed(_) => quote! {pub struct #ident (#_fields)},
                Fields::Named(_) => quote! {pub struct #ident {#_fields}},
            };

            let _extra_specs = extract_extra_specs(&ident);

            quote! {
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                #_attrs
                #_struct

                impl #ident {
                    pub fn specs() -> ::serde_json::Value {
                        ::serde_json::json!({
                            "fields": {
                                #_specs
                                #_extra_specs
                            }
                        })
                    }
                }
            }
        }
        Schema::Enum(schema) => {
            let (_attrs, _) = quote_attrs(schema.attrs);
            let ident = schema.ident;
            let (_variants, _specs) = quote_variants(schema.variants.into_iter());

            let _extra_specs = extract_extra_specs(&ident);

            quote! {
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                #_attrs
                pub enum #ident {
                    #_variants
                }

                impl #ident {
                    pub fn specs() -> ::serde_json::Value {
                        ::serde_json::json!({
                            "fields": {
                                #_specs
                                #_extra_specs
                            }
                        })
                    }
                }
            }
        }
    })
}
