// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::str::FromStr;

use fancy_regex::{self, RegexBuilder};

pub struct Regex(fancy_regex::Regex);

impl Regex {
    pub fn from_pattern(pattern: &str) -> Result<Self, fnmatch_regex::error::Error> {
        let str_regex = fnmatch_regex::glob_to_regex(pattern)?.to_string();
        Ok(Regex(fancy_regex::Regex::from_str(&str_regex).expect(
            "Failed to convert regex::Regex to fancy_regex::Regex",
        )))
    }

    pub fn from_regex_str(regex_str: &str) -> Result<Self, fancy_regex::Error> {
        let builder = RegexBuilder::new(regex_str);
        Ok(Regex(builder.build()?))
    }
}
