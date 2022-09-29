pub mod collection;
pub mod custom_type;
pub mod field;
pub mod major_minor_version;
pub mod request;
pub mod response;

pub use collection::ProtocolCollection;
pub use custom_type::{CustomEnum, CustomStruct, CustomType, Variant};
pub use field::Field;
pub use major_minor_version::MajorMinorVersion;
pub use request::Request;
pub use response::Response;

use serde::Deserialize;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize)]
pub struct Protocol(pub Vec<Cmd>);

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Cmd {
    pub label: String,
    pub major_versions: Vec<u32>,
    pub req: Request,
    #[serde(rename = "reps")]
    pub possible_responses: Vec<Response>,
    #[serde(default)]
    // TODO: May need to be put in a option.
    pub nested_types: Vec<CustomType>,
}

#[cfg(test)]
impl Default for Cmd {
    fn default() -> Self {
        Self {
            label: "FooCmd".to_string(),
            major_versions: vec![],
            req: Request::default(),
            nested_types: vec![],
            possible_responses: vec![],
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
                "other_fields": []
            },
            "reps": [],
            "nested_types": []
        },
        {
            "label": "FooCmd",
            "major_versions": [],
            "req": {
                "cmd": "foo_cmd",
                "other_fields": []
            },
            "reps": [],
            "nested_types": []
        }
    ]"#,
    Protocol(vec![
        Cmd::default(),
        Cmd::default()
    ])
)]
fn test_deserialize(#[case] raw_str: &str, #[case] expected: Protocol) {
    use pretty_assertions::assert_eq;

    let res: Protocol = serde_json::from_str(raw_str).expect("Valid json provided");
    assert_eq!(res, expected);
}
