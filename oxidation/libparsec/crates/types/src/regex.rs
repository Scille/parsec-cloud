// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    collections::HashSet,
    fmt::Display,
    fs,
    io::{BufRead, BufReader, Read},
    path::Path,
};

use crate::{RegexError, RegexResult};

#[derive(Clone, Debug)]
pub struct Regex(pub Vec<regex::Regex>);

/// The fnmatch_regex crate does not escape the character '$'. It is a problem
/// for us. Since we need to match strings like `$RECYCLE.BIN` we need to escape
/// it anyway.
fn escape_globing_pattern(string: &str) -> String {
    str::replace(string, "$", "\\$")
}

impl Regex {
    /// Returns a regex which is built from a file that contains shell like patterns
    pub fn from_file(file_path: &Path) -> RegexResult<Self> {
        let reader = fs::File::open(file_path).map(BufReader::new).map_err(|e| {
            RegexError::PatternFileIOError {
                file_path: file_path.to_path_buf(),
                err: e,
            }
        })?;

        Self::from_glob_reader(file_path, reader)
    }

    pub fn from_glob_reader<P: AsRef<Path>, R: Read>(
        path: P,
        reader: BufReader<R>,
    ) -> RegexResult<Self> {
        reader
            .lines()
            .filter_map(|line| match line {
                Ok(line) => {
                    let l = line.trim();

                    if l != "\n" && !l.starts_with('#') {
                        Some(from_glob_pattern(l))
                    } else {
                        None
                    }
                }
                Err(e) => Some(Err(RegexError::PatternFileIOError {
                    file_path: path.as_ref().to_path_buf(),
                    err: e,
                })),
            })
            .collect::<RegexResult<Vec<regex::Regex>>>()
            .map(Self)
    }

    /// Returns a regex which is an union of all regexes from `raw_regexes` slice parameter
    pub fn from_raw_regexes(raw_regexes: &[&str]) -> RegexResult<Self> {
        Ok(Self(
            raw_regexes
                .iter()
                .map(|l| regex::Regex::new(l).map_err(|err| RegexError::ParseError { err }))
                .collect::<Result<Vec<regex::Regex>, RegexError>>()?,
        ))
    }

    /// Returns a regex from a glob pattern
    pub fn from_glob_pattern(pattern: &str) -> RegexResult<Self> {
        from_glob_pattern(pattern).map(|re| Self(vec![re]))
    }

    pub fn from_regex_str(regex_str: &str) -> RegexResult<Self> {
        Self::from_raw_regexes(&[regex_str])
    }

    pub fn is_match(&self, string: &str) -> bool {
        self.0.iter().any(|r| r.is_match(string))
    }
}

/// Parse a glob pattern like `*.rs` and convert it to an regex.
fn from_glob_pattern(pattern: &str) -> RegexResult<regex::Regex> {
    let escaped_str = escape_globing_pattern(pattern);
    fnmatch_regex::glob_to_regex(&escaped_str).map_err(|err| RegexError::GlobPatternError { err })
}

impl PartialEq for Regex {
    // This overload is only used in python tests. We are not using HashSet directly
    // in the `Regex` type because `regex::Regex` has no implementation of `Hash`.
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() == other.0.len() {
            let set: HashSet<&str> = HashSet::from_iter(self.0.iter().map(|r| r.as_str()));

            other.0.iter().all(|r| set.contains(r.as_str()))
        } else {
            false
        }
    }
}

impl Display for Regex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(regex::Regex::as_str)
                .collect::<Vec<_>>()
                .join("|")
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{BufReader, Cursor},
        path::Path,
    };

    use rstest::rstest;

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
        let regex = Regex::from_glob_reader(filename, BufReader::new(reader))
            .expect("Regex should be valid");

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

    #[rstest]
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

    #[rstest]
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
        let r = crate::regex::Regex::from_regex_str(r"fooo][?");
        assert!(r.is_err());
    }
}
