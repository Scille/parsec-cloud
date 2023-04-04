// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_tests_types::rstest;
use libparsec_types::Regex;

#[rstest::rstest]
#[case("*.rs", "foo.rs", "bar.py", "bar.rs")]
#[case("*bar*", "foobar.rs", "dummy", "foobarrrrr.py")]
#[case("$myf$le.*", "$myf$le.txt", "bar", "$myf$le.rs")]
fn test_wildcard_pattern(
    #[case] pattern_str: &str,
    #[case] valid_case: &str,
    #[case] bad_base: &str,
    #[case] other_case: &str,
) {
    let regex = Regex::from_glob_pattern(pattern_str).expect("Should be valid");
    assert!(regex.is_match(valid_case));
    assert!(!regex.is_match(bad_base));
    assert!(regex.is_match(other_case));
}

#[rstest::rstest]
#[case(r"fooo?$", "foo", "fo", "fooo")]
fn test_regex_pattern(
    #[case] regex_str: &str,
    #[case] valid_case: &str,
    #[case] bad_base: &str,
    #[case] other_case: &str,
) {
    let regex = Regex::from_regex_str(regex_str).expect("Should be valid");
    assert!(regex.is_match(valid_case));
    assert!(!regex.is_match(bad_base));
    assert!(regex.is_match(other_case));
}

#[test]
fn test_bad_regex_str_creation() {
    let r = Regex::from_regex_str(r"fooo][?");
    assert!(r.is_err());
}
