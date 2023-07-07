// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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
fn active_users_limit_ord(
    #[case] lhs: ActiveUsersLimit,
    #[case] rhs: ActiveUsersLimit,
    #[case] expected: Ordering,
) {
    assert_eq!(lhs.cmp(&rhs), expected)
}
