// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Ident, Type};

use super::utils::{inspect_type, quote_serde_as};

#[derive(Deserialize)]
pub(crate) struct Field {
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) ty: String,
    introduced_in_revision: Option<u32>,
    default: Option<String>,
}

impl Field {
    pub(crate) fn quote(&self, types: &HashMap<String, String>) -> TokenStream {
        let (rename, name) = self.quote_name();

        let (ty, serde_skip) = self.quote_type(types);
        let rename = rename
            .map(|rename| quote::quote!(#[serde(rename = #rename)]))
            .unwrap_or_default();
        let serde_as = quote_serde_as(&ty, false);
        let serde_default = if let Some(default) = &self.default {
            quote! { #[serde(default = #default)] }
        } else {
            quote! {}
        };

        quote! {
            #rename
            #serde_as
            #serde_skip
            #serde_default
            pub #name: #ty
        }
    }

    pub(crate) fn quote_name(&self) -> (Option<&'static str>, Ident) {
        let rename = match self.name.as_str() {
            "type" => Some("ty"),
            _ => None,
        };
        let name: Ident =
            syn::parse_str(rename.unwrap_or(&self.name)).expect("Expected a valid name (Field)");

        (rename, name)
    }

    pub(crate) fn quote_type(&self, types: &HashMap<String, String>) -> (Type, TokenStream) {
        let ty = inspect_type(&self.ty, types);
        let (inspected_ty, serde_skip) = if self.introduced_in_revision.is_some() {
            (
                "libparsec_types::Maybe".to_string() + "<" + &ty + ">",
                quote! {
                    #[serde(default, skip_serializing_if = "libparsec_types::Maybe::is_absent")]
                },
            )
        } else {
            (ty, quote! {})
        };
        let ty = syn::parse_str::<Type>(&inspected_ty).expect("Expected a valid type (Field)");
        (ty, serde_skip)
    }
}

pub(crate) fn quote_fields(_fields: &[Field], types: &HashMap<String, String>) -> TokenStream {
    let mut fields = quote! {};

    for field in _fields {
        let field = field.quote(types);
        fields = quote! {
            #fields
            #field,
        };
    }
    fields
}
