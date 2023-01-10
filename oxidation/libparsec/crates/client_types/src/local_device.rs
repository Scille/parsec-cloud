// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::*;
use sha2::Digest;
use std::path::{Path, PathBuf};

use libparsec_crypto::prelude::*;
use libparsec_serialization_format::parsec_data;
use libparsec_types::*;

use crate::{DeviceFile, DeviceFileType, LocalDeviceError};

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
    pub time_provider: TimeProvider,
}

impl LocalDevice {
    pub fn get_default_key_file(config_dir: &Path, device: &LocalDevice) -> PathBuf {
        let mut default_key_path = Self::get_devices_dir(config_dir);
        default_key_path.push(format!(
            "{}.{}",
            device.slughash(),
            crate::DEVICE_FILE_SUFFIX
        ));
        default_key_path
    }

    pub fn get_devices_dir(config_dir: &Path) -> PathBuf {
        let mut device_path = config_dir.to_path_buf();
        device_path.push("devices");

        device_path
    }

    pub fn load_device_with_password(
        key_file: &Path,
        password: &str,
    ) -> Result<Self, LocalDeviceError> {
        let ciphertext = std::fs::read(key_file)
            .map_err(|_| LocalDeviceError::Access(key_file.to_path_buf()))?;
        let device_file: DeviceFile = match rmp_serde::from_slice(&ciphertext) {
            Ok(result) => result,
            Err(_) => todo!("Handle legacy device"),
        };

        match device_file {
            DeviceFile::Password(p) => {
                let key = SecretKey::from_password(password, &p.salt);
                let data = key
                    .decrypt(&p.ciphertext)
                    .map_err(LocalDeviceError::CryptoError)?;

                LocalDevice::load(&data)
                    .map_err(|_| LocalDeviceError::Validation(DeviceFileType::Password))
            }
            _ => unreachable!(
                "Tried to load recovery/smartcard device with `load_device_with_password`"
            ),
        }
    }

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
        self.device_id.device_name()
    }

    pub fn user_id(&self) -> &UserID {
        self.device_id.user_id()
    }

    pub fn verify_key(&self) -> VerifyKey {
        self.signing_key.verify_key()
    }

    pub fn public_key(&self) -> PublicKey {
        self.private_key.public_key()
    }

    pub fn user_display(&self) -> &str {
        match &self.human_handle {
            Some(human_handle) => human_handle.as_ref(),
            None => self.user_id().as_ref(),
        }
    }

    pub fn short_user_display(&self) -> &str {
        match &self.human_handle {
            Some(human_handle) => human_handle.label(),
            None => self.user_id().as_ref(),
        }
    }

    pub fn device_display(&self) -> &str {
        match &self.device_label {
            Some(device_label) => device_label.as_ref(),
            None => self.device_id.device_name().as_ref(),
        }
    }

    /// This method centralizes the production of current time timestamps for a given device.
    /// This is meant to avoid relying on side effect and hence be able to do per-device
    /// time mock.
    pub fn now(&self) -> DateTime {
        self.time_provider.now()
    }
}

parsec_data!("schema/local_device.json");

impl TryFrom<LocalDeviceData> for LocalDevice {
    type Error = &'static str;

    fn try_from(data: LocalDeviceData) -> Result<Self, Self::Error> {
        let profile = match data.profile {
            Maybe::Present(profile) => {
                // `profile` field is defined, however `is_admin` is also present for
                // backward compatibility and we must ensure they are both compatible.
                let expected_is_admin = profile == UserProfile::Admin;
                if data.is_admin != expected_is_admin {
                    return Err("Fields `profile` and `is_admin` have incompatible values");
                }
                profile
            }
            Maybe::Absent => {
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
            time_provider: TimeProvider::default(),
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
            device_label: Maybe::Present(obj.device_label),
            human_handle: Maybe::Present(obj.human_handle),
            signing_key: obj.signing_key,
            private_key: obj.private_key,
            profile: Maybe::Present(obj.profile),
            is_admin,
            user_manifest_id: obj.user_manifest_id,
            user_manifest_key: obj.user_manifest_key,
            local_symkey: obj.local_symkey,
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct UserInfo {
    pub user_id: UserID,
    pub human_handle: Option<HumanHandle>,
    pub profile: UserProfile,
    pub created_on: DateTime,
    pub revoked_on: Option<DateTime>,
}

impl UserInfo {
    pub fn user_display(&self) -> &str {
        self.human_handle
            .as_ref()
            .map(|x| x.as_ref())
            .unwrap_or_else(|| self.user_id.as_ref())
    }

    pub fn short_user_display(&self) -> &str {
        self.human_handle
            .as_ref()
            .map(|x| x.label())
            .unwrap_or_else(|| self.user_id.as_ref())
    }

    /// Note that we might consider a user revoked even though our current time is still
    /// below the revocation timestamp. This is because there is no clear causality between
    /// our time and the production of the revocation timestamp (as it might have been produced
    /// by another device). So we simply consider a user revoked if a revocation timestamp has
    /// been issued.
    pub fn is_revoked(&self) -> bool {
        self.revoked_on.is_some()
    }
}

impl std::fmt::Debug for UserInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<UserInfo {}>", self.user_display())
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct DeviceInfo {
    pub device_id: DeviceID,
    pub device_label: Option<DeviceLabel>,
    pub created_on: DateTime,
}

impl DeviceInfo {
    pub fn device_name(&self) -> &DeviceName {
        self.device_id.device_name()
    }

    pub fn device_display(&self) -> &str {
        self.device_label
            .as_ref()
            .map(|x| x.as_ref())
            .unwrap_or_else(|| self.device_id.device_name().as_ref())
    }
}

impl std::fmt::Debug for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeviceInfo({})", self.device_display())
    }
}
