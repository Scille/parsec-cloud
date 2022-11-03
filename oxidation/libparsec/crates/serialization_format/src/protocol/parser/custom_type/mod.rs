// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub mod custom_enum;
pub mod custom_struct;
pub mod variant;

use std::collections::HashMap;

use serde::Deserialize;

pub use custom_enum::CustomEnum;
pub use custom_struct::CustomStruct;
pub use variant::{Variant, Variants};

/// A collection of [CustomType].
/// Each keys is the name of the custom type.
pub type CustomTypes = HashMap<String, CustomType>;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CustomType {
    Struct(CustomStruct),
    Enum(CustomEnum),
}

impl CustomType {
    pub fn quote(&self, name: &str, types: &HashMap<String, String>) -> syn::Item {
        match self {
            CustomType::Struct(custom_struct) => {
                syn::Item::Struct(custom_struct.quote(name, types))
            }
            CustomType::Enum(custom_enum) => syn::Item::Enum(custom_enum.quote(name, types)),
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
    use super::{
        variant::{Variant, Variants},
        CustomEnum, CustomStruct, CustomType,
    };
    use crate::protocol::parser::field::{Field, Fields};
    use rstest::rstest;

    #[rstest]
    #[case::deserialize_enum(
        r#"{
        "type": "enum",
        "discriminant_field": "event",
        "variants": {
            "Pinged": {
                "discriminant_value": "pinged",
                "fields": {
                    "ping": {
                        "type": "String"
                    }
                }
            }
        }
    }"#
    , CustomType::Enum(CustomEnum {
        discriminant_field: Some("event".to_string()),
        variants: Variants::from([
            ("Pinged".to_string(),
            Variant {
                discriminant_value: Some("pinged".to_string()),
                fields: Fields::from([
                    ("ping".to_string(),
                    Field {
                        ty: "String".to_string(),
                        introduced_in: None,
                        default: None,
                    }
                )
                ])
            }
        )
        ])
    }))]
    #[case::deserialize_struct(
        r#"{
        "type": "struct",
        "fields": {
            "user_id": {
                "type": "UserID"
            },
            "human_handle": {
                "type": "Option<HumanHandle>"
            },
            "revoked": {
                "type": "Boolean"
            }
        }
    }"#,
    CustomType::Struct(CustomStruct {
        fields: Fields::from([
            ("user_id".to_string(),
            Field {
                ty: "UserID".to_string(),
                introduced_in: None,
                default: None,
            },
        ),
        ("human_handle".to_string(),
        Field {
            ty: "Option<HumanHandle>".to_string(),
            introduced_in: None,
            default: None,
        },
    ),
    ("revoked".to_string(),
    Field {
        ty: "Boolean".to_string(),
        introduced_in: None,
        default: None,
    }
)
        ])
    }))]
    fn test_deserialization(#[case] input: &str, #[case] expected: CustomType) {
        let custom_type =
            serde_json::from_str::<CustomType>(input).expect("Got error on valid data");
        assert_eq!(custom_type, expected);
    }
}
