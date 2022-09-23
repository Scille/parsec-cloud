use super::MajorMinorVersion;

use serde::Deserialize;

#[derive(Clone, Copy)]
pub enum Vis {
    Public,
    Private,
}

pub enum SerdeAttr {
    Rename,
    Tag,
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Field {
    /// The name of the field.
    pub name: String,
    /// The type's name of the field.
    #[serde(rename = "type")]
    pub ty: String,
    /// In which version the current field was introduced.
    #[serde(default)]
    pub introduced_in: Option<MajorMinorVersion>,
    /// The name of the function to get the default value from.
    #[serde(default)]
    pub default: Option<String>,
}
