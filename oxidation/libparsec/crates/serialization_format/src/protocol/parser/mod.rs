// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub mod collection;
pub mod custom_type;
pub mod request;
pub mod response;

pub use collection::ProtocolCollection;
pub use custom_type::{CustomEnum, CustomStruct, CustomType, CustomTypes, Variant, Variants};
pub use request::Request;
pub use response::{Response, Responses};

use crate::shared::MajorMinorVersion;

use serde::Deserialize;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize)]
pub struct Protocol(pub Vec<Cmd>);

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Cmd {
    pub label: String,
    pub major_versions: Vec<u32>,
    /// When the Command was introduced during a major version lifetime but not from the start.
    /// This field is only used as documentation purpose.
    pub introduced_in: Option<MajorMinorVersion>,
    pub req: Request,
    #[serde(rename = "reps")]
    pub possible_responses: Responses,
    #[serde(default)]
    pub nested_types: CustomTypes,
}

#[cfg(test)]
impl Default for Cmd {
    fn default() -> Self {
        Self {
            label: "FooCmd".to_string(),
            major_versions: vec![],
            introduced_in: None,
            req: Request::default(),
            nested_types: CustomTypes::default(),
            possible_responses: Responses::default(),
        }
    }
}

#[cfg(test)]
#[rstest::rstest]
#[case::basic(
    r#"[]"#,
    Protocol(vec![])
)]
#[case::with_cmds(
    r#"[
        {
            "label": "FooCmd",
            "major_versions": [],
            "req": {
                "cmd": "foo_cmd",
                "fields": {}
            },
            "reps": {},
            "nested_types": {}
        },
        {
            "label": "FooCmd",
            "major_versions": [],
            "req": {
                "cmd": "foo_cmd",
                "fields": {}
            },
            "reps": {},
            "nested_types": {}
        }
    ]"#,
    Protocol(vec![
        Cmd::default(),
        Cmd::default()
    ])
)]
#[case::with_introduced_field(
    r#"[
        {
            "label": "FooCmd",
            "major_versions": [ 42 ],
            "introduced_in": "42.2",
            "req": {
                "cmd": "foo_cmd",
                "fields": {}
            },
            "reps": {}
        }
    ]"#,
    Protocol(vec![
        Cmd {
            major_versions: vec![42],
            introduced_in: Some("42.2".parse().unwrap()),
            ..Default::default()
        }
    ])
)]
fn test_deserialize(#[case] raw_str: &str, #[case] expected: Protocol) {
    use pretty_assertions::assert_eq;

    let res: Protocol = serde_json::from_str(raw_str).expect("Valid json provided");
    assert_eq!(res, expected);
}
