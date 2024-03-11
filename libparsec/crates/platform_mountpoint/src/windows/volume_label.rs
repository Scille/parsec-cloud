// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use winfsp_wrs::U16String;

use libparsec_types::prelude::*;

/// Volume label is limited to 32 WCHAR (i.e. u16) characters, if `workspace_name` is too
/// long it will be truncated.
pub(crate) fn generate_volume_label(workspace_name: &EntryName) -> U16String {
    let name_as_u16 = U16String::from_str(workspace_name.as_ref());
    // Truncating is not trivial given a unicode scalar value is encoded in UTF16 as
    // one or two u16 characters... So if we encode in UTF16 then truncate we might
    // end with half a surrogate pair (imagine 31 'a' characters, then a single emoji
    // that would be encoded as 2 u16 characters).
    if name_as_u16.len() > 32 {
        let mut name_as_u16_truncated = U16String::new();
        let mut budget: isize = 32;
        for c in name_as_u16.chars_lossy() {
            budget -= c.len_utf16() as isize;
            if budget < 0 {
                break;
            }
            name_as_u16_truncated.push_char(c);
        }
        name_as_u16_truncated
    } else {
        name_as_u16
    }
}

#[cfg(test)]
#[path = "../../tests/unit/windows_volume_label.rs"]
mod tests;
