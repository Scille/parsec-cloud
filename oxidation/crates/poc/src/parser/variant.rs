// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use super::{quote_fields, Field, SerdeAttr, Vis};

#[derive(Deserialize)]
pub(crate) struct Variant {
    name: String,
    fields: Vec<Field>,
    discriminant_value: Option<String>,
}

impl Variant {
    pub(crate) fn quote(&self) -> TokenStream {
        let rename = match self.name.as_str() {
            "type" => Some("ty"),
            _ => None,
        };
        let name: Ident =
            syn::parse_str(rename.unwrap_or(&self.name)).unwrap_or_else(|_| unreachable!());
        let rename = SerdeAttr::Rename.quote(
            self.discriminant_value
                .as_ref()
                .or_else(|| rename.map(|_| &self.name)),
        );
        let fields = quote_fields(&self.fields, Vis::Private);

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

pub(crate) fn quote_variants(_variants: &[Variant]) -> TokenStream {
    let mut variants = quote! {};

    for variant in _variants {
        let variant = variant.quote();

        variants = quote! {
            #variants
            #variant,
        };
    }
    variants
}
