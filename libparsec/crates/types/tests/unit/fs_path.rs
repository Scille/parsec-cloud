// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec_tests_lite::prelude::*;

use super::*;

#[test]
fn entry_name_too_long() {
    let good = "a".repeat(255);
    p_assert_matches!(good.parse(), Ok(EntryName(name)) if name == good);

    let bad = "a".repeat(256);
    p_assert_matches!(bad.parse::<EntryName>(), Err(EntryNameError::NameTooLong));

    // Same for FsPath
    let bad = format!("/{}", bad);
    p_assert_matches!(
        bad.parse::<FsPath>(),
        Err(FsPathError::InvalidEntry(EntryNameError::NameTooLong))
    );
}

#[rstest]
#[case("")]
#[case(".")]
#[case("..")]
#[case("/")]
#[case("a/a")]
#[case("a/")]
#[case("\x00")]
#[case("a\x00")]
#[case("\x00a")]
fn entry_name_invalid(#[case] bad: &str) {
    p_assert_matches!(bad.parse::<EntryName>(), Err(EntryNameError::InvalidName));
}

#[rstest]
#[case("/\x00")]
#[case("/a\x00")]
#[case("/\x00a")]
fn fs_path_invalid(#[case] bad: &str) {
    p_assert_matches!(
        bad.parse::<FsPath>(),
        Err(FsPathError::InvalidEntry(EntryNameError::InvalidName))
    );
}

#[test]
fn fs_path_not_absolute() {
    p_assert_matches!("foo".parse::<FsPath>(), Err(FsPathError::NotAbsolute));
}

#[rstest]
#[case("a")]
#[case("Aa")]
#[case("_-\\#@!~&")]
#[case("🌊⛵")]
#[case("張飛")]
fn entry_name_parse_roundtrip(#[case] good: &str) {
    let parsed = good.parse::<EntryName>().unwrap();
    p_assert_matches!(&parsed, EntryName(name) if name == good);
    p_assert_eq!(parsed.to_string(), good);
}

#[test]
fn entry_name_as_ref() {
    let entry_name = "foo".parse::<EntryName>().unwrap();
    p_assert_eq!(entry_name.as_ref(), "foo");
    p_assert_eq!(format!("{}", entry_name), "foo");
}

#[test]
fn entry_name_debug() {
    let entry_name = "foo".parse::<EntryName>().unwrap();
    p_assert_eq!(format!("{:?}", entry_name), "EntryName(\"foo\")");
}

#[rstest]
#[case("/", "/", None, true, "/", &[])]
#[case("/a", "/a", Some("a"), false, "/", &["a"])]
#[case("/a/b", "/a/b", Some("b"), false, "/a", &["a", "b"])]
#[case("/a/b/", "/a/b", Some("b"), false, "/a", &["a", "b"])]
#[case("/.", "/", None, true, "/", &[])]
#[case("/..", "/", None, true, "/", &[])]
#[case("/../.././", "/", None, true, "/", &[])]
#[case("/a/../b", "/b", Some("b"), false, "/", &["b"])]
#[case("/a/b/../c", "/a/c", Some("c"), false, "/a", &["a", "c"])]
#[case("/../a", "/a", Some("a"), false, "/", &["a"])]
#[case("/.././../a/.", "/a", Some("a"), false, "/", &["a"])]
fn fs_path_attributes(
    #[case] raw: &str,
    #[case] cooked_raw: &str,
    #[case] name: Option<&str>,
    #[case] is_root: bool,
    #[case] parent: &str,
    #[case] parts: &[&str],
) {
    let parsed: FsPath = raw.parse().unwrap();

    // Roundtrip & display
    p_assert_eq!(parsed.to_string(), cooked_raw);
    p_assert_eq!(format!("{}", &parsed), cooked_raw);

    let name = name.map(|raw| raw.parse::<EntryName>().unwrap());
    p_assert_eq!(parsed.name(), name.as_ref());

    p_assert_eq!(parsed.is_root(), is_root);

    let parts = parts
        .iter()
        .map(|raw| raw.parse::<EntryName>().unwrap())
        .collect::<Vec<_>>();
    p_assert_eq!(parsed.parts(), parts);

    p_assert_eq!(parsed.parent(), parent.parse().unwrap());
}

#[test]
fn fs_path_debug() {
    let fs_path = "/foo/bar".parse::<FsPath>().unwrap();
    p_assert_eq!(format!("{:?}", fs_path), "FsPath(\"/foo/bar\")");
}

#[test]
fn fs_path_with_mountpoint() {
    let fs_path = "/foo/bar".parse::<FsPath>().unwrap();
    let base_path = PathBuf::try_from("/home/bob/parsec").unwrap();
    p_assert_eq!(
        fs_path.with_mountpoint(&base_path),
        base_path.join("foo/bar")
    )
}

#[test]
fn fs_path_join() {
    let fs_path = "/".parse::<FsPath>().unwrap();

    let fs_path2 = fs_path.join("foo".parse().unwrap());
    p_assert_eq!(fs_path, "/".parse().unwrap());
    p_assert_eq!(fs_path2, "/foo".parse().unwrap());

    let fs_path3 = fs_path2.join("bar".parse().unwrap());
    p_assert_eq!(fs_path3, "/foo/bar".parse().unwrap());
}

#[test]
fn fs_path_into_parent() {
    let fs_path = "/foo/bar".parse::<FsPath>().unwrap();

    let (fs_path, child) = fs_path.into_parent();
    p_assert_eq!(fs_path, "/foo".parse().unwrap());
    p_assert_eq!(child, Some("bar".parse().unwrap()));

    let (fs_path, child) = fs_path.into_parent();
    p_assert_eq!(fs_path, "/".parse().unwrap());
    p_assert_eq!(child, Some("foo".parse().unwrap()));

    let (fs_path, child) = fs_path.into_parent();
    p_assert_eq!(fs_path, "/".parse().unwrap());
    p_assert_eq!(child, None);
}

#[test]
fn fs_path_into_child() {
    let fs_path = "/".parse::<FsPath>().unwrap();

    let fs_path = fs_path.into_child("foo".parse().unwrap());
    p_assert_eq!(fs_path, "/foo".parse().unwrap());

    let fs_path = fs_path.into_child("bar".parse().unwrap());
    p_assert_eq!(fs_path, "/foo/bar".parse().unwrap());
}

#[rstest]
#[case::simple("/a/b", "/a/b")]
#[case::single_dot("/a/./b", "/a/b")]
#[case::multiple_dot("/a/././b/./c", "/a/b/c")]
#[case::single_dot_dot("/a/../b", "/b")]
#[case::dot_dot_to_the_root("/a/../../../b", "/b")]
#[case::multiple_dot_dot("/a/b/../../c", "/c")]
#[case::mix_dot_and_dot_dot("/./a/../../b/././c/.", "/b/c")]
#[case::start_end_with_dot_dot("/../../a/b/../c/../../..", "/")]
fn fs_path_normalize(#[case] raw_path: &str, #[case] expected: FsPath) {
    let raw_parts = raw_path
        .split('/')
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        })
        .map(EntryName)
        .collect();
    let dirty_path = FsPath { parts: raw_parts };

    let cleaned_path = dirty_path.normalize();

    p_assert_eq!(cleaned_path, expected)
}
