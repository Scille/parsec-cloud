use std::collections::HashMap;

use serde::Deserialize;

use super::{
    shared_attribute,
    variant::{quote_variants, Variant},
};

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct CustomEnum {
    pub label: String,
    #[serde(default)]
    pub discriminant_field: Option<String>,
    pub variants: Vec<Variant>,
}

#[cfg(test)]
impl Default for CustomEnum {
    fn default() -> Self {
        Self {
            discriminant_field: None,
            label: "FooEnum".to_string(),
            variants: vec![],
        }
    }
}

impl CustomEnum {
    pub fn quote(&self, types: &HashMap<String, String>) -> syn::ItemEnum {
        let name = self.quote_label();

        let mut attrs = shared_attribute().to_vec();
        if let Some(discriminant_field) = &self.discriminant_field {
            attrs.push(syn::parse_quote!(#[serde(tag = #discriminant_field)]))
        }

        let variants = quote_variants(&self.variants, types);

        syn::parse_quote! {
            #(#attrs)*
            pub enum #name {
                #(#variants),*
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
    CustomEnum::default(),
    quote::quote! {
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
    quote::quote! {
        #[::serde_with::serde_as]
        #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
        #[serde(tag = "response_type")]
        pub enum FooEnum {}
    }
)]
#[case::with_variant(
    CustomEnum {
        variants: vec![
            Variant::default(),
            Variant::default(),
        ],
        ..Default::default()
    },
    quote::quote! {
        #[::serde_with::serde_as]
        #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
        pub enum FooEnum {
            FooVariant,
            FooVariant
        }
    }
)]
fn test_quote(#[case] custom_enum: CustomEnum, #[case] expected: proc_macro2::TokenStream) {
    use quote::ToTokens;

    assert_eq!(
        custom_enum
            .quote(&HashMap::new())
            .into_token_stream()
            .to_string(),
        expected.to_string()
    )
}
