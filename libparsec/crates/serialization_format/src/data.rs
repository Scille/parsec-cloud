// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::types::FieldType;

pub(crate) fn generate_data(data: JsonData) -> TokenStream {
    quote_data(GenData::new(data))
}

//
// JSON format schemas
//

#[derive(Deserialize)]
pub(crate) struct JsonData {
    label: String,
    #[serde(rename = "type")]
    ty: Option<String>,
    other_fields: Vec<JsonDataField>,
}

#[derive(Deserialize)]
struct JsonDataField {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    introduced_in_revision: Option<u32>,
    default: Option<String>,
}

//
// Cooked structures that will generate the code
//

pub(crate) struct GenData {
    label: String,
    ty: Option<String>,
    fields: Vec<GenDataField>,
}

impl GenData {
    fn new(data: JsonData) -> Self {
        GenData {
            label: data.label,
            ty: data.ty,
            fields: data
                .other_fields
                .into_iter()
                .map(|f| GenDataField {
                    name: f.name,
                    ty: FieldType::from_json_type(&f.ty, None),
                    introduced_in_revision: f.introduced_in_revision,
                    default: f.default,
                })
                .collect(),
        }
    }
}

struct GenDataField {
    name: String,
    ty: FieldType,
    introduced_in_revision: Option<u32>,
    default: Option<String>,
}

//
// Code generation
//

pub(crate) fn quote_data(data: GenData) -> TokenStream {
    let name = format_ident!("{}Data", data.label);
    let name_type = format_ident!("{}DataType", data.label);
    let ty = &data.ty;
    let fields: Vec<TokenStream> = data.fields.iter().map(quote_field).collect();

    if let Some(ty) = ty {
        quote! {
            #[serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
            pub struct #name {
                #[serde(rename="type")]
                pub ty: #name_type,
                #(#fields),*
            }

            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
            pub struct #name_type;

            impl ::serde::Serialize for #name_type {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(#ty)
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #name_type {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::de::Deserializer<'de>,
                {
                    struct Visitor;

                    impl<'de> serde::de::Visitor<'de> for Visitor {
                        type Value = #name_type;

                        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            formatter.write_str(concat!("the `", #ty, "` string"))
                        }

                        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error,
                        {
                            if v == #ty {
                                Ok(#name_type)
                            } else {
                                Err(serde::de::Error::invalid_type(
                                    serde::de::Unexpected::Str(v),
                                    &self,
                                ))
                            }
                        }
                    }

                    deserializer.deserialize_str(Visitor)
                }
            }
        }
    } else {
        quote! {
            #[serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
            pub struct #name {
                #(#fields),*
            }
        }
    }
}

fn quote_field(field: &GenDataField) -> TokenStream {
    let mut attrs = Vec::<TokenStream>::new();

    let name = match field.name.as_ref() {
        "type" => {
            attrs.push(quote! {#[serde(rename = "type")]});
            format_ident!("ty")
        }
        name => format_ident!("{}", name),
    };

    let ty = if field.introduced_in_revision.is_some() {
        let rust_type = field.ty.to_rust_type();
        attrs.push(quote! {
            #[serde(default, skip_serializing_if = "libparsec_types::Maybe::is_absent")]
        });
        quote! {libparsec_types::Maybe<#rust_type>}
    } else {
        field.ty.to_rust_type()
    };

    let can_only_be_null = matches!(field.ty, FieldType::RequiredOption(_));
    // let can_be_missing_or_null = matches!(field.ty, FieldType::NonRequiredOption(_));
    let serde_as_allow_default = if can_only_be_null {
        quote! { no_default }
    } else {
        quote! {}
    };

    match (
        field.introduced_in_revision.is_some(),
        field.ty.to_serde_as(),
    ) {
        (true, Some(serde_as_type)) => {
            let serde_as_type = quote! { libparsec_types::Maybe<#serde_as_type> }.to_string();
            attrs.push(quote! { #[serde_as(as = #serde_as_type, #serde_as_allow_default)] });
        }
        (true, None) => {
            let serde_as_type = quote! { libparsec_types::Maybe<_> }.to_string();
            attrs.push(quote! { #[serde_as(as = #serde_as_type, #serde_as_allow_default)] });
        }
        (false, Some(serde_as_type)) => {
            let serde_as_type = serde_as_type.to_string();
            attrs.push(quote! { #[serde_as(as = #serde_as_type, #serde_as_allow_default)] });
        }
        _ => (),
    }

    if let Some(default) = &field.default {
        attrs.push(quote! { #[serde(default = #default)] });
    }

    quote! {
        #(#attrs)*
        pub #name: #ty
    }
}
