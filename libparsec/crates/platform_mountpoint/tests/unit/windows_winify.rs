// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::{unwinify_entry_name, winify_entry_name};

#[parsec_test]
// except `/` which is not allowed in Parsec
#[case::invalid_chars(
    "<>:\"\\|?*".parse().unwrap(),
    "~3c~3e~3a~22~5c~7c~3f~2a".into()
)]
#[case::invalid_control_characters(
    (1u8..32).map(|x| x as char).collect::<String>().parse().unwrap(),
    (1u8..32).map(|x| format!("~{x:02x}")).collect::<Vec<_>>().concat(),
)]
#[case::trailing_dot_not_allowed(
    "foo.".parse().unwrap(),
    "foo~2e".into(),
)]
#[case::trailing_space_not_allowed(
    "foo ".parse().unwrap(),
    "foo~20".into(),
)]
#[case::invalid_name(
    "CON".parse().unwrap(),
    "CO~4e".into(),
)]
#[case::invalid_name_with_extension(
    "COM1.foo".parse().unwrap(),
    "COM~31.foo".into()
)]
#[case::literal_tilde(
    "foo~bar".parse().unwrap(),
    "foo~7ebar".into()  // cspell:disable-line
)]
#[case::literal_double_tilde(
    "foo~~bar".parse().unwrap(),
    "foo~7e~7ebar".into()  // cspell:disable-line
)]
#[case::literal_tilde_trailing(
    "foo~".parse().unwrap(),
    "foo~7e".into()
)]
fn test_winify(#[case] name: EntryName, #[case] expected: String) {
    let winified = winify_entry_name(&name);

    p_assert_eq!(winified, expected);

    let unwinified = unwinify_entry_name(&winified).unwrap();

    p_assert_eq!(unwinified, name);
}

#[parsec_test]
#[case::zero("foo\0bar")]
#[case::slash("foo/bar")]
#[case::tilde_zero("foo~\0bar")]
#[case::tilde_slash("foo~/bar")]
fn unwinify_forbidden_chars(#[case] bad: &str) {
    p_assert_matches!(unwinify_entry_name(bad), Err(EntryNameError::InvalidName));
}
