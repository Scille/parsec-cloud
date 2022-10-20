// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{fmt::Display, str::FromStr};

use serde::Deserialize;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(try_from = "&str")]
pub struct MajorMinorVersion {
    pub major: u32,
    pub minor: u32,
}

impl FromStr for MajorMinorVersion {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
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

impl TryFrom<&str> for MajorMinorVersion {
    type Error = <MajorMinorVersion as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        MajorMinorVersion::from_str(value)
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
fn test_from_str(#[case] raw: &str, #[case] expected: Result<MajorMinorVersion, String>) {
    assert_eq!(MajorMinorVersion::from_str(raw), expected);
}
