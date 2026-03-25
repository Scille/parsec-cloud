// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(from = "Option<u64>", into = "Option<u64>")]
pub enum ActiveUsersLimit {
    LimitedTo(u64),
    NoLimit,
}

impl From<Option<u64>> for ActiveUsersLimit {
    fn from(value: Option<u64>) -> Self {
        match value {
            Some(x) => Self::LimitedTo(x),
            None => Self::NoLimit,
        }
    }
}

impl From<ActiveUsersLimit> for Option<u64> {
    fn from(value: ActiveUsersLimit) -> Self {
        match value {
            ActiveUsersLimit::LimitedTo(x) => Some(x),
            ActiveUsersLimit::NoLimit => None,
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/organization.rs"]
mod tests;
