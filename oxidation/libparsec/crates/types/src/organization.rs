// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
use serde::{Deserialize, Serialize};

use crate::UsersPerProfileDetailItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationStats {
    pub users: u64,
    pub active_users: u64,
    pub realms: u64,
    pub data_size: u64,
    pub metadata_size: u64,
    pub users_per_profile_detail: Vec<UsersPerProfileDetailItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationConfig {
    pub user_profile_outsider_allowed: bool,
    pub active_users_limit: ActiveUsersLimit,
    // TODO: Should this be `SequesterAuthorityCertificate` instead of bytes?
    pub sequester_authority: Option<Vec<u8>>,
    // TODO: Should this be `SequesterServiceCertificate` instead of bytes?
    pub sequester_services: Option<Vec<Vec<u8>>>,
}

impl OrganizationConfig {
    pub fn is_sequestered_organization(&self) -> bool {
        self.sequester_authority.is_some()
    }
}

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
mod tests {
    use std::cmp::Ordering;

    use rstest::rstest;

    use super::ActiveUsersLimit;

    #[rstest]
    #[case::equal_no_limit(ActiveUsersLimit::NoLimit, ActiveUsersLimit::NoLimit, Ordering::Equal)]
    #[case::equal_with_limit(
        ActiveUsersLimit::LimitedTo(42),
        ActiveUsersLimit::LimitedTo(42),
        Ordering::Equal
    )]
    #[case::less_no_limit(
        ActiveUsersLimit::LimitedTo(42),
        ActiveUsersLimit::NoLimit,
        Ordering::Less
    )]
    #[case::less_with_limit(
        ActiveUsersLimit::LimitedTo(42),
        ActiveUsersLimit::LimitedTo(51),
        Ordering::Less
    )]
    #[case::greater_no_limit(
        ActiveUsersLimit::NoLimit,
        ActiveUsersLimit::LimitedTo(51),
        Ordering::Greater
    )]
    #[case::greater_with_limit(
        ActiveUsersLimit::LimitedTo(51),
        ActiveUsersLimit::LimitedTo(42),
        Ordering::Greater
    )]
    fn test_active_users_limit_ord(
        #[case] lhs: ActiveUsersLimit,
        #[case] rhs: ActiveUsersLimit,
        #[case] expected: Ordering,
    ) {
        assert_eq!(lhs.cmp(&rhs), expected)
    }
}
