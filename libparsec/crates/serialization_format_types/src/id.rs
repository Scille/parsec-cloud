// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{fmt::Display, str::FromStr};

/*
 * UUID-based types
 */

macro_rules! new_uuid_type {
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

        impl ::std::convert::AsRef<[u8; 16]> for $name {
            #[inline]
            fn as_ref(&self) -> &[u8; 16] {
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

        impl $crate::rmp_serialize::Serialize for $name {
            fn serialize(
                &self,
                writer: &mut Vec<u8>,
            ) -> Result<(), $crate::rmp_serialize::SerializeError> {
                $crate::rmp_serialize::encode::write_ext_meta(
                    writer,
                    16,
                    $crate::ext_types::UUID_EXT_ID,
                )?;
                writer.extend_from_slice(self.0.as_bytes());
                Ok(())
            }
        }

        impl $crate::rmp_serialize::Deserialize for $name {
            fn deserialize(
                value: $crate::rmp_serialize::ValueRef<'_>,
            ) -> Result<Self, $crate::rmp_serialize::DeserializeError> {
                match value {
                    $crate::rmp_serialize::ValueRef::Ext(tag, data)
                        if tag == $crate::ext_types::UUID_EXT_ID =>
                    {
                        ::uuid::Uuid::from_slice(data).map(Self).map_err(|e| {
                            $crate::rmp_serialize::DeserializeError::InvalidValue(e.to_string())
                        })
                    }
                    other => Err($crate::rmp_serialize::DeserializeError::InvalidType {
                        expected: "ext(uuid)",
                        got: $crate::rmp_serialize::value_kind(&other),
                    }),
                }
            }
        }
    };
}

// Make the macro available as a public item of this crate for use by libparsec_types
pub(crate) use new_uuid_type;

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

/*
 * String-based types
 */

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

macro_rules! new_string_based_id_type {
    (pub $name:ident, $match_fn:expr) => {
        #[derive(
            Clone,
            serde_with::SerializeDisplay,
            serde_with::DeserializeFromStr,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
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

            impl std::convert::TryFrom<&str> for $name {
                type Error = [<Invalid $name>];

                fn try_from(s: &str) -> Result<Self, Self::Error> {
                    use ::unicode_normalization::UnicodeNormalization;
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

        impl $crate::rmp_serialize::Serialize for $name {
            fn serialize(
                &self,
                writer: &mut Vec<u8>,
            ) -> Result<(), $crate::rmp_serialize::SerializeError> {
                $crate::rmp_serialize::Serialize::serialize(self.0.as_str(), writer)
            }
        }

        impl $crate::rmp_serialize::Deserialize for $name {
            fn deserialize(
                value: $crate::rmp_serialize::ValueRef<'_>,
            ) -> Result<Self, $crate::rmp_serialize::DeserializeError> {
                let s: String = $crate::rmp_serialize::Deserialize::deserialize(value)?;
                s.as_str().try_into().map_err(|_| {
                    $crate::rmp_serialize::DeserializeError::InvalidValue(
                        concat!("invalid ", stringify!($name)).to_owned(),
                    )
                })
            }
        }
    };
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
 * UserProfile
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, serde::Serialize, serde::Deserialize)]
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

impl crate::rmp_serialize::Serialize for UserProfile {
    fn serialize(
        &self,
        writer: &mut Vec<u8>,
    ) -> Result<(), crate::rmp_serialize::SerializeError> {
        let s = match self {
            Self::Admin => "ADMIN",
            Self::Standard => "STANDARD",
            Self::Outsider => "OUTSIDER",
        };
        crate::rmp_serialize::Serialize::serialize(s, writer)
    }
}

impl crate::rmp_serialize::Deserialize for UserProfile {
    fn deserialize(
        value: crate::rmp_serialize::ValueRef<'_>,
    ) -> Result<Self, crate::rmp_serialize::DeserializeError> {
        let s: String = crate::rmp_serialize::Deserialize::deserialize(value)?;
        s.parse().map_err(|_| {
            crate::rmp_serialize::DeserializeError::InvalidValue(format!(
                "invalid UserProfile: {s}"
            ))
        })
    }
}
