use std::collections::HashMap;

use ::quote::quote;
use proc_macro2::TokenStream;
use rstest::rstest;

use crate::protocol::parser::{field::Vis, Field, MajorMinorVersion};

#[rstest]
#[case::public(
    Vis::Public,
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
    Vis::Public,
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
    Vis::Public,
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
    Vis::Private,
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
    Vis::Private,
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
    Vis::Private,
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
fn quote(#[case] vis: Vis, #[case] field: Field, #[case] expected: TokenStream) {
    todo!()
    // assert_eq!(
    //     field.quote(vis, &HashMap::new()).to_string(),
    //     expected.to_string()
    // )
}
