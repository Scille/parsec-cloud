use std::collections::HashMap;

use serde::Deserialize;

use crate::protocol::parser::Field;

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

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct CustomStruct {
    pub label: String,
    pub fields: Vec<Field>,
}

impl CustomStruct {
    pub fn quote(&self, types: &HashMap<String, String>) -> syn::ItemStruct {
        todo!()
    }
}

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
        todo!()
    }
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Variant {
    pub name: String,
    pub discriminant_value: String,
    pub fields: Vec<Field>,
}

#[cfg(test)]
impl Default for Variant {
    fn default() -> Self {
        Self {
            discriminant_value: "type".to_string(),
            fields: vec![],
            name: "FooVariant".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{CustomEnum, CustomStruct, CustomType, Field, Variant};
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
                discriminant_value: "pinged".to_string(),
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
    fn custom_type(#[case] input: &str, #[case] expected: CustomType) {
        let custom_type =
            serde_json::from_str::<CustomType>(input).expect("Got error on valid data");
        assert_eq!(custom_type, expected);
    }
}
