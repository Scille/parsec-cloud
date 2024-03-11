// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::generate_volume_label;

use libparsec_tests_fixtures::prelude::*;
use winfsp_wrs::u16str;

#[test]
fn ok() {
    let label = generate_volume_label(&"test".parse().unwrap());
    p_assert_eq!(label, u16str!("test"));

    let max_len = "01234567890123456789012345678912";
    assert_eq!(max_len.len(), 32); // Sanity check
    let label = generate_volume_label(&max_len.parse().unwrap());
    p_assert_eq!(label, u16str!("01234567890123456789012345678912"));

    let max_with_surrogate = "𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢";
    assert_eq!(max_len.len(), 32); // Sanity check
    let label = generate_volume_label(&max_with_surrogate.parse().unwrap());
    p_assert_eq!(label, u16str!("𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢𤭢"));
}

#[test]
fn truncated() {
    let too_long = "012345678901234567890123456789123";
    assert_eq!(too_long.len(), 33); // Sanity check

    let label = generate_volume_label(&too_long.parse().unwrap());
    p_assert_eq!(label, u16str!("01234567890123456789012345678912"));
}

#[test]
fn truncated_with_surrogate() {
    // Last character is encoded in UTF16 as two u16, but it is on the edge of the
    // limit... so should be entirely removed.
    let too_long = "0123456789012345678901234567891𤭢";

    let label = generate_volume_label(&too_long.parse().unwrap());
    p_assert_eq!(label, u16str!("0123456789012345678901234567891"));
}
