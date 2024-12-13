// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use once_cell::sync::OnceCell;
use regex::{Captures, Regex};
use std::str::FromStr;

use libparsec_types::prelude::*;

// https://docs.microsoft.com/en-us/windows/win32/fileio/naming-a-file
// tl;dr: https://twitter.com/foone/status/1058676834940776450
const WIN32_RES_CHARS: [char; 39] = [
    '\x01', '\x02', '\x03', '\x04', '\x05', '\x06', '\x07', '\x08', '\t', '\n', '\x0b', '\x0c',
    '\r', '\x0e', '\x0f', '\x10', '\x11', '\x12', '\x13', '\x14', '\x15', '\x16', '\x17', '\x18',
    '\x19', '\x1a', '\x1b', '\x1c', '\x1d', '\x1e', '\x1f', '<', '>', ':', '"', '\\', '|', '?',
    '*',
]; // Ignore `\`
const WIN32_RES_NAMES: [&str; 22] = [
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

static RE: OnceCell<Regex> = OnceCell::new();

pub(crate) fn winify_entry_name(name: &EntryName) -> String {
    // Tilde is not a reserved character but it is used to escape reserved characters
    let ord_reserved = '~' as u8;
    let mut name = name.as_ref().replace('~', &format!("~{ord_reserved:02x}"));

    let (prefix, suffix) = name.split_once('.').unwrap_or((&name, ""));

    if WIN32_RES_NAMES.contains(&prefix) {
        let ord_last = prefix.chars().last().expect("Invalid EntryName") as u8;
        let prefix = &prefix[..prefix.len() - 1];
        if suffix.is_empty() {
            name = format!("{prefix}~{ord_last:02x}");
        } else {
            name = format!("{prefix}~{ord_last:02x}.{suffix}");
        }
    } else {
        for reserved in WIN32_RES_CHARS {
            let ord_reserved = reserved as u8;
            name = name.replace(reserved, &format!("~{ord_reserved:02x}"));
        }
    }

    match name.chars().last() {
        Some(c @ '.') | Some(c @ ' ') => {
            name = format!("{}~{:02x}", &name[..name.len() - 1], c as u8);
        }
        _ => (),
    }

    name
}

pub(crate) fn unwinify_entry_name(name: &str) -> EntryNameResult<EntryName> {
    // Given / is not allowed, no need to check if path already contains it
    if name.contains('~') {
        let re = RE.get_or_init(|| Regex::new("(~[0-9A-Fa-f]{2})").expect("Must be a valid regex"));
        let name = re.replace_all(name, |caps: &Captures| -> String {
            (u8::from_str_radix(&caps[0][1..], 16).unwrap_or_default() as char).to_string()
        });

        EntryName::from_str(&name)
    } else {
        EntryName::from_str(name)
    }
}

#[cfg(test)]
#[path = "../../tests/unit/windows_winify.rs"]
mod tests;
