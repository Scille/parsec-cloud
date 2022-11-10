// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use email_address_parser::EmailAddress;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::convert::TryFrom;
use std::hash::Hash;
use std::str::FromStr;
use unicode_normalization::UnicodeNormalization;

use crate::impl_from_maybe;

macro_rules! impl_debug_from_display {
    ($name:ident) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    (pub $name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
        pub struct $name(::uuid::Uuid);

        impl $name {
            pub fn as_hyphenated(&self) -> String {
                self.0.as_hyphenated().to_string()
            }

            pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
                ::uuid::Uuid::parse_str(hex)
                    .map(Self)
                    .or(Err(concat!("Invalid ", stringify!($name))))
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.0.as_simple())
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
    (pub $name:ident, $bytes_size:expr, $pattern:expr) => {
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

        // Note: FromStr is used for Deserialization !
        impl FromStr for $name {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let id: String = s.nfc().collect();
                lazy_static! {
                    static ref PATTERN: Regex =
                        Regex::new($pattern).unwrap_or_else(|_| unreachable!());
                }
                // ID must respect regex AND be contained within $bytes_size bytes
                if PATTERN.is_match(&id) && id.len() <= $bytes_size {
                    Ok(Self(id))
                } else {
                    Err(concat!("Invalid ", stringify!($name)))
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

new_uuid_type!(pub EntryID);
new_uuid_type!(pub BlockID);
new_uuid_type!(pub RealmID);
new_uuid_type!(pub VlobID);
new_uuid_type!(pub ChunkID);
new_uuid_type!(pub SequesterServiceID);
new_uuid_type!(pub InvitationToken);
new_uuid_type!(pub EnrollmentID);
impl_from_maybe!(std::collections::HashSet<EntryID>);

/*
 * OrganizationID
 */

new_string_based_id_type!(pub OrganizationID, 32, r"^[\w\-]{1,32}$");

/*
 * UserID
 */

new_string_based_id_type!(pub UserID, 32, r"^[\w\-]{1,32}$");

impl UserID {
    pub fn to_device_id(&self, device_name: &DeviceName) -> DeviceID {
        DeviceID {
            user_id: self.to_owned(),
            device_name: device_name.to_owned(),
        }
    }
}

/*
 * DeviceName
 */

new_string_based_id_type!(pub DeviceName, 32, r"^[\w\-]{1,32}$");

/*
 * DeviceLabel
*/

new_string_based_id_type!(pub DeviceLabel, 255, r"^.+$");
impl_from_maybe!(Option<DeviceLabel>);

/*
 * DeviceID
 */

#[derive(
    Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct DeviceID {
    pub user_id: UserID,
    pub device_name: DeviceName,
}

impl_debug_from_display!(DeviceID);

// Note: Display is used for Serialization !
impl std::fmt::Display for DeviceID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.user_id.as_ref())?;
        f.write_str("@")?;
        f.write_str(self.device_name.as_ref())
    }
}

// Note: FromStr is used for Deserialization !
impl FromStr for DeviceID {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERR: &str = "Invalid DeviceID";
        let (raw_user_id, raw_device_name) = s.split_once('@').ok_or(ERR)?;
        Ok(Self {
            user_id: raw_user_id.parse().map_err(|_| ERR)?,
            device_name: raw_device_name.parse().map_err(|_| ERR)?,
        })
    }
}

impl From<DeviceID> for String {
    fn from(item: DeviceID) -> String {
        format!("{}@{}", item.user_id, item.device_name)
    }
}

/*
 * HumanHandle
 */

#[derive(Clone, Serialize, Deserialize, Eq, PartialOrd)]
#[serde(try_from = "(&str, &str)", into = "(String, String)")]
#[non_exhaustive] // Prevent initialization without going through the factory
pub struct HumanHandle {
    pub email: String,
    // Label is purely informative
    pub label: String,
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
        write!(f, "{} <{}>", self.label, self.email)
    }
}

impl HumanHandle {
    pub fn new(email: &str, label: &str) -> Result<Self, &'static str> {
        let email = email.nfc().collect::<String>();
        let label = label.nfc().collect::<String>();

        if !EmailAddress::is_valid(&email, None) || email.len() >= 255 {
            return Err("Invalid email address");
        }
        // According to https://www.rfc-editor.org/rfc/rfc5322#section-3.2.3, these special characters are not allowed
        if label.is_empty()
            || label.len() >= 255
            || label.chars().any(|c| {
                matches!(
                    c,
                    '(' | ')' | '<' | '>' | '@' | ',' | ':' | ';' | '\\' | '"' | '[' | ']'
                )
            })
        {
            return Err("Invalid label");
        }

        Ok(Self { email, label })
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
pub enum UserProfile {
    /// Standard user can create new realms and invite new devices for himself.
    ///
    /// Admin can invite and revoke users and on top of what standard user can do.
    ///
    /// Outsider is only able to collaborate on existing realm and can only
    /// access redacted certificates (i.e. the realms created by an outsider
    /// cannot be shared and the outsider cannot be OWNER/MANAGER
    /// on a realm shared with him)
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
            _ => Err("Invalid InvitationType"),
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
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let too_long = "a".repeat(33);

        assert!(too_long.parse::<DeviceName>().is_err());
        assert!("pc1".parse::<DeviceName>().is_ok());

        assert!(too_long.parse::<UserID>().is_err());
        assert!("alice".parse::<UserID>().is_ok());

        assert!("dummy".parse::<DeviceID>().is_err());
        assert!(format!("alice@{}", too_long).parse::<DeviceID>().is_err());
        assert!("alice@pc1".parse::<DeviceID>().is_ok());
    }
}
