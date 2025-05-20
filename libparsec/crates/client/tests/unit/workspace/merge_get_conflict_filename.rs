// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashSet;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::{FILENAME_CONFLICT_SUFFIX, get_conflict_filename};

#[test]
fn simple_with_extension() {
    let result = get_conflict_filename(
        &"child.txt".parse().unwrap(),
        FILENAME_CONFLICT_SUFFIX,
        |_entry_name| false,
    );
    p_assert_eq!(
        result,
        "child (Parsec - name conflict).txt".parse().unwrap()
    )
}

#[test]
fn simple_with_multiple_extensions() {
    let result = get_conflict_filename(
        &"child.tar.bz2".parse().unwrap(),
        FILENAME_CONFLICT_SUFFIX,
        |_entry_name| false,
    );
    p_assert_eq!(
        result,
        "child (Parsec - name conflict).tar.bz2".parse().unwrap()
    )
}

#[test]
fn simple_without_extension() {
    let result = get_conflict_filename(
        &"child".parse().unwrap(),
        FILENAME_CONFLICT_SUFFIX,
        |_entry_name| false,
    );
    p_assert_eq!(result, "child (Parsec - name conflict)".parse().unwrap())
}

#[test]
fn simple_with_leading_dot_and_extension() {
    let result = get_conflict_filename(
        &".child.txt".parse().unwrap(),
        FILENAME_CONFLICT_SUFFIX,
        |_entry_name| false,
    );
    p_assert_eq!(
        result,
        ".child (Parsec - name conflict).txt".parse().unwrap()
    )
}

#[test]
fn simple_with_leading_dot_and_no_extension() {
    let result = get_conflict_filename(
        &".child".parse().unwrap(),
        FILENAME_CONFLICT_SUFFIX,
        |_entry_name| false,
    );
    p_assert_eq!(result, ".child (Parsec - name conflict)".parse().unwrap())
}

#[test]
fn name_size_limit() {
    // EntryName is max 255 bytes long
    let expected_suffix = " (Parsec - name conflict)";
    let name: EntryName = "a".repeat(255 - expected_suffix.len()).parse().unwrap();
    let expected: EntryName = format!("{}{}", name, expected_suffix).parse().unwrap();
    p_assert_eq!(expected.as_ref().len(), 255); // Sanity check
    let result = get_conflict_filename(&name, FILENAME_CONFLICT_SUFFIX, |_entry_name| false);
    p_assert_eq!(result, expected)
}

#[test]
fn name_size_limit_with_leading_dot() {
    // EntryName is max 255 bytes long
    let expected_suffix = " (Parsec - name conflict)";
    let name: EntryName = format!(".{}", "a".repeat(254 - expected_suffix.len()))
        .parse()
        .unwrap();
    let expected: EntryName = format!("{}{}", name, expected_suffix).parse().unwrap();
    p_assert_eq!(expected.as_ref().len(), 255); // Sanity check
    let result = get_conflict_filename(&name, FILENAME_CONFLICT_SUFFIX, |_entry_name| false);
    p_assert_eq!(result, expected)
}

#[test]
fn name_size_limit_with_leading_dot_and_extension() {
    // EntryName is max 255 bytes long
    let expected_suffix = " (Parsec - name conflict)";
    let name: EntryName = format!(".{}.txt", "a".repeat(250 - expected_suffix.len()))
        .parse()
        .unwrap();
    let expected: EntryName = format!(
        ".{}{}.txt",
        "a".repeat(250 - expected_suffix.len()),
        expected_suffix
    )
    .parse()
    .unwrap();
    p_assert_eq!(expected.as_ref().len(), 255); // Sanity check
    let result = get_conflict_filename(&name, FILENAME_CONFLICT_SUFFIX, |_entry_name| false);
    p_assert_eq!(result, expected)
}

#[test]
fn name_size_limit_with_extension() {
    // EntryName is max 255 bytes long
    let expected_suffix = " (Parsec - name conflict)";
    let name: EntryName = format!(
        "{}.{}.{}.{}.{}.{}",
        "a".repeat(200 - expected_suffix.len()),
        "1".repeat(10),
        "2".repeat(10),
        "3".repeat(10),
        "4".repeat(10),
        "5".repeat(10),
    )
    .parse()
    .unwrap();
    p_assert_eq!(name.as_ref().len(), 255 - expected_suffix.len()); // Sanity check
    let expected: EntryName = format!(
        "{}{}.{}.{}.{}.{}.{}",
        "a".repeat(200 - expected_suffix.len()),
        expected_suffix,
        "1".repeat(10),
        "2".repeat(10),
        "3".repeat(10),
        "4".repeat(10),
        "5".repeat(10),
    )
    .parse()
    .unwrap();
    p_assert_eq!(expected.as_ref().len(), 255); // Sanity check
    let result = get_conflict_filename(&name, FILENAME_CONFLICT_SUFFIX, |_entry_name| false);
    p_assert_eq!(result, expected)
}

#[test]
fn name_too_long() {
    // EntryName is max 255 bytes long
    let expected_suffix = " (Parsec - name conflict)";
    assert_eq!(expected_suffix.len(), 25); // Sanity check, see below
    // When there is not enough space, name is truncated multiple times with
    // a 10 characters step until it fits.
    // Hence with our suffix, 30 characters should have been removed.
    let name: EntryName = "a".repeat(255).parse().unwrap();
    p_assert_eq!(name.as_ref().len(), 255); // Sanity check
    let expected: EntryName = format!("{}{}", &name.as_ref()[..255 - 30], expected_suffix)
        .parse()
        .unwrap();

    let result = get_conflict_filename(&name, FILENAME_CONFLICT_SUFFIX, |_entry_name| false);
    p_assert_eq!(result, expected)
}

#[test]
fn name_too_long_with_extension() {
    // EntryName is max 255 bytes long
    let expected_suffix = " (Parsec - name conflict)";
    assert_eq!(expected_suffix.len(), 25); // Sanity check, see below
    // When there is not enough space, name is truncated multiple times with
    // a 10 characters step until it fits.
    // Hence with our suffix, 30 characters should have been removed.
    let name: EntryName = format!("{}.txt", "a".repeat(251)).parse().unwrap();
    p_assert_eq!(name.as_ref().len(), 255); // Sanity check
    let expected: EntryName = format!("{}{}.txt", "a".repeat(251 - 30), expected_suffix)
        .parse()
        .unwrap();

    let result = get_conflict_filename(&name, FILENAME_CONFLICT_SUFFIX, |_entry_name| false);
    p_assert_eq!(result, expected)
}

#[test]
fn name_too_long_with_leading_dot() {
    // EntryName is max 255 bytes long
    let expected_suffix = " (Parsec - name conflict)";
    assert_eq!(expected_suffix.len(), 25); // Sanity check, see below
    // When there is not enough space, name is truncated multiple times with
    // a 10 characters step until it fits.
    // Hence with our suffix, 30 characters should have been removed.
    let name: EntryName = format!(".{}", "a".repeat(254)).parse().unwrap();
    p_assert_eq!(name.as_ref().len(), 255); // Sanity check
    let expected: EntryName = format!(".{}{}", "a".repeat(254 - 30), expected_suffix)
        .parse()
        .unwrap();

    let result = get_conflict_filename(&name, FILENAME_CONFLICT_SUFFIX, |_entry_name| false);
    p_assert_eq!(result, expected)
}

#[test]
fn name_too_long_with_leading_dot_and_extension() {
    // EntryName is max 255 bytes long
    let expected_suffix = " (Parsec - name conflict)";
    assert_eq!(expected_suffix.len(), 25); // Sanity check, see below
    // When there is not enough space, name is truncated multiple times with
    // a 10 characters step until it fits.
    // Hence with our suffix, 30 characters should have been removed.
    let name: EntryName = format!(".{}.txt", "a".repeat(250)).parse().unwrap();
    p_assert_eq!(name.as_ref().len(), 255); // Sanity check
    let expected: EntryName = format!(".{}{}.txt", "a".repeat(250 - 30), expected_suffix)
        .parse()
        .unwrap();

    let result = get_conflict_filename(&name, FILENAME_CONFLICT_SUFFIX, |_entry_name| false);
    p_assert_eq!(result, expected)
}

#[test]
fn one_extension_too_long() {
    // EntryName is max 255 bytes long
    let expected_suffix = " (Parsec - name conflict)";
    // There is not enough space, and it is all taken by the extension...
    // So the only solution is to remove the extension altogether.
    let name: EntryName = format!("a.{}", "x".repeat(253)).parse().unwrap();
    p_assert_eq!(name.as_ref().len(), 255); // Sanity check
    let expected: EntryName = format!("a{}", expected_suffix).parse().unwrap();

    let result = get_conflict_filename(&name, FILENAME_CONFLICT_SUFFIX, |_entry_name| false);
    p_assert_eq!(result, expected)
}

#[test]
fn multiple_extensions_too_long() {
    // EntryName is max 255 bytes long
    let expected_suffix = " (Parsec - name conflict)";
    // There is not enough space, and it is all taken by the extensions...
    // So the extensions are popped by the left one after another until there is enough space.
    let name: EntryName = format!("a..{}.{}.{}", "x", "y".repeat(150), "z".repeat(99),)
        .parse()
        .unwrap();
    p_assert_eq!(name.as_ref().len(), 255); // Sanity check
    let expected: EntryName = format!("a{}.{}", expected_suffix, "z".repeat(99),)
        .parse()
        .unwrap();

    let result = get_conflict_filename(&name, FILENAME_CONFLICT_SUFFIX, |_entry_name| false);
    p_assert_eq!(result, expected)
}

#[test]
fn multiple_extensions_too_long_with_leading_dot() {
    // EntryName is max 255 bytes long
    let expected_suffix = " (Parsec - name conflict)";
    // There is not enough space, and it is all taken by the extensions...
    // So the extensions are popped by the left one after another until there is enough space.
    let name: EntryName = format!(".a..{}.{}.{}", "x", "y".repeat(150), "z".repeat(98),)
        .parse()
        .unwrap();
    p_assert_eq!(name.as_ref().len(), 255); // Sanity check
    let expected: EntryName = format!(".a{}.{}", expected_suffix, "z".repeat(98),)
        .parse()
        .unwrap();

    let result = get_conflict_filename(&name, FILENAME_CONFLICT_SUFFIX, |_entry_name| false);
    p_assert_eq!(result, expected)
}

#[test]
fn name_clash_up_to_counter_2() {
    let bad_values =
        HashSet::<EntryName>::from_iter(["child (Parsec - name conflict).txt".parse().unwrap()]);
    let result = get_conflict_filename(
        &"child.txt".parse().unwrap(),
        FILENAME_CONFLICT_SUFFIX,
        move |entry_name| bad_values.contains(entry_name),
    );
    p_assert_eq!(
        result,
        "child (Parsec - name conflict 2).txt".parse().unwrap()
    )
}

#[test]
fn name_clash_up_to_counter_10() {
    let bad_values = HashSet::<EntryName>::from_iter([
        "child (Parsec - name conflict).txt".parse().unwrap(),
        "child (Parsec - name conflict 2).txt".parse().unwrap(),
        "child (Parsec - name conflict 3).txt".parse().unwrap(),
        "child (Parsec - name conflict 4).txt".parse().unwrap(),
        "child (Parsec - name conflict 5).txt".parse().unwrap(),
        "child (Parsec - name conflict 6).txt".parse().unwrap(),
        "child (Parsec - name conflict 7).txt".parse().unwrap(),
        "child (Parsec - name conflict 8).txt".parse().unwrap(),
        "child (Parsec - name conflict 9).txt".parse().unwrap(),
    ]);
    let result = get_conflict_filename(
        &"child.txt".parse().unwrap(),
        FILENAME_CONFLICT_SUFFIX,
        move |entry_name| bad_values.contains(entry_name),
    );
    p_assert_eq!(
        result,
        "child (Parsec - name conflict 10).txt".parse().unwrap()
    )
}
