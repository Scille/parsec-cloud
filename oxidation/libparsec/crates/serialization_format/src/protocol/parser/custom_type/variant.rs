// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use serde::Deserialize;

use crate::protocol::parser::field::{quote_fields, Field};

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Variant {
    pub name: String,
    pub discriminant_value: Option<String>,
    pub fields: Vec<Field>,
}

#[cfg(test)]
impl Default for Variant {
    fn default() -> Self {
        Self {
            discriminant_value: None,
            fields: vec![],
            name: "FooVariant".to_string(),
        }
    }
}

impl Variant {
    pub fn quote(&self, types: &HashMap<String, String>) -> syn::Variant {
        let rename = match self.name.as_str() {
            "type" => Some("ty"),
            _ => None,
        };
        // TODO: Format the name in pascal case
        let name = syn::parse_str::<syn::Ident>(rename.unwrap_or(&self.name))
            .expect("A valid variant name");

        let mut attrs: Vec<syn::Attribute> = Vec::new();
        // Add the serde attribute to rename the variant during serialization
        // if we have set a specific value for it
        // or the variant needed to be renamed to avoid collision with keyword.
        if rename.is_some() || self.discriminant_value.is_some() {
            let rename = self.discriminant_value.as_ref().unwrap_or(&self.name);
            attrs.push(syn::parse_quote!(#[serde(rename = #rename)]))
        }

        let fields = quote_fields(&self.fields, Some(syn::Visibility::Inherited), types);

        if fields.is_empty() {
            syn::parse_quote! {
                #(#attrs)*
                #name
            }
        } else {
            syn::parse_quote! {
                #(#attrs)*
                #name {
                    #(#fields),*
                }
            }
        }
    }
}

pub fn quote_variants(variants: &[Variant], types: &HashMap<String, String>) -> Vec<syn::Variant> {
    variants
        .iter()
        .map(|variant| variant.quote(types))
        .collect()
}

#[cfg(test)]
#[rstest::rstest]
#[case::empty(
    Variant {
        ..Default::default()
    },
    quote::quote! {
        FooVariant
    }
)]
#[case::with_field(
    Variant {
        fields: vec![
            Field::default()
        ],
        ..Default::default()
    },
    quote::quote! {
        FooVariant {
            #[serde_as(as = "_")]
            foo_type: String
        }
    }
)]
#[case::with_multiple_fields(
    Variant {
        fields: vec![
            Field::default(),
            Field::default(),
        ],
        ..Default::default()
    },
    quote::quote! {
        FooVariant {
            #[serde_as(as = "_")]
            foo_type: String,
            #[serde_as(as = "_")]
            foo_type: String
        }
    }
)]
#[case::rename(
    Variant {
        name: "type".to_string(),
        ..Default::default()
    },
    quote::quote! {
        #[serde(rename = "type")]
        ty
    }
)]
#[case::lowercase_name(
    Variant {
        name: "foo_bar".to_string(),
        ..Default::default()
    },
    quote::quote! {
        foo_bar
    }
)]
#[case::discriminant_value(
    Variant {
        discriminant_value: Some("foobar".to_string()),
        ..Default::default()
    },
    quote::quote! {
        #[serde(rename = "foobar")]
        FooVariant
    }
)]
#[case::discriminant_clashing_with_name(
    Variant {
        name: "type".to_string(),
        discriminant_value: Some("foobar".to_string()),
        ..Default::default()
    },
    quote::quote! {
        #[serde(rename = "foobar")]
        ty
    }
)]
fn test_quote(#[case] variant: Variant, #[case] expected: proc_macro2::TokenStream) {
    use quote::ToTokens;

    pretty_assertions::assert_eq!(
        variant
            .quote(&HashMap::new())
            .into_token_stream()
            .to_string(),
        expected.to_string()
    )
}
