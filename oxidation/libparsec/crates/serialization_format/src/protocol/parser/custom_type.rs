use serde::Deserialize;

use crate::protocol::parser::Field;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum CustomType {
    Struct(CustomStruct),
    Enum(CustomEnum),
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct CustomStruct {
    pub label: String,
    pub fields: Vec<Field>,
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct CustomEnum {
    pub label: String,
    #[serde(default)]
    pub discriminant_field: Option<String>,
    pub variants: Vec<Variant>,
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Variant {
    pub name: String,
    pub discriminant_value: String,
    pub fields: Vec<Field>,
}
