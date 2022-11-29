// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use crate::{
    config::CratesPaths,
    shared::{
        utils::{quote_serde_as, validate_raw_type},
        MajorMinorVersion,
    },
};

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
        crates_override: &CratesPaths,
    ) -> anyhow::Result<syn::Field> {
        let (rename, name) = self
            .quote_name(name)
            .map_err(|e| anyhow::anyhow!("Invalid field name `{name}`: {e}"))?;
        let (ty, serde_attr) = self.quote_type(types, crates_override).map_err(|e| {
            anyhow::anyhow!(
                "Cannot quote type `{}` for field `{}`: {}",
                self.ty,
                name,
                e
            )
        })?;

        let mut attrs: Vec<syn::Attribute> = serde_attr.into_iter().collect_vec();

        attrs.push(quote_serde_as(&ty, self.can_only_be_null())?);
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
        .map_err(|e| anyhow::Error::from(e).context(format!("quoting Field `{name}`")))
    }

    fn quote_name(&self, name: &str) -> anyhow::Result<(Option<&'static str>, syn::Ident)> {
        let rename = match name {
            "type" => Some("ty"),
            _ => None,
        };
        let quote = syn::parse_str(rename.unwrap_or(name)).map_err(anyhow::Error::from)?;

        Ok((rename, quote))
    }

    fn quote_type(
        &self,
        types: &HashMap<String, String>,
        crates_override: &CratesPaths,
    ) -> anyhow::Result<(syn::Type, Option<syn::Attribute>)> {
        let ty = validate_raw_type(&self.ty, types, crates_override)?;

        if self.introduced_in.is_some() {
            let libparsec_types = &crates_override["libparsec_types"];
            let maybe_root_path = syn::parse_str::<syn::Path>(libparsec_types)
                .map_err(|e| anyhow::anyhow!("Invalid maybe root path `{libparsec_types}`: {e}"))?;
            let maybe_is_absent = format!("{libparsec_types}::Maybe::is_absent");

            Ok((
                syn::parse_quote!(#maybe_root_path::Maybe<#ty>),
                Some(syn::parse_quote!(#[serde(default, skip_serializing_if = #maybe_is_absent)])),
            ))
        } else {
            Ok((ty, None))
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
    crates_override: &CratesPaths,
) -> anyhow::Result<Vec<syn::Field>> {
    let visibility = visibility.unwrap_or_else(|| syn::parse_quote!(pub));
    let (fields, errors): (Vec<_>, Vec<_>) = fields
        .iter()
        .sorted_by_key(|(name, _)| *name)
        .map(|(name, field)| field.quote(name, visibility.clone(), types, crates_override))
        .partition_result();

    anyhow::ensure!(
        errors.is_empty(),
        "Cannot quote all fields:\n{}",
        errors.into_iter().map(|e| e.to_string()).join(",\n")
    );

    Ok(fields)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use ::quote::{quote, ToTokens};
    use pretty_assertions::assert_eq;
    use proc_macro2::TokenStream;
    use rstest::rstest;

    use super::{Field, MajorMinorVersion};
    use crate::config::CratesPaths;

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
                .quote("foo", vis, &HashMap::new(), &CratesPaths::default())
                .unwrap()
                .into_token_stream()
                .to_string(),
            expected.to_string()
        )
    }
}

/// Filter out fields that will be introduced in a future major version.
macro_rules! filter_out_future_fields {
    ($current_version:expr, $fields:expr) => {
        $fields
            .iter()
            .filter_map(move |(name, field)| {
                // We check if the current field need to be present at the `current_version`
                if field
                    .introduced_in
                    .map(|mj_version| mj_version.major <= $current_version)
                    .unwrap_or(true)
                {
                    let mut dup_field = field.clone();
                    // We remove the `introduced_in` tag if the field was introduced in a prior version to the current version.
                    // We consider at this step that the field is now `stable`.
                    if dup_field
                        .introduced_in
                        .map(|mj_version| mj_version.major < $current_version)
                        .unwrap_or_default()
                    {
                        dup_field.introduced_in = None;
                    }
                    Some((name.clone(), dup_field))
                } else {
                    None
                }
            })
            .collect()
    };
}

pub(crate) use filter_out_future_fields;
