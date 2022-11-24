// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use crate::protocol::utils::{quote_serde_as, validate_raw_type};

use super::MajorMinorVersion;

use itertools::Itertools;
use serde::Deserialize;

/// A Collection of fields.
/// The keys of this dict are the name of the field.
pub type Fields = HashMap<String, Field>;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Field {
    /// The type's name of the field.
    #[serde(rename = "type")]
    pub ty: String,
    /// In which version the current field was introduced.
    #[serde(default)]
    pub introduced_in: Option<MajorMinorVersion>,
    /// The name of the function to get the default value from.
    #[serde(default)]
    pub default: Option<String>,
}

#[cfg(test)]
impl Default for Field {
    fn default() -> Self {
        Self {
            ty: "String".to_string(),
            introduced_in: None,
            default: None,
        }
    }
}

impl Field {
    pub fn quote(
        &self,
        name: &str,
        visibility: syn::Visibility,
        types: &HashMap<String, String>,
    ) -> syn::Field {
        let (rename, name) = self.quote_name(name);
        let (ty, serde_attr) = self.quote_type(types);

        let mut attrs: Vec<syn::Attribute> = serde_attr.into_iter().collect_vec();

        attrs.push(quote_serde_as(&ty, self.can_only_be_null()));
        if self.can_only_be_null() {
            attrs.push(syn::parse_quote!(#[doc = "The field may be null."]))
        } else if self.can_be_missing_or_null() {
            attrs.push(
                syn::parse_quote!(#[doc = "The field may be absent or its value to be null."]),
            )
        }
        if let Some(rename) = rename {
            attrs.push(syn::parse_quote!(#[serde(rename = #rename)]))
        }
        if let Some(default) = &self.default {
            attrs.push(syn::parse_quote!(#[serde(default = #default)]));
        }
        if let Some(introduced_in) = self.introduced_in {
            let introduced_text = format!("Introduced in `API-{introduced_in}`.");
            attrs.push(syn::parse_quote!(#[doc = #introduced_text]));
        }

        syn::parse::Parser::parse2(
            syn::Field::parse_named,
            quote::quote! {
                #(#attrs)*
                #visibility #name: #ty
            },
        )
        .unwrap()
    }

    fn quote_name(&self, name: &str) -> (Option<&'static str>, syn::Ident) {
        let rename = match name {
            "type" => Some("ty"),
            _ => None,
        };
        let quote = syn::parse_str(rename.unwrap_or(name)).expect("A valid name");

        (rename, quote)
    }

    fn quote_type(&self, types: &HashMap<String, String>) -> (syn::Type, Option<syn::Attribute>) {
        let ty = validate_raw_type(&self.ty, types).expect("A valid type");

        if self.introduced_in.is_some() {
            (
                syn::parse_quote!(libparsec_types::Maybe<#ty>),
                Some(
                    syn::parse_quote!(#[serde(default, skip_serializing_if = "libparsec_types::Maybe::is_absent")]),
                ),
            )
        } else {
            (ty, None)
        }
    }

    /// True when the type of the field is the wrapper `RequiredOption<ty>`.
    fn can_only_be_null(&self) -> bool {
        self.ty.starts_with("RequiredOption")
    }

    /// True when the type of the field is the wrapper `NonRequiredOption<ty>`.
    fn can_be_missing_or_null(&self) -> bool {
        self.ty.starts_with("NonRequiredOption")
    }
}

/// Quote multiple fields.
/// Can specify the visibility used by default it's public (`pub`)
///
/// > The fields will be sorted by their name in binary mode.
pub fn quote_fields(
    fields: &Fields,
    visibility: Option<syn::Visibility>,
    types: &HashMap<String, String>,
) -> Vec<syn::Field> {
    let visibility = visibility.unwrap_or_else(|| syn::parse_quote!(pub));
    fields
        .iter()
        .sorted_by_key(|(name, _)| *name)
        .map(|(name, field)| field.quote(name, visibility.clone(), types))
        .collect()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use ::quote::{quote, ToTokens};
    use pretty_assertions::assert_eq;
    use proc_macro2::TokenStream;
    use rstest::rstest;

    use super::{Field, MajorMinorVersion};

    #[rstest]
    #[case::basic_field(
        r#"{"type": "String"}"#,
        Field { ty: "String".to_string(), introduced_in: None, default: None }
    )]
    #[case::with_introduced_in(
        r#"{"type": "Boolean", "introduced_in": "5.2"}"#,
        Field { ty: "Boolean".to_string(), introduced_in: Some("5.2".parse().unwrap()), default: None}
    )]
    #[case::with_default(
        r#"{"type": "u32", "default": "42"}"#,
        Field { ty: "u32".to_string(), default: Some("42".to_string()), introduced_in: None}
    )]
    fn deserialization(#[case] input: &str, #[case] expected: Field) {
        let field = serde_json::from_str::<Field>(input).expect("Got error on valid data");
        assert_eq!(field, expected)
    }

    #[rstest]
    #[case::public(
        syn::parse_quote!(pub),
        Field {
            ty: "Integer".to_string(),
            introduced_in: None,
            default: None,
        },
        quote! {
            #[serde_as(as = "_")]
            pub foo: i64
        }
    )]
    #[case::public_with_introduced(
        syn::parse_quote!(pub),
        Field {
            ty: "Integer".to_string(),
            introduced_in: Some(MajorMinorVersion { major: 4, minor: 1}),
            default: None,
        },
        quote! {
            #[serde(default, skip_serializing_if = "libparsec_types::Maybe::is_absent")]
            #[serde_as(as = "libparsec_types::Maybe<_>")]
            #[doc = "Introduced in `API-4.1`."]
            pub foo: libparsec_types::Maybe<i64>
        }
    )]
    #[case::public_with_default(
        syn::parse_quote!(pub),
        Field {
            ty: "Integer".to_string(),
            introduced_in: None,
            default: Some("42".to_string())
        },
        quote! {
            #[serde_as(as = "_")]
            #[serde(default = "42")]
            pub foo: i64
        }
    )]
    #[case::private(
        syn::Visibility::Inherited,
        Field {
            ty: "Integer".to_string(),
            introduced_in: None,
            default: None,
        },
        quote! {
            #[serde_as(as = "_")]
            foo: i64
        }
    )]
    #[case::private_with_introduced(
        syn::Visibility::Inherited,
        Field {
            ty: "Integer".to_string(),
            introduced_in: Some(MajorMinorVersion { major: 4, minor: 1}),
            default: None,
        },
        quote! {
            #[serde(default, skip_serializing_if = "libparsec_types::Maybe::is_absent")]
            #[serde_as(as = "libparsec_types::Maybe<_>")]
            #[doc = "Introduced in `API-4.1`."]
            foo: libparsec_types::Maybe<i64>
        }
    )]
    #[case::private_with_default(
        syn::Visibility::Inherited,
        Field {
            ty: "Integer".to_string(),
            introduced_in: None,
            default: Some("42".to_string())
        },
        quote! {
            #[serde_as(as = "_")]
            #[serde(default = "42")]
            foo: i64
        }
    )]
    #[case::required_option(
        syn::Visibility::Inherited,
        Field {
            ty: "RequiredOption<Integer>".to_string(),
            ..Default::default()
        },
        quote! {
            #[serde_as(as = "Option<_>", no_default)]
            #[doc = "The field may be null."]
            foo: Option<i64>
        }
    )]
    #[case::non_required_option(
        syn::Visibility::Inherited,
        Field {
            ty: "NonRequiredOption<Integer>".to_string(),
            ..Default::default()
        },
        quote! {
            #[serde_as(as = "Option<_>")]
            #[doc = "The field may be absent or its value to be null."]
            foo: Option<i64>
        }
    )]
    #[case::with_default_function(
        syn::Visibility::Inherited,
        Field {
            default: Some("fn_to_call".to_string()),
            ..Default::default()
        },
        quote! {
            #[serde_as(as = "_")]
            #[serde(default = "fn_to_call")]
            foo: String
        }
    )]
    fn test_quote(
        #[case] vis: syn::Visibility,
        #[case] field: Field,
        #[case] expected: TokenStream,
    ) {
        assert_eq!(
            field
                .quote("foo", vis, &HashMap::new())
                .into_token_stream()
                .to_string(),
            expected.to_string()
        )
    }
}
