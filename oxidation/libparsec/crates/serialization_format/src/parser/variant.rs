// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::Ident;

use super::{quote_fields, Field, SerdeAttr, Vis};

#[derive(Deserialize)]
pub(crate) struct Variant {
    pub(crate) name: String,
    pub(crate) fields: Vec<Field>,
    pub(crate) discriminant_value: Option<String>,
}

impl Variant {
    pub(crate) fn quote(&self, types: &HashMap<String, String>) -> TokenStream {
        let rename = match self.name.as_str() {
            "type" => Some("ty"),
            _ => None,
        };
        let name: Ident =
            syn::parse_str(rename.unwrap_or(&self.name)).expect("Expected a valid name (Variant)");
        let rename = SerdeAttr::Rename.quote(
            self.discriminant_value
                .as_ref()
                .or_else(|| rename.map(|_| &self.name)),
        );
        let fields = quote_fields(&self.fields, Vis::Private, types);

        if self.fields.is_empty() {
            quote! {
                #rename
                #name
            }
        } else {
            quote! {
                #rename
                #name {
                    #fields
                }
            }
        }
    }
}

pub(crate) fn quote_variants(
    _variants: &[Variant],
    types: &HashMap<String, String>,
) -> TokenStream {
    let mut variants = quote! {};

    for variant in _variants {
        let variant = variant.quote(types);

        variants = quote! {
            #variants
            #variant,
        };
    }
    variants
}
