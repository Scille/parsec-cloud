// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use serde::Deserialize;

use crate::shared::{Fields, MajorMinorVersion};

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize)]
pub struct VersionedType(pub Vec<Type>);

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Type {
    pub label: String,
    pub major_versions: Vec<u32>,
    /// When the type was introduced during a major version but not from the start.
    /// This field is only used as documentation purpose.
    pub introduced_in: Option<MajorMinorVersion>,
    pub fields: Fields,
    #[serde(rename = "type")]
    pub ty: Option<String>,
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::shared::Fields;

    use super::Type;

    #[rstest]
    #[case::basic(
        r#"{
            "label": "LocalFooBar",
            "major_versions": [1,2,3],
            "fields": {}
        }"#,
        Type {
            label: "LocalFooBar".to_string(),
            major_versions: vec![1,2,3],
            introduced_in: None,
            fields: Fields::default(),
            ty: None,
        }
    )]
    #[case::with_introduced_in(
        r#"{
            "label": "FooBar",
            "major_versions": [1],
            "introduced_in": "1.4",
            "fields": {}
        }"#,
        Type {
            label: "FooBar".to_string(),
            major_versions: vec![1],
            introduced_in: Some("1.4".parse().unwrap()),
            fields: Fields::default(),
            ty: None
        }
    )]
    #[case::with_type(
        r#"{
            "label": "Foo",
            "major_versions": [],
            "type": "hello_foo_world",
            "fields": {}
        }"#,
        Type {
            label: "Foo".to_string(),
            major_versions: vec![],
            introduced_in: None,
            fields: Fields::default(),
            ty: Some("hello_foo_world".to_string())
        }
    )]
    fn test_deserialize(#[case] raw_data: &str, #[case] expected_data: Type) {
        let res = serde_json::from_str::<Type>(raw_data);

        assert!(
            res.is_ok(),
            "Failed to decode the raw json data: {}",
            res.unwrap_err()
        );
        assert_eq!(res.unwrap(), expected_data);
    }
}
