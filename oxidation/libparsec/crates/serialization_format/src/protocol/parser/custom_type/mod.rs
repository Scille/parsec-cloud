// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub mod custom_enum;
pub mod custom_struct;
pub mod variant;

use std::collections::HashMap;

use serde::Deserialize;

pub use custom_enum::CustomEnum;
pub use custom_struct::CustomStruct;
pub use variant::Variant;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum CustomType {
    Struct(CustomStruct),
    Enum(CustomEnum),
}

impl CustomType {
    pub fn label(&self) -> &str {
        match self {
            CustomType::Struct(custom_struct) => &custom_struct.label,
            CustomType::Enum(custom_enum) => &custom_enum.label,
        }
    }
}

impl CustomType {
    pub fn quote(&self, types: &HashMap<String, String>) -> syn::Item {
        match self {
            CustomType::Struct(custom_struct) => syn::Item::Struct(custom_struct.quote(types)),
            CustomType::Enum(custom_enum) => syn::Item::Enum(custom_enum.quote(types)),
        }
    }
}

fn shared_attribute() -> Vec<syn::Attribute> {
    vec![
        syn::parse_quote!(#[::serde_with::serde_as]),
        syn::parse_quote!(#[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, PartialEq, Eq)]),
    ]
}

#[cfg(test)]
mod test {
    use super::{variant::Variant, CustomEnum, CustomStruct, CustomType};
    use crate::protocol::parser::field::Field;
    use rstest::rstest;

    #[rstest]
    #[case::deserialize_enum(
        r#"{
        "label": "APIEvent",
        "discriminant_field": "event",
        "variants": [
            {
                "name": "Pinged",
                "discriminant_value": "pinged",
                "fields": [
                    {
                        "name": "ping",
                        "type": "String"
                    }
                ]
            }
        ]
    }"#
    , CustomType::Enum(CustomEnum {
        label: "APIEvent".to_string(),
        discriminant_field: Some("event".to_string()),
        variants: vec![
            Variant {
                name: "Pinged".to_string(),
                discriminant_value: Some("pinged".to_string()),
                fields: vec![
                    Field {
                        name: "ping".to_string(),
                        ty: "String".to_string(),
                        introduced_in: None,
                        default: None,
                    }
                ]
            }
        ]
    }))]
    #[case::deserialize_struct(
        r#"{
        "label": "HumanFindResultItem",
        "fields": [
            {
                "name": "user_id",
                "type": "UserID"
            },
            {
                "name": "human_handle",
                "type": "Option<HumanHandle>"
            },
            {
                "name": "revoked",
                "type": "Boolean"
            }
        ]
    }"#,
    CustomType::Struct(CustomStruct {
        label: "HumanFindResultItem".to_string(),
        fields: vec![
            Field {
                name: "user_id".to_string(),
                ty: "UserID".to_string(),
                introduced_in: None,
                default: None,
            },
            Field {
                name: "human_handle".to_string(),
                ty: "Option<HumanHandle>".to_string(),
                introduced_in: None,
                default: None,
            },
            Field {
                name: "revoked".to_string(),
                ty: "Boolean".to_string(),
                introduced_in: None,
                default: None,
            }
        ]
    }))]
    fn test_deserialization(#[case] input: &str, #[case] expected: CustomType) {
        let custom_type =
            serde_json::from_str::<CustomType>(input).expect("Got error on valid data");
        assert_eq!(custom_type, expected);
    }
}
