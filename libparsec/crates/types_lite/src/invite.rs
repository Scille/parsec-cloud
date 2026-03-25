// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

/*
 * InvitationType
 */

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvitationType {
    User,
    Device,
    ShamirRecovery,
}

#[derive(Debug, Clone)]
pub struct InvitationTypeParseError;

impl std::error::Error for InvitationTypeParseError {}

impl std::fmt::Display for InvitationTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid InvitationType")
    }
}

impl FromStr for InvitationType {
    type Err = InvitationTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "USER" => Ok(Self::User),
            "DEVICE" => Ok(Self::Device),
            _ => Err(InvitationTypeParseError),
        }
    }
}

impl Display for InvitationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "USER"),
            Self::Device => write!(f, "DEVICE"),
            Self::ShamirRecovery => write!(f, "SHAMIR_RECOVERY"),
        }
    }
}
