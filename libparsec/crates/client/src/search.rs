// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;
use unicode_normalization::UnicodeNormalization;

/// The algorithm is a word-based substring matcher:
/// - The query is first NFC-normalized (since that's what `FsPath` does)
/// - The query is then split on unicode whitespace into *needles*.
/// - Every needle must appear as a contiguous, case-insensitive substring
///   somewhere in the path.  This is the accept/reject gate.
/// - The score rewards patterns that indicate a strong match:
///   - full word match (no unicode alphabetic/numeric chars on either side) (+3 per needle)
///   - first match at position 0                                            (+5)
///   - match lands in the name component                                    (+8 per needle)
///
/// Examples (path = `/foo/bar.txt`):
///   "foo bar"  → matches ("foo" ✓, "bar" ✓)
///   "bar foo"  → matches ("bar" ✓, "foo" ✓)
///   "foo/bar"  → matches (single needle, substring of path)
///   "f b"      → matches ("f" ✓, "b" ✓)
///   "fb"       → no match (not a contiguous substring)
///   "foobar"   → no match (not a contiguous substring)
pub(crate) struct FuzzyMatch {
    /// Pre-processed needles: each is already NFC-normalized and lowercased.
    needles: Vec<Vec<char>>,
}

impl FuzzyMatch {
    pub(crate) fn new(query: &str) -> Self {
        // NFC-normalize the query so that decomposed characters (e.g. NFD "é")
        // match the NFC-normalized form stored in FsPath.
        let query_nfc: String = query.nfc().collect();
        let needles = query_nfc
            .split_whitespace()
            .map(|needle| needle.to_lowercase().chars().collect())
            .collect();
        Self { needles }
    }

    /// Returns `Some((score, char_positions))` when `self` matches `path`,
    /// `None` otherwise.
    ///
    /// `char_positions` contains the *character* (not byte) indices into
    /// `path.to_string()` of each matched character, in ascending order.
    pub(crate) fn matches(&self, path: &FsPath) -> Option<(u32, Vec<u32>)> {
        if self.needles.is_empty() {
            return Some((0, vec![]));
        }

        let path_str = path.to_string();
        let path_chars: Vec<char> = path_str.chars().collect();
        let path_lower_chars: Vec<char> = path_str.to_lowercase().chars().collect();

        let name_char_start = name_char_start(path, &path_str);
        let mut all_positions: Vec<u32> = Vec::new();
        let mut score = 0u32;

        for needle_lower in &self.needles {
            let nlen = needle_lower.len();

            // Find the first case-insensitive substring occurrence.
            let found = (0..=path_lower_chars.len().saturating_sub(nlen))
                .find(|&start| path_lower_chars[start..start + nlen] == needle_lower[..]);

            match found {
                Some(start) => {
                    for i in 0..nlen {
                        all_positions.push((start + i) as u32);
                    }

                    // Full word match bonus: neither the character immediately before
                    // the match nor the one immediately after is alphabetic or numeric.
                    let before_is_word = start > 0 && {
                        let c = path_chars[start - 1];
                        c.is_alphabetic() || c.is_numeric()
                    };
                    let after_is_word = start + nlen < path_chars.len() && {
                        let c = path_chars[start + nlen];
                        c.is_alphabetic() || c.is_numeric()
                    };
                    if !before_is_word && !after_is_word {
                        score += 3;
                    }

                    // Leading name match bonus: needle starts at the first character
                    // of the name component (e.g. "rep" in "/report.txt").
                    if start as u32 == name_char_start {
                        score += 5;
                    }
                }
                None => return None,
            }
        }

        all_positions.sort();
        all_positions.dedup();

        // First match at position 0 bonus (e.g. needle contains '/').
        if all_positions.first().copied() == Some(0) {
            score += 5;
        }

        score += score_positions(&all_positions, name_char_start);

        Some((score, all_positions))
    }
}

/// Returns the character index at which the last path component (the name)
/// starts inside `path_str`, or `u32::MAX` when there is no name (root).
fn name_char_start(path: &FsPath, path_str: &str) -> u32 {
    match path.name() {
        None => u32::MAX, // root has no name
        Some(name) => {
            let name_bytes = name.as_ref().len();
            let path_bytes = path_str.len();
            // The name occupies the last `name_bytes` bytes, preceded by '/'.
            // Convert the byte offset to a character offset.
            let byte_start = path_bytes.saturating_sub(name_bytes);
            path_str[..byte_start].chars().count() as u32
        }
    }
}

/// Per-character score contribution: base point plus name-component bonus.
fn score_positions(positions: &[u32], name_char_start: u32) -> u32 {
    positions.iter().fold(0u32, |acc, &pos| {
        let mut pts = 1; // base: one point per matched character
        if pos >= name_char_start {
            pts += 8; // name-component bonus
        }
        acc + pts
    })
}

#[cfg(test)]
#[path = "../tests/unit/search.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
