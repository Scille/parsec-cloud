// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashSet;

use itertools::Itertools;
use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::types::FieldType;

pub(crate) fn generate_data(data: JsonData) -> TokenStream {
    quote_data(GenData::new(data), SerializationImpl::Serde)
}

pub(crate) fn generate_data_with_dynamic_serialization(data: JsonData) -> TokenStream {
    quote_data(GenData::new(data), SerializationImpl::DynamicRmp)
}

#[derive(Clone, Copy)]
enum SerializationImpl {
    Serde,
    DynamicRmp,
}

//
// JSON format schemas
//

#[derive(Deserialize)]
pub(crate) struct JsonData {
    label: String,
    #[serde(rename = "type")]
    ty: String,
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
            // `GenData` can represent a structure without type, however we currently
            // require type for all data schemes.
            ty: Some(data.ty),
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

fn quote_data(data: GenData, serialization_impl: SerializationImpl) -> TokenStream {
    let name = format_ident!("{}Data", data.label);
    let name_type = format_ident!("{}DataType", data.label);
    let ty = &data.ty;
    let fields: Vec<TokenStream> = data
        .fields
        .iter()
        .filter_map(|f| quote_field(f, true))
        .collect();
    let nested_types = quote_data_nested_types(&data.nested_types, serialization_impl);

    let data_derive = match serialization_impl {
        SerializationImpl::Serde => {
            quote! { #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)] }
        }
        SerializationImpl::DynamicRmp => {
            quote! { #[derive(Debug, Clone, ::serde::Deserialize, PartialEq, Eq)] }
        }
    };

    if let Some(ty) = ty {
        let data_serialize_impl = match serialization_impl {
            SerializationImpl::Serde => quote! {},
            SerializationImpl::DynamicRmp => {
                quote_dynamic_struct_serialize_impl(&name, &data.fields, true, true)
            }
        };

        let data_deserialize_impl = match serialization_impl {
            SerializationImpl::Serde => quote! {},
            SerializationImpl::DynamicRmp => {
                quote_dynamic_struct_deserialize_impl(&name, &data.fields, true, true)
            }
        };

        let data_type_serialize_impl = match serialization_impl {
            SerializationImpl::Serde => {
                quote! {
                    impl ::serde::Serialize for #name_type {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(#ty)
                        }
                    }
                }
            }
            SerializationImpl::DynamicRmp => {
                quote! {
                    impl libparsec_types_lite::rmp_serialize::Serialize for #name_type {
                        fn serialize(
                            &self,
                            writer: &mut ::std::vec::Vec<u8>,
                        ) -> Result<(), libparsec_types_lite::rmp_serialize::SerializeError> {
                            libparsec_types_lite::rmp_serialize::encode::write_str(writer, #ty).map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)
                        }
                    }
                }
            }
        };

        let data_type_deserialize_impl = match serialization_impl {
            SerializationImpl::Serde => quote! {},
            SerializationImpl::DynamicRmp => {
                quote! {
                    impl libparsec_types_lite::rmp_serialize::Deserialize for #name_type {
                        fn deserialize(
                            value: libparsec_types_lite::rmp_serialize::ValueRef<'_>,
                        ) -> Result<Self, libparsec_types_lite::rmp_serialize::DeserializeError> {
                            let s = String::deserialize(value)?;
                            if s == #ty {
                                Ok(#name_type)
                            } else {
                                Err(libparsec_types_lite::rmp_serialize::DeserializeError::InvalidValue(
                                    format!("invalid type value `{s}`, expected `{}`", #ty),
                                ))
                            }
                        }
                    }
                }
            }
        };

        quote! {
            #(#nested_types)*

            #[serde_with::serde_as]
            #data_derive
            pub struct #name {
                #[serde(rename="type")]
                pub ty: #name_type,
                #(#fields),*
            }

            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
            pub struct #name_type;

            #data_type_serialize_impl
            #data_type_deserialize_impl

            #data_serialize_impl
            #data_deserialize_impl

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
        let data_serialize_impl = match serialization_impl {
            SerializationImpl::Serde => quote! {},
            SerializationImpl::DynamicRmp => {
                quote_dynamic_struct_serialize_impl(&name, &data.fields, false, true)
            }
        };

        let data_deserialize_impl = match serialization_impl {
            SerializationImpl::Serde => quote! {},
            SerializationImpl::DynamicRmp => {
                quote_dynamic_struct_deserialize_impl(&name, &data.fields, false, true)
            }
        };

        quote! {
            #(#nested_types)*

            #[serde_with::serde_as]
            #data_derive
            pub struct #name {
                #(#fields),*
            }

            #data_serialize_impl
            #data_deserialize_impl
        }
    }
}

fn quote_custom_struct_union(
    name: &str,
    variants: &[GenDataNestedTypeVariant],
    discriminant_field: &str,
    serialization_impl: SerializationImpl,
) -> TokenStream {
    let name = format_ident!("{}", name);
    let variant_field_names = variants
        .iter()
        .map(|variant| {
            serializable_fields(&variant.fields)
                .iter()
                .map(|field| field.name.as_str())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let variants_def = variants.iter().map(|variant| {
        let variant_name = format_ident!("{}", variant.name);
        let value_literal = &variant.discriminant_value;
        let variant_fields = quote_fields(&variant.fields, false);
        if variant_fields.is_empty() {
            quote! {
                #[serde(rename = #value_literal)]
                #variant_name
            }
        } else {
            quote! {
                #[serde(rename = #value_literal)]
                #variant_name {
                    #(#variant_fields),*
                }
            }
        }
    });

    let derive = match serialization_impl {
        SerializationImpl::Serde => {
            quote! { #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)] }
        }
        SerializationImpl::DynamicRmp => {
            quote! { #[derive(Debug, Clone, ::serde::Deserialize, PartialEq, Eq)] }
        }
    };

    let serialize_impl = match serialization_impl {
        SerializationImpl::Serde => quote! {},
        SerializationImpl::DynamicRmp => {
            let variant_serializers = variants.iter().map(|variant| {
                let variant_name = format_ident!("{}", variant.name);
                let value_literal = &variant.discriminant_value;
                let field_writes = quote_dynamic_struct_field_writes_from_bindings(&variant.fields);
                let field_count_expr =
                    quote_dynamic_struct_field_count_expr_from_bindings(&variant.fields);

                if serializable_fields(&variant.fields).is_empty() {
                    quote! {
                        Self::#variant_name => {
                            libparsec_types_lite::rmp_serialize::encode::write_map_len(writer, 1).map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)?;
                            libparsec_types_lite::rmp_serialize::encode::write_str(writer, #discriminant_field).map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)?;
                            libparsec_types_lite::rmp_serialize::encode::write_str(writer, #value_literal).map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)?;
                            Ok(())
                        }
                    }
                } else {
                    let field_names = serializable_fields(&variant.fields)
                        .iter()
                        .map(|field| field_name_to_ident(field.name.as_str()))
                        .collect::<Vec<_>>();
                    quote! {
                        Self::#variant_name { #(#field_names),* } => {
                            let map_len = (1usize + #field_count_expr)
                                .try_into()
                                .map_err(|_| libparsec_types_lite::rmp_serialize::SerializeError::LengthOverflow {
                                    kind: "map",
                                    len: 1usize + #field_count_expr,
                                })?;
                            libparsec_types_lite::rmp_serialize::encode::write_map_len(writer, map_len).map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)?;
                            libparsec_types_lite::rmp_serialize::encode::write_str(writer, #discriminant_field).map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)?;
                            libparsec_types_lite::rmp_serialize::encode::write_str(writer, #value_literal).map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)?;
                            #(#field_writes)*
                            Ok(())
                        }
                    }
                }
            });

            quote! {
                impl libparsec_types_lite::rmp_serialize::Serialize for #name {
                    fn serialize(
                        &self,
                        writer: &mut ::std::vec::Vec<u8>,
                    ) -> Result<(), libparsec_types_lite::rmp_serialize::SerializeError> {
                        match self {
                            #(#variant_serializers),*
                        }
                    }
                }
            }
        }
    };

    let deserialize_impl = match serialization_impl {
        SerializationImpl::Serde => quote! {},
        SerializationImpl::DynamicRmp => {
            let variants_deserialize =
                variants
                    .iter()
                    .zip(variant_field_names.iter())
                    .map(|(variant, field_names)| {
                        let variant_name = format_ident!("{}", variant.name);
                        let value_literal = &variant.discriminant_value;
                        let field_count = field_names.len();

                        if field_count == 0 {
                            quote! {
                                #value_literal => {
                                    Ok(Self::#variant_name)
                                }
                            }
                        } else {
                            let field_idents = field_names
                                .iter()
                                .map(|field_name| field_name_to_ident(field_name))
                                .collect::<Vec<_>>();
                            let field_extractors = serializable_fields(&variant.fields)
                                .into_iter()
                                .map(quote_dynamic_named_field_extract)
                                .collect::<Vec<_>>();
                            quote! {
                                #value_literal => {
                                    #(#field_extractors)*
                                    Ok(Self::#variant_name {
                                        #(#field_idents),*
                                    })
                                }
                            }
                        }
                    });

            quote! {
                impl libparsec_types_lite::rmp_serialize::Deserialize for #name {
                    fn deserialize(
                        value: libparsec_types_lite::rmp_serialize::ValueRef<'_>,
                    ) -> Result<Self, libparsec_types_lite::rmp_serialize::DeserializeError> {
                        let entries = match value {
                            rmpv::ValueRef::Map(entries) => entries,
                            other => {
                                return Err(libparsec_types_lite::rmp_serialize::DeserializeError::InvalidType {
                                    expected: "map",
                                    got: libparsec_types_lite::rmp_serialize::value_kind(&other),
                                })
                            }
                        };

                        let mut obj = std::collections::HashMap::with_capacity(entries.len());
                        for (raw_key, raw_value) in entries {
                            let key = String::deserialize(raw_key)?;
                            obj.insert(key, raw_value);
                        }

                        let discriminant = obj
                            .remove(#discriminant_field)
                            .ok_or(libparsec_types_lite::rmp_serialize::DeserializeError::MissingField(#discriminant_field))?;
                        let discriminant = String::deserialize(discriminant)?;

                        match discriminant.as_str() {
                            #(#variants_deserialize),*,
                            _ => Err(libparsec_types_lite::rmp_serialize::DeserializeError::InvalidValue(
                                format!("unknown discriminant `{}`", discriminant),
                            )),
                        }
                    }
                }
            }
        }
    };

    quote! {
        #[::serde_with::serde_as]
        #derive
        #[serde(tag = #discriminant_field)]
        pub enum #name {
            #(#variants_def),*
        }

        #serialize_impl
        #deserialize_impl
    }
}

fn quote_custom_literal_union(
    name: &str,
    variants: &[(String, String)],
    serialization_impl: SerializationImpl,
) -> TokenStream {
    let name = format_ident!("{}", name);
    let variants_def = variants.iter().map(|(name, value)| {
        let variant_name = format_ident!("{}", name);
        let value_literal = &value;
        quote! {
            #[serde(rename = #value_literal)]
            #variant_name
        }
    });

    let derive = match serialization_impl {
        SerializationImpl::Serde => {
            quote! { #[derive(Debug, Clone, Copy, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq, Hash)] }
        }
        SerializationImpl::DynamicRmp => {
            quote! { #[derive(Debug, Clone, Copy, ::serde::Deserialize, PartialEq, Eq, Hash)] }
        }
    };

    let serialize_impl = match serialization_impl {
        SerializationImpl::Serde => quote! {},
        SerializationImpl::DynamicRmp => {
            let variants_serialize = variants.iter().map(|(name, value)| {
                let variant_name = format_ident!("{}", name);
                quote! {
                    Self::#variant_name => libparsec_types_lite::rmp_serialize::encode::write_str(writer, #value).map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)
                }
            });

            quote! {
                impl libparsec_types_lite::rmp_serialize::Serialize for #name {
                    fn serialize(
                        &self,
                        writer: &mut ::std::vec::Vec<u8>,
                    ) -> Result<(), libparsec_types_lite::rmp_serialize::SerializeError> {
                        match self {
                            #(#variants_serialize),*
                        }
                    }
                }
            }
        }
    };

    let deserialize_impl = match serialization_impl {
        SerializationImpl::Serde => quote! {},
        SerializationImpl::DynamicRmp => {
            let variants_deserialize = variants.iter().map(|(name, value)| {
                let variant_name = format_ident!("{}", name);
                quote! { #value => Ok(Self::#variant_name) }
            });

            quote! {
                impl libparsec_types_lite::rmp_serialize::Deserialize for #name {
                    fn deserialize(
                        value: libparsec_types_lite::rmp_serialize::ValueRef<'_>,
                    ) -> Result<Self, libparsec_types_lite::rmp_serialize::DeserializeError> {
                        let discriminant = String::deserialize(value)?;
                        match discriminant.as_str() {
                            #(#variants_deserialize),*,
                            _ => Err(libparsec_types_lite::rmp_serialize::DeserializeError::InvalidValue(
                                format!("unknown literal discriminant `{}`", discriminant),
                            )),
                        }
                    }
                }
            }
        }
    };

    quote! {
        #[::serde_with::serde_as]
        #derive
        pub enum #name {
            #(#variants_def),*
        }

        #serialize_impl
        #deserialize_impl
    }
}

fn quote_custom_struct(
    name: &str,
    fields: &[GenDataField],
    serialization_impl: SerializationImpl,
) -> TokenStream {
    let name = format_ident!("{}", name);
    let fields_def = quote_fields(fields, true);
    let derive = match serialization_impl {
        SerializationImpl::Serde => {
            quote! { #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)] }
        }
        SerializationImpl::DynamicRmp => {
            quote! { #[derive(Debug, Clone, ::serde::Deserialize, PartialEq, Eq)] }
        }
    };

    let serialize_impl = match serialization_impl {
        SerializationImpl::Serde => quote! {},
        SerializationImpl::DynamicRmp => {
            quote_dynamic_struct_serialize_impl(&name, fields, false, false)
        }
    };

    let deserialize_impl = match serialization_impl {
        SerializationImpl::Serde => quote! {},
        SerializationImpl::DynamicRmp => {
            quote_dynamic_struct_deserialize_impl(&name, fields, false, false)
        }
    };

    quote! {
        #[::serde_with::serde_as]
        #derive
        pub struct #name {
            #(#fields_def),*
        }

        #serialize_impl
        #deserialize_impl
    }
}

fn quote_data_nested_types(
    nested_types: &[GenDataNestedType],
    serialization_impl: SerializationImpl,
) -> Vec<TokenStream> {
    nested_types
        .iter()
        .map(|nested_type| match nested_type {
            GenDataNestedType::StructsUnion {
                name,
                discriminant_field,
                variants,
                ..
            } => quote_custom_struct_union(
                name,
                variants.as_ref(),
                discriminant_field,
                serialization_impl,
            ),
            GenDataNestedType::LiteralsUnion { name, variants, .. } => {
                quote_custom_literal_union(name, variants.as_ref(), serialization_impl)
            }
            GenDataNestedType::Struct { name, fields, .. } => {
                quote_custom_struct(name, fields.as_ref(), serialization_impl)
            }
        })
        .collect()
}

fn serializable_fields(fields: &[GenDataField]) -> Vec<&GenDataField> {
    fields
        .iter()
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .filter(|field| field.deprecated_in_revision.is_none())
        .collect()
}

fn field_name_to_ident(name: &str) -> proc_macro2::Ident {
    match name {
        "type" => format_ident!("ty"),
        _ => format_ident!("{}", name),
    }
}

fn quote_dynamic_named_field_extract(field: &GenDataField) -> TokenStream {
    let field_name = field.name.as_str();
    let field_ident = field_name_to_ident(field_name);
    let field_type = field.ty.to_rust_type();

    if field.introduced_in_revision.is_some() {
        quote! {
            let #field_ident: libparsec_types_lite::Maybe<#field_type> = match obj.remove(#field_name) {
                Some(value) => {
                    libparsec_types_lite::Maybe::Present(
                        <#field_type as libparsec_types_lite::rmp_serialize::Deserialize>::deserialize(value)?,
                    )
                }
                None => libparsec_types_lite::Maybe::Absent,
            };
        }
    } else if let Some(default_fn_name) = &field.default {
        let default_ident = format_ident!("{}", default_fn_name);
        quote! {
            let #field_ident: #field_type = match obj.remove(#field_name) {
                Some(value) => <#field_type as libparsec_types_lite::rmp_serialize::Deserialize>::deserialize(value)?,
                None => #default_ident(),
            };
        }
    } else {
        quote! {
            let #field_ident: #field_type = {
                let value = obj
                    .remove(#field_name)
                    .ok_or(libparsec_types_lite::rmp_serialize::DeserializeError::MissingField(#field_name))?;
                <#field_type as libparsec_types_lite::rmp_serialize::Deserialize>::deserialize(value)?
            };
        }
    }
}

fn quote_dynamic_struct_field_writes_from_self(fields: &[GenDataField]) -> Vec<TokenStream> {
    serializable_fields(fields)
        .iter()
        .map(|field| {
            let field_name = field.name.as_str();
            let field_ident = field_name_to_ident(field_name);
            if field.introduced_in_revision.is_some() {
                quote! {
                    if let libparsec_types_lite::Maybe::Present(value) = &self.#field_ident {
                        libparsec_types_lite::rmp_serialize::encode::write_str(writer, #field_name).map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)?;
                        libparsec_types_lite::rmp_serialize::Serialize::serialize(value, writer)?;
                    }
                }
            } else {
                quote! {
                    libparsec_types_lite::rmp_serialize::encode::write_str(writer, #field_name).map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)?;
                    libparsec_types_lite::rmp_serialize::Serialize::serialize(&self.#field_ident, writer)?;
                }
            }
        })
        .collect()
}

fn quote_dynamic_struct_field_count_expr_from_bindings(fields: &[GenDataField]) -> TokenStream {
    let fields = serializable_fields(fields);
    let terms = fields.into_iter().map(|field| {
        let field_ident = field_name_to_ident(field.name.as_str());
        if field.introduced_in_revision.is_some() {
            quote! {
                match #field_ident {
                    libparsec_types_lite::Maybe::Present(_) => 1usize,
                    libparsec_types_lite::Maybe::Absent => 0usize,
                }
            }
        } else {
            quote! { 1usize }
        }
    });
    quote! { 0usize #(+ #terms)* }
}

fn quote_dynamic_struct_field_writes_from_bindings(fields: &[GenDataField]) -> Vec<TokenStream> {
    serializable_fields(fields)
        .iter()
        .map(|field| {
            let field_name = field.name.as_str();
            let field_ident = field_name_to_ident(field_name);
            if field.introduced_in_revision.is_some() {
                quote! {
                    if let libparsec_types_lite::Maybe::Present(value) = #field_ident {
                        libparsec_types_lite::rmp_serialize::encode::write_str(writer, #field_name).map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)?;
                        libparsec_types_lite::rmp_serialize::Serialize::serialize(value, writer)?;
                    }
                }
            } else {
                quote! {
                    libparsec_types_lite::rmp_serialize::encode::write_str(writer, #field_name).map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)?;
                    libparsec_types_lite::rmp_serialize::Serialize::serialize(#field_ident, writer)?;
                }
            }
        })
        .collect()
}

fn quote_dynamic_struct_serialize_impl(
    name: &proc_macro2::Ident,
    fields: &[GenDataField],
    has_data_type_field: bool,
    add_dump_method: bool,
) -> TokenStream {
    let field_writes = quote_dynamic_struct_field_writes_from_self(fields);
    let dynamic_count_terms = serializable_fields(fields)
        .iter()
        .filter(|field| field.introduced_in_revision.is_some())
        .map(|field| {
            let field_ident = field_name_to_ident(field.name.as_str());
            quote! {
                if matches!(self.#field_ident, libparsec_types_lite::Maybe::Present(_)) {
                    map_len += 1;
                }
            }
        })
        .collect::<Vec<_>>();
    let static_count = serializable_fields(fields)
        .iter()
        .filter(|field| field.introduced_in_revision.is_none())
        .count()
        + usize::from(has_data_type_field);

    let type_field_write = if has_data_type_field {
        quote! {
            libparsec_types_lite::rmp_serialize::encode::write_str(writer, "type").map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)?;
            libparsec_types_lite::rmp_serialize::Serialize::serialize(&self.ty, writer)?;
        }
    } else {
        quote! {}
    };

    let dump_method = if add_dump_method {
        quote! {
            impl #name {
                pub fn dump(&self) -> Result<Vec<u8>, libparsec_types_lite::rmp_serialize::SerializeError> {
                    let mut buff = vec![];
                    libparsec_types_lite::rmp_serialize::Serialize::serialize(self, &mut buff)?;
                    Ok(buff)
                }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        impl libparsec_types_lite::rmp_serialize::Serialize for #name {
            fn serialize(
                &self,
                writer: &mut ::std::vec::Vec<u8>,
            ) -> Result<(), libparsec_types_lite::rmp_serialize::SerializeError> {
                let mut map_len = #static_count;
                #(#dynamic_count_terms)*
                let map_len: u32 = map_len.try_into().map_err(|_| {
                    libparsec_types_lite::rmp_serialize::SerializeError::LengthOverflow {
                        kind: "map",
                        len: map_len,
                    }
                })?;
                libparsec_types_lite::rmp_serialize::encode::write_map_len(writer, map_len).map_err(libparsec_types_lite::rmp_serialize::SerializeError::from)?;
                #type_field_write
                #(#field_writes)*
                Ok(())
            }
        }

        #dump_method
    }
}

fn quote_dynamic_struct_deserialize_impl(
    name: &proc_macro2::Ident,
    fields: &[GenDataField],
    has_data_type_field: bool,
    add_load_method: bool,
) -> TokenStream {
    let extractors = serializable_fields(fields)
        .into_iter()
        .map(quote_dynamic_named_field_extract)
        .collect::<Vec<_>>();
    let field_idents = serializable_fields(fields)
        .iter()
        .map(|field| field_name_to_ident(field.name.as_str()))
        .collect::<Vec<_>>();

    let type_field_extractor = if has_data_type_field {
        quote! {
            let ty = {
                let value = obj
                    .remove("type")
                    .ok_or(libparsec_types_lite::rmp_serialize::DeserializeError::MissingField("type"))?;
                <_ as libparsec_types_lite::rmp_serialize::Deserialize>::deserialize(value)?
            };
        }
    } else {
        quote! {}
    };

    let struct_init = if has_data_type_field {
        quote! {
            Ok(Self {
                ty,
                #(#field_idents),*
            })
        }
    } else {
        quote! {
            Ok(Self {
                #(#field_idents),*
            })
        }
    };

    let load_method = if add_load_method {
        quote! {
            impl #name {
                pub fn load(raw: &[u8]) -> Result<Self, libparsec_types_lite::rmp_serialize::DeserializeError> {
                    libparsec_types_lite::rmp_serialize::from_slice(raw)
                }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        impl libparsec_types_lite::rmp_serialize::Deserialize for #name {
            fn deserialize(
                value: libparsec_types_lite::rmp_serialize::ValueRef<'_>,
            ) -> Result<Self, libparsec_types_lite::rmp_serialize::DeserializeError> {
                let entries = match value {
                    rmpv::ValueRef::Map(entries) => entries,
                    other => {
                        return Err(libparsec_types_lite::rmp_serialize::DeserializeError::InvalidType {
                            expected: "map",
                            got: libparsec_types_lite::rmp_serialize::value_kind(&other),
                        })
                    }
                };

                let mut obj = std::collections::HashMap::with_capacity(entries.len());
                for (raw_key, raw_value) in entries {
                    let key = String::deserialize(raw_key)?;
                    obj.insert(key, raw_value);
                }

                #type_field_extractor
                #(#extractors)*

                #struct_init
            }
        }

        #load_method
    }
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
            #[serde(default, skip_serializing_if = "libparsec_types_lite::Maybe::is_absent")]
        });
        quote! {libparsec_types_lite::Maybe<#rust_type>}
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
            let serde_as_type = quote! { libparsec_types_lite::Maybe<#serde_as_type> }.to_string();
            attrs.push(quote! { #[serde_as(as = #serde_as_type, #serde_as_allow_default)] });
        }
        (true, None) => {
            let serde_as_type = quote! { libparsec_types_lite::Maybe<_> }.to_string();
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
