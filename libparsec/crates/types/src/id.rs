// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! UserID and DeviceID are kept in this crate (instead of `libparsec_types_lite`)
//! because their test-fixture support requires access to `crate::fixtures`.

use std::str::FromStr;

// Re-use the `new_uuid_type!` macro from `libparsec_types_lite`
use libparsec_types_lite::new_uuid_type;

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
