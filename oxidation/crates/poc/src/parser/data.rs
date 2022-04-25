// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use super::{quote_fields, Field, Vis};

#[derive(Deserialize)]
pub(crate) struct Data {
    label: String,
    #[serde(rename = "type")]
    _ty: String,
    other_fields: Vec<Field>,
}

impl Data {
    pub(crate) fn quote(&self) -> TokenStream {
        let name: Ident = syn::parse_str(&self.label).unwrap_or_else(|_| unreachable!());
        let remote_str = self.label.clone() + "Data";
        let fields = quote_fields(&self.other_fields, Vis::Public);
        quote! {
            #[serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
            #[serde(remote = #remote_str)]
            pub struct #name {
                #fields
            }
        }
    }
}
