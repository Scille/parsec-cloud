// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    config::CratesPaths,
    shared::{filter_out_future_fields, quote_fields, Fields, MajorMinorVersion},
};

use super::parser;

#[derive(Default)]
pub struct TypeCollection {
    pub versioned_types: HashMap<u32, Vec<Type>>,
}

impl From<parser::VersionedType> for TypeCollection {
    fn from(versioned_type: parser::VersionedType) -> Self {
        let mut collection = TypeCollection::default();

        for variant in versioned_type.0 {
            for version in &variant.major_versions {
                let entry = collection.get_or_insert_default_versioned_types(*version);
                let ty = Type::from_parsec_type(variant.clone(), *version);
                entry.push(ty);
            }
        }
        collection
    }
}

impl From<Vec<parser::VersionedType>> for TypeCollection {
    fn from(versioned_types: Vec<parser::VersionedType>) -> Self {
        let mut collection = TypeCollection::default();

        for versioned_type in versioned_types {
            for variant in versioned_type.0 {
                for version in &variant.major_versions {
                    let entry = collection.get_or_insert_default_versioned_types(*version);
                    let ty = Type::from_parsec_type(variant.clone(), *version);
                    entry.push(ty);
                }
            }
        }
        collection
    }
}

impl TypeCollection {
    fn get_or_insert_default_versioned_types(&mut self, version: u32) -> &mut Vec<Type> {
        self.versioned_types.entry(version).or_default()
    }
}

impl TypeCollection {
    pub fn quote(&self, crates_override: &CratesPaths) -> anyhow::Result<Vec<syn::ItemMod>> {
        let (modules, errors): (Vec<_>, Vec<_>) = self
            .versioned_types
            .iter()
            .sorted_by_key(|(v, _)| *v)
            .map(|(version, ty)| quote_versioned_types(*version, ty, crates_override))
            .partition_result();

        anyhow::ensure!(
            errors.is_empty(),
            "Cannot quote all versioned type:\n{}",
            errors.into_iter().map(|e| e.to_string()).join(",\n"),
        );
        Ok(modules)
    }
}

fn quote_versioned_types(
    version: u32,
    types: &[Type],
    crates_override: &CratesPaths,
) -> anyhow::Result<syn::ItemMod> {
    let versioned_mod = syn::parse_str::<syn::Ident>(&format!("v{version}"))
        .expect("Name in the format `vN` should be valid for module name");
    let (types, errors): (Vec<_>, Vec<_>) = types
        .iter()
        .map(|ty| ty.quote(crates_override))
        .partition_result();

    anyhow::ensure!(
        errors.is_empty(),
        "Cannot quote all types:\n{}",
        errors.iter().map(|e| e.to_string()).join(",\n")
    );
    let (types_struct, types_impl): (Vec<Vec<syn::ItemStruct>>, Vec<Vec<syn::ItemImpl>>) =
        types.into_iter().unzip();
    let types_struct = types_struct.into_iter().concat();
    let types_impl = types_impl.into_iter().concat();

    let module = syn::parse_quote! {
        pub mod #versioned_mod {
            #(#types_struct)*

            #(#types_impl)*
        }
    };

    Ok(module)
}

pub struct Type {
    pub label: String,
    pub introduced_in: Option<MajorMinorVersion>,
    pub fields: Fields,
    pub ty: Option<String>,
}

impl Type {
    fn from_parsec_type(ty: parser::Type, version: u32) -> Self {
        let introduced_in = ty
            .introduced_in
            .filter(|mm_version| mm_version.major == version);
        let fields = filter_out_future_fields!(version, ty.fields);

        Self {
            label: ty.label,
            introduced_in,
            fields,
            ty: ty.ty,
        }
    }
}

impl Type {
    fn quote(
        &self,
        crates_override: &CratesPaths,
    ) -> anyhow::Result<(Vec<syn::ItemStruct>, Vec<syn::ItemImpl>)> {
        let name = syn::parse_str::<syn::Ident>(&format!("{}Data", self.label))
            .map_err(|e| anyhow::anyhow!("Invalid Data name `{}`: {}", self.label, e))?;
        let fields = quote_fields(&self.fields, None, &HashMap::default(), crates_override)?;

        if let Some(ty) = &self.ty {
            self.quote_with_type(name, fields, ty)
        } else {
            let item_mod = self.quote_without_type(name, fields);
            Ok((vec![item_mod], vec![]))
        }
    }

    fn quote_with_type(
        &self,
        name: syn::Ident,
        fields: Vec<syn::Field>,
        ty: &str,
    ) -> anyhow::Result<(Vec<syn::ItemStruct>, Vec<syn::ItemImpl>)> {
        let name_type = syn::parse_str::<syn::Ident>(&format!("{}DataType", self.label))
            .map_err(|e| anyhow::anyhow!("Invalid DateType name `{}`: {}", self.label, e))?;

        let mut items_struct = Vec::with_capacity(2);
        let mut items_impl = Vec::with_capacity(2);

        items_struct.push(syn::parse_quote! {
            #[serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
            pub struct #name {
                #[serde(rename = "type")]
                pub ty: #name_type,
                #(#fields),*
            }
        });
        items_struct.push(syn::parse_quote! {
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
            pub struct #name_type;
        });

        items_impl.push(syn::parse_quote! {
            impl ::serde::Serialize for #name_type {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(#ty)
                }
            }
        });
        items_impl.push(syn::parse_quote! {
            impl<'de> ::serde::Deserialize<'de> for #name_type {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>
                {
                    struct Visitor;

                    impl<'de> ::serde::de::Visitor<'de> for Visitor {
                        type Value = #name_type;

                        fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                            fmt.write_str(concat!("The `", #ty, "` string"))
                        }

                        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                        where
                            E: ::serde::de::Error,
                        {
                            if v == #ty {
                                Ok(#name_type)
                            } else {
                                Err(::serde::de::Error::invalid_type(
                                    ::serde::de::Unexpected::Str(v),
                                    &self,
                                ))
                            }
                        }
                    }

                    deserializer.deserialize_str(Visitor)
                }
            }
        });

        Ok((items_struct, items_impl))
    }

    fn quote_without_type(&self, name: syn::Ident, fields: Vec<syn::Field>) -> syn::ItemStruct {
        syn::parse_quote! {
            #[serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]
            pub struct #name {
                #(#fields),*
            }
        }
    }
}
