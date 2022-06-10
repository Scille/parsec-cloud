// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Ident, Type};

use super::utils::{inspect_type, quote_serde_as};
use super::SerdeAttr;

#[derive(Clone, Copy)]
pub(crate) enum Vis {
    Public,
    Private,
}

#[derive(Deserialize)]
pub(crate) struct Field {
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) ty: String,
    introduced_in_revision: Option<u32>,
    default: Option<String>,
}

impl Field {
    pub(crate) fn quote(&self, vis: Vis, types: &HashMap<String, String>) -> TokenStream {
        let (rename, name) = self.quote_name();

        let (ty, serde_skip) = self.quote_type(types);
        let rename = SerdeAttr::Rename.quote(rename.map(|_| &self.name));
        let serde_as = quote_serde_as(&ty);
        let serde_default = if let Some(default) = &self.default {
            quote! { #[serde(default = #default)] }
        } else {
            quote! {}
        };

        match vis {
            Vis::Public => quote! {
                #rename
                #serde_as
                #serde_skip
                #serde_default
                pub #name: #ty
            },
            Vis::Private => quote! {
                #rename
                #serde_as
                #serde_skip
                #serde_default
                #name: #ty
            },
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
                "parsec_api_types::Maybe".to_string() + "<" + &ty + ">",
                quote! {
                    #[serde(default, skip_serializing_if = "parsec_api_types::Maybe::is_absent")]
                },
            )
        } else {
            (ty, quote! {})
        };
        let ty = syn::parse_str::<Type>(&inspected_ty).expect("Expected a valid type (Field)");
        (ty, serde_skip)
    }
}

pub(crate) fn quote_fields(
    _fields: &[Field],
    vis: Vis,
    types: &HashMap<String, String>,
) -> TokenStream {
    let mut fields = quote! {};

    for field in _fields {
        let field = field.quote(vis, types);
        fields = quote! {
            #fields
            #field,
        };
    }
    fields
}
