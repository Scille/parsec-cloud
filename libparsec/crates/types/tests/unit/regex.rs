// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    io::{BufReader, Cursor},
    path::Path,
};

use libparsec_tests_lite::prelude::*;

use crate::regex::Regex;

#[rstest]
#[case::base("*.rs\n*.py", "base.tmp")]
#[case::trim_whitespace("  *.rs\n   *.py   ", "trim_whitespace.tmp")]
#[case::empty_lines("*.rs\n\n\n*.py", "empty_lines.tmp")]
#[case::trim_whitespace_and_empty_lines(
    "   *.rs\n\n  \n\n*.py  ",
    "trim_whitespace_and_empty_lines.tmp"
)]
#[case::ignore_comment(
    "# This contains patterns\n## yes\n   *.rs\n\n  \n\n*.py  ",
    "ignore_comment.tmp"
)]
fn from_pattern_file_content(#[case] file_content: &str, #[case] filename: &str) {
    let reader = Cursor::new(file_content.to_string());
    let regex =
        Regex::from_glob_reader(filename, BufReader::new(reader)).expect("Regex should be valid");

    assert!(regex.is_match("file.py"));
    assert!(regex.is_match("file.rs"));
    assert!(!regex.is_match("stories.txt"));
}

#[test]
fn load_default_pattern_file() {
    let regex = Regex::from_file(Path::new(
        "../client/src/workspace_ops/default_pattern.ignore",
    ))
    .expect("Load default pattern file failed");

    for pattern in &[
        "file.swp",
        "file~",
        "$RECYCLE.BIN",
        "desktop.ini",
        "shortcut.lnk",
    ] {
        assert!(regex.is_match(pattern), "Pattern {} should match", pattern);
    }

    for pattern in &["secrets.txt", "a.docx", "picture.png"] {
        assert!(
            !regex.is_match(pattern),
            "Pattern {} should not match",
            pattern
        );
    }
}

#[rstest]
#[case("*.rs", "foo.rs", "bar.py", "bar.rs")]
#[case("*bar*", "foobar.rs", "dummy", "foobarrrrr.py")]
#[case("$myf$le.*", "$myf$le.txt", "bar", "$myf$le.rs")]
fn wildcard_pattern(
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

#[rstest]
#[case(r"fooo?$", "foo", "fo", "fooo")]
fn regex_pattern(
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
fn bad_regex_str_creation() {
    let r = crate::regex::Regex::from_regex_str(r"fooo][?");
    assert!(r.is_err());
}
