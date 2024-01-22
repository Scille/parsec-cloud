// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashSet;

use itertools::Itertools;
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
    nested_types: Option<Vec<JsonNestedType>>,
    // Field is only required if the data always had it
    introduced_in_revision: Option<u32>,
    // Once deprecated, the field is ignored (we keep it only as documentation)
    deprecated_in_revision: Option<u32>,
}

#[derive(Deserialize, Clone)]
struct JsonDataField {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    introduced_in_revision: Option<u32>,
    deprecated_in_revision: Option<u32>,
    default: Option<String>,
}

#[derive(Deserialize, Clone)]
struct JsonNestedTypeVariant {
    name: String,
    discriminant_value: String,
    fields: Option<Vec<JsonDataField>>,
}

#[derive(Deserialize, Clone)]
struct JsonNestedType {
    name: String,
    variants: Option<Vec<JsonNestedTypeVariant>>,
    discriminant_field: Option<String>,
    fields: Option<Vec<JsonDataField>>,
    // Field is only required if the data always had it
    introduced_in_revision: Option<u32>,
    // Once deprecated, the field is ignored (we keep it only as documentation)
    deprecated_in_revision: Option<u32>,
}

//
// Cooked structures that will generate the code
//

pub(crate) struct GenData {
    label: String,
    ty: Option<String>,
    fields: Vec<GenDataField>,
    nested_types: Vec<GenDataNestedType>,
    #[allow(unused)]
    introduced_in_revision: Option<u32>,
    #[allow(unused)]
    deprecated_in_revision: Option<u32>,
}

impl GenData {
    fn new(data: JsonData) -> Self {
        // `allowed_extra_types` is needed for each field to be parsed (given a field
        // may reference a type defined in `nested_types`)
        let allowed_extra_types: HashSet<String> = data
            .nested_types
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|nt| nt.name.clone())
            .collect();

        let convert_field = move |field: JsonDataField| GenDataField {
            name: field.name,
            ty: FieldType::from_json_type(&field.ty, Some(&allowed_extra_types)),
            introduced_in_revision: field.introduced_in_revision,
            deprecated_in_revision: field.deprecated_in_revision,
            default: field.default,
        };

        let gen_nested_types = data.nested_types.unwrap_or_default().into_iter().map(|nested_type| {
                match (nested_type.fields, nested_type.variants, nested_type.discriminant_field) {
                    (None, Some(variants), None) => {
                        let variants = variants.into_iter().map(|v| {
                            assert!(v.fields.is_none(), "`{}::{}` is supposed to be a literal union, but has fields ! (missing `discriminant_field` ?)", &nested_type.name, v.discriminant_value);
                            (v.name, v.discriminant_value)
                        }).collect();
                        GenDataNestedType::LiteralsUnion {
                            name: nested_type.name,
                            variants,
                            deprecated_in_revision: nested_type.deprecated_in_revision,
                            introduced_in_revision: nested_type.introduced_in_revision,
                        }
                    }
                    (None, Some(variants), Some(discriminant_field)) => {
                        let variants = variants.into_iter().map(|v| {
                            let fields = v.fields.unwrap_or_default();
                            GenDataNestedTypeVariant {
                                name: v.name,
                                discriminant_value: v.discriminant_value,
                                fields: fields.into_iter()
                                    .map(&convert_field)
                                    .collect(),
                            }
                        }).collect();
                        GenDataNestedType::StructsUnion {
                            name: nested_type.name,
                            discriminant_field,
                            variants,
                            deprecated_in_revision: nested_type.deprecated_in_revision,
                            introduced_in_revision: nested_type.introduced_in_revision,
                        }
                    },
                    (Some(fields), None, None) => {
                        GenDataNestedType::Struct {
                            name: nested_type.name,
                            fields: fields
                                .into_iter()
                                .map(&convert_field)
                                .collect(),
                            deprecated_in_revision: nested_type.deprecated_in_revision,
                            introduced_in_revision: nested_type.introduced_in_revision,
                        }
                    }
                    _ => {
                        panic!("Nested type {:?} is neither union nor struct. Union should have `variants`&`discriminant_field`, Struct should have `fields`.", &nested_type.name);
                    },
                }
            }).collect();

        GenData {
            label: data.label,
            ty: data.ty,
            fields: data.other_fields.into_iter().map(convert_field).collect(),
            nested_types: gen_nested_types,
            introduced_in_revision: data.introduced_in_revision,
            deprecated_in_revision: data.deprecated_in_revision,
        }
    }
}

struct GenDataField {
    name: String,
    ty: FieldType,
    introduced_in_revision: Option<u32>,
    deprecated_in_revision: Option<u32>,
    default: Option<String>,
}
struct GenDataNestedTypeVariant {
    name: String,
    discriminant_value: String,
    fields: Vec<GenDataField>,
}

enum GenDataNestedType {
    StructsUnion {
        name: String,
        discriminant_field: String,
        variants: Vec<GenDataNestedTypeVariant>,
        #[allow(unused)]
        introduced_in_revision: Option<u32>,
        #[allow(unused)]
        deprecated_in_revision: Option<u32>,
    },
    LiteralsUnion {
        name: String,
        variants: Vec<(String, String)>,
        #[allow(unused)]
        introduced_in_revision: Option<u32>,
        #[allow(unused)]
        deprecated_in_revision: Option<u32>,
    },
    Struct {
        name: String,
        fields: Vec<GenDataField>,
        #[allow(unused)]
        introduced_in_revision: Option<u32>,
        #[allow(unused)]
        deprecated_in_revision: Option<u32>,
    },
}

//
// Code generation
//

pub(crate) fn quote_data(data: GenData) -> TokenStream {
    let name = format_ident!("{}Data", data.label);
    let name_type = format_ident!("{}DataType", data.label);
    let ty = &data.ty;
    let fields: Vec<TokenStream> = data
        .fields
        .iter()
        .filter_map(|f| quote_field(f, true))
        .collect();
    let nested_types = quote_data_nested_types(&data.nested_types);

    if let Some(ty) = ty {
        quote! {
            #(#nested_types)*

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
            #(#nested_types)*

            #[serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
            pub struct #name {
                #(#fields),*
            }
        }
    }
}

fn quote_custom_struct_union(
    name: &str,
    variants: &[GenDataNestedTypeVariant],
    discriminant_field: &str,
) -> TokenStream {
    let name = format_ident!("{}", name);
    let variants = variants.iter().map(|variant| {
        let variant_name = format_ident!("{}", variant.name);
        let value_literal = &variant.discriminant_value;
        if variant.fields.is_empty() {
            quote! {
                #[serde(rename = #value_literal)]
                #variant_name
            }
        } else {
            let variant_fields = quote_fields(&variant.fields, false);
            quote! {
                #[serde(rename = #value_literal)]
                #variant_name {
                    #(#variant_fields),*
                }
            }
        }
    });
    quote! {
        #[::serde_with::serde_as]
        #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
        #[serde(tag = #discriminant_field)]
        pub enum #name {
            #(#variants),*
        }
    }
}

fn quote_custom_literal_union(name: &str, variants: &[(String, String)]) -> TokenStream {
    let name = format_ident!("{}", name);
    let variants = variants.iter().map(|(name, value)| {
        let variant_name = format_ident!("{}", name);
        let value_literal = &value;
        quote! {
            #[serde(rename = #value_literal)]
            #variant_name
        }
    });

    quote! {
        #[::serde_with::serde_as]
        #[derive(Debug, Clone, Copy, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq, Hash)]
        pub enum #name {
            #(#variants),*
        }
    }
}

fn quote_custom_struct(name: &str, fields: &[GenDataField]) -> TokenStream {
    let name = format_ident!("{}", name);
    let fields = quote_fields(fields, true);
    quote! {
        #[::serde_with::serde_as]
        #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
        pub struct #name {
            #(#fields),*
        }
    }
}

fn quote_data_nested_types(nested_types: &[GenDataNestedType]) -> Vec<TokenStream> {
    nested_types
        .iter()
        .map(|nested_type| match nested_type {
            GenDataNestedType::StructsUnion {
                name,
                discriminant_field,
                variants,
                ..
            } => quote_custom_struct_union(name, variants.as_ref(), discriminant_field),
            GenDataNestedType::LiteralsUnion { name, variants, .. } => {
                quote_custom_literal_union(name, variants.as_ref())
            }
            GenDataNestedType::Struct { name, fields, .. } => {
                quote_custom_struct(name, fields.as_ref())
            }
        })
        .collect()
}

fn quote_fields(fields: &[GenDataField], with_pub: bool) -> Vec<TokenStream> {
    fields
        .iter()
        // The fields will be sorted by their name in binary mode.
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .filter_map(|x| quote_field(x, with_pub))
        .collect()
}

fn quote_field(field: &GenDataField, with_pub: bool) -> Option<TokenStream> {
    // Deprecated field are just ignored
    if field.deprecated_in_revision.is_some() {
        return None;
    }

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

    let token_stream = if with_pub {
        quote! {
            #(#attrs)*
            pub #name: #ty
        }
    } else {
        quote! {
            #(#attrs)*
            #name: #ty
        }
    };

    Some(token_stream)
}
