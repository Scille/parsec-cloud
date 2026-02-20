// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DeserializeFromStr, SerializeDisplay};
use std::{convert::TryFrom, fmt::Display, hash::Hash, ops::Deref, str::FromStr};
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

        paste::paste! {
            #[derive(Debug, Clone, Copy)]
            pub struct [<Invalid $name>];

            impl ::std::error::Error for [<Invalid $name>] {}

            impl ::std::fmt::Display for [<Invalid $name>] {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    f.write_str(concat!("Invalid ", stringify!($name)))
                }
            }

            impl ::std::convert::TryFrom<&[u8]> for $name {
                type Error = [<Invalid $name>];

                fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
                    ::uuid::Uuid::from_slice(bytes)
                        .map(Self)
                        .or(Err([<Invalid $name>]))
                }
            }

            impl $name {
                pub fn from_hex(hex: &str) -> Result<Self, [<Invalid $name>]> {
                    ::uuid::Uuid::parse_str(hex)
                        .map(Self)
                        .or(Err([<Invalid $name>]))
                }
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

        paste::paste! {
            #[derive(Debug, Clone, Copy)]
            pub struct [<Invalid $name>];

            impl ::std::error::Error for [<Invalid $name>] {}

            impl ::std::fmt::Display for [<Invalid $name>] {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    f.write_str(concat!("Invalid ", stringify!($name)))
                }
            }

            impl TryFrom<&str> for $name {
                type Error = [<Invalid $name>];

                fn try_from(s: &str) -> Result<Self, Self::Error> {
                    let id: String = s.nfc().collect();

                    if $match_fn(&id) {
                        Ok(Self(id))
                    } else {
                        Err([<Invalid $name>])
                    }
                }
            }

            // Note: FromStr is used for Deserialization !
            impl FromStr for $name {
                type Err = [<Invalid $name>];

                #[inline]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    $name::try_from(s)
                }
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
new_uuid_type!(pub AsyncEnrollmentID);
new_uuid_type!(pub PKIEnrollmentID);
new_uuid_type!(pub GreetingAttemptID);
new_uuid_type!(pub AccountAuthMethodID);
new_uuid_type!(pub AccountVaultItemOpaqueKeyID);
new_uuid_type!(pub TOTPOpaqueKeyID);
impl_from_maybe!(Option<TOTPOpaqueKeyID>);

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
fn match_organization_id(id: &str) -> bool {
    // `str::len` returns the number of bytes
    let bytes_size = id.len();

    let is_valid_char = |c: char| c == '-' || regex_syntax::is_word_character(c);

    (1..=32).contains(&bytes_size) && id.chars().all(is_valid_char)
}

/*
 * OrganizationID
 */

new_string_based_id_type!(pub OrganizationID, match_organization_id);

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
                    "Invalid nickname `{nickname}`, only a few are allowed (e.g. `alice`)"
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
                    "Invalid nickname `{nickname}`, only a few are allowed (e.g. `alice@dev1`)"
                )),
            }
        }
    }
}

pub use user_device_ids::{DeviceID, InvalidDeviceID, InvalidUserID, UserID};

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

#[serde_as]
#[derive(Clone, Serialize, Deserialize, Eq, PartialOrd)]
#[serde(try_from = "(&str, &str)", into = "(String, String)")]
#[non_exhaustive] // Prevent initialization without going through the factory
pub struct HumanHandle {
    email: EmailAddress,
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

#[derive(Debug, thiserror::Error)]
pub enum HumanHandleParseError {
    #[error("Email is missing")]
    MissingEmail,
    #[error("Invalid email address")]
    InvalidEmail,
    #[error("Invalid label")]
    InvalidLabel,
}

impl TryFrom<&str> for HumanHandle {
    type Error = HumanHandleParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let start = s
            .chars()
            .position(|c| c == '<')
            .ok_or(HumanHandleParseError::MissingEmail)?;
        let stop = s
            .chars()
            .position(|c| c == '>')
            .ok_or(HumanHandleParseError::MissingEmail)?;
        Self::from_raw(&s[start + 1..stop], &s[..start - 1])
    }
}

// Note: FromStr is used for Deserialization !
impl FromStr for HumanHandle {
    type Err = HumanHandleParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        HumanHandle::try_from(s)
    }
}

impl HumanHandle {
    pub fn new(email: EmailAddress, label: &str) -> Result<Self, HumanHandleParseError> {
        // A word about `<string>.nfc()`: In the unicode world we have multiple forms to represent the same glyph.
        // We have 2 notable forms :_Normalization From canonical Decomposition_ (NFD) and _Normalization From canonical Composition_ (NFC)
        // For example: the `small letter A with acute` (á) would be encoded in NFD as `small letter a + acute accent` as for NFC `small letter a with acute`.
        //
        // So we need to normalize the string to have consistent comparison latter on.
        let label = label.nfc().collect::<String>();
        let display = format!("{label} <{email}>");
        if !Self::label_is_valid(label.as_str()) {
            return Err(HumanHandleParseError::InvalidLabel);
        }

        Ok(Self {
            email,
            label,
            display,
        })
    }

    pub fn from_raw(email: &str, label: &str) -> Result<Self, HumanHandleParseError> {
        email
            .parse()
            .map_err(|_| HumanHandleParseError::InvalidEmail)
            .and_then(|email| Self::new(email, label))
    }

    /// Redacted certificate doesn't provide the real human handle, here we build
    /// a best effort one from the user ID:
    ///
    /// - label is user ID unchanged
    /// - email is `<user ID as hex>@redacted.invalid`
    pub fn new_redacted(user_id: UserID) -> Self {
        let label = user_id.hex();
        // Note we build `EmailAddress` while skipping its validation here:
        // - `HUMAN_HANDLE_RESERVED_REDACTED_DOMAIN` is not allowed since it is a
        //   special marker... but we specifically want to use this marker here!
        // - The email must contain only ASCII, this is guaranteed here since
        //   format is `<user ID as hex>@redacted.invalid`.
        let email = EmailAddress(
            email_address_parser::EmailAddress::new(
                &label,
                HUMAN_HANDLE_RESERVED_REDACTED_DOMAIN,
                None,
            )
            .expect("Invalid generated redacted email"),
        );
        let display = format!("{label} <{email}>");

        Self {
            email,
            label,
            display,
        }
    }

    pub fn uses_redacted_domain(&self) -> bool {
        self.email.get_domain() == HUMAN_HANDLE_RESERVED_REDACTED_DOMAIN
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

    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

impl TryFrom<(&str, &str)> for HumanHandle {
    type Error = HumanHandleParseError;

    fn try_from((email, label): (&str, &str)) -> Result<Self, Self::Error> {
        Self::from_raw(email, label)
    }
}

impl From<HumanHandle> for (String, String) {
    fn from(item: HumanHandle) -> (String, String) {
        (item.email.to_string(), item.label)
    }
}

impl_from_maybe!(Option<HumanHandle>);

/// An email address, nothing fancy... or is it ?
/// Email basically exists since the dawn of humanity,
/// and hence it is not that well standardized.
///
/// In a nutshell it is surprisingly hard to validate an email
/// address, and unicode support adds yet another layer of complexity :/
///
/// For instance:
/// - Domain names are case insensitive.
/// - Domain names support unicode with RFC5890.
/// - Unicode domain names can also be encoded in a ASCII format using punycode RFC3492.
/// - Local part of an email address (i.e. what is before the `@`) is left
///   to the mail server, so it may or may not be case insensitive and/or
///   support unicode and/or punycode.
///
/// However in practice:
/// - Unicode in email address is mostly used for homoglyph attacks.
/// - Actual people using unicode in their email address have learn to
///   provide them in punycode since most 3rd party services (e.g. registering
///   to a website) are very conservative when parsing email addresses (from my
///   experience, a `+` sign in the local part is already asking too much...).
///
/// Hence we choose the simple way here: only allow ASCII in email address.
#[derive(Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct EmailAddress(email_address_parser::EmailAddress);

impl Deref for EmailAddress {
    type Target = email_address_parser::EmailAddress;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialOrd for EmailAddress {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EmailAddress {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_domain()
            .cmp(other.get_domain())
            .then_with(|| self.get_local_part().cmp(other.get_local_part()))
    }
}

impl std::fmt::Debug for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EmailAddressParseError {
    #[error("Failed to parse email: {}", .0)]
    ParseError(#[from] ::core::fmt::Error),
    #[error("Invalid email domain")]
    InvalidDomain,
    #[error("Unicode email are not supported, use Punycode instead")]
    UnicodeNotSupported,
}

impl FromStr for EmailAddress {
    type Err = EmailAddressParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Note we don't use NFC normalization here, this is because any address
        // containing unicode is going to be rejected anyway when passed to
        // `EmailAddress::try_from`.
        s.parse::<email_address_parser::EmailAddress>()
            .map_err(EmailAddressParseError::ParseError)
            .and_then(Self::try_from)
    }
}

impl TryFrom<&str> for EmailAddress {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<email_address_parser::EmailAddress> for EmailAddress {
    type Error = EmailAddressParseError;

    fn try_from(value: email_address_parser::EmailAddress) -> Result<Self, Self::Error> {
        if !value.get_domain().is_ascii() || !value.get_local_part().is_ascii() {
            return Err(EmailAddressParseError::UnicodeNotSupported);
        }
        if value.get_domain() == HUMAN_HANDLE_RESERVED_REDACTED_DOMAIN {
            return Err(EmailAddressParseError::InvalidDomain);
        }
        Ok(Self(value))
    }
}

/*
 * UserProfile
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// `UserProfile` represents the different profiles a user can have in the organization.
///
/// This should not be confused with `RealmRole` (which represents the different roles
/// a user can have in a realm) !
pub enum UserProfile {
    ///`Admin` can invite and revoke users and on top of what standard user can do.
    Admin,
    /// `Standard` user can create new realms and invite new devices for himself.
    Standard,
    /// `Outsider` is only able to collaborate on existing realm and can only
    /// access redacted certificates (i.e. the realms created by an outsider
    /// cannot be shared and the outsider cannot be OWNER/MANAGER
    /// on a realm shared with him)
    Outsider,
}

#[derive(Debug, Clone, Copy)]
pub struct InvalidUserProfile;

impl std::error::Error for InvalidUserProfile {}

impl std::fmt::Display for InvalidUserProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Invalid UserProfile")
    }
}

impl FromStr for UserProfile {
    type Err = InvalidUserProfile;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ADMIN" => Ok(Self::Admin),
            "STANDARD" => Ok(Self::Standard),
            "OUTSIDER" => Ok(Self::Outsider),
            _ => Err(InvalidUserProfile),
        }
    }
}

impl Display for UserProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Admin => write!(f, "ADMIN"),
            Self::Standard => write!(f, "STANDARD"),
            Self::Outsider => write!(f, "OUTSIDER"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileDescriptor(pub u32);

#[cfg(test)]
#[path = "../tests/unit/id.rs"]
mod tests;
