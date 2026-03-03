// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::FuzzyMatch;

fn path(s: &str) -> FsPath {
    s.parse().unwrap()
}

fn fuzzy_match(query: &str, path: &FsPath) -> Option<(u32, Vec<u32>)> {
    FuzzyMatch::new(query).matches(path)
}

#[test]
fn empty_query_matches_everything() {
    assert!(fuzzy_match("", &path("/")).is_some());
    assert!(fuzzy_match("", &path("/a/b/c.txt")).is_some());
}

#[test]
fn no_substring_does_not_match() {
    assert!(fuzzy_match("xyz", &path("/abc")).is_none());
    assert!(fuzzy_match("az", &path("/abc")).is_none());
}

#[test]
fn simple_substring_matches() {
    assert!(fuzzy_match("abc", &path("/abc")).is_some());
    assert!(fuzzy_match("rep", &path("/docs/report.txt")).is_some());
}

#[test]
fn word_based_multi_needle() {
    let p = path("/foo/bar.txt");
    // Each needle is independently a substring → match
    assert!(fuzzy_match("foo bar", &p).is_some());
    assert!(fuzzy_match("bar foo", &p).is_some());
    assert!(fuzzy_match("f b", &p).is_some());
    // Single needle containing '/' is a substring of the path
    assert!(fuzzy_match("foo/bar", &p).is_some());
    // Not a contiguous substring → no match
    assert!(fuzzy_match("fb", &p).is_none());
    assert!(fuzzy_match("foobar", &p).is_none());
}

#[test]
fn case_insensitive_match() {
    assert!(fuzzy_match("REP", &path("/docs/report.txt")).is_some());
    assert!(fuzzy_match("Rep", &path("/docs/report.txt")).is_some());
}

/// An NFD-encoded query (e.g. "e\u{301}" for "é") must match an NFC path
/// because the query is NFC-normalized before matching.
#[test]
fn nfd_query_matches_nfc_path() {
    // "café" in NFC: c a f \u{e9}
    // "café" in NFD: c a f e \u{301}  (e + combining acute accent)
    let nfc_path = path("/docs/café.txt");
    let nfd_query = "cafe\u{301}"; // NFD form of "café"
    assert!(fuzzy_match(nfd_query, &nfc_path).is_some());
}

#[test]
fn match_positions_are_ascending() {
    let (_, positions) = fuzzy_match("rep", &path("/docs/report.txt")).unwrap();
    assert!(positions.windows(2).all(|w| w[0] < w[1]));
}

#[test]
fn match_positions_cover_query_chars() {
    let p = path("/docs/report.txt");
    let path_str = p.to_string();
    let path_chars: Vec<char> = path_str.chars().collect();

    let (_, positions) = fuzzy_match("rep", &p).unwrap();
    let matched: String = positions.iter().map(|&i| path_chars[i as usize]).collect();
    assert_eq!(matched.to_lowercase(), "rep");
}

#[test]
fn leading_match_scores_higher() {
    let (leading, _) = fuzzy_match("rep", &path("/report.txt")).unwrap();
    let (other, _) = fuzzy_match("epo", &path("/report.txt")).unwrap();
    assert!(
        leading > other,
        "leading={leading} should be > other={other}"
    );
}

#[test]
fn name_match_scores_higher_than_parent_match() {
    let (in_name, _) = fuzzy_match("rep", &path("/other/report.txt")).unwrap();
    let (in_parent, _) = fuzzy_match("rep", &path("/reports/other.txt")).unwrap();
    assert!(
        in_name > in_parent,
        "in_name={in_name} should be > in_parent={in_parent}"
    );
}

#[test]
fn full_word_match_scores_higher_than_partial() {
    let (full, _) = fuzzy_match("report", &path("/docs/report.txt")).unwrap();
    let (part, _) = fuzzy_match("rep", &path("/docs/report.txt")).unwrap();
    assert!(full > part, "full={full} should be > part={part}");
}
