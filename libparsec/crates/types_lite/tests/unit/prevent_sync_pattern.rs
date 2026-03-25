// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::{prevent_sync_pattern::PreventSyncPattern, PreventSyncPatternError};

#[rstest]
#[case::base("*.rs\n*.py")]
#[case::trim_whitespace("  *.rs\n   *.py   ")]
#[case::empty_lines("*.rs\n\n\n*.py")]
#[case::trim_whitespace_and_empty_lines("   *.rs\n\n  \n\n*.py  ")]
#[case::ignore_comment("# This contains patterns\n## yes\n   *.rs\n\n  \n\n*.py  ")]
fn from_glob_ignore_file(#[case] file_content: &str) {
    let pattern = PreventSyncPattern::from_glob_ignore_file(file_content).unwrap();

    assert!(pattern.is_match("file.py"));
    assert!(pattern.is_match("file.rs"));
    assert!(!pattern.is_match("stories.txt"));
}

#[test]
fn load_default_pattern_file() {
    let pattern = PreventSyncPattern::default();

    for candidate in &[
        "file.swp",
        "file~",
        "~$file.docx",
        "$RECYCLE.BIN",
        "desktop.ini",
        "shortcut.lnk",
    ] {
        assert!(pattern.is_match(candidate), "{candidate:?} should match");
    }

    for candidate in &["secrets.txt", "a.docx", "picture.png"] {
        assert!(
            !pattern.is_match(candidate),
            "{candidate:?} should not match"
        );
    }
}

#[rstest]
#[case::match_wildcard("*.rs", "foo.rs", true)]
#[case::no_match_wildcard_differs("*.rs", "foo.py", false)]
#[case::match_wildcard_literal("*.rs", "*.rs", true)]
#[case::no_match_sub_extension("*.rs", "foo.rs.py", false)]
#[case::match_sub_extension("*.tar.gz", "foo.tar.gz", true)]
#[case::no_match_missing_wildcard(".py", "foo.py", false)]
#[case::match_with_wildcard_empty("*.rs", ".rs", true)]
#[case::match_multi_wildcard("*bar*", "foobar.rs", true)]
#[case::match_multi_wildcard_empty("*bar*", "bar", true)]
#[case::match_multi_wildcard_suffix("*bar*", "foo.bar", true)]
#[case::no_match_multi_wildcard("*bar*", "dummy", false)]
#[case::match_question("fo?bar", "foobar", true)]
#[case::match_question_literal("fo?bar", "fo?bar", true)]
#[case::no_match_question_missing("fo?bar", "fobar", false)] // cspell:disable-line
#[case::match_caret("^foo.bar^", "^foo.bar^", true)]
#[case::no_match_caret("^foo", "foo", false)]
#[case::match_dollar("$foo.bar$", "$foo.bar$", true)]
#[case::no_match_dollar("foo$", "foo", false)]
#[case::no_match_empty("", "foo", false)]
#[case::match_empty("", "", true)]
fn glob_pattern(#[case] glob: &str, #[case] candidate: &str, #[case] expected_match: bool) {
    let pattern = PreventSyncPattern::from_glob(glob).unwrap();
    p_assert_eq!(pattern.is_match(candidate), expected_match);
}

#[test]
fn from_multiple_globs() {
    let pattern =
        PreventSyncPattern::from_multiple_globs(["", "foo", "*.bar"].into_iter()).unwrap();

    assert!(pattern.is_match(""));
    assert!(pattern.is_match("foo"));
    assert!(pattern.is_match("foo.bar"));

    assert!(!pattern.is_match(".foo"));
}

#[rstest]
#[case::match_simple(r"fooo?$", "foo", true)]
#[case::match_complex(r"^spam.*fooo?$", "spambarfoo", true)] // cspell:disable-line
#[case::no_match_simple(r"fooo?$", "fo", false)]
fn regex_pattern(#[case] regex: &str, #[case] candidate: &str, #[case] expected_match: bool) {
    let pattern = PreventSyncPattern::from_regex(regex).unwrap();
    p_assert_eq!(pattern.is_match(candidate), expected_match);
}

#[test]
fn bad_regex() {
    p_assert_matches!(
        PreventSyncPattern::from_regex(r"fooo][?"),
        Err(PreventSyncPatternError::BadRegex { .. })
    );
}

#[test]
fn bad_glob() {
    p_assert_matches!(
        PreventSyncPattern::from_glob(r"fooo][?"),
        Err(PreventSyncPatternError::BadGlob { .. })
    );
}

#[test]
fn bad_multiple_globs() {
    p_assert_matches!(
        PreventSyncPattern::from_multiple_globs(["", "fooo][?"].into_iter()),
        Err(PreventSyncPatternError::BadGlob { .. })
    );
}

#[test]
fn display() {
    let pattern = PreventSyncPattern::from_multiple_globs(["foo.*", "bar?"].into_iter()).unwrap();
    p_assert_eq!(
        format!("{:?}", pattern),
        r#"PreventSyncPattern([Regex("^foo\\..*$"), Regex("^bar[^/]$")])"#
    );
}
