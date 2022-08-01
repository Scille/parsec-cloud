// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::*;
use sha2::Digest;

use libparsec_crypto::*;
use libparsec_types::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalDeviceData", try_from = "LocalDeviceData")]
pub struct LocalDevice {
    pub organization_addr: BackendOrganizationAddr,
    pub device_id: DeviceID,
    pub device_label: Option<DeviceLabel>,
    pub human_handle: Option<HumanHandle>,
    pub signing_key: SigningKey,
    pub private_key: PrivateKey,
    pub profile: UserProfile,
    pub user_manifest_id: EntryID,
    pub user_manifest_key: SecretKey,
    pub local_symkey: SecretKey,
}

impl LocalDevice {
    pub fn dump(&self) -> Vec<u8> {
        rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!())
    }

    pub fn load(serialized: &[u8]) -> Result<Self, &'static str> {
        rmp_serde::from_slice(serialized).map_err(|_| "Invalid serialization")
    }

    pub fn decrypt_and_load(encrypted: &[u8], key: &SecretKey) -> Result<Self, &'static str> {
        let serialized = key.decrypt(encrypted).map_err(|_| "Invalid encryption")?;
        Self::load(&serialized)
    }

    pub fn dump_and_encrypt(&self, key: &SecretKey) -> Vec<u8> {
        key.encrypt(&self.dump())
    }

    /// The slug is unique identifier for a particular device.
    ///
    /// It is composed of a small part of the RVK hash, the organization ID
    /// and the device ID, although it shouldn't be assumed that this information
    /// can be recovered from the slug as this might change in the future.
    ///
    /// The purpose of the slog is simply to tell whether `LocalDevice` and
    /// `AvailableDevice` objects corresponds to the same device.
    pub fn slug(&self) -> String {
        // Add a hash to avoid clash when the backend is reseted and we recreate
        // a device with same OrganizationID/DeviceID than a previous one
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.root_verify_key().as_ref());
        let hashed_rvk = format!("{:x}", hasher.finalize());
        format!(
            "{}#{}#{}",
            &hashed_rvk[..10],
            self.organization_id(),
            self.device_id
        )
    }

    /// Slug is long and not readable enough (given DeviceID is now made of uuids).
    /// Hence it's often simpler to rely on it hash instead (e.g. select the
    /// device to use in the CLI by providing the beginning of the hash)
    pub fn slughash(&self) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.slug().as_bytes()); // Slug is encoded as utf8
        format!("{:x}", hasher.finalize())
    }

    pub fn load_slug(slug: &str) -> Result<(OrganizationID, DeviceID), &'static str> {
        let msg = "Invalid slug";

        let mut parts = slug.split('#');
        parts.next(); // Drop hashed root verify key
        let organization_id: OrganizationID = parts.next().ok_or(msg)?.parse().map_err(|_| msg)?;
        let device_id: DeviceID = parts.next().ok_or(msg)?.parse().map_err(|_| msg)?;
        if parts.next().is_none() {
            Ok((organization_id, device_id))
        } else {
            Err(msg)
        }
    }

    pub fn root_verify_key(&self) -> &VerifyKey {
        self.organization_addr.root_verify_key()
    }

    pub fn organization_id(&self) -> &OrganizationID {
        self.organization_addr.organization_id()
    }

    pub fn device_name(&self) -> &DeviceName {
        &self.device_id.device_name
    }

    pub fn user_id(&self) -> &UserID {
        &self.device_id.user_id
    }

    pub fn verify_key(&self) -> VerifyKey {
        self.signing_key.verify_key()
    }

    pub fn public_key(&self) -> PublicKey {
        self.private_key.public_key()
    }

    pub fn user_display(&self) -> String {
        match self.human_handle {
            Some(ref human_handle) => human_handle.to_string(),
            None => self.user_id().to_string(),
        }
    }

    pub fn short_user_display(&self) -> String {
        match self.human_handle {
            Some(ref human_handle) => human_handle.label.to_string(),
            None => self.user_id().to_string(),
        }
    }

    pub fn device_display(&self) -> String {
        match self.device_label {
            Some(ref device_label) => device_label.to_string(),
            None => self.device_id.device_name.to_string(),
        }
    }

    /// This method centralizes the production of parsec timestamps for a given device.
    /// At the moment it is simply an alias to [DateTime::now] but it has two main benefits:
    /// 1. Allowing for easier testing by patching this method in device-specific way
    /// 2. Allowing for other implementation in the future allowing to track, check and
    ///    possibly alter the production of timestamps.
    pub fn timestamp(&self) -> DateTime {
        DateTime::now()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LocalDeviceData {
    pub organization_addr: BackendOrganizationAddr,
    pub device_id: DeviceID,
    // Added in Parsec v1.14
    // `device_label` and `human_handle` are new fields (so legacy data may not contain
    // them) that are optional. Hence missing field is handled similarly that `None`.
    #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
    pub device_label: Option<Option<DeviceLabel>>,
    // Added in Parsec v1.13
    #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
    pub human_handle: Option<Option<HumanHandle>>,
    pub signing_key: SigningKey,
    pub private_key: PrivateKey,
    // `profile` replaces `is_admin` field (which is still required for
    // backward compatibility), hence `None` is not a valid value (only missing
    // allowed
    pub is_admin: bool,
    // Added in Parsec v1.14
    #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
    pub profile: Option<UserProfile>,
    pub user_manifest_id: EntryID,
    pub user_manifest_key: SecretKey,
    pub local_symkey: SecretKey,
}

impl TryFrom<LocalDeviceData> for LocalDevice {
    type Error = &'static str;

    fn try_from(data: LocalDeviceData) -> Result<Self, Self::Error> {
        let profile = match data.profile {
            Some(profile) => {
                // `profile` field is defined, however `is_admin` is also present for
                // backward compatibility and we must ensure they are both compatible.
                let expected_is_admin = profile == UserProfile::Admin;
                if data.is_admin != expected_is_admin {
                    return Err("Fields `profile` and `is_admin` have incompatible values");
                }
                profile
            }
            None => {
                // `profile` field not present, this is legacy data
                if data.is_admin {
                    UserProfile::Admin
                } else {
                    UserProfile::Standard
                }
            }
        };

        Ok(Self {
            organization_addr: data.organization_addr,
            device_id: data.device_id,
            // Consider missing field as a `None` value
            device_label: data.device_label.unwrap_or(None),
            human_handle: data.human_handle.unwrap_or(None),
            signing_key: data.signing_key,
            private_key: data.private_key,
            profile,
            user_manifest_id: data.user_manifest_id,
            user_manifest_key: data.user_manifest_key,
            local_symkey: data.local_symkey,
        })
    }
}

impl From<LocalDevice> for LocalDeviceData {
    fn from(obj: LocalDevice) -> Self {
        // Handle legacy `is_admin` field
        let is_admin = obj.profile == UserProfile::Admin;
        Self {
            organization_addr: obj.organization_addr,
            device_id: obj.device_id,
            device_label: Some(obj.device_label),
            human_handle: Some(obj.human_handle),
            signing_key: obj.signing_key,
            private_key: obj.private_key,
            profile: Some(obj.profile),
            is_admin,
            user_manifest_id: obj.user_manifest_id,
            user_manifest_key: obj.user_manifest_key,
            local_symkey: obj.local_symkey,
        }
    }
}
