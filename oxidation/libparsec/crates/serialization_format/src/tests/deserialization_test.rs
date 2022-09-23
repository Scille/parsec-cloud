use std::convert::{TryFrom, TryInto};

use crate::protocol::parser::{
    CustomEnum, CustomStruct, CustomType, Field, MajorMinorVersion, Variant,
};
use rstest::rstest;

#[rstest]
#[case("0.0", Ok(MajorMinorVersion { major: 0, minor: 0 }))]
#[case("4.2", Ok(MajorMinorVersion { major: 4, minor: 2 }))]
#[case::invalid_sep("1,2", Err("Invalid major minor version format: `1,2`".to_string()))]
#[case::major_not_number("a.2", Err("Invalid major value `a`: invalid digit found in string".to_string()))]
#[case::minor_not_number("1.b", Err("Invalid minor value `b`: invalid digit found in string".to_string()))]
#[case::empty_major(".2", Err("Invalid major value ``: cannot parse integer from empty string".to_string()))]
#[case::empty_minor("1.", Err("Invalid minor value ``: cannot parse integer from empty string".to_string()))]
fn major_minor_version(#[case] raw: &str, #[case] expected: Result<MajorMinorVersion, String>) {
    assert_eq!(MajorMinorVersion::try_from(raw), expected);
}

#[rstest]
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
    let custom_type = serde_json::from_str::<CustomType>(input).expect("Got error on valid data");
    assert_eq!(custom_type, expected);
}
