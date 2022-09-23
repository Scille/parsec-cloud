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
