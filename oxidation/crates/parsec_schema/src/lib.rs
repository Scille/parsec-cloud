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
    let mut serde_as = false;

    TokenStream::from(match input {
        Schema::Struct(schema) => {
            let attrs = quote_attrs(&schema.attrs);
            let ident = schema.ident;
            let fields = quote_fields(schema.fields.iter(), &mut serde_as);

            let extra = if serde_as {
                quote! {
                    #[::serde_with::serde_as]
                    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
                }
            } else {
                quote! {
                    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
                }
            };

            match schema.fields {
                Fields::Unit => {
                    quote! {
                        #extra
                        #attrs
                        pub struct #ident;
                    }
                }
                Fields::Unnamed(_) => {
                    quote! {
                        #extra
                        #attrs
                        pub struct #ident (
                            #fields
                        )
                    }
                }
                Fields::Named(_) => {
                    quote! {
                        #extra
                        #attrs
                        pub struct #ident {
                            #fields
                        }
                    }
                }
            }
        }
        Schema::Enum(schema) => {
            let attrs = quote_attrs(&schema.attrs);
            let ident = schema.ident;
            let variants = quote_variants(schema.variants.iter(), &mut serde_as);

            let extra = if serde_as {
                quote! {
                    #[::serde_with::serde_as]
                    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
                }
            } else {
                quote! {
                    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
                }
            };

            quote! {
                #extra
                #attrs
                pub enum #ident {
                    #variants
                }
            }
        }
    })
}
