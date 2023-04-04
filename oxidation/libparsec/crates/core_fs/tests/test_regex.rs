// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::Path;

use libparsec_tests_fixtures::{parsec_test, rstest, tmp_path, TmpPath};
use libparsec_types::Regex;

#[parsec_test]
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
fn from_pattern_file_content(
    tmp_path: TmpPath,
    #[case] file_content: &str,
    #[case] filename: &str,
) {
    let filepath = tmp_path.join(filename);
    std::fs::write(&filepath, file_content).unwrap();
    let regex = Regex::from_file(&filepath).expect("Regex should be valid");

    assert!(regex.is_match("file.py"));
    assert!(regex.is_match("file.rs"));
    assert!(!regex.is_match("stories.txt"));
}

#[test]
fn load_default_pattern_file() {
    let regex = Regex::from_file(Path::new(
        "../../../../parsec/core/resources/default_pattern.ignore",
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
