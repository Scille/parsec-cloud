// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use super::{quote_fields, quote_variants, Field, SerdeAttr, Serializing, Variant, Vis};

#[derive(Deserialize)]
pub(crate) struct Schema {
    label: String,
    serializing: Serializing,
    // Struct based
    fields: Option<Vec<Field>>,
    // Enum based
    discriminant_field: Option<String>,
    variants: Option<Vec<Variant>>,
}

impl Schema {
    pub(crate) fn quote(&self) -> TokenStream {
        let name: Ident = syn::parse_str(&self.label).unwrap_or_else(|_| unreachable!());
        let tag = SerdeAttr::Tag.quote(self.discriminant_field.as_ref());
        let serializing = self.serializing.quote(&name);
        if let Some(variants) = &self.variants {
            let variants = quote_variants(variants);

            quote! {
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                #tag
                pub enum #name {
                    #variants

                    // TODO: How to serialize/deserialize an enum with encryption ?
                }
            }
        } else if let Some(fields) = &self.fields {
            let fields = quote_fields(fields, Vis::Public);

            quote! {
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                pub struct #name {
                    #fields
                }

                #serializing
            }
        } else {
            quote! {
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                pub struct #name;

                #serializing
            }
        }
    }
}
