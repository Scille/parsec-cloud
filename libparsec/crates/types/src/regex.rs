// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
#[path = "../tests/unit/regex.rs"]
mod tests;
