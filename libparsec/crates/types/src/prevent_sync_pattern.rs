// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashSet, fmt::Display};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PreventSyncPatternError {
    #[error("Regex parsing error: {0}")]
    BadRegex(regex::Error),
    #[error("Glob parsing error: {0}")]
    BadGlob(fnmatch_regex::error::Error),
}

pub type PreventSyncPatternResult<T> = Result<T, PreventSyncPatternError>;

#[derive(Clone, Debug)]
pub struct PreventSyncPattern(pub Vec<regex::Regex>);

const DEFAULT_PREVENT_SYNC_PATTERN: &str = std::include_str!("default_pattern.ignore");

impl Default for PreventSyncPattern {
    fn default() -> Self {
        Self::from_glob_ignore_file(DEFAULT_PREVENT_SYNC_PATTERN)
            .expect("Cannot parse default prevent sync pattern")
    }
}

impl PreventSyncPattern {
    /// Glob ignore file format is simply:
    /// - Each line contains either a glob pattern of a comment
    /// - Each line is first trimmed of leading/trailing whitespaces
    /// - Comment lines start with a `#` character
    ///
    /// Example:
    /// ```raw
    /// # Ignore C stuff
    /// *.{so,o}
    ///
    /// # Ignore Python stuff
    /// *.pyc
    /// ```
    ///
    /// (see `default_pattern.ignore` for a more complex example)
    pub fn from_glob_ignore_file(file_content: &str) -> PreventSyncPatternResult<Self> {
        let regexes = file_content
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    None
                } else {
                    Some(regex_from_glob_pattern(line))
                }
            })
            .collect::<Result<_, _>>()?;
        Ok(Self(regexes))
    }

    /// Create a prevent sync pattern from a regex (regular expression, e.g. `^.*\.(txt|md)$`)
    pub fn from_regex(regex: &str) -> PreventSyncPatternResult<Self> {
        let regex = regex::Regex::new(regex).map_err(PreventSyncPatternError::BadRegex)?;
        Ok(Self(vec![regex]))
    }

    /// Create a prevent sync pattern from a glob pattern (e.g. `*.{txt,md}`)
    pub fn from_glob(pattern: &str) -> PreventSyncPatternResult<Self> {
        regex_from_glob_pattern(pattern).map(|re| Self(vec![re]))
    }

    pub fn from_multiple_globs<'a>(
        patterns: impl Iterator<Item = &'a str>,
    ) -> PreventSyncPatternResult<Self> {
        let regexes = patterns
            .map(regex_from_glob_pattern)
            .collect::<Result<_, _>>()?;
        Ok(Self(regexes))
    }

    pub fn is_match(&self, string: &str) -> bool {
        self.0.iter().any(|r| r.is_match(string))
    }

    /// Create an empty prevent sync pattern that will never match anything
    pub const fn empty() -> Self {
        Self(Vec::new())
    }
}

/// Parse a glob pattern like `*.rs` and convert it to an regex.
fn regex_from_glob_pattern(pattern: &str) -> PreventSyncPatternResult<regex::Regex> {
    fnmatch_regex::glob_to_regex(pattern).map_err(PreventSyncPatternError::BadGlob)
}

impl PartialEq for PreventSyncPattern {
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

impl Display for PreventSyncPattern {
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
#[path = "../tests/unit/prevent_sync_pattern.rs"]
mod tests;
