// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use crate::{RegexError, RegexResult};
use std::{collections::HashSet, fmt::Display, fs, path::Path};

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
    pub fn from_file(file_path: &Path) -> RegexResult {
        let file_content =
            fs::read_to_string(file_path).map_err(|err| RegexError::PatternFileIOError {
                file_path: file_path.to_path_buf(),
                err,
            })?;

        Ok(Self(
            file_content
                .lines()
                .map(str::trim)
                .filter(|l| *l != "\n" && !l.starts_with('#'))
                .map(|l| {
                    fnmatch_regex::glob_to_regex(&escape_globing_pattern(l))
                        .map_err(|err| RegexError::GlobPatternError { err })
                })
                .collect::<Result<Vec<regex::Regex>, RegexError>>()?,
        ))
    }

    /// Returns a regex which is an union of all regexes from `raw_regexes` slice parameter
    pub fn from_raw_regexes(raw_regexes: &[&str]) -> RegexResult {
        Ok(Self(
            raw_regexes
                .iter()
                .map(|l| regex::Regex::new(l).map_err(|err| RegexError::ParseError { err }))
                .collect::<Result<Vec<regex::Regex>, RegexError>>()?,
        ))
    }

    /// Returns a regex from a glob pattern
    pub fn from_glob_pattern(pattern: &str) -> RegexResult {
        let escaped_str = escape_globing_pattern(pattern);
        Ok(Regex(vec![fnmatch_regex::glob_to_regex(&escaped_str)
            .map_err(|err| RegexError::GlobPatternError { err })?]))
    }

    pub fn from_regex_str(regex_str: &str) -> RegexResult {
        Self::from_raw_regexes(&[regex_str])
    }

    pub fn is_match(&self, string: &str) -> bool {
        self.0.iter().any(|r| r.is_match(string))
    }
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
