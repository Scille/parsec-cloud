// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use serde::{Deserialize, Serialize};
use sha2::Digest;

use libparsec_crypto::prelude::*;
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types, BackendOrganizationAddr, DateTime, DeviceID, DeviceLabel, DeviceName,
    HumanHandle, Maybe, OrganizationID, TimeProvider, UserID, UserProfile, VlobID,
};

pub fn local_device_slug(
    organization_id: &OrganizationID,
    device_id: &DeviceID,
    root_verify_key: &VerifyKey,
) -> String {
    // Add a hash to avoid clash when the backend is reseted and we recreate
    // a device with same OrganizationID/DeviceID than a previous one
    let mut hasher = sha2::Sha256::new();
    hasher.update(root_verify_key.as_ref());
    let hashed_rvk = format!("{:x}", hasher.finalize());
    format!("{}#{}#{}", &hashed_rvk[..10], organization_id, device_id)
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalDeviceData", try_from = "LocalDeviceData")]
pub struct LocalDevice {
    pub organization_addr: BackendOrganizationAddr,
    pub device_id: DeviceID,
    pub device_label: DeviceLabel,
    pub human_handle: HumanHandle,
    pub signing_key: SigningKey,
    pub private_key: PrivateKey,
    /// Profile the user had at enrollment time, use `CertificateOps::get_current_self_profile`
    /// instead of relying on this.
    pub initial_profile: UserProfile,
    pub user_realm_id: VlobID,
    pub user_realm_key: SecretKey,
    pub local_symkey: SecretKey,
    pub time_provider: TimeProvider,
}

impl std::fmt::Debug for LocalDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LocalDevice")
            .field("organization_addr", &self.organization_addr)
            .field("device_id", &self.device_id)
            .finish_non_exhaustive()
    }
}

impl LocalDevice {
    pub fn generate_new_device(
        organization_addr: BackendOrganizationAddr,
        initial_profile: UserProfile,
        human_handle: HumanHandle,
        device_label: DeviceLabel,
        device_id: Option<DeviceID>,
        signing_key: Option<SigningKey>,
        private_key: Option<PrivateKey>,
    ) -> Self {
        Self {
            organization_addr,
            device_id: device_id.unwrap_or_default(),
            device_label,
            human_handle,
            signing_key: signing_key.unwrap_or_else(SigningKey::generate),
            private_key: private_key.unwrap_or_else(PrivateKey::generate),
            initial_profile,
            user_realm_id: VlobID::default(),
            user_realm_key: SecretKey::generate(),
            local_symkey: SecretKey::generate(),
            time_provider: TimeProvider::default(),
        }
    }

    pub fn dump(&self) -> Vec<u8> {
        rmp_serde::to_vec_named(&self).expect("LocalDevice serialization should not fail")
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
    /// The purpose of the slug is simply to tell whether `LocalDevice` and
    /// `AvailableDevice` objects corresponds to the same device.
    pub fn slug(&self) -> String {
        local_device_slug(
            self.organization_id(),
            &self.device_id,
            self.root_verify_key(),
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
        self.human_handle.as_ref()
    }

    pub fn short_user_display(&self) -> &str {
        self.human_handle.label()
    }

    pub fn device_display(&self) -> &str {
        self.device_label.as_ref()
    }

    /// This method centralizes the production of current time timestamps for a given device.
    /// This is meant to avoid relying on side effect and hence be able to do per-device
    /// time mock.
    pub fn now(&self) -> DateTime {
        self.time_provider.now()
    }
}

parsec_data!("schema/local_device/local_device.json5");

impl TryFrom<LocalDeviceData> for LocalDevice {
    type Error = &'static str;

    fn try_from(data: LocalDeviceData) -> Result<Self, Self::Error> {
        // Since the introduction of UserUpdateCertificate, user profile can change.
        // Hence this field only contains the initial profile the user had when it
        // was enrolled.
        let initial_profile = match data.profile {
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

        // `device_label` & `human_handle` fields has been introduced in Parsec v1.14,
        // hence they are basically always here.
        // If it's not the case, we are in an exotic case (very old device), so we don't
        // bother much an use the redacted system to obtain device label & human handle.
        // Of course redacted certificate has nothing to do with this, but it's just
        // convenient and "good enough" to go this way ;-)
        let device_label = match data.device_label {
            Maybe::Absent | Maybe::Present(None) => {
                DeviceLabel::new_redacted(data.device_id.device_name())
            }
            Maybe::Present(Some(device_label)) => device_label,
        };
        let human_handle = match data.human_handle {
            Maybe::Absent | Maybe::Present(None) => {
                HumanHandle::new_redacted(data.device_id.user_id())
            }
            Maybe::Present(Some(human_handle)) => human_handle,
        };

        Ok(Self {
            organization_addr: data.organization_addr,
            device_id: data.device_id,
            // Consider missing field as a `None` value
            device_label,
            human_handle,
            signing_key: data.signing_key,
            private_key: data.private_key,
            initial_profile,
            // For historical reason, we focus on the user manifest but in fact we
            // refer to the realm here, so rename `user_manifest_*` -> `user_realm_*`.
            user_realm_id: data.user_manifest_id,
            user_realm_key: data.user_manifest_key,
            local_symkey: data.local_symkey,
            time_provider: TimeProvider::default(),
        })
    }
}

impl From<LocalDevice> for LocalDeviceData {
    fn from(obj: LocalDevice) -> Self {
        // Handle legacy `is_admin` field
        let is_admin = obj.initial_profile == UserProfile::Admin;
        // In case the human handle is redacted (i.e. we are dealing with a device
        // created before Parsec v1.14) we cannot serialize it given then it would
        // appear as if the human handle is a regular one using the redacted domain,
        // which is not allowed !
        let human_handle = {
            if obj.human_handle.uses_redacted_domain() {
                Maybe::Absent
            } else {
                Maybe::Present(Some(obj.human_handle))
            }
        };
        Self {
            organization_addr: obj.organization_addr,
            device_id: obj.device_id,
            device_label: Maybe::Present(Some(obj.device_label)),
            human_handle,
            signing_key: obj.signing_key,
            private_key: obj.private_key,
            profile: Maybe::Present(obj.initial_profile),
            is_admin,
            // For historical reason, we focus on the user manifest but in fact we
            // refer to the realm here, so rename `user_manifest_*` -> `user_realm_*`.
            user_manifest_id: obj.user_realm_id,
            user_manifest_key: obj.user_realm_key,
            local_symkey: obj.local_symkey,
        }
    }
}
