// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(feature = "python-bindings-support")]
#[path = "./protocol_python_bindings.rs"]
pub(crate) mod python_bindings;

#[path = "./protocol_tests.rs"]
pub(crate) mod cmds_tests_generator;

use itertools::Itertools;
use miniserde::Deserialize;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashSet;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::types::FieldType;
use crate::utils::snake_to_pascal_case;

fn parse_api_version(raw: &str) -> Result<(u32, u32), &'static str> {
    let mut splitted = raw.splitn(2, '.');
    let major = splitted.next().and_then(|x| x.parse::<u32>().ok());
    let minor = splitted.next().and_then(|x| x.parse::<u32>().ok());
    match (major, minor, splitted.next()) {
        (Some(major), Some(minor), None) => Ok((major, minor)),
        _ => Err("Bad API version, must be e.g. `1.2`"),
    }
}

pub(crate) fn generate_protocol_cmds_family(
    cmds: Vec<JsonCmd>,
    family_name: String,
) -> TokenStream {
    quote_cmds_family(
        &GenCmdsFamily::new(cmds, family_name, ReuseSchemaStrategy::Default),
        SerializationImpl::Serde,
    )
}

pub(crate) fn generate_protocol_cmds_family_with_dynamic_serialization(
    cmds: Vec<JsonCmd>,
    family_name: String,
) -> TokenStream {
    quote_cmds_family(
        &GenCmdsFamily::new(cmds, family_name, ReuseSchemaStrategy::Default),
        SerializationImpl::DynamicRmp,
    )
}

#[derive(Clone, Copy)]
enum SerializationImpl {
    Serde,
    DynamicRmp,
}

//
// JSON format schemas
//

#[derive(Deserialize, Clone)]
pub(crate) struct JsonCmd {
    // Miniserde only supports struct as root, hence this field that is never
    // present in the actual json files: it will be added just before
    // deserialization by the parsec_protocol_family macro !
    items: Vec<JsonCmdFlavour>,
}

#[derive(Deserialize, Clone)]
struct JsonCmdFlavour {
    major_versions: Vec<u32>,
    #[allow(unused)]
    introduced_in: Option<String>,
    cmd: String,
    req: JsonCmdReq,
    reps: Vec<JsonCmdRep>,
    nested_types: Option<Vec<JsonNestedType>>,
}

#[derive(Deserialize, Clone)]
struct JsonNestedTypeVariant {
    name: String,
    discriminant_value: String,
    fields: Option<Vec<JsonCmdField>>,
}

#[derive(Deserialize, Clone)]
struct JsonNestedType {
    name: String,
    variants: Option<Vec<JsonNestedTypeVariant>>,
    discriminant_field: Option<String>,
    fields: Option<Vec<JsonCmdField>>,
}

#[derive(Deserialize, Clone)]
struct JsonCmdReq {
    fields: Option<Vec<JsonCmdField>>,
    unit: Option<String>,
}

#[derive(Deserialize, Clone)]
struct JsonCmdRep {
    status: String,
    fields: Option<Vec<JsonCmdField>>,
    unit: Option<String>,
    // In which API version the current response type was introduced.
    // Must be in `X.Y` format
    #[allow(unused)]
    introduced_in: Option<String>,
}

#[derive(Deserialize, Clone)]
struct JsonCmdField {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    // In which API version the current field was introduced.
    // Must be in `X.Y` format
    introduced_in: Option<String>,
}

//
// Cooked structures that will generate the code
//

struct GenCmd {
    cmd: String,
    spec: GenCmdSpec,
}

enum GenCmdSpec {
    Original {
        req: GenCmdReq,
        reps: Vec<GenCmdRep>,
        nested_types: Vec<GenCmdNestedType>,
    },
    ReusedFromVersion {
        version: u32,
    },
}
enum GenCmdReqKind {
    Unit { nested_type_name: String },
    Composed { fields: Vec<GenCmdField> },
}

struct GenCmdReq {
    cmd: String,
    kind: GenCmdReqKind,
}

enum GenCmdRepKind {
    Unit { nested_type_name: String },
    Composed { fields: Vec<GenCmdField> },
}

struct GenCmdRep {
    // TODO: do we also need an `introduced_in` field for response status ?
    status: String,
    kind: GenCmdRepKind,
}

struct GenCmdField {
    name: String,
    ty: FieldType,
    // Field is required if the command always had it, or if
    // it was introduced in a previous major version of the API
    added_in_minor_revision: bool,
}

struct GenCmdNestedTypeVariant {
    name: String,
    discriminant_value: String,
    fields: Vec<GenCmdField>,
}

enum GenCmdNestedType {
    StructsUnion {
        name: String,
        discriminant_field: String,
        variants: Vec<GenCmdNestedTypeVariant>,
    },
    LiteralsUnion {
        name: String,
        variants: Vec<(String, String)>,
    },
    Struct {
        name: String,
        fields: Vec<GenCmdField>,
    },
}

struct GenCmdsFamily {
    pub name: String,
    pub versions: HashMap<u32, Vec<GenCmd>>,
}

fn field_name_to_ident(name: &str) -> proc_macro2::Ident {
    match name {
        "type" => format_ident!("ty"),
        _ => format_ident!("{}", name),
    }
}

fn quote_dynamic_cmd_named_field_extract(field: &GenCmdField) -> TokenStream {
    let field_name = field.name.as_str();
    let field_ident = field_name_to_ident(field_name);
    let field_type = field.ty.to_rust_type();

    if field.added_in_minor_revision {
        quote! {
            let #field_ident: libparsec_types::Maybe<#field_type> = match obj.remove(#field_name) {
                Some(value) => {
                    libparsec_types::Maybe::Present(
                        <#field_type as libparsec_types::rmp_serialize::Deserialize>::deserialize(value)?,
                    )
                }
                None => libparsec_types::Maybe::Absent,
            };
        }
    } else {
        quote! {
            let #field_ident: #field_type = {
                let value = obj
                    .remove(#field_name)
                    .ok_or(libparsec_types::rmp_serialize::DeserializeError::MissingField(#field_name))?;
                <#field_type as libparsec_types::rmp_serialize::Deserialize>::deserialize(value)?
            };
        }
    }
}

fn quote_dynamic_cmd_field_writes_from_self(fields: &[GenCmdField]) -> Vec<TokenStream> {
    fields
        .iter()
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .map(|field| {
            let field_name = field.name.as_str();
            let field_ident = field_name_to_ident(field_name);
            if field.added_in_minor_revision {
                quote! {
                    if let libparsec_types::Maybe::Present(value) = &self.#field_ident {
                        libparsec_types::rmp_serialize::encode::write_str(writer, #field_name).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                        libparsec_types::rmp_serialize::Serialize::serialize(value, writer)?;
                    }
                }
            } else {
                quote! {
                    libparsec_types::rmp_serialize::encode::write_str(writer, #field_name).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                    libparsec_types::rmp_serialize::Serialize::serialize(&self.#field_ident, writer)?;
                }
            }
        })
        .collect()
}

fn quote_dynamic_cmd_field_count_expr_from_bindings(fields: &[GenCmdField]) -> TokenStream {
    let terms = fields
        .iter()
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .map(|field| {
            let field_ident = field_name_to_ident(field.name.as_str());
            if field.added_in_minor_revision {
                quote! {
                    match #field_ident {
                        libparsec_types::Maybe::Present(_) => 1usize,
                        libparsec_types::Maybe::Absent => 0usize,
                    }
                }
            } else {
                quote! { 1usize }
            }
        });
    quote! { 0usize #(+ #terms)* }
}

fn quote_dynamic_cmd_field_writes_from_bindings(fields: &[GenCmdField]) -> Vec<TokenStream> {
    fields
        .iter()
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .map(|field| {
            let field_name = field.name.as_str();
            let field_ident = field_name_to_ident(field_name);
            if field.added_in_minor_revision {
                quote! {
                    if let libparsec_types::Maybe::Present(value) = #field_ident {
                        libparsec_types::rmp_serialize::encode::write_str(writer, #field_name).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                        libparsec_types::rmp_serialize::Serialize::serialize(value, writer)?;
                    }
                }
            } else {
                quote! {
                    libparsec_types::rmp_serialize::encode::write_str(writer, #field_name).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                    libparsec_types::rmp_serialize::Serialize::serialize(#field_ident, writer)?;
                }
            }
        })
        .collect()
}

fn quote_dynamic_cmd_struct_serialize_impl(
    name: &proc_macro2::Ident,
    fields: &[GenCmdField],
) -> TokenStream {
    let field_writes = quote_dynamic_cmd_field_writes_from_self(fields);
    let dynamic_count_terms = fields
        .iter()
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .filter(|field| field.added_in_minor_revision)
        .map(|field| {
            let field_ident = field_name_to_ident(field.name.as_str());
            quote! {
                if matches!(self.#field_ident, libparsec_types::Maybe::Present(_)) {
                    map_len += 1;
                }
            }
        })
        .collect::<Vec<_>>();
    let static_count = fields
        .iter()
        .filter(|field| !field.added_in_minor_revision)
        .count();

    quote! {
        impl libparsec_types::rmp_serialize::Serialize for #name {
            fn serialize(
                &self,
                writer: &mut ::std::vec::Vec<u8>,
            ) -> Result<(), libparsec_types::rmp_serialize::SerializeError> {
                let mut map_len = #static_count;
                #(#dynamic_count_terms)*
                let map_len: u32 = map_len.try_into().map_err(|_| {
                    libparsec_types::rmp_serialize::SerializeError::LengthOverflow {
                        kind: "map",
                        len: map_len,
                    }
                })?;
                libparsec_types::rmp_serialize::encode::write_map_len(writer, map_len).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                #(#field_writes)*
                Ok(())
            }
        }
    }
}

fn quote_dynamic_cmd_struct_deserialize_impl(
    name: &proc_macro2::Ident,
    fields: &[GenCmdField],
) -> TokenStream {
    let extractors = fields
        .iter()
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .map(quote_dynamic_cmd_named_field_extract)
        .collect::<Vec<_>>();
    let field_idents = fields
        .iter()
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .map(|field| field_name_to_ident(field.name.as_str()))
        .collect::<Vec<_>>();

    quote! {
        impl libparsec_types::rmp_serialize::Deserialize for #name {
            fn deserialize(
                value: libparsec_types::rmp_serialize::ValueRef<'_>,
            ) -> Result<Self, libparsec_types::rmp_serialize::DeserializeError> {
                let entries = match value {
                    rmpv::ValueRef::Map(entries) => entries,
                    other => {
                        return Err(libparsec_types::rmp_serialize::DeserializeError::InvalidType {
                            expected: "map",
                            got: libparsec_types::rmp_serialize::value_kind(&other),
                        })
                    }
                };

                let mut obj = std::collections::HashMap::with_capacity(entries.len());
                for (raw_key, raw_value) in entries {
                    let key = String::deserialize(raw_key)?;
                    obj.insert(key, raw_value);
                }

                #(#extractors)*

                Ok(Self {
                    #(#field_idents),*
                })
            }
        }
    }
}

fn quote_unknown_status_deserialize_impl(serialization_impl: SerializationImpl) -> TokenStream {
    match serialization_impl {
        SerializationImpl::Serde => quote! {},
        SerializationImpl::DynamicRmp => {
            quote! {
                impl libparsec_types::rmp_serialize::Deserialize for UnknownStatus {
                    fn deserialize(
                        value: libparsec_types::rmp_serialize::ValueRef<'_>,
                    ) -> Result<Self, libparsec_types::rmp_serialize::DeserializeError> {
                        let entries = match value {
                            rmpv::ValueRef::Map(entries) => entries,
                            other => {
                                return Err(libparsec_types::rmp_serialize::DeserializeError::InvalidType {
                                    expected: "map",
                                    got: libparsec_types::rmp_serialize::value_kind(&other),
                                })
                            }
                        };

                        let mut obj = std::collections::HashMap::with_capacity(entries.len());
                        for (raw_key, raw_value) in entries {
                            let key = String::deserialize(raw_key)?;
                            obj.insert(key, raw_value);
                        }

                        let status = {
                            let value = obj
                                .remove("status")
                                .ok_or(libparsec_types::rmp_serialize::DeserializeError::MissingField("status"))?;
                            String::deserialize(value)?
                        };
                        let reason: Option<String> = match obj.remove("reason") {
                            Some(value) => Option::<String>::deserialize(value)?,
                            None => None,
                        };

                        Ok(Self { status, reason })
                    }
                }
            }
        }
    }
}

//
// JSON-to-gen parsing & struct conversion
//

/// Strategy for JSON schema reuse between protocol versions.
///
/// When a protocol command does not change between major protocol versions,
/// its schema will not change either. In those cases, the code can be reused.
enum ReuseSchemaStrategy {
    /// The default behaviour is to reuse schemas if possible
    /// (e.g. when no change was introduced in a minor version)
    Default,

    /// Force to disable schema reuse
    Never,
}

impl GenCmdsFamily {
    fn new(cmds: Vec<JsonCmd>, family_name: String, reuse_schemas: ReuseSchemaStrategy) -> Self {
        let mut gen_versions: HashMap<u32, Vec<GenCmd>> = HashMap::new();
        let mut version_cmd_couples: HashSet<(u32, String)> = HashSet::new();

        // Main work here is to "un-factorize the command": we convert a JsonCmdFlavour
        // struct that is marked as `major_versions: [1, 2]` into two separate GenCmd
        // structs, or into a single GenCmd referenced twice if the command doesn't
        // change between API versions.

        for cmd in cmds.into_iter().flat_map(|json_cmd| json_cmd.items) {
            let JsonCmdFlavour {
                major_versions,
                req,
                reps,
                nested_types,
                cmd: cmd_name,
                ..
            } = cmd;
            assert!(
                !major_versions.is_empty(),
                "{cmd_name}: major_versions field cannot be empty !",
            );

            // We don't try to play smart here: if `introduced_in` is set
            // somewhere we consider the schema cannot be shared across
            // versions.
            // Cleaver optimizations are possible but it is not needed
            // for the moment given how few schema with `introduced_in`
            // we have for the moment.
            let has_introduced_in_field = RefCell::new(false);

            let mut can_reuse_schema_from_version: Option<u32> = None;
            for major_version in major_versions {
                // Now it's time to convert the Json struct into a Gen one !

                // `allowed_extra_types` is needed for each field to be parsed (given a field
                // may reference a type defined in `nested_types`)
                let allowed_extra_types: Rc<HashSet<String>> = Rc::new({
                    match nested_types.as_ref() {
                        None => HashSet::new(),
                        Some(nested_types) => {
                            nested_types.iter().map(|nt| nt.name.clone()).collect()
                        }
                    }
                });

                let convert_field = |field: JsonCmdField| {
                    // Field can be omitted in a schema if it has been added in a minor
                    // version revision (e.g. field added in APIv1.1, we must allow it
                    // to be missing to still be compatible with APIv1.0)
                    let added_in_minor_revision = match &field.introduced_in {
                        Some(introduced_in) => {
                            has_introduced_in_field.replace(true);
                            match parse_api_version(introduced_in) {
                                Ok((introduced_in_major, _)) => {
                                    if introduced_in_major > major_version {
                                        // Field has been added in a subsequent major version,
                                        // hence our current major version doesn't know about it !
                                        return None;
                                    }
                                    introduced_in_major == major_version
                                }
                                Err(err) => panic!("{cmd_name}: {err:?}"),
                            }
                        }
                        None => false,
                    };
                    Some(GenCmdField {
                        name: field.name.clone(),
                        ty: FieldType::from_json_type(
                            &field.ty,
                            Some(allowed_extra_types.as_ref()),
                        ),
                        added_in_minor_revision,
                    })
                };

                let gen_nested_types = {
                    if let Some(nested_types) = nested_types.clone() {
                        nested_types.into_iter().map(|nested_type| {
                            match (nested_type.fields, nested_type.variants, nested_type.discriminant_field) {
                                (None, Some(variants), None) => {
                                    let variants = variants.into_iter().map(|v| {
                                        assert!(v.fields.is_none(), "{}: `{}::{}` is supposed to be a literal union, but has fields ! (missing `discriminant_field` ?)", cmd_name, &nested_type.name, v.discriminant_value);
                                        (v.name, v.discriminant_value)
                                    }).collect();
                                    GenCmdNestedType::LiteralsUnion {
                                        name: nested_type.name,
                                        variants,
                                    }
                                }
                                (None, Some(variants), Some(discriminant_field)) => {
                                    let variants = variants.into_iter().map(|v| {
                                        let fields = v.fields.unwrap_or_default();
                                        GenCmdNestedTypeVariant {
                                            name: v.name,
                                            discriminant_value: v.discriminant_value,
                                            fields: fields.into_iter()
                                                .filter_map(&convert_field)
                                                .collect(),
                                        }
                                    }).collect();
                                    GenCmdNestedType::StructsUnion {
                                        name: nested_type.name,
                                        discriminant_field,
                                        variants,
                                    }
                                },
                                (Some(fields), None, None) => {
                                    GenCmdNestedType::Struct {
                                        name: nested_type.name,
                                        fields: fields
                                            .into_iter()
                                            .filter_map(&convert_field)
                                            .collect(),
                                    }
                                }
                                _ => {
                                    panic!("{}: Nested type {:?} is neither union nor struct. Union should have `variants`&`discriminant_field`, Struct should have `fields`.", cmd_name, &nested_type.name);
                                },
                            }
                        }).collect()
                    } else {
                        vec![]
                    }
                };

                let JsonCmdReq {
                    fields: req_fields,
                    unit: req_unit,
                } = &req;
                let gen_req_kind = match (req_unit, req_fields) {
                    (Some(nested_type_name), None) => GenCmdReqKind::Unit {
                        nested_type_name: nested_type_name.to_owned(),
                    },
                    (None, Some(fields)) => GenCmdReqKind::Composed {
                        fields: fields.iter().cloned().filter_map(&convert_field).collect(),
                    },
                    (None, None) => GenCmdReqKind::Composed { fields: vec![] },
                    _ => panic!("{cmd_name}: `unit`/`fields` are mutually exclusives in req",),
                };
                let gen_req = GenCmdReq {
                    cmd: cmd_name.to_owned(),
                    kind: gen_req_kind,
                };

                let gen_reps = reps
                    .clone()
                    .into_iter()
                    .map(|rep| {
                        let kind: GenCmdRepKind = match (&rep.unit, rep.fields) {
                            (Some(nested_type_name), None) => GenCmdRepKind::Unit {
                                nested_type_name: nested_type_name.to_owned(),
                            },
                            (None, Some(fields)) => GenCmdRepKind::Composed {
                                fields: fields.into_iter().filter_map(&convert_field).collect(),
                            },
                            (None, None) => GenCmdRepKind::Composed { fields: vec![] },
                            _ => {
                                panic!(
                                    "{}: Status {:?} must have `unit` or `fields` field",
                                    cmd_name, &rep.status
                                );
                            }
                        };
                        GenCmdRep {
                            status: rep.status,
                            kind,
                        }
                    })
                    .collect();

                let gen_cmd = match (
                    &reuse_schemas,
                    *has_introduced_in_field.borrow(),
                    can_reuse_schema_from_version,
                ) {
                    // Schema are never reused in some cases (i.e. protocol tests)
                    (ReuseSchemaStrategy::Never, _, _) => GenCmd {
                        cmd: cmd_name.to_owned(),
                        spec: GenCmdSpec::Original {
                            req: gen_req,
                            reps: gen_reps,
                            nested_types: gen_nested_types,
                        },
                    },
                    (_, true, _) => {
                        // Never reuse schema that contains `introduced_in` field
                        assert!(can_reuse_schema_from_version.is_none()); // Sanity check
                        GenCmd {
                            cmd: cmd_name.to_owned(),
                            spec: GenCmdSpec::Original {
                                req: gen_req,
                                reps: gen_reps,
                                nested_types: gen_nested_types,
                            },
                        }
                    }
                    (_, false, None) => {
                        // First time we see this schema
                        can_reuse_schema_from_version.replace(major_version);
                        GenCmd {
                            cmd: cmd_name.to_owned(),
                            spec: GenCmdSpec::Original {
                                req: gen_req,
                                reps: gen_reps,
                                nested_types: gen_nested_types,
                            },
                        }
                    }
                    (_, false, Some(version)) => GenCmd {
                        cmd: cmd_name.to_owned(),
                        spec: GenCmdSpec::ReusedFromVersion { version },
                    },
                };

                assert!(
                    version_cmd_couples.insert((major_version, cmd_name.to_owned())),
                    "APIv{major_version:?} has multiple implementations of {cmd_name:?} !"
                );
                gen_versions.entry(major_version).or_default().push(gen_cmd);
            }
        }

        Self {
            name: family_name,
            versions: gen_versions,
        }
    }
}

//
// Code generation
//

fn quote_cmds_family(family: &GenCmdsFamily, serialization_impl: SerializationImpl) -> TokenStream {
    let family_mod_name = format_ident!("{}_cmds", &family.name);
    let mut latest_cmds_mod_name = None;

    let versioned_cmds: Vec<_> = family
        .versions
        .iter()
        .sorted_by_key(|(v, _)| *v)
        .map(|(version, cmds)| {
            let (mod_name, code) =
                quote_versioned_cmds(&family.name, *version, cmds, serialization_impl);
            latest_cmds_mod_name.replace(mod_name);
            code
        })
        .collect();

    let latest_cmds_mod_name = latest_cmds_mod_name.expect("at least one version exists");
    let unknown_status_deserialize_impl = quote_unknown_status_deserialize_impl(serialization_impl);

    quote! {
        pub mod #family_mod_name {
            use super::libparsec_types; // Allow to mock types in tests

            // Define `UnknownStatus` here instead of where is it actually used (i.e.
            // near each command's `Rep::load` definition) to have a single common
            // definition that will have it deserialization code compiled once \o/
            #[derive(::serde::Deserialize)]
            struct UnknownStatus {
                status: String,
                reason: Option<String>
            }

            #unknown_status_deserialize_impl

            #(#versioned_cmds)*

            pub mod latest {
                pub use super::#latest_cmds_mod_name::*;
            }
        }
    }
}

fn quote_versioned_cmds(
    family: &str,
    version: u32,
    cmds: &[GenCmd],
    serialization_impl: SerializationImpl,
) -> (Ident, TokenStream) {
    let versioned_cmds_mod = format_ident!("v{version}");
    let (any_cmd_req_variants, cmd_structs): (Vec<TokenStream>, Vec<TokenStream>) = cmds
        .iter()
        .map(|cmd| quote_cmd(family, version, cmd, serialization_impl))
        .unzip();
    let any_cmd_req_dynamic_deserialize_cases = cmds
        .iter()
        .map(|cmd| {
            let variant_name = format_ident!("{}", snake_to_pascal_case(&cmd.cmd));
            let module_name = format_ident!("{}", &cmd.cmd);
            let cmd_name_literal = cmd.cmd.as_str();
            quote! {
                #cmd_name_literal => Ok(Self::#variant_name(
                    <#module_name::Req as libparsec_types::rmp_serialize::Deserialize>::deserialize(
                        rmpv::ValueRef::Map(entries),
                    )?,
                )),
            }
        })
        .collect::<Vec<_>>();

    let any_cmd_req_deserialize = match serialization_impl {
        SerializationImpl::Serde => quote! {
            #[derive(Debug, Clone, ::serde::Deserialize, PartialEq, Eq)]
            #[serde(tag = "cmd")]
        },
        SerializationImpl::DynamicRmp => quote! {
            #[derive(Debug, Clone, ::serde::Deserialize, PartialEq, Eq)]
        },
    };

    let any_cmd_req_load_impl = match serialization_impl {
        SerializationImpl::Serde => quote! {
            impl AnyCmdReq {
                pub fn load(buf: &[u8]) -> Result<Self, ::rmp_serde::decode::Error> {
                    ::rmp_serde::from_slice(buf)
                }
            }
        },
        SerializationImpl::DynamicRmp => quote! {
            impl AnyCmdReq {
                pub fn load(
                    buf: &[u8],
                ) -> Result<Self, libparsec_types::rmp_serialize::DeserializeError> {
                    libparsec_types::rmp_serialize::from_slice(buf)
                }
            }

            impl libparsec_types::rmp_serialize::Deserialize for AnyCmdReq {
                fn deserialize(
                    value: libparsec_types::rmp_serialize::ValueRef<'_>,
                ) -> Result<Self, libparsec_types::rmp_serialize::DeserializeError> {
                    let entries = match value {
                        rmpv::ValueRef::Map(entries) => entries,
                        other => {
                            return Err(libparsec_types::rmp_serialize::DeserializeError::InvalidType {
                                expected: "map",
                                got: libparsec_types::rmp_serialize::value_kind(&other),
                            })
                        }
                    };

                    let mut cmd: Option<String> = None;
                    for (raw_key, raw_value) in entries.iter() {
                        let key = match raw_key {
                            rmpv::ValueRef::String(s) => s.as_str().ok_or_else(|| {
                                libparsec_types::rmp_serialize::DeserializeError::InvalidValue(
                                    "invalid UTF-8 string".to_owned(),
                                )
                            })?,
                            _ => continue,
                        };
                        if key != "cmd" {
                            continue;
                        }
                        let value = match raw_value {
                            rmpv::ValueRef::String(s) => s.as_str().ok_or_else(|| {
                                libparsec_types::rmp_serialize::DeserializeError::InvalidValue(
                                    "invalid UTF-8 string".to_owned(),
                                )
                            })?,
                            other => {
                                return Err(
                                    libparsec_types::rmp_serialize::DeserializeError::InvalidType {
                                        expected: "string",
                                        got: libparsec_types::rmp_serialize::value_kind(other),
                                    },
                                )
                            }
                        };
                        cmd = Some(value.to_owned());
                        break;
                    }

                    let cmd = cmd
                        .ok_or(libparsec_types::rmp_serialize::DeserializeError::MissingField("cmd"))?;

                    match cmd.as_str() {
                        #(#any_cmd_req_dynamic_deserialize_cases)*
                        _ => Err(libparsec_types::rmp_serialize::DeserializeError::InvalidValue(
                            format!("unknown command `{}`", cmd),
                        )),
                    }
                }
            }
        },
    };

    let code = quote! {
        pub mod #versioned_cmds_mod {
            use super::libparsec_types; // Allow to mock types in tests
            use super::UnknownStatus;

            #any_cmd_req_deserialize
            pub enum AnyCmdReq {
                #(#any_cmd_req_variants),*
            }

            #any_cmd_req_load_impl

            #(#cmd_structs)*
        }
    };

    (versioned_cmds_mod, code)
}

fn quote_cmd(
    family: &str,
    cmd_version: u32,
    cmd: &GenCmd,
    serialization_impl: SerializationImpl,
) -> (TokenStream, TokenStream) {
    let pascal_case_name = &snake_to_pascal_case(&cmd.cmd);
    let snake_case_name = &cmd.cmd;

    let variant_name = format_ident!("{}", pascal_case_name);
    let module_name = format_ident!("{}", snake_case_name);
    let command_name = snake_case_name;

    // `authenticated` -> `Authenticated`
    // `authenticated_account` -> `AuthenticatedAccount`
    let family_enum_variant = format_ident!("{}", snake_to_pascal_case(family));

    let module = match &cmd.spec {
        GenCmdSpec::ReusedFromVersion { version } => {
            let reused_version_module_name = format_ident!("v{}", version);
            let protocol_request_impl = match serialization_impl {
                SerializationImpl::Serde => quote! {
                    impl super::libparsec_types::ProtocolRequest<#cmd_version> for Req {
                        type Response = Rep;
                        const FAMILY: super::libparsec_types::ProtocolFamily = super::libparsec_types::ProtocolFamily::#family_enum_variant;

                        fn api_dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                            self.dump()
                        }

                        fn api_load_response(buf: &[u8]) -> Result<Self::Response, ::rmp_serde::decode::Error> {
                            Self::load_response(buf)
                        }
                    }
                },
                SerializationImpl::DynamicRmp => quote! {
                    impl super::libparsec_types::ProtocolRequest<#cmd_version> for Req {
                        type Response = Rep;
                        const FAMILY: super::libparsec_types::ProtocolFamily = super::libparsec_types::ProtocolFamily::#family_enum_variant;

                        fn api_dump(&self) -> Result<Vec<u8>, super::libparsec_types::rmp_serialize::SerializeError> {
                            self.dump()
                        }

                        fn api_load_response(buf: &[u8]) -> Result<Self::Response, ::rmp_serde::decode::Error> {
                            Self::load_response(buf).map_err(|err| {
                                <::rmp_serde::decode::Error as ::serde::de::Error>::custom(format!("{err:?}"))
                            })
                        }
                    }
                },
            };

            quote! {
                pub mod #module_name {
                    pub use super::super::#reused_version_module_name::#module_name::*;

                    #protocol_request_impl
                }
            }
        }

        GenCmdSpec::Original {
            req,
            reps,
            nested_types,
        } => {
            let struct_req = quote_cmd_req_struct(pascal_case_name, req, serialization_impl);
            let (
                variants_rep,
                status_match,
                dynamic_rep_serialize_arms,
                dynamic_rep_deserialize_cases,
            ) = quote_cmd_rep_variants(reps);
            let nested_types = quote_cmd_nested_types(nested_types, serialization_impl);
            let known_rep_statuses = reps.iter().map(|rep| rep.status.to_owned());

            let req_dump_and_load_response = match serialization_impl {
                SerializationImpl::Serde => quote! {
                    impl Req {
                        pub fn dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                            ::rmp_serde::to_vec_named(self)
                        }

                        pub fn load_response(buf: &[u8]) -> Result<Rep, ::rmp_serde::decode::Error> {
                            Rep::load(buf)
                        }
                    }
                },
                SerializationImpl::DynamicRmp => quote! {
                    impl Req {
                        pub fn dump(&self) -> Result<Vec<u8>, libparsec_types::rmp_serialize::SerializeError> {
                            let mut buff = vec![];
                            libparsec_types::rmp_serialize::Serialize::serialize(self, &mut buff)?;
                            Ok(buff)
                        }

                        pub fn load_response(
                            buf: &[u8],
                        ) -> Result<Rep, libparsec_types::rmp_serialize::DeserializeError> {
                            Rep::load(buf)
                        }
                    }
                },
            };

            let protocol_request_impl = match serialization_impl {
                SerializationImpl::Serde => quote! {
                    impl libparsec_types::ProtocolRequest<#cmd_version> for Req {
                        type Response = Rep;
                        const FAMILY: super::libparsec_types::ProtocolFamily = super::libparsec_types::ProtocolFamily::#family_enum_variant;

                        fn api_dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                            self.dump()
                        }

                        fn api_load_response(buf: &[u8]) -> Result<Self::Response, ::rmp_serde::decode::Error> {
                            Self::load_response(buf)
                        }
                    }
                },
                SerializationImpl::DynamicRmp => quote! {
                    impl libparsec_types::ProtocolRequest<#cmd_version> for Req {
                        type Response = Rep;
                        const FAMILY: super::libparsec_types::ProtocolFamily = super::libparsec_types::ProtocolFamily::#family_enum_variant;

                        fn api_dump(&self) -> Result<Vec<u8>, super::libparsec_types::rmp_serialize::SerializeError> {
                            self.dump()
                        }

                        fn api_load_response(buf: &[u8]) -> Result<Self::Response, ::rmp_serde::decode::Error> {
                            Self::load_response(buf).map_err(|err| {
                                <::rmp_serde::decode::Error as ::serde::de::Error>::custom(format!("{err:?}"))
                            })
                        }
                    }
                },
            };

            let rep_derive = match serialization_impl {
                SerializationImpl::Serde => {
                    quote! { #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq)] }
                }
                SerializationImpl::DynamicRmp => {
                    quote! { #[derive(Debug, Clone, ::serde::Deserialize, PartialEq)] }
                }
            };

            let rep_serialize_impl = match serialization_impl {
                SerializationImpl::Serde => quote! {},
                SerializationImpl::DynamicRmp => quote! {
                    impl libparsec_types::rmp_serialize::Serialize for Rep {
                        fn serialize(
                            &self,
                            writer: &mut ::std::vec::Vec<u8>,
                        ) -> Result<(), libparsec_types::rmp_serialize::SerializeError> {
                            match self {
                                #(#dynamic_rep_serialize_arms),*
                                Self::UnknownStatus {
                                    unknown_status,
                                    reason,
                                } => {
                                    let mut map_len = 1usize;
                                    if reason.is_some() {
                                        map_len += 1;
                                    }
                                    let map_len: u32 = map_len.try_into().map_err(|_| {
                                        libparsec_types::rmp_serialize::SerializeError::LengthOverflow {
                                            kind: "map",
                                            len: map_len,
                                        }
                                    })?;
                                    libparsec_types::rmp_serialize::encode::write_map_len(writer, map_len).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                                    libparsec_types::rmp_serialize::encode::write_str(writer, "status").map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                                    libparsec_types::rmp_serialize::Serialize::serialize(unknown_status, writer)?;
                                    if let Some(reason) = reason {
                                        libparsec_types::rmp_serialize::encode::write_str(writer, "reason").map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                                        libparsec_types::rmp_serialize::Serialize::serialize(reason, writer)?;
                                    }
                                    Ok(())
                                }
                            }
                        }
                    }
                },
            };

            let rep_deserialize_impl = match serialization_impl {
                SerializationImpl::Serde => quote! {},
                SerializationImpl::DynamicRmp => quote! {
                    impl libparsec_types::rmp_serialize::Deserialize for Rep {
                        fn deserialize(
                            value: libparsec_types::rmp_serialize::ValueRef<'_>,
                        ) -> Result<Self, libparsec_types::rmp_serialize::DeserializeError> {
                            let entries = match value {
                                rmpv::ValueRef::Map(entries) => entries,
                                other => {
                                    return Err(libparsec_types::rmp_serialize::DeserializeError::InvalidType {
                                        expected: "map",
                                        got: libparsec_types::rmp_serialize::value_kind(&other),
                                    })
                                }
                            };

                            let mut obj = std::collections::HashMap::with_capacity(entries.len());
                            for (raw_key, raw_value) in entries {
                                let key = String::deserialize(raw_key)?;
                                obj.insert(key, raw_value);
                            }

                            let status = {
                                let value = obj
                                    .remove("status")
                                    .ok_or(libparsec_types::rmp_serialize::DeserializeError::MissingField("status"))?;
                                String::deserialize(value)?
                            };

                            match status.as_str() {
                                #(#dynamic_rep_deserialize_cases,)*
                                _ => {
                                    let reason: Option<String> = match obj.remove("reason") {
                                        Some(value) => Option::<String>::deserialize(value)?,
                                        None => None,
                                    };
                                    Ok(Self::UnknownStatus {
                                        unknown_status: status,
                                        reason,
                                    })
                                }
                            }
                        }
                    }
                },
            };

            let rep_dump_method = match serialization_impl {
                SerializationImpl::Serde => quote! {
                    pub fn dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                        ::rmp_serde::to_vec_named(self)
                    }
                },
                SerializationImpl::DynamicRmp => quote! {
                    pub fn dump(&self) -> Result<Vec<u8>, libparsec_types::rmp_serialize::SerializeError> {
                        let mut buff = vec![];
                        libparsec_types::rmp_serialize::Serialize::serialize(self, &mut buff)?;
                        Ok(buff)
                    }
                },
            };

            let rep_load_method = match serialization_impl {
                SerializationImpl::Serde => quote! {
                    pub fn load(buf: &[u8]) -> Result<Self, ::rmp_serde::decode::Error> {
                        ::rmp_serde::from_slice::<Self>(buf)
                            .or_else(|err| {
                                // Due to how Serde handles variant discriminant, we cannot express unknown
                                // status as a default case in the main schema.
                                // Instead we have this additional deserialization attempt fallback.
                                let data = ::rmp_serde::from_slice::<UnknownStatus>(buf)?;

                                match data.status.as_str() {
                                    #(#known_rep_statuses => Err(err),)*
                                    _ => Ok(Self::UnknownStatus {
                                        unknown_status: data.status,
                                        reason: data.reason,
                                    })
                                }
                            })
                    }
                },
                SerializationImpl::DynamicRmp => quote! {
                    pub fn load(
                        buf: &[u8],
                    ) -> Result<Self, libparsec_types::rmp_serialize::DeserializeError> {
                        libparsec_types::rmp_serialize::from_slice(buf)
                    }
                },
            };

            // The rep struct is exposed as `Rep`, however we want to have it named
            // `<CmdName>Rep` for debug display
            let rep_struct_name = format_ident!("{}Rep", pascal_case_name);
            quote! {
                pub mod #module_name {
                    use super::libparsec_types; // Allow to mock types in tests
                    use super::AnyCmdReq;
                    use super::UnknownStatus;

                    #(#nested_types)*

                    #struct_req

                    #req_dump_and_load_response

                    #protocol_request_impl

                    // Can't derive Eq because some Rep have f64 field
                    #[allow(clippy::derive_partial_eq_without_eq)]
                    #[::serde_with::serde_as]
                    #rep_derive
                    #[serde(tag = "status")]
                    pub enum #rep_struct_name {
                        #(#variants_rep),*
                    }
                    pub use #rep_struct_name as Rep;

                    #rep_serialize_impl
                    #rep_deserialize_impl

                    impl Rep {
                        pub fn status(&self) -> &str {
                            match self {
                                #(#status_match)*
                                _ => "unknown",
                            }
                        }

                        #rep_dump_method

                        #rep_load_method
                    }
                }
            }
        }
    };

    let variant = quote! {
        #[serde(rename = #command_name)]
        #variant_name(#module_name::Req)
    };

    (variant, module)
}

fn quote_custom_struct_union(
    name: &str,
    variants: &[GenCmdNestedTypeVariant],
    discriminant_field: &str,
    serialization_impl: SerializationImpl,
) -> TokenStream {
    let name = format_ident!("{}", name);
    let variants_def = variants.iter().map(|variant| {
        let variant_name = format_ident!("{}", variant.name);
        let value_literal = &variant.discriminant_value;
        if variant.fields.is_empty() {
            quote! {
                #[serde(rename = #value_literal)]
                #variant_name
            }
        } else {
            let variant_fields = quote_cmd_fields(&variant.fields, false);
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
                let field_writes = quote_dynamic_cmd_field_writes_from_bindings(&variant.fields);
                let field_count_expr = quote_dynamic_cmd_field_count_expr_from_bindings(&variant.fields);

                if variant.fields.is_empty() {
                    quote! {
                        Self::#variant_name => {
                            libparsec_types::rmp_serialize::encode::write_map_len(writer, 1).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                            libparsec_types::rmp_serialize::encode::write_str(writer, #discriminant_field).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                            libparsec_types::rmp_serialize::encode::write_str(writer, #value_literal).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                            Ok(())
                        }
                    }
                } else {
                    let field_names = variant
                        .fields
                        .iter()
                        .sorted_by(|a, b| a.name.cmp(&b.name))
                        .map(|field| field_name_to_ident(field.name.as_str()))
                        .collect::<Vec<_>>();

                    quote! {
                        Self::#variant_name { #(#field_names),* } => {
                            let map_len = (1usize + #field_count_expr)
                                .try_into()
                                .map_err(|_| libparsec_types::rmp_serialize::SerializeError::LengthOverflow {
                                    kind: "map",
                                    len: 1usize + #field_count_expr,
                                })?;
                            libparsec_types::rmp_serialize::encode::write_map_len(writer, map_len).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                            libparsec_types::rmp_serialize::encode::write_str(writer, #discriminant_field).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                            libparsec_types::rmp_serialize::encode::write_str(writer, #value_literal).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                            #(#field_writes)*
                            Ok(())
                        }
                    }
                }
            });

            quote! {
                impl libparsec_types::rmp_serialize::Serialize for #name {
                    fn serialize(
                        &self,
                        writer: &mut ::std::vec::Vec<u8>,
                    ) -> Result<(), libparsec_types::rmp_serialize::SerializeError> {
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
            let variants_deserialize = variants.iter().map(|variant| {
                let variant_name = format_ident!("{}", variant.name);
                let value_literal = &variant.discriminant_value;

                if variant.fields.is_empty() {
                    quote! {
                        #value_literal => {
                            Ok(Self::#variant_name)
                        }
                    }
                } else {
                    let field_extractors = variant
                        .fields
                        .iter()
                        .sorted_by(|a, b| a.name.cmp(&b.name))
                        .map(quote_dynamic_cmd_named_field_extract)
                        .collect::<Vec<_>>();
                    let field_idents = variant
                        .fields
                        .iter()
                        .sorted_by(|a, b| a.name.cmp(&b.name))
                        .map(|field| field_name_to_ident(field.name.as_str()))
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
                impl libparsec_types::rmp_serialize::Deserialize for #name {
                    fn deserialize(
                        value: libparsec_types::rmp_serialize::ValueRef<'_>,
                    ) -> Result<Self, libparsec_types::rmp_serialize::DeserializeError> {
                        let entries = match value {
                            rmpv::ValueRef::Map(entries) => entries,
                            other => {
                                return Err(libparsec_types::rmp_serialize::DeserializeError::InvalidType {
                                    expected: "map",
                                    got: libparsec_types::rmp_serialize::value_kind(&other),
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
                            .ok_or(libparsec_types::rmp_serialize::DeserializeError::MissingField(#discriminant_field))?;
                        let discriminant = String::deserialize(discriminant)?;

                        match discriminant.as_str() {
                            #(#variants_deserialize),*,
                            _ => Err(libparsec_types::rmp_serialize::DeserializeError::InvalidValue(
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
                    Self::#variant_name => libparsec_types::rmp_serialize::encode::write_str(writer, #value).map_err(libparsec_types::rmp_serialize::SerializeError::from)
                }
            });

            quote! {
                impl libparsec_types::rmp_serialize::Serialize for #name {
                    fn serialize(
                        &self,
                        writer: &mut ::std::vec::Vec<u8>,
                    ) -> Result<(), libparsec_types::rmp_serialize::SerializeError> {
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
                impl libparsec_types::rmp_serialize::Deserialize for #name {
                    fn deserialize(
                        value: libparsec_types::rmp_serialize::ValueRef<'_>,
                    ) -> Result<Self, libparsec_types::rmp_serialize::DeserializeError> {
                        let discriminant = String::deserialize(value)?;
                        match discriminant.as_str() {
                            #(#variants_deserialize),*,
                            _ => Err(libparsec_types::rmp_serialize::DeserializeError::InvalidValue(
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
    fields: &[GenCmdField],
    serialization_impl: SerializationImpl,
) -> TokenStream {
    let name = format_ident!("{}", name);
    let fields_def = quote_cmd_fields(fields, true);

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
        SerializationImpl::DynamicRmp => quote_dynamic_cmd_struct_serialize_impl(&name, fields),
    };

    let deserialize_impl = match serialization_impl {
        SerializationImpl::Serde => quote! {},
        SerializationImpl::DynamicRmp => quote_dynamic_cmd_struct_deserialize_impl(&name, fields),
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

fn quote_cmd_nested_types(
    nested_types: &[GenCmdNestedType],
    serialization_impl: SerializationImpl,
) -> Vec<TokenStream> {
    nested_types
        .iter()
        .map(|nested_type| match nested_type {
            GenCmdNestedType::StructsUnion {
                name,
                discriminant_field,
                variants,
            } => quote_custom_struct_union(
                name,
                variants.as_ref(),
                discriminant_field,
                serialization_impl,
            ),
            GenCmdNestedType::LiteralsUnion { name, variants } => {
                quote_custom_literal_union(name, variants.as_ref(), serialization_impl)
            }
            GenCmdNestedType::Struct { name, fields } => {
                quote_custom_struct(name, fields.as_ref(), serialization_impl)
            }
        })
        .collect()
}

fn quote_cmd_req_struct(
    pascal_case_cmd_name: &str,
    req: &GenCmdReq,
    serialization_impl: SerializationImpl,
) -> TokenStream {
    // The req struct is exposed as `Req`, however we want to have it named
    // `<CmdName>Req` for debug display
    let struct_name = format_ident!("{}Req", pascal_case_cmd_name);
    let req_derive = match serialization_impl {
        SerializationImpl::Serde => {
            quote! { #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)] }
        }
        SerializationImpl::DynamicRmp => {
            quote! { #[derive(Debug, Clone, ::serde::Deserialize, PartialEq, Eq)] }
        }
    };
    match &req.kind {
        GenCmdReqKind::Unit { nested_type_name } => {
            let cmd_name_literal = &req.cmd;
            let nested_type_name = format_ident!("{}", nested_type_name);
            let serialize_impl = match serialization_impl {
                SerializationImpl::Serde => quote! {},
                SerializationImpl::DynamicRmp => quote! {
                    impl libparsec_types::rmp_serialize::Serialize for Req {
                        fn serialize(
                            &self,
                            writer: &mut ::std::vec::Vec<u8>,
                        ) -> Result<(), libparsec_types::rmp_serialize::SerializeError> {
                            libparsec_types::rmp_serialize::encode::write_map_len(writer, 2).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                            libparsec_types::rmp_serialize::encode::write_str(writer, "cmd").map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                            libparsec_types::rmp_serialize::encode::write_str(writer, #cmd_name_literal).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                            libparsec_types::rmp_serialize::encode::write_str(writer, "unit").map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                            libparsec_types::rmp_serialize::Serialize::serialize(&self.0, writer)?;
                            Ok(())
                        }
                    }
                },
            };
            let deserialize_impl = match serialization_impl {
                SerializationImpl::Serde => quote! {},
                SerializationImpl::DynamicRmp => quote! {
                    impl libparsec_types::rmp_serialize::Deserialize for Req {
                        fn deserialize(
                            value: libparsec_types::rmp_serialize::ValueRef<'_>,
                        ) -> Result<Self, libparsec_types::rmp_serialize::DeserializeError> {
                            let entries = match value {
                                rmpv::ValueRef::Map(entries) => entries,
                                other => {
                                    return Err(libparsec_types::rmp_serialize::DeserializeError::InvalidType {
                                        expected: "map",
                                        got: libparsec_types::rmp_serialize::value_kind(&other),
                                    })
                                }
                            };

                            let mut obj = std::collections::HashMap::with_capacity(entries.len());
                            for (raw_key, raw_value) in entries {
                                let key = String::deserialize(raw_key)?;
                                obj.insert(key, raw_value);
                            }

                            let unit = {
                                let value = obj
                                    .remove("unit")
                                    .ok_or(libparsec_types::rmp_serialize::DeserializeError::MissingField("unit"))?;
                                <#nested_type_name as libparsec_types::rmp_serialize::Deserialize>::deserialize(value)?
                            };
                            Ok(Self(unit))
                        }
                    }
                },
            };

            quote! {
                #[::serde_with::serde_as]
                #req_derive
                #[serde(transparent)]
                pub struct #struct_name(pub #nested_type_name);
                pub use #struct_name as Req;

                #serialize_impl
                #deserialize_impl
            }
        }
        GenCmdReqKind::Composed { fields } => {
            if fields.is_empty() {
                let cmd_name_literal = &req.cmd;
                let serialize_impl = match serialization_impl {
                    SerializationImpl::Serde => quote! {},
                    SerializationImpl::DynamicRmp => quote! {
                        impl libparsec_types::rmp_serialize::Serialize for Req {
                            fn serialize(
                                &self,
                                writer: &mut ::std::vec::Vec<u8>,
                            ) -> Result<(), libparsec_types::rmp_serialize::SerializeError> {
                                libparsec_types::rmp_serialize::encode::write_map_len(writer, 1).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                                libparsec_types::rmp_serialize::encode::write_str(writer, "cmd").map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                                libparsec_types::rmp_serialize::encode::write_str(writer, #cmd_name_literal).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                                Ok(())
                            }
                        }
                    },
                };
                let deserialize_impl = match serialization_impl {
                    SerializationImpl::Serde => quote! {},
                    SerializationImpl::DynamicRmp => quote! {
                        impl libparsec_types::rmp_serialize::Deserialize for Req {
                            fn deserialize(
                                value: libparsec_types::rmp_serialize::ValueRef<'_>,
                            ) -> Result<Self, libparsec_types::rmp_serialize::DeserializeError> {
                                match value {
                                    rmpv::ValueRef::Map(_) => Ok(Self),
                                    other => Err(libparsec_types::rmp_serialize::DeserializeError::InvalidType {
                                        expected: "map",
                                        got: libparsec_types::rmp_serialize::value_kind(&other),
                                    }),
                                }
                            }
                        }
                    },
                };

                quote! {
                    #[::serde_with::serde_as]
                    #req_derive
                    pub struct #struct_name;
                    pub use #struct_name as Req;

                    #serialize_impl
                    #deserialize_impl
                }
            } else {
                let fields_codes = quote_cmd_fields(fields, true);
                let cmd_name_literal = &req.cmd;
                let dynamic_serialize_fields = quote_dynamic_cmd_field_writes_from_self(fields);
                let dynamic_count_terms = fields
                    .iter()
                    .sorted_by(|a, b| a.name.cmp(&b.name))
                    .filter(|field| field.added_in_minor_revision)
                    .map(|field| {
                        let field_name = field_name_to_ident(field.name.as_str());
                        quote! {
                            if matches!(self.#field_name, libparsec_types::Maybe::Present(_)) {
                                map_len += 1;
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                let dynamic_static_count = fields
                    .iter()
                    .filter(|field| !field.added_in_minor_revision)
                    .count()
                    + 1;
                let dynamic_extractors = fields
                    .iter()
                    .sorted_by(|a, b| a.name.cmp(&b.name))
                    .map(quote_dynamic_cmd_named_field_extract)
                    .collect::<Vec<_>>();
                let dynamic_field_idents = fields
                    .iter()
                    .sorted_by(|a, b| a.name.cmp(&b.name))
                    .map(|field| field_name_to_ident(field.name.as_str()))
                    .collect::<Vec<_>>();
                let serialize_impl = match serialization_impl {
                    SerializationImpl::Serde => quote! {},
                    SerializationImpl::DynamicRmp => quote! {
                        impl libparsec_types::rmp_serialize::Serialize for Req {
                            fn serialize(
                                &self,
                                writer: &mut ::std::vec::Vec<u8>,
                            ) -> Result<(), libparsec_types::rmp_serialize::SerializeError> {
                                let mut map_len = #dynamic_static_count;
                                #(#dynamic_count_terms)*
                                let map_len: u32 = map_len.try_into().map_err(|_| {
                                    libparsec_types::rmp_serialize::SerializeError::LengthOverflow {
                                        kind: "map",
                                        len: map_len,
                                    }
                                })?;
                                libparsec_types::rmp_serialize::encode::write_map_len(writer, map_len).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                                libparsec_types::rmp_serialize::encode::write_str(writer, "cmd").map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                                libparsec_types::rmp_serialize::encode::write_str(writer, #cmd_name_literal).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                                #(#dynamic_serialize_fields)*
                                Ok(())
                            }
                        }
                    },
                };
                let deserialize_impl = match serialization_impl {
                    SerializationImpl::Serde => quote! {},
                    SerializationImpl::DynamicRmp => quote! {
                        impl libparsec_types::rmp_serialize::Deserialize for Req {
                            fn deserialize(
                                value: libparsec_types::rmp_serialize::ValueRef<'_>,
                            ) -> Result<Self, libparsec_types::rmp_serialize::DeserializeError> {
                                let entries = match value {
                                    rmpv::ValueRef::Map(entries) => entries,
                                    other => {
                                        return Err(libparsec_types::rmp_serialize::DeserializeError::InvalidType {
                                            expected: "map",
                                            got: libparsec_types::rmp_serialize::value_kind(&other),
                                        })
                                    }
                                };

                                let mut obj = std::collections::HashMap::with_capacity(entries.len());
                                for (raw_key, raw_value) in entries {
                                    let key = String::deserialize(raw_key)?;
                                    obj.insert(key, raw_value);
                                }
                                obj.remove("cmd");

                                #(#dynamic_extractors)*

                                Ok(Self {
                                    #(#dynamic_field_idents),*
                                })
                            }
                        }
                    },
                };

                quote! {
                    #[::serde_with::serde_as]
                    #req_derive
                    pub struct #struct_name {
                        #(#fields_codes),*
                    }
                    pub use #struct_name as Req;

                    #serialize_impl
                    #deserialize_impl
                }
            }
        }
    }
}

/// Output:
/// - 0: Enum variant with its rename
/// - 1: Status match branch
/// - 2: Dynamic serialize match arm
/// - 3: Dynamic deserialize match branch
fn quote_cmd_rep_variant(rep: &GenCmdRep) -> (TokenStream, TokenStream, TokenStream, TokenStream) {
    let variant_name = format_ident!("{}", &snake_to_pascal_case(&rep.status));
    let status_name = &rep.status;

    let variant = match &rep.kind {
        GenCmdRepKind::Unit { nested_type_name } => {
            let nested_type_name = format_ident!("{}", nested_type_name);
            quote! {
                #[serde(rename = #status_name)]
                #variant_name(#nested_type_name)
            }
        }
        GenCmdRepKind::Composed { fields } => {
            if fields.is_empty() {
                quote! {
                    #[serde(rename = #status_name)]
                    #variant_name
                }
            } else {
                let fields = quote_cmd_fields(fields, false);
                quote! {
                    #[serde(rename = #status_name)]
                    #variant_name {
                        #(#fields),*
                    }
                }
            }
        }
    };

    let status_match = match &rep.kind {
        GenCmdRepKind::Unit { .. } => quote! {
            Self::#variant_name(..) => #status_name,
        },
        GenCmdRepKind::Composed { fields } => {
            if fields.is_empty() {
                quote! {
                    Self::#variant_name => #status_name,
                }
            } else {
                quote! {
                    Self::#variant_name { .. } => #status_name,
                }
            }
        }
    };

    let dynamic_serialize_arm = match &rep.kind {
        GenCmdRepKind::Unit { .. } => {
            quote! {
                Self::#variant_name(unit) => {
                    libparsec_types::rmp_serialize::encode::write_map_len(writer, 2).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                    libparsec_types::rmp_serialize::encode::write_str(writer, "status").map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                    libparsec_types::rmp_serialize::encode::write_str(writer, #status_name).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                    libparsec_types::rmp_serialize::encode::write_str(writer, "unit").map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                    libparsec_types::rmp_serialize::Serialize::serialize(unit, writer)?;
                    Ok(())
                }
            }
        }
        GenCmdRepKind::Composed { fields } => {
            if fields.is_empty() {
                quote! {
                    Self::#variant_name => {
                        libparsec_types::rmp_serialize::encode::write_map_len(writer, 1).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                        libparsec_types::rmp_serialize::encode::write_str(writer, "status").map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                        libparsec_types::rmp_serialize::encode::write_str(writer, #status_name).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                        Ok(())
                    }
                }
            } else {
                let field_bindings = fields
                    .iter()
                    .sorted_by(|a, b| a.name.cmp(&b.name))
                    .map(|f| field_name_to_ident(f.name.as_str()))
                    .collect::<Vec<_>>();
                let field_count_expr = quote_dynamic_cmd_field_count_expr_from_bindings(fields);
                let field_writes = quote_dynamic_cmd_field_writes_from_bindings(fields);

                quote! {
                    Self::#variant_name { #(#field_bindings),* } => {
                        let map_len = (1usize + #field_count_expr)
                            .try_into()
                            .map_err(|_| libparsec_types::rmp_serialize::SerializeError::LengthOverflow {
                                kind: "map",
                                len: 1usize + #field_count_expr,
                            })?;
                        libparsec_types::rmp_serialize::encode::write_map_len(writer, map_len).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                        libparsec_types::rmp_serialize::encode::write_str(writer, "status").map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                        libparsec_types::rmp_serialize::encode::write_str(writer, #status_name).map_err(libparsec_types::rmp_serialize::SerializeError::from)?;
                        #(#field_writes)*
                        Ok(())
                    }
                }
            }
        }
    };

    let dynamic_deserialize_case = match &rep.kind {
        GenCmdRepKind::Unit { nested_type_name } => {
            let nested_type_name = format_ident!("{}", nested_type_name);
            quote! {
                #status_name => {
                    let value = obj
                        .remove("unit")
                        .ok_or(libparsec_types::rmp_serialize::DeserializeError::MissingField("unit"))?;
                    Ok(Self::#variant_name(
                        <#nested_type_name as libparsec_types::rmp_serialize::Deserialize>::deserialize(value)?,
                    ))
                }
            }
        }
        GenCmdRepKind::Composed { fields } => {
            if fields.is_empty() {
                quote! {
                    #status_name => Ok(Self::#variant_name)
                }
            } else {
                let field_extractors = fields
                    .iter()
                    .sorted_by(|a, b| a.name.cmp(&b.name))
                    .map(quote_dynamic_cmd_named_field_extract)
                    .collect::<Vec<_>>();
                let field_idents = fields
                    .iter()
                    .sorted_by(|a, b| a.name.cmp(&b.name))
                    .map(|field| field_name_to_ident(field.name.as_str()))
                    .collect::<Vec<_>>();

                quote! {
                    #status_name => {
                        #(#field_extractors)*
                        Ok(Self::#variant_name {
                            #(#field_idents),*
                        })
                    }
                }
            }
        }
    };

    (
        variant,
        status_match,
        dynamic_serialize_arm,
        dynamic_deserialize_case,
    )
}

/// Output:
/// - 0: Enum variants with their rename + UnknownStatus
/// - 1: Status match branches
/// - 2: Dynamic serialize match arms
/// - 3: Dynamic deserialize match branches
fn quote_cmd_rep_variants(
    reps: &[GenCmdRep],
) -> (
    Vec<TokenStream>,
    Vec<TokenStream>,
    Vec<TokenStream>,
    Vec<TokenStream>,
) {
    let mut variants = Vec::new();
    let mut status_match = Vec::new();
    let mut dynamic_serialize_arms = Vec::new();
    let mut dynamic_deserialize_cases = Vec::new();

    for rep in reps.iter().sorted_by(|a, b| a.status.cmp(&b.status)) {
        let (variant, status, dynamic_serialize_arm, dynamic_deserialize_case) =
            quote_cmd_rep_variant(rep);
        variants.push(variant);
        status_match.push(status);
        dynamic_serialize_arms.push(dynamic_serialize_arm);
        dynamic_deserialize_cases.push(dynamic_deserialize_case);
    }

    // `UnknownStatus` covers the case the server returns a valid message but with an unknown
    // status value (given change in error status only cause a minor bump in API version)
    // Note it is meaningless to serialize a `UnknownStatus` (you created the object from
    // scratch, you know what it is for, baka !)
    variants.push(quote! {
        #[serde(skip)]
        UnknownStatus {
            unknown_status: String,
            reason: Option<String>
        }
    });

    (
        variants,
        status_match,
        dynamic_serialize_arms,
        dynamic_deserialize_cases,
    )
}

fn quote_cmd_fields(fields: &[GenCmdField], with_pub: bool) -> Vec<TokenStream> {
    fields
        .iter()
        // The fields will be sorted by their name in binary mode.
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .map(|x| quote_cmd_field(x, with_pub))
        .collect()
}

fn quote_cmd_field(field: &GenCmdField, with_pub: bool) -> TokenStream {
    let mut attrs = Vec::<TokenStream>::new();

    let ty = {
        if field.added_in_minor_revision {
            attrs.push(quote! {
                #[serde(default, skip_serializing_if = "libparsec_types::Maybe::is_absent")]
            });
            let rust_type = field.ty.to_rust_type();
            quote! {
                libparsec_types::Maybe<#rust_type>
            }
        } else {
            field.ty.to_rust_type()
        }
    };

    let can_only_be_null = matches!(field.ty, FieldType::RequiredOption(_));
    // let can_be_missing_or_null = matches!(field.ty, FieldType::NonRequiredOption(_));
    let default = if can_only_be_null {
        quote! { no_default }
    } else {
        quote! {}
    };

    match (field.added_in_minor_revision, field.ty.to_serde_as()) {
        (true, Some(serde_as_type)) => {
            let serde_as_type = quote! { libparsec_types::Maybe<#serde_as_type> }.to_string();
            attrs.push(quote! { #[serde_as(as = #serde_as_type, #default)] });
        }
        (true, None) => {
            let serde_as_type = quote! { libparsec_types::Maybe<_> }.to_string();
            attrs.push(quote! { #[serde_as(as = #serde_as_type, #default)] });
        }
        (false, Some(serde_as_type)) => {
            let serde_as_type = serde_as_type.to_string();
            attrs.push(quote! { #[serde_as(as = #serde_as_type, #default)] });
        }
        _ => (),
    }

    let name = match field.name.as_ref() {
        "type" => {
            attrs.push(quote! {#[serde(rename = "type")]});
            format_ident!("ty")
        }
        name => format_ident!("{}", name),
    };

    if with_pub {
        quote! {
            #(#attrs)*
            pub #name: #ty
        }
    } else {
        quote! {
            #(#attrs)*
            #name: #ty
        }
    }
}
