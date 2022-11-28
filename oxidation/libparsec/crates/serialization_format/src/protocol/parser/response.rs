// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use serde::Deserialize;

use crate::shared::{quote_fields, to_pascal_case, Fields};

/// A collection of [Response].
/// Each keys correspond to a response type/status.
pub type Responses = HashMap<String, Response>;

#[cfg_attr(test, derive(PartialEq, Eq, Default))]
#[derive(Debug, Deserialize, Clone)]
pub struct Response {
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub fields: Fields,
}

impl Response {
    pub fn quote(&self, name: &str, types: &HashMap<String, String>) -> syn::Variant {
        if let Some(unit) = &self.unit {
            self.quote_unit(name, unit)
        } else if self.fields.is_empty() {
            self.quote_empty(name)
        } else {
            self.quote_fields(name, types)
        }
    }

    fn quote_unit(&self, name: &str, unit: &str) -> syn::Variant {
        let name_ident = self.quote_name(name);
        let rename = name;
        let unit = syn::parse_str::<syn::Type>(unit).expect("A valid unit");

        syn::parse_quote! {
            #[serde(rename = #rename)]
            #name_ident(#unit)
        }
    }

    fn quote_empty(&self, name: &str) -> syn::Variant {
        let name_ident = self.quote_name(name);
        let rename = name;

        syn::parse_quote! {
            #[serde(rename = #rename)]
            #name_ident
        }
    }

    fn quote_fields(&self, name: &str, types: &HashMap<String, String>) -> syn::Variant {
        let name_ident = self.quote_name(name);
        let rename = name;
        let fields = quote_fields(&self.fields, Some(syn::Visibility::Inherited), types);

        syn::parse_quote! {
            #[serde(rename = #rename)]
            #name_ident {
                #(#fields),*
            }
        }
    }

    fn quote_name(&self, name: &str) -> syn::Ident {
        syn::parse_str(&to_pascal_case(name)).expect("A valid status")
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use quote::{quote, ToTokens};
    use rstest::rstest;

    use super::{Fields, HashMap, Response};

    use crate::shared::Field;

    #[rstest]
    #[case::empty(
        Response::default(),
        quote! {
            #[serde(rename = "foo_response")]
            FooResponse
        }
    )]
    #[case::with_unit(
        Response {
            unit: Some("String".to_string()),
            ..Default::default()
        },
        quote! {
            #[serde(rename = "foo_response")]
            FooResponse(String)
        }
    )]
    #[case::with_fields(
        Response {
            fields: Fields::from([
                ("foo".to_string(), Field::default()),
                ("bar".to_string(), Field::default())
            ]),
            ..Default::default()
        },
        quote! {
            #[serde(rename = "foo_response")]
            FooResponse {
                #[serde_as(as = "_")]
                bar: String,
                #[serde_as(as = "_")]
                foo: String
            }
        }
    )]
    fn test_quote(#[case] request: Response, #[case] expected: proc_macro2::TokenStream) {
        assert_eq!(
            request
                .quote("foo_response", &HashMap::new())
                .into_token_stream()
                .to_string(),
            expected.to_string()
        );
    }
}
