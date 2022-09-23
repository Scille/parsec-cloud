use std::{convert::TryFrom, fmt::Display};

use serde::Deserialize;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(try_from = "&str")]
pub struct MajorMinorVersion {
    pub major: u32,
    pub minor: u32,
}

impl TryFrom<&str> for MajorMinorVersion {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        fn parse_value(raw: &str, part: &str) -> Result<u32, String> {
            raw.parse()
                .map_err(|e| format!("Invalid {part} value `{raw}`: {e}"))
        }
        let (raw_major, raw_minor) = value
            .split_once('.')
            .ok_or_else(|| format!("Invalid major minor version format: `{value}`"))?;

        let major = parse_value(raw_major, "major")?;
        let minor = parse_value(raw_minor, "minor")?;
        Ok(Self { major, minor })
    }
}

impl Display for MajorMinorVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[cfg(test)]
#[rstest::rstest]
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
