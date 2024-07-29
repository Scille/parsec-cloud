// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use email_address_parser::EmailAddress;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::convert::TryFrom;
use std::hash::Hash;
use std::str::FromStr;
use unicode_normalization::UnicodeNormalization;

use crate::impl_from_maybe;

const HUMAN_HANDLE_RESERVED_REDACTED_DOMAIN: &str = "redacted.invalid";

macro_rules! impl_debug_from_display {
    ($name:ident) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let display = self.to_string();
                f.debug_tuple(stringify!($name)).field(&display).finish()
            }
        }
    };
}

/*
 * UUID
 */

macro_rules! new_uuid_type {
    (pub $name:ident, no_debug) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub(crate) ::uuid::Uuid);

        new_uuid_type!(pub $name, __internal__);
    };
    (pub $name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub(crate) ::uuid::Uuid);

        new_uuid_type!(pub $name, __internal__);
    };
    (pub $name:ident, __internal__) => {
        impl $name {
            pub fn as_hyphenated(&self) -> &::uuid::fmt::Hyphenated {
                self.0.as_hyphenated()
            }

            pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
                ::uuid::Uuid::parse_str(hex)
                    .map(Self)
                    .or(Err(concat!("Invalid ", stringify!($name))))
            }

            pub fn hex(&self) -> String {
                self.0.as_simple().to_string()
            }

            pub fn as_u128(&self) -> u128 {
                self.0.as_u128()
            }

            pub fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes()
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.0.as_hyphenated())
            }
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self(::uuid::Uuid::new_v4())
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = ::uuid::Uuid;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::convert::AsRef<::uuid::Uuid> for $name {
            #[inline]
            fn as_ref(&self) -> &::uuid::Uuid {
                &self.0
            }
        }

        impl ::std::convert::AsRef<[u8;16]> for $name {
            #[inline]
            fn as_ref(&self) -> &[u8;16] {
                self.0.as_bytes()
            }
        }

        impl ::std::convert::From<::uuid::Uuid> for $name {
            #[inline]
            fn from(id: ::uuid::Uuid) -> Self {
                Self(id)
            }
        }

        impl ::std::convert::From<::uuid::Bytes> for $name {
            #[inline]
            fn from(bytes: ::uuid::Bytes) -> Self {
                Self(::uuid::Uuid::from_bytes(bytes))
            }
        }

        impl ::std::convert::TryFrom<&[u8]> for $name {
            type Error = &'static str;

            fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
                ::uuid::Uuid::from_slice(bytes)
                    .map(Self)
                    .or(Err(concat!("Invalid ", stringify!($name))))
            }
        }

        impl ::serde::Serialize for $name {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                // `rmp_serde::MSGPACK_EXT_STRUCT_NAME` is a magic value to tell
                // rmp_serde this should be treated as an extension type
                serializer.serialize_newtype_struct(
                    ::rmp_serde::MSGPACK_EXT_STRUCT_NAME,
                    &(
                        $crate::ext_types::UUID_EXT_ID,
                        ::serde_bytes::Bytes::new(self.as_bytes()),
                    ),
                )
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                // `rmp_serde::MSGPACK_EXT_STRUCT_NAME` is a magic value to tell
                // rmp_serde this should be treated as an extension type
                deserializer
                    .deserialize_newtype_struct(
                        ::rmp_serde::MSGPACK_EXT_STRUCT_NAME,
                        $crate::ext_types::UuidExtVisitor,
                    )
                    .map($name)
            }
        }
    };
}

macro_rules! new_string_based_id_type {
    (pub $name:ident, $match_fn:expr) => {
        #[derive(
            Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash, PartialOrd, Ord,
        )]
        pub struct $name(String);

        impl Default for $name {
            fn default() -> Self {
                Self(uuid::Uuid::new_v4().as_simple().to_string())
            }
        }

        impl std::convert::AsRef<str> for $name {
            #[inline]
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl_debug_from_display!($name);

        // Note: Display is used for Serialization !
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = &'static str;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                let id: String = s.nfc().collect();

                if $match_fn(&id) {
                    Ok(Self(id))
                } else {
                    Err(concat!("Invalid ", stringify!($name)))
                }
            }
        }

        // Note: FromStr is used for Deserialization !
        impl FromStr for $name {
            type Err = &'static str;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $name::try_from(s)
            }
        }

        impl From<$name> for String {
            fn from(item: $name) -> String {
                item.0
            }
        }
    };
}

new_uuid_type!(pub VlobID);
new_uuid_type!(pub BlockID);
new_uuid_type!(pub ChunkID);
new_uuid_type!(pub SequesterServiceID);
new_uuid_type!(pub EnrollmentID);
impl_from_maybe!(std::collections::HashSet<VlobID>);

// ChunkID are often created from file BlockID, so conversion is useful
impl From<BlockID> for ChunkID {
    fn from(value: BlockID) -> Self {
        Self(value.0)
    }
}
impl From<ChunkID> for BlockID {
    fn from(value: ChunkID) -> Self {
        Self(value.0)
    }
}

// Equivalent to pattern "^[\w\-]{1,32}$" on 32 bytes
#[inline]
fn match_legacy_id(id: &str) -> bool {
    // `str::len` returns the number of bytes
    let bytes_size = id.len();

    let is_valid_char = |c: char| c == '-' || regex_syntax::is_word_character(c);

    (1..=32).contains(&bytes_size) && id.chars().all(is_valid_char)
}

/*
 * OrganizationID
 */

new_string_based_id_type!(pub OrganizationID, match_legacy_id);

/*
 * UserID & DeviceID
 */

#[cfg(not(any(test, feature = "test-fixtures")))]
mod user_device_ids {
    use super::*;

    new_uuid_type!(pub UserID);
    new_uuid_type!(pub DeviceID);
}

#[cfg(any(test, feature = "test-fixtures"))]
mod user_device_ids {
    use super::*;
    use crate::fixtures;

    new_uuid_type!(pub UserID, no_debug);
    new_uuid_type!(pub DeviceID, no_debug);

    impl std::fmt::Debug for UserID {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.test_nickname() {
                Some(nickname) => f
                    .debug_struct("UserID")
                    .field("nickname", &nickname)
                    .field("id", &self.0.as_hyphenated().to_string())
                    .finish(),
                None => f
                    .debug_tuple("UserID")
                    .field(&self.0.as_hyphenated().to_string())
                    .finish(),
            }
        }
    }

    impl std::fmt::Debug for DeviceID {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.test_nickname() {
                Some(nickname) => f
                    .debug_struct("DeviceID")
                    .field("nickname", &nickname)
                    .field("id", &self.0.as_hyphenated().to_string())
                    .finish(),
                None => f
                    .debug_tuple("DeviceID")
                    .field(&self.0.as_hyphenated().to_string())
                    .finish(),
            }
        }
    }

    impl FromStr for UserID {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::test_from_nickname(s)
        }
    }

    impl TryFrom<&str> for UserID {
        type Error = String;

        fn try_from(s: &str) -> Result<Self, Self::Error> {
            Self::test_from_nickname(s)
        }
    }

    impl UserID {
        pub fn test_nickname(&self) -> Option<&'static str> {
            match *self {
                fixtures::ALICE_USER_ID => Some("alice"),
                fixtures::BOB_USER_ID => Some("bob"),
                fixtures::MALLORY_USER_ID => Some("mallory"),
                fixtures::MIKE_USER_ID => Some("mike"),
                fixtures::PHILIP_USER_ID => Some("philip"),
                _ => None,
            }
        }

        pub fn test_from_nickname(nickname: &str) -> Result<Self, String> {
            match nickname {
                "alice" => Ok(fixtures::ALICE_USER_ID),
                "bob" => Ok(fixtures::BOB_USER_ID),
                "carl" => Ok(fixtures::CARL_USER_ID),
                "diana" => Ok(fixtures::DIANA_USER_ID),
                "mallory" => Ok(fixtures::MALLORY_USER_ID),
                "mike" => Ok(fixtures::MIKE_USER_ID),
                "philip" => Ok(fixtures::PHILIP_USER_ID),
                _ => Err(format!(
                    "Invalid nickname `{}`, only a few are allowed (e.g. `alice`)",
                    nickname
                )),
            }
        }
    }

    impl FromStr for DeviceID {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::test_from_nickname(s)
        }
    }

    impl TryFrom<&str> for DeviceID {
        type Error = String;

        fn try_from(s: &str) -> Result<Self, Self::Error> {
            Self::test_from_nickname(s)
        }
    }

    impl DeviceID {
        pub fn test_nickname(&self) -> Option<&'static str> {
            match *self {
                fixtures::ALICE_DEV1_DEVICE_ID => Some("alice@dev1"),
                fixtures::BOB_DEV1_DEVICE_ID => Some("bob@dev1"),
                fixtures::MALLORY_DEV1_DEVICE_ID => Some("mallory@dev1"),
                fixtures::ALICE_DEV2_DEVICE_ID => Some("alice@dev2"),
                fixtures::BOB_DEV2_DEVICE_ID => Some("bob@dev2"),
                fixtures::MALLORY_DEV2_DEVICE_ID => Some("mallory@dev2"),
                fixtures::ALICE_DEV3_DEVICE_ID => Some("alice@dev3"),
                fixtures::MIKE_DEV1_DEVICE_ID => Some("mike@dev1"),
                fixtures::PHILIP_DEV1_DEVICE_ID => Some("philip@dev1"),
                _ => None,
            }
        }

        pub fn test_from_user_nickname(
            user_nickname: UserID,
            dev: u8,
        ) -> Result<DeviceID, &'static str> {
            match (user_nickname, dev) {
                (fixtures::ALICE_USER_ID, 1) => Ok(fixtures::ALICE_DEV1_DEVICE_ID),
                (fixtures::BOB_USER_ID, 1) => Ok(fixtures::BOB_DEV1_DEVICE_ID),
                (fixtures::MALLORY_USER_ID, 1) => Ok(fixtures::MALLORY_DEV1_DEVICE_ID),
                (fixtures::ALICE_USER_ID, 2) => Ok(fixtures::ALICE_DEV2_DEVICE_ID),
                (fixtures::BOB_USER_ID, 2) => Ok(fixtures::BOB_DEV2_DEVICE_ID),
                (fixtures::MALLORY_USER_ID, 2) => Ok(fixtures::MALLORY_DEV2_DEVICE_ID),
                (fixtures::ALICE_USER_ID, 3) => Ok(fixtures::ALICE_DEV3_DEVICE_ID),
                (fixtures::MIKE_USER_ID, 1) => Ok(fixtures::MIKE_DEV1_DEVICE_ID),
                (fixtures::PHILIP_USER_ID, 1) => Ok(fixtures::PHILIP_DEV1_DEVICE_ID),
                _ => Err("Invalid couple user/dev"),
            }
        }

        pub fn test_from_nickname(nickname: &str) -> Result<Self, String> {
            match nickname {
                "alice@dev1" => Ok(fixtures::ALICE_DEV1_DEVICE_ID),
                "bob@dev1" => Ok(fixtures::BOB_DEV1_DEVICE_ID),
                "mallory@dev1" => Ok(fixtures::MALLORY_DEV1_DEVICE_ID),
                "alice@dev2" => Ok(fixtures::ALICE_DEV2_DEVICE_ID),
                "bob@dev2" => Ok(fixtures::BOB_DEV2_DEVICE_ID),
                "mallory@dev2" => Ok(fixtures::MALLORY_DEV2_DEVICE_ID),
                "alice@dev3" => Ok(fixtures::ALICE_DEV3_DEVICE_ID),
                "mike@dev1" => Ok(fixtures::MIKE_DEV1_DEVICE_ID),
                "philip@dev1" => Ok(fixtures::PHILIP_DEV1_DEVICE_ID),
                _ => Err(format!(
                    "Invalid nickname `{}`, only a few are allowed (e.g. `alice@dev1`)",
                    nickname
                )),
            }
        }
    }
}

pub use user_device_ids::{DeviceID, UserID};

/*
 * DeviceLabel
*/

// Equivalent to pattern r"^.+$" (i.e. at least 1 character) on 255 bytes
#[inline]
fn match_device_label(id: &str) -> bool {
    // `str::len` returns the number of bytes
    let bytes_size = id.len();

    (1..=255).contains(&bytes_size)
}

new_string_based_id_type!(pub DeviceLabel, match_device_label);

impl DeviceLabel {
    pub fn new_redacted(device_id: DeviceID) -> Self {
        Self(device_id.hex())
    }
}
impl_from_maybe!(Option<DeviceLabel>);

/*
 * MaybeRedacted (used for DeviceLabel & HumanHandle in certificates)
 */

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaybeRedacted<T> {
    Real(T),
    Redacted(T),
}

impl<T> AsRef<T> for MaybeRedacted<T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Real(x) => x,
            Self::Redacted(x) => x,
        }
    }
}

impl<T> From<MaybeRedacted<T>> for Option<T> {
    fn from(value: MaybeRedacted<T>) -> Self {
        match value {
            MaybeRedacted::Real(x) => Some(x),
            MaybeRedacted::Redacted(_) => None,
        }
    }
}

/*
 * HumanHandle
 */

#[derive(Clone, Serialize, Deserialize, Eq, PartialOrd)]
#[serde(try_from = "(&str, &str)", into = "(String, String)")]
#[non_exhaustive] // Prevent initialization without going through the factory
pub struct HumanHandle {
    email: String,
    // Label is purely informative
    label: String,
    // Cache the display str
    display: String,
}

impl PartialEq for HumanHandle {
    fn eq(&self, other: &Self) -> bool {
        // Ignore label given it is purely informative
        self.email == other.email
    }
}

impl Hash for HumanHandle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.email.hash(state);
        self.label.hash(state);
    }
}

impl_debug_from_display!(HumanHandle);

impl std::fmt::Display for HumanHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.display.fmt(f)
    }
}

impl AsRef<str> for HumanHandle {
    fn as_ref(&self) -> &str {
        &self.display
    }
}

impl TryFrom<&str> for HumanHandle {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let start = s.chars().position(|c| c == '<').ok_or("Email is missing")?;
        let stop = s.chars().position(|c| c == '>').ok_or("Email is missing")?;
        Self::new(&s[start + 1..stop], &s[..start - 1])
    }
}

// Note: FromStr is used for Deserialization !
impl FromStr for HumanHandle {
    type Err = &'static str;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        HumanHandle::try_from(s)
    }
}

impl HumanHandle {
    pub fn new(email: &str, label: &str) -> Result<Self, &'static str> {
        // A word about `<string>.nfc()`: In the unicode code we have multiple forms to represent the same glyph.
        // We have 2 notable forms _Normalization From canonical Decomposition_ (NFD) and _Normalization From canonical Composition_ (NFC)
        // For example: the `small letter A with acute` (á) would be encoded in NFD as `small letter a + acute accent` as for NFC `small letter a with acute`.
        //
        // So we need to normalize the string to have consistant comparison latter on.
        let email = email.nfc().collect::<String>();
        let label = label.nfc().collect::<String>();
        let display = format!("{label} <{email}>");

        if !Self::email_is_valid(email.as_str()) {
            return Err("Invalid email address");
        }
        if !Self::label_is_valid(label.as_str()) {
            return Err("Invalid label");
        }

        Ok(Self {
            email,
            label,
            display,
        })
    }

    /// Redacted certificate doesn't provide the real human handle, here we build
    /// a best effort one from the user ID:
    ///
    /// - label is user ID unchanged
    /// - email is `<user ID as hex>@redacted.invalid``
    ///
    /// Note given user_id is case sensitive and email address is not, `uncaseify`
    /// is used to do the conversion while retaining ID unicity.
    pub fn new_redacted(user_id: UserID) -> Self {
        let label = user_id.hex();
        let email = format!("{}@{}", &label, HUMAN_HANDLE_RESERVED_REDACTED_DOMAIN);
        let display = format!("{label} <{email}>");

        Self {
            email,
            label,
            display,
        }
    }

    pub fn uses_redacted_domain(&self) -> bool {
        matches!(
            self.email.rsplit_once('@'),
            Some((_, domain)) if domain == HUMAN_HANDLE_RESERVED_REDACTED_DOMAIN
        )
    }

    pub fn email_is_valid(email: &str) -> bool {
        if email.len() < 255 {
            if let Some(parsed) = EmailAddress::parse(email, None) {
                if parsed.get_domain() != HUMAN_HANDLE_RESERVED_REDACTED_DOMAIN {
                    return true;
                }
            }
        }
        false
    }

    pub fn label_is_valid(label: &str) -> bool {
        !label.is_empty()
            && label.len() < 255
            && !label.chars().any(|c| {
                matches!(
                    c,
                    // According to https://www.rfc-editor.org/rfc/rfc5322#section-3.2.3, these special characters are not allowed
                    '<' | '>' | '@' | ',' | ':' | ';' | '\\' | '"' | '[' | ']'
                )
            })
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

impl TryFrom<(&str, &str)> for HumanHandle {
    type Error = &'static str;

    fn try_from(id: (&str, &str)) -> Result<Self, Self::Error> {
        Self::new(id.0, id.1)
    }
}

impl From<HumanHandle> for (String, String) {
    fn from(item: HumanHandle) -> (String, String) {
        (item.email, item.label)
    }
}

crate::impl_from_maybe!(Option<HumanHandle>);

/*
 * UserProfile
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
/// `UserProfile` represents the different profiles a user can have in the organization.
///
/// This should not be confused with `RealmRole` (which represents the different roles
/// a user can have in a realm) !
///
/// The different possible profiles are:
/// - `Admin` can invite and revoke users and on top of what standard user can do.
/// - `Standard` user can create new realms and invite new devices for himself.
/// - `Outsider` is only able to collaborate on existing realm and can only
///   access redacted certificates (i.e. the realms created by an outsider
///   cannot be shared and the outsider cannot be OWNER/MANAGER
///   on a realm shared with him)
pub enum UserProfile {
    Admin,
    Standard,
    Outsider,
}

impl FromStr for UserProfile {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ADMIN" => Ok(Self::Admin),
            "STANDARD" => Ok(Self::Standard),
            "OUTSIDER" => Ok(Self::Outsider),
            _ => Err("Invalid UserProfile"),
        }
    }
}

impl ToString for UserProfile {
    fn to_string(&self) -> String {
        match self {
            Self::Admin => String::from("ADMIN"),
            Self::Standard => String::from("STANDARD"),
            Self::Outsider => String::from("OUTSIDER"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileDescriptor(pub u32);

#[cfg(test)]
#[path = "../tests/unit/id.rs"]
mod tests;
