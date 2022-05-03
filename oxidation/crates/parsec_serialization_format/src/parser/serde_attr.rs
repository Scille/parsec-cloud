// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use proc_macro2::TokenStream;
use quote::quote;

pub(crate) enum SerdeAttr {
    Rename,
    Tag,
}

impl SerdeAttr {
    pub(crate) fn quote(self, value: Option<&String>) -> TokenStream {
        match value {
            Some(value) => match self {
                Self::Rename => quote! { #[serde(rename = #value)] },
                Self::Tag => quote! { #[serde(tag = #value)] },
            },
            None => quote! {},
        }
    }
}
