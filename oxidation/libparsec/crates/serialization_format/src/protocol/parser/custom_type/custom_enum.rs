// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use serde::Deserialize;

use super::{
    shared_attribute,
    variant::{quote_variants, Variants},
};

#[cfg_attr(test, derive(PartialEq, Eq, Default))]
#[derive(Debug, Deserialize, Clone)]
pub struct CustomEnum {
    #[serde(default)]
    pub discriminant_field: Option<String>,
    pub variants: Variants,
}

impl CustomEnum {
    pub fn quote(
        &self,
        name: &str,
        types: &HashMap<String, String>,
    ) -> anyhow::Result<syn::ItemEnum> {
        let name = self.quote_label(name)?;

        let mut attrs = shared_attribute().to_vec();
        if let Some(discriminant_field) = &self.discriminant_field {
            attrs.push(syn::parse_quote!(#[serde(tag = #discriminant_field)]))
        }

        let variants = quote_variants(&self.variants, types)?;

        Ok(syn::parse_quote! {
            #(#attrs)*
            pub enum #name {
                #(#variants),*
            }
        })
    }

    pub fn quote_label(&self, name: &str) -> anyhow::Result<syn::Ident> {
        syn::parse_str(name).map_err(|e| anyhow::anyhow!("Invalid Custom Enum name `{name}`: {e}"))
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use quote::{quote, ToTokens};
    use rstest::rstest;

    use super::{CustomEnum, HashMap, Variants};

    use crate::protocol::parser::Variant;

    #[rstest]
    #[case::basic(
        CustomEnum::default(),
        quote! {
            #[::serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
            pub enum FooEnum {}
        }
    )]
    #[case::with_discriminant_field(
        CustomEnum {
            discriminant_field: Some("response_type".to_string()),
            ..Default::default()
        },
        quote! {
            #[::serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
            #[serde(tag = "response_type")]
            pub enum FooEnum {}
        }
    )]
    #[case::with_variant(
        CustomEnum {
            variants: Variants::from([
                ("Foo".to_string(), Variant::default()),
                ("Bar".to_string(), Variant::default())
            ]),
            ..Default::default()
        },
        quote! {
            #[::serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
            pub enum FooEnum {
                Bar,
                Foo
            }
        }
    )]
    fn test_quote(#[case] custom_enum: CustomEnum, #[case] expected: proc_macro2::TokenStream) {
        assert_eq!(
            custom_enum
                .quote("FooEnum", &HashMap::new())
                .unwrap()
                .into_token_stream()
                .to_string(),
            expected.to_string()
        )
    }
}
