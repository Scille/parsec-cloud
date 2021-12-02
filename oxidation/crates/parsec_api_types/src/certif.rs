// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::utils::ts_with_nanoseconds_as_double;
use crate::{DeviceID, EntryID, HumanHandle, RealmRole, UserID, UserProfile};
use parsec_api_crypto::{PublicKey, VerifyKey};

#[allow(unused_macros)]
macro_rules! impl_serialization {
    ($name:ident, $raw_name:ident) => {
        impl $name {
            pub fn dump(&self) -> Vec<u8> {
                rmp_serde::to_vec(&self).unwrap();
            }

            pub fn dump_and_sign(&self, author_signkey: &SigningKey) -> Vec<u8> {
                author_signkey.sign(&self.dump())
            }

            pub fn dump_sign_and_encrypt(
                &self,
                author_signkey: &SigningKey,
                key: &SecretKey,
            ) -> Vec<u8> {
                key.encrypt(&self.dump_and_sign(author_signkey))
            }

            pub fn dump_sign_and_encrypt_for(
                &self,
                author_signkey: &SigningKey,
                recipient_pubkey: &PublicKey,
            ) -> Vec<u8> {
                recipient_pubkey.encrypt_from_self(&self.dump_and_sign(author_signkey))
            }

            pub fn unsecure_load(signed: &[u8]) -> $name {}

            pub fn verify_and_load(
                signed: &[u8],
                author_verify_key: &VerifyKey,
                expected_author: Option<DeviceID>,
                expected_timestamp: Option<DateTime>,
            ) -> Result<$name, &'static str> {
                let serialized = author_verify_key.verify(signed)?;
                content = $name::try_from(serialized)?;
            }

            pub fn decrypt_verify_and_load(
                encrypted: bytes,
                key: SecretKey,
                author_verify_key: VerifyKey,
                expected_author: DeviceID,
                expected_timestamp: DateTime,
            ) -> $name {
            }

            pub fn decrypt_verify_and_load_for(
                encrypted: bytes,
                recipient_privkey: PrivateKey,
                author_verify_key: VerifyKey,
                expected_author: DeviceID,
                expected_timestamp: DateTime,
            ) -> $name {
            }
        }
    };
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum CertificateContent {
    #[serde(rename = "user_certificate")]
    User {
        // Author is None if signed by the root key
        author: Option<DeviceID>,
        #[serde(with = "ts_with_nanoseconds_as_double")]
        timestamp: DateTime<Utc>,

        user_id: UserID,
        human_handle: Option<HumanHandle>,
        public_key: PublicKey,
        profile: UserProfile,
    },

    #[serde(rename = "revoked_user_certificate")]
    RevokedUser {
        author: DeviceID,
        #[serde(with = "ts_with_nanoseconds_as_double")]
        timestamp: DateTime<Utc>,

        user_id: UserID,
    },

    #[serde(rename = "device_certificate")]
    Device {
        // Author is None if signed by the root key
        author: Option<DeviceID>,
        #[serde(with = "ts_with_nanoseconds_as_double")]
        timestamp: DateTime<Utc>,

        device_id: DeviceID,
        // Device label can be none in case of redacted certificate
        device_label: Option<String>,
        verify_key: VerifyKey,
    },

    #[serde(rename = "realm_role_certificate")]
    RealmRole {
        // Author is None if signed by the root key
        author: Option<DeviceID>,
        #[serde(with = "ts_with_nanoseconds_as_double")]
        timestamp: DateTime<Utc>,

        realm_id: EntryID,
        user_id: UserID,
        // Set to None if role removed
        role: Option<RealmRole>,
    },
}

/*
 * UserCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "CertificateContent", try_from = "CertificateContent")]
pub struct UserCertificate {
    // Author is None if signed by the root key
    pub author: Option<DeviceID>,
    pub timestamp: DateTime<Utc>,
    pub user_id: UserID,
    pub human_handle: Option<HumanHandle>,
    pub public_key: PublicKey,
    pub profile: UserProfile,
}

impl TryFrom<CertificateContent> for UserCertificate {
    type Error = &'static str;
    fn try_from(data: CertificateContent) -> Result<Self, Self::Error> {
        if let CertificateContent::User {
            author,
            timestamp,
            user_id,
            human_handle,
            public_key,
            profile,
        } = data
        {
            Ok(Self {
                author,
                timestamp,
                user_id,
                human_handle,
                public_key,
                profile,
            })
        } else {
            Err("Invalid manifest type")
        }
    }
}

impl From<UserCertificate> for CertificateContent {
    fn from(certif: UserCertificate) -> Self {
        CertificateContent::User {
            author: certif.author,
            timestamp: certif.timestamp,
            user_id: certif.user_id,
            human_handle: certif.human_handle,
            public_key: certif.public_key,
            profile: certif.profile,
        }
    }
}

/*
 * RevokedUserCertificate
 */

#[derive(Debug, PartialEq, Eq)]
pub struct RevokedUserCertificate {
    // Author is None if signed by the root key
    pub author: Option<DeviceID>,
    pub timestamp: DateTime<Utc>,
    pub user_id: UserID,
}

/*
 * DeviceCertificate
 */

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceCertificate {
    // Author is None if signed by the root key
    pub author: Option<DeviceID>,
    pub timestamp: DateTime<Utc>,
    pub device_id: DeviceID,
    // Device label can be none in case of redacted certificate
    pub device_label: Option<String>,
    pub verify_key: VerifyKey,
}

/*
 * RealmRoleCertificate
 */

#[derive(Debug, PartialEq, Eq)]
pub struct RealmRoleCertificate {
    // Author is None if signed by the root key
    pub author: Option<DeviceID>,
    pub timestamp: DateTime<Utc>,

    pub realm_id: EntryID,
    pub user_id: UserID,
    // Set to None if role removed
    pub role: Option<RealmRole>,
}

// @attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
// class UserCertificateContent(BaseAPISignedData):
//     class SCHEMA_CLS(BaseSignedDataSchema):
//         # Override author field to allow for None value if signed by the root key
//         author = DeviceIDField(required=True, allow_none=True)

//         type = fields.CheckedConstant("user_certificate", required=True)
//         user_id = UserIDField(required=True)
//         # Human handle can be none in case of redacted certificate
//         human_handle = HumanHandleField(allow_none=True, missing=None)
//         public_key = fields.PublicKey(required=True)
//         # `profile` replaces `is_admin` field (which is still required for backward
//         # compatibility), hence `None` is not allowed
//         is_admin = fields.Boolean(required=True)
//         profile = UserProfileField(allow_none=False)

//         @post_load
//         def make_obj(self, data: Dict[str, Any]) -> "UserCertificateContent":
//             data.pop("type")

//             # Handle legacy `is_admin` field
//             default_profile = UserProfile.ADMIN if data.pop("is_admin") else UserProfile.STANDARD
//             try:
//                 profile = data["profile"]
//             except KeyError:
//                 data["profile"] = default_profile
//             else:
//                 if default_profile == UserProfile.ADMIN and profile != UserProfile.ADMIN:
//                     raise ValidationError(
//                         "Fields `profile` and `is_admin` have incompatible values"
//                     )

//             return UserCertificateContent(**data)

//     # Override author attribute to allow for None value if signed by the root key
//     author: Optional[DeviceID]  # type: ignore[assignment]

//     user_id: UserID
//     human_handle: Optional[HumanHandle]
//     public_key: PublicKey
//     profile: UserProfile

//     # Only used during schema serialization
//     @property
//     def is_admin(self) -> bool:
//         return self.profile == UserProfile.ADMIN

//     @classmethod
//     def verify_and_load(
//         cls,
//         *args,
//         expected_user: Optional[UserID] = None,
//         expected_human_handle: Optional[HumanHandle] = None,
//         **kwargs,
//     ) -> "UserCertificateContent":
//         data = super().verify_and_load(*args, **kwargs)
//         if expected_user is not None and data.user_id != expected_user:
//             raise DataValidationError(
//                 f"Invalid user ID: expected `{expected_user}`, got `{data.user_id}`"
//             )
//         if expected_human_handle is not None and data.human_handle != expected_human_handle:
//             raise DataValidationError(
//                 f"Invalid human handle: expected `{expected_human_handle}`, got `{data.human_handle}`"
//             )
//         return data

// @attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
// class RevokedUserCertificateContent(BaseAPISignedData):
//     class SCHEMA_CLS(BaseSignedDataSchema):
//         type = fields.CheckedConstant("revoked_user_certificate", required=True)
//         user_id = UserIDField(required=True)

//         @post_load
//         def make_obj(self, data: Dict[str, Any]) -> "RevokedUserCertificateContent":
//             data.pop("type")
//             return RevokedUserCertificateContent(**data)

//     user_id: UserID

//     @classmethod
//     def verify_and_load(
//         cls, *args, expected_user: Optional[UserID] = None, **kwargs
//     ) -> "RevokedUserCertificateContent":
//         data = super().verify_and_load(*args, **kwargs)
//         if expected_user is not None and data.user_id != expected_user:
//             raise DataValidationError(
//                 f"Invalid user ID: expected `{expected_user}`, got `{data.user_id}`"
//             )
//         return data

// DeviceCertificateContentTypeVar = TypeVar(
//     "DeviceCertificateContentTypeVar", bound="DeviceCertificateContent"
// )

// @attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
// class DeviceCertificateContent(BaseAPISignedData):
//     class SCHEMA_CLS(BaseSignedDataSchema):
//         # Override author field to allow for None value if signed by the root key
//         author = DeviceIDField(required=True, allow_none=True)

//         type = fields.CheckedConstant("device_certificate", required=True)
//         device_id = DeviceIDField(required=True)
//         # Device label can be none in case of redacted certificate
//         device_label = fields.String(allow_none=True, missing=None)
//         verify_key = fields.VerifyKey(required=True)

//         @post_load
//         def make_obj(self, data: Dict[str, Any]) -> "DeviceCertificateContent":
//             data.pop("type")
//             return DeviceCertificateContent(**data)

//     # Override author attribute to allow for None value if signed by the root key
//     author: Optional[DeviceID]  # type: ignore[assignment]

//     device_id: DeviceID
//     device_label: Optional[str]
//     verify_key: VerifyKey

//     @classmethod
//     def verify_and_load(
//         cls: Type[DeviceCertificateContentTypeVar],
//         *args: Any,
//         expected_device: Optional[DeviceID] = None,
//         **kwargs: Any,
//     ) -> "DeviceCertificateContent":
//         data = super().verify_and_load(*args, **kwargs)
//         if expected_device is not None and data.device_id != expected_device:
//             raise DataValidationError(
//                 f"Invalid device ID: expected `{expected_device}`, got `{data.device_id}`"
//             )
//         return data

// @attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
// class RealmRoleCertificateContent(BaseAPISignedData):
//     class SCHEMA_CLS(BaseSignedDataSchema):
//         type = fields.CheckedConstant("realm_role_certificate", required=True)
//         realm_id = fields.UUID(required=True)
//         user_id = UserIDField(required=True)
//         role = RealmRoleField(required=True, allow_none=True)

//         @post_load
//         def make_obj(self, data: Dict[str, Any]) -> "RealmRoleCertificateContent":
//             data.pop("type")
//             return RealmRoleCertificateContent(**data)

//     realm_id: UUID
//     user_id: UserID
//     role: Optional[RealmRole]  # Set to None if role removed

//     @classmethod
//     def build_realm_root_certif(cls, author, timestamp, realm_id):
//         return cls(
//             author=author,
//             timestamp=timestamp,
//             realm_id=realm_id,
//             user_id=author.user_id,
//             role=RealmRole.OWNER,
//         )

//     @classmethod
//     def verify_and_load(
//         cls,
//         *args,
//         expected_realm: Optional[UUID] = None,
//         expected_user: Optional[UserID] = None,
//         expected_role: Optional[RealmRole] = None,
//         **kwargs,
//     ) -> "RealmRoleCertificateContent":
//         data = super().verify_and_load(*args, **kwargs)
//         if expected_user is not None and data.user_id != expected_user:
//             raise DataValidationError(
//                 f"Invalid user ID: expected `{expected_user}`, got `{data.user_id}`"
//             )
//         if expected_realm is not None and data.realm_id != expected_realm:
//             raise DataValidationError(
//                 f"Invalid realm ID: expected `{expected_realm}`, got `{data.realm_id}`"
//             )
//         if expected_role is not None and data.role != expected_role:
//             raise DataValidationError(
//                 f"Invalid role: expected `{expected_role}`, got `{data.role}`"
//             )
//         return data
