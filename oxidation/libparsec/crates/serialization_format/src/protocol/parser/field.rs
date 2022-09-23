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

#[cfg(test)]
#[rstest::rstest]
#[case::basic_field(
    r#"{"name": "Foo", "type": "String"}"#,
    Field { name: "Foo".to_string() , ty: "String".to_string(), introduced_in: None, default: None }
)]
#[case::field_introduced_in(
    r#"{"name": "Bar", "type": "Boolean", "introduced_in": "5.2"}"#,
    Field { name: "Bar".to_string(), ty: "Boolean".to_string(), introduced_in: Some("5.2".try_into().unwrap()), default: None}
)]
fn field(#[case] input: &str, #[case] expected: Field) {
    let field = serde_json::from_str::<Field>(input).expect("Got error on valid data");
    assert_eq!(field, expected)
}
