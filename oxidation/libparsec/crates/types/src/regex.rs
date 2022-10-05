// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::fmt::Display;

pub struct Regex(pub regex::Regex);

/// The fnmatch_regex crate does not escape the character '$'. It is a problem
/// for us. Since we need to match strings like `$RECYCLE.BIN` we need to escape
/// it anyway.
fn escape_globing_pattern(string: &str) -> String {
    str::replace(string, "$", "\\$")
}

impl Regex {
    pub fn from_pattern(pattern: &str) -> Result<Self, fnmatch_regex::error::Error> {
        let escaped_str = escape_globing_pattern(pattern);
        Ok(Regex(fnmatch_regex::glob_to_regex(&escaped_str)?))
    }

    pub fn from_regex_str(regex_str: &str) -> Result<Self, regex::Error> {
        Ok(Regex(regex::Regex::new(regex_str)?))
    }
}

impl AsRef<str> for Regex {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Display for Regex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::regex::Regex;
    use rstest::rstest;

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
        let regex = Regex::from_pattern(pattern_str).expect("Should be valid");
        assert!(regex.0.is_match(valid_case));
        assert!(!regex.0.is_match(bad_base));
        assert!(regex.0.is_match(other_case));
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
        assert!(regex.0.is_match(valid_case));
        assert!(!regex.0.is_match(bad_base));
        assert!(regex.0.is_match(other_case));
    }

    #[test]
    fn test_bad_regex_str_creation() {
        let r = crate::regex::Regex::from_regex_str(r"fooo][?");
        assert!(r.is_err());
    }
}
