// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use serde::Deserialize;

use crate::protocol::parser::field::{quote_fields, Fields};

use super::shared_attribute;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct CustomStruct {
    pub fields: Fields,
}

impl CustomStruct {
    pub fn quote(&self, name: &str, types: &HashMap<String, String>) -> syn::ItemStruct {
        let name = self.quote_label(name);
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

    pub fn quote_label(&self, name: &str) -> syn::Ident {
        syn::parse_str(name).expect("A valid label for Custom struct")
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use quote::{quote, ToTokens};
    use rstest::rstest;

    use crate::protocol::parser::field::Field;

    use super::{CustomStruct, Fields, HashMap};

    #[rstest]
    #[case::basic(
        CustomStruct {
            fields: Fields::default(),
        },
        quote! {
            #[::serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
            pub struct FooBar;
        }
    )]
    #[case::with_fields(
        CustomStruct {
            fields: Fields::from([
                ("foo".to_string(), Field::default()),
                ("bar".to_string(), Field::default()),
            ])
        },
        quote! {
            #[::serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
            pub struct FooBar {
                #[serde_as(as = "_")]
                pub bar: String,
                #[serde_as(as = "_")]
                pub foo: String
            }
        }
    )]
    fn test_quote(#[case] custom_struct: CustomStruct, #[case] expected: proc_macro2::TokenStream) {
        assert_eq!(
            custom_struct
                .quote("FooBar", &HashMap::new())
                .into_token_stream()
                .to_string(),
            expected.to_string()
        )
    }
}
