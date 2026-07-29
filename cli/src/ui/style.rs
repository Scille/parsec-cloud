use clap::{builder::PossibleValue, ValueEnum};
use std::{fmt::Display, io::IsTerminal};

/// How message should be styled.
///
/// A message is a piece of information that is not data but use to inform that the process
/// continues to operate.
///
/// Some messages could be:
///
/// - "We are syncing with the server"
/// - "Opening the device"
/// - "Cleaning the data, may takes awhile…"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressStyle {
    /// Do not display any message.
    Quiet,
    /// Display message but as plain text.
    Plain,
    /// Display message using spinner.
    Spinner,
}

impl Default for ProgressStyle {
    fn default() -> Self {
        if std::io::stderr().is_terminal() {
            ProgressStyle::Spinner
        } else {
            ProgressStyle::Plain
        }
    }
}

impl ValueEnum for ProgressStyle {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Quiet, Self::Plain, Self::Spinner]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            ProgressStyle::Quiet => PossibleValue::new("quiet"),
            ProgressStyle::Plain => PossibleValue::new("plain"),
            ProgressStyle::Spinner => PossibleValue::new("spinner"),
        })
    }
}

impl Display for ProgressStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

/// How data should be styled.
///
/// A data is the necessary output of a command (e.g. the list of users in `user list`)
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    /// Like [`DataStyle::Color`], but without the ANSI escape code.
    #[default]
    Plain,
    /// Display the data in JSON format.
    Json,
}

impl ValueEnum for DataFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Plain, Self::Json]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            DataFormat::Plain => PossibleValue::new("plain"),
            DataFormat::Json => PossibleValue::new("json"),
        })
    }
}

impl Display for DataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}
