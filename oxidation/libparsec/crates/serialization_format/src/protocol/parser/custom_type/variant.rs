// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use itertools::Itertools;
use serde::Deserialize;

use crate::protocol::parser::field::{quote_fields, Fields};

/// A collection of [Variant] for an `enum`.
/// Each key of the Dict is a variant name.
pub type Variants = HashMap<String, Variant>;

#[cfg_attr(test, derive(PartialEq, Eq, Default))]
#[derive(Debug, Deserialize, Clone)]
pub struct Variant {
    pub discriminant_value: Option<String>,
    pub fields: Fields,
}

impl Variant {
    pub fn quote(&self, name: &str, types: &HashMap<String, String>) -> syn::Variant {
        let rename = match name {
            "type" => Some("ty"),
            _ => None,
        };
        // TODO: Format the name in pascal case
        let ident_name =
            syn::parse_str::<syn::Ident>(rename.unwrap_or(name)).expect("A valid variant name");

        let mut attrs: Vec<syn::Attribute> = Vec::new();
        // Add the serde attribute to rename the variant during serialization
        // if we have set a specific value for it
        // or the variant needed to be renamed to avoid collision with keyword.
        if rename.is_some() || self.discriminant_value.is_some() {
            let rename = self.discriminant_value.as_deref().unwrap_or(name);
            attrs.push(syn::parse_quote!(#[serde(rename = #rename)]))
        }

        let fields = quote_fields(&self.fields, Some(syn::Visibility::Inherited), types);

        if fields.is_empty() {
            syn::parse_quote! {
                #(#attrs)*
                #ident_name
            }
        } else {
            syn::parse_quote! {
                #(#attrs)*
                #ident_name {
                    #(#fields),*
                }
            }
        }
    }
}

/// Quotify the variants of an enum.
///
/// > The variants will be sorted in binary mode.
pub fn quote_variants(variants: &Variants, types: &HashMap<String, String>) -> Vec<syn::Variant> {
    variants
        .iter()
        .sorted_by_key(|(name, _)| *name)
        .map(|(name, variant)| variant.quote(name, types))
        .collect()
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use quote::{quote, ToTokens};
    use rstest::rstest;

    use crate::protocol::parser::field::Field;

    use super::{Fields, HashMap, Variant};

    #[rstest]
    #[case::empty(
        "FooVariant",
        Variant {
            ..Default::default()
        },
        quote! {
            FooVariant
        }
    )]
    #[case::with_field(
        "FooVariant",
        Variant {
            fields: Fields::from([
                ("foo".to_string(), Field::default())
            ]),
            ..Default::default()
        },
        quote! {
            FooVariant {
                #[serde_as(as = "_")]
                foo: String
            }
        }
    )]
    #[case::with_multiple_fields(
        "FooVariant",
        Variant {
            fields: Fields::from([
                ("foo".to_string(), Field::default()),
                ("bar".to_string(), Field::default()),
            ]),
            ..Default::default()
        },
        quote! {
            FooVariant {
                #[serde_as(as = "_")]
                bar: String,
                #[serde_as(as = "_")]
                foo: String
            }
        }
    )]
    #[case::rename(
        "type",
        Variant {
            ..Default::default()
        },
        quote! {
            #[serde(rename = "type")]
            ty
        }
    )]
    #[case::lowercase_name(
        "foo_bar",
        Variant {
            ..Default::default()
        },
        quote! {
            foo_bar
        }
    )]
    #[case::discriminant_value(
        "FooVariant",
        Variant {
            discriminant_value: Some("foobar".to_string()),
            ..Default::default()
        },
        quote! {
            #[serde(rename = "foobar")]
            FooVariant
        }
    )]
    #[case::discriminant_clashing_with_name(
        "type",
        Variant {
            discriminant_value: Some("foobar".to_string()),
            ..Default::default()
        },
        quote! {
            #[serde(rename = "foobar")]
            ty
        }
    )]
    fn test_quote(
        #[case] name: &str,
        #[case] variant: Variant,
        #[case] expected: proc_macro2::TokenStream,
    ) {
        assert_eq!(
            variant
                .quote(name, &HashMap::new())
                .into_token_stream()
                .to_string(),
            expected.to_string()
        )
    }
}
