// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[cfg(feature = "python-bindings-support")]
#[path = "./protocol_python_bindings.rs"]
pub(crate) mod python_bindings;

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

pub(crate) fn generate_protocol_cmds_family(cmds: Vec<JsonCmd>, family_name: &str) -> TokenStream {
    quote_cmds_family(&GenCmdsFamily::new(cmds, family_name))
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
    cmd: String,
    fields: Option<Vec<JsonCmdField>>,
    unit: Option<String>,
}

#[derive(Deserialize, Clone)]
struct JsonCmdRep {
    status: String,
    fields: Option<Vec<JsonCmdField>>,
    unit: Option<String>,
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
    // Field is required if the command has always contain it, or if
    // it has been introduced in a previous major version of the API
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

//
// JSON-to-gen parsing & struct conversion
//

impl GenCmdsFamily {
    fn new(cmds: Vec<JsonCmd>, family_name: &str) -> Self {
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
                ..
            } = cmd;
            let cmd_name = &req.cmd.clone();
            assert!(
                !major_versions.is_empty(),
                "{}: major_versions field cannot be empty !",
                cmd_name
            );

            // We don't try to play smart here: if `introduced_in` is set
            // somewhere we consider the schema cannot be shared accross
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
                                Err(err) => panic!("{}: {:?}", cmd_name, err),
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
                                        assert!(v.fields.is_none(), "{}: `{}::{}` is supposed to be a literal union, but has fields !", cmd_name, &nested_type.name, v.discriminant_value);
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
                    cmd: req_cmd,
                    fields: req_fields,
                    unit: req_unit,
                } = req.clone();
                let gen_req_kind = match (req_unit, req_fields) {
                    (Some(nested_type_name), None) => GenCmdReqKind::Unit { nested_type_name },
                    (None, Some(fields)) => GenCmdReqKind::Composed {
                        fields: fields.into_iter().filter_map(&convert_field).collect(),
                    },
                    (None, None) => GenCmdReqKind::Composed { fields: vec![] },
                    _ => panic!(
                        "{}: `unit`/`fields` are mutually exclusives in req",
                        &req_cmd
                    ),
                };
                let gen_req = GenCmdReq {
                    cmd: req_cmd,
                    kind: gen_req_kind,
                };

                let gen_reps = reps
                    .clone()
                    .into_iter()
                    .map(|rep| {
                        let kind = match (&rep.unit, rep.fields) {
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
                    *has_introduced_in_field.borrow(),
                    can_reuse_schema_from_version,
                ) {
                    (true, _) => {
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
                    (false, None) => {
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
                    (false, Some(version)) => GenCmd {
                        cmd: cmd_name.to_owned(),
                        spec: GenCmdSpec::ReusedFromVersion { version },
                    },
                };

                assert!(
                    version_cmd_couples.insert((major_version, cmd_name.to_owned())),
                    "APIv{:?} has multiple implementations of {:?} !",
                    major_version,
                    cmd_name
                );
                gen_versions.entry(major_version).or_default().push(gen_cmd);
            }
        }

        Self {
            name: family_name.to_owned(),
            versions: gen_versions,
        }
    }
}

//
// Code generation
//

fn quote_cmds_family(family: &GenCmdsFamily) -> TokenStream {
    let family_name = format_ident!("{}", &family.name);
    let mut latest_cmds_mod_name = None;

    let versioned_cmds: Vec<_> = family
        .versions
        .iter()
        .sorted_by_key(|(v, _)| *v)
        .map(|(version, cmds)| {
            let (mod_name, code) = quote_versioned_cmds(*version, cmds);
            latest_cmds_mod_name.replace(mod_name);
            code
        })
        .collect();

    let latest_cmds_mod_name = latest_cmds_mod_name.expect("at least one version exists");

    quote! {
        pub mod #family_name {
            use super::libparsec_types; // Allow to mock types in tests

            // Define `UnknownStatus` here instead of where is it actually used (i.e.
            // near each command's `Rep::load` definition) to have a single common
            // definition that will have it deserialization code compiled once \o/
            #[derive(::serde::Deserialize)]
            struct UnknownStatus {
                status: String,
                reason: Option<String>
            }

            #(#versioned_cmds)*

            pub mod latest {
                pub use super::#latest_cmds_mod_name::*;
            }
        }
    }
}

fn quote_versioned_cmds(version: u32, cmds: &[GenCmd]) -> (Ident, TokenStream) {
    let versioned_cmds_mod = format_ident!("v{version}");
    let (any_cmd_req_variants, cmd_structs): (Vec<TokenStream>, Vec<TokenStream>) =
        cmds.iter().map(|cmd| quote_cmd(version, cmd)).unzip();

    let code = quote! {
        pub mod #versioned_cmds_mod {
            use super::libparsec_types; // Allow to mock types in tests
            use super::UnknownStatus;

            #[derive(Debug, Clone, ::serde::Deserialize, PartialEq, Eq)]
            #[serde(tag = "cmd")]
            pub enum AnyCmdReq {
                #(#any_cmd_req_variants),*
            }

            impl AnyCmdReq {
                pub fn load(buf: &[u8]) -> Result<Self, ::rmp_serde::decode::Error> {
                    ::rmp_serde::from_slice(buf)
                }
            }

            #(#cmd_structs)*
        }
    };

    (versioned_cmds_mod, code)
}

fn quote_cmd(cmd_version: u32, cmd: &GenCmd) -> (TokenStream, TokenStream) {
    let pascal_case_name = &snake_to_pascal_case(&cmd.cmd);
    let snake_case_name = &cmd.cmd;

    let variant_name = format_ident!("{}", pascal_case_name);
    let module_name = format_ident!("{}", snake_case_name);
    let command_name = snake_case_name;

    let module = match &cmd.spec {
        GenCmdSpec::ReusedFromVersion { version } => {
            let reused_version_module_name = format_ident!("v{}", version);
            quote! {
                pub mod #module_name {
                    pub use super::super::#reused_version_module_name::#module_name::*;

                    impl super::libparsec_types::ProtocolRequest<#cmd_version> for Req {
                        type Response = Rep;

                        fn api_dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                            self.dump()
                        }

                        fn api_load_response(buf: &[u8]) -> Result<Self::Response, ::rmp_serde::decode::Error> {
                            Self::load_response(buf)
                        }
                    }
                }
            }
        }

        GenCmdSpec::Original {
            req,
            reps,
            nested_types,
        } => {
            let struct_req = quote_cmd_req_struct(req);
            let (variants_rep, status_match) = quote_cmd_rep_variants(reps);
            let nested_types = quote_cmd_nested_types(nested_types);
            let known_rep_statuses = reps.iter().map(|rep| rep.status.to_owned());

            quote! {

                pub mod #module_name {
                    use super::libparsec_types; // Allow to mock types in tests
                    use super::AnyCmdReq;
                    use super::UnknownStatus;

                    #(#nested_types)*

                    #struct_req

                    impl Req {
                        pub fn dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                            ::rmp_serde::to_vec_named(self)
                        }

                        pub fn load_response(buf: &[u8]) -> Result<Rep, ::rmp_serde::decode::Error> {
                            Rep::load(buf)
                        }
                    }

                    impl libparsec_types::ProtocolRequest<#cmd_version> for Req {
                        type Response = Rep;

                        fn api_dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                            self.dump()
                        }

                        fn api_load_response(buf: &[u8]) -> Result<Self::Response, ::rmp_serde::decode::Error> {
                            Self::load_response(buf)
                        }
                    }

                    // Can't derive Eq because some Rep have f64 field
                    #[allow(clippy::derive_partial_eq_without_eq)]
                    #[::serde_with::serde_as]
                    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq)]
                    #[serde(tag = "status")]
                    pub enum Rep {
                        #(#variants_rep),*
                    }

                    impl Rep {
                        pub fn status(&self) -> &str {
                            match self {
                                #(#status_match)*
                                _ => "unknown",
                            }
                        }

                        pub fn dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                            ::rmp_serde::to_vec_named(self)
                        }

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
) -> TokenStream {
    let name = format_ident!("{}", name);
    let variants = variants.iter().map(|variant| {
        let variant_name = format_ident!("{}", variant.name);
        let value_literal = &variant.discriminant_value;
        let variant_fields = quote_cmd_fields(&variant.fields, false);
        quote! {
            #[serde(rename = #value_literal)]
            #variant_name {
                #(#variant_fields),*
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
        #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
        pub enum #name {
            #(#variants),*
        }
    }
}

fn quote_custom_struct(name: &str, fields: &[GenCmdField]) -> TokenStream {
    let name = format_ident!("{}", name);
    let fields = quote_cmd_fields(fields, true);
    quote! {
        #[::serde_with::serde_as]
        #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
        pub struct #name {
            #(#fields),*
        }
    }
}

fn quote_cmd_nested_types(nested_types: &[GenCmdNestedType]) -> Vec<TokenStream> {
    nested_types
        .iter()
        .map(|nested_type| match nested_type {
            GenCmdNestedType::StructsUnion {
                name,
                discriminant_field,
                variants,
            } => quote_custom_struct_union(name, variants.as_ref(), discriminant_field),
            GenCmdNestedType::LiteralsUnion { name, variants } => {
                quote_custom_literal_union(name, variants.as_ref())
            }
            GenCmdNestedType::Struct { name, fields } => quote_custom_struct(name, fields.as_ref()),
        })
        .collect()
}

fn quote_cmd_req_struct(req: &GenCmdReq) -> TokenStream {
    match &req.kind {
        GenCmdReqKind::Unit { nested_type_name } => {
            let cmd_name_literal = &req.cmd;
            let nested_type_name = format_ident!("{}", nested_type_name);
            quote! {
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Deserialize, PartialEq, Eq)]
                #[serde(transparent)]
                pub struct Req(pub #nested_type_name);

                impl ::serde::Serialize for Req
                {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: ::serde::Serializer,
                    {
                        #[derive(::serde::Serialize)]
                        enum CmdEnum {
                            #[serde(rename=#cmd_name_literal)]
                            Val,
                        }
                        #[derive(::serde::Serialize)]
                        struct ReqWithCmd<'a> {
                            cmd: CmdEnum,
                            #[serde(flatten)]
                            unit: &'a #nested_type_name,
                        }
                        let to_serialize = ReqWithCmd {cmd: CmdEnum::Val, unit: &self.0 };
                        to_serialize.serialize(serializer)
                    }
                }
            }
        }
        GenCmdReqKind::Composed { fields } => {
            if fields.is_empty() {
                let cmd_name_literal = &req.cmd;
                quote! {
                    #[::serde_with::serde_as]
                    #[derive(Debug, Clone, ::serde::Deserialize, PartialEq, Eq)]
                    pub struct Req;

                    impl ::serde::Serialize for Req
                    {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: ::serde::Serializer,
                        {
                            use ::serde::ser::SerializeStruct;
                            let mut state = serializer.serialize_struct("Req", 1)?;
                            state.serialize_field("cmd", #cmd_name_literal)?;
                            state.end()
                        }
                    }
                }
            } else {
                let fields_codes = quote_cmd_fields(fields, true);
                let number_of_always_present_fields: usize = fields.iter().fold(0, |ac, f| match f
                    .added_in_minor_revision
                {
                    true => ac,
                    false => ac + 1,
                });
                let determine_maybe_present_fields_codes = fields.iter().filter_map(|f| {
                    if !f.added_in_minor_revision {
                        return None;
                    }
                    let field_name = format_ident!("{}", &f.name);
                    Some(quote! {
                        if let libparsec_types::Maybe::Present(_) =  &self.#field_name {
                            fields_count += 1;
                        }
                    })
                });
                let serialize_fields = fields.iter().map(|f| {
                    let field_name_literal = &f.name;
                    let field_name = format_ident!("{}", field_name_literal);
                    if f.added_in_minor_revision {
                        quote!{
                            if let libparsec_types::Maybe::Present(field_data) =  &self.#field_name {
                                state.serialize_field(#field_name_literal, field_data)?;
                            }
                        }
                    } else {
                        quote!{ state.serialize_field(#field_name_literal, &self.#field_name)?; }
                    }
                });
                let cmd_name_literal = &req.cmd;

                quote! {
                    #[::serde_with::serde_as]
                    #[derive(Debug, Clone, ::serde::Deserialize, PartialEq, Eq)]
                    pub struct Req {
                        #(#fields_codes),*
                    }

                    impl ::serde::Serialize for Req
                    {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: ::serde::Serializer,
                        {
                            use ::serde::ser::SerializeStruct;
                            let mut fields_count = #number_of_always_present_fields + 1;
                            #(#determine_maybe_present_fields_codes)*
                            let mut state = serializer.serialize_struct("Req", fields_count)?;
                            state.serialize_field("cmd", #cmd_name_literal)?;
                            #(#serialize_fields)*
                            state.end()
                        }
                    }
                }
            }
        }
    }
}

/// Output:
/// - 0: Enum variant with it rename
/// - 1: Status match branch
fn quote_cmd_rep_variant(rep: &GenCmdRep) -> (TokenStream, TokenStream) {
    let variant_name = format_ident!("{}", &snake_to_pascal_case(&rep.status));
    let status_name = &rep.status;
    (
        match &rep.kind {
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
        },
        match &rep.kind {
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
        },
    )
}

/// Output:
/// - 0: Enum variants with their rename + UnknownStatus
/// - 1: Status match branches
fn quote_cmd_rep_variants(reps: &[GenCmdRep]) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let (mut variants, status_match): (Vec<TokenStream>, Vec<TokenStream>) = reps
        .iter()
        .sorted_by(|a, b| a.status.cmp(&b.status))
        .map(quote_cmd_rep_variant)
        .unzip();

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
    (variants, status_match)
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
