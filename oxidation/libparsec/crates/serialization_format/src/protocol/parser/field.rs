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
    ) -> syn::ExprType {
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
            let introduced_text = format!("Introduced in API-{introduced_in}");
            attrs.push(syn::parse_quote!(#[doc = #introduced_text]));
        }

        syn::parse_quote! {
            #(#attrs)*
            #visibility #name: #ty
        }
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

#[cfg(test)]
#[rstest::rstest]
#[case::basic_field(
    r#"{"name": "Foo", "type": "String"}"#,
    Field { name: "Foo".to_string() , ty: "String".to_string(), introduced_in: None, default: None }
)]
#[case::field_introduced_in(
    r#"{"name": "Bar", "type": "Boolean", "introduced_in": "5.2"}"#,
    Field { name: "Bar".to_string(), ty: "Boolean".to_string(), introduced_in: Some("5.2".try_into().unwrap()), default: None}
)]
fn field(#[case] input: &str, #[case] expected: Field) {
    let field = serde_json::from_str::<Field>(input).expect("Got error on valid data");
    assert_eq!(field, expected)
}
