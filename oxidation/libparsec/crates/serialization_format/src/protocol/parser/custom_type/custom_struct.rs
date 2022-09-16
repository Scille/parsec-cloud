// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use serde::Deserialize;

use crate::protocol::parser::field::{quote_fields, Field};

use super::shared_attribute;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct CustomStruct {
    pub label: String,
    pub fields: Vec<Field>,
}

impl CustomStruct {
    pub fn quote(&self, types: &HashMap<String, String>) -> syn::ItemStruct {
        let name = self.quote_label();
        let fields = quote_fields(&self.fields, None, types);
        let attrs = shared_attribute();

        if fields.is_empty() {
            syn::parse_quote! {
                #(#attrs)*
                pub struct #name;
            }
        } else {
            syn::parse_quote! {
                #(#attrs)*
                pub struct #name {
                    #(#fields),*
                }
            }
        }
    }

    pub fn quote_label(&self) -> syn::Ident {
        syn::parse_str(&self.label).expect("A valid label for Custom struct")
    }
}

#[cfg(test)]
#[rstest::rstest]
#[case::basic(
    CustomStruct {
        label: "FooBar".to_string(),
        fields: vec![]
    },
    quote::quote! {
        #[::serde_with::serde_as]
        #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
        pub struct FooBar;
    }
)]
#[case::with_fields(
    CustomStruct {
        label: "FooBar".to_string(),
        fields: vec![
            Field::default(),
            Field::default(),
        ]
    },
    quote::quote! {
        #[::serde_with::serde_as]
        #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
        pub struct FooBar {
            #[serde_as(as = "_")]
            pub foo_type: String,
            #[serde_as(as = "_")]
            pub foo_type: String
        }
    }
)]
fn test_quote(#[case] custom_struct: CustomStruct, #[case] expected: proc_macro2::TokenStream) {
    use pretty_assertions::assert_eq;
    use quote::ToTokens;

    assert_eq!(
        custom_struct
            .quote(&HashMap::new())
            .into_token_stream()
            .to_string(),
        expected.to_string()
    )
}
