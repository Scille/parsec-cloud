use std::collections::HashMap;

use crate::protocol::utils::{quote_serde_as, validate_raw_type};

use super::MajorMinorVersion;

use serde::Deserialize;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Field {
    /// The name of the field.
    pub name: String,
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
            name: "foo_type".to_string(),
            ty: "String".to_string(),
            introduced_in: None,
            default: None,
        }
    }
}

impl Field {
    pub fn quote(
        &self,
        visibility: syn::Visibility,
        types: &HashMap<String, String>,
    ) -> syn::Field {
        let (rename, name) = self.quote_name();
        let (ty, serde_skip) = self.quote_type(types);

        let mut attrs: Vec<syn::Attribute> = Vec::new();

        if let Some(rename) = rename {
            attrs.push(syn::parse_quote!(#[serde(rename = #rename)]))
        }
        attrs.push(quote_serde_as(&ty));
        if let Some(serde_skip) = serde_skip {
            attrs.push(serde_skip)
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

    fn quote_name(&self) -> (Option<&'static str>, syn::Ident) {
        let rename = match self.name.as_str() {
            "type" => Some("ty"),
            _ => None,
        };
        let quote = syn::parse_str(rename.unwrap_or(&self.name)).expect("A valid name");

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
}

/// Quote multiple fields.
/// Can specify the visibility used by default it's public (`pub`)
pub fn quote_fields(
    fields: &[Field],
    visibility: Option<syn::Visibility>,
    types: &HashMap<String, String>,
) -> Vec<syn::Field> {
    let visibility = visibility.unwrap_or_else(|| syn::parse_quote!(pub));
    fields
        .iter()
        .map(|field| field.quote(visibility.clone(), types))
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
        r#"{"name": "Foo", "type": "String"}"#,
        Field { name: "Foo".to_string() , ty: "String".to_string(), introduced_in: None, default: None }
    )]
    #[case::field_introduced_in(
        r#"{"name": "Bar", "type": "Boolean", "introduced_in": "5.2"}"#,
        Field { name: "Bar".to_string(), ty: "Boolean".to_string(), introduced_in: Some("5.2".try_into().unwrap()), default: None}
    )]
    fn deserialization(#[case] input: &str, #[case] expected: Field) {
        let field = serde_json::from_str::<Field>(input).expect("Got error on valid data");
        assert_eq!(field, expected)
    }

    #[rstest]
    #[case::public(
        syn::parse_quote!(pub),
        Field {
            name: "foo".to_string(),
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
            name: "foo".to_string(),
            ty: "Integer".to_string(),
            introduced_in: Some(MajorMinorVersion { major: 4, minor: 1}),
            default: None,
        },
        quote! {
            #[serde_as(as = "libparsec_types::Maybe<_>")]
            #[serde(default, skip_serializing_if = "libparsec_types::Maybe::is_absent")]
            #[doc = "Introduced in `API-4.1`."]
            pub foo: libparsec_types::Maybe<i64>
        }
    )]
    #[case::public_with_default(
        syn::parse_quote!(pub),
        Field {
            name: "foo".to_string(),
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
            name: "foo".to_string(),
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
            name: "foo".to_string(),
            ty: "Integer".to_string(),
            introduced_in: Some(MajorMinorVersion { major: 4, minor: 1}),
            default: None,
        },
        quote! {
            #[serde_as(as = "libparsec_types::Maybe<_>")]
            #[serde(default, skip_serializing_if = "libparsec_types::Maybe::is_absent")]
            #[doc = "Introduced in `API-4.1`."]
            foo: libparsec_types::Maybe<i64>
        }
    )]
    #[case::private_with_default(
        syn::Visibility::Inherited,
        Field {
            name: "foo".to_string(),
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
    fn test_quote(
        #[case] vis: syn::Visibility,
        #[case] field: Field,
        #[case] expected: TokenStream,
    ) {
        assert_eq!(
            field
                .quote(vis, &HashMap::new())
                .into_token_stream()
                .to_string(),
            expected.to_string()
        )
    }
}
