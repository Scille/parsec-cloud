// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use anyhow::Context;
use serde::{Deserialize, Serialize};

use libparsec_crypto::prelude::*;
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types, DateTime, DeviceID, DeviceLabel, HumanHandle, OrganizationID,
    ParsecAddr, ParsecOrganizationAddr, TimeProvider, UserID, UserProfile, VlobID,
};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalDeviceData", try_from = "LocalDeviceData")]
pub struct LocalDevice {
    pub organization_addr: ParsecOrganizationAddr,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub device_label: DeviceLabel,
    /// user human readable info
    pub human_handle: HumanHandle,
    /// used by the device to sign certificates and vlobs
    pub signing_key: SigningKey,
    /// used to receive messages
    pub private_key: PrivateKey,
    /// Profile the user had at enrollment time, use `CertificateOps::get_current_self_profile`
    /// instead of relying on this.
    pub initial_profile: UserProfile,

    pub user_realm_id: VlobID,
    pub user_realm_key: SecretKey,
    /// used to locally encrypt data
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
    /// TODO: Use it only for tests see #8790
    #[allow(clippy::too_many_arguments)]
    pub fn generate_new_device(
        organization_addr: ParsecOrganizationAddr,
        initial_profile: UserProfile,
        human_handle: HumanHandle,
        device_label: DeviceLabel,
        user_id: Option<UserID>,
        device_id: Option<DeviceID>,
        signing_key: Option<SigningKey>,
        private_key: Option<PrivateKey>,
        time_provider: Option<TimeProvider>,
        user_realm_id: Option<VlobID>,
        user_realm_key: Option<SecretKey>,
    ) -> Self {
        Self {
            organization_addr,
            user_id: user_id.unwrap_or_default(),
            device_id: device_id.unwrap_or_default(),
            device_label,
            human_handle,
            signing_key: signing_key.unwrap_or_else(SigningKey::generate),
            private_key: private_key.unwrap_or_else(PrivateKey::generate),
            initial_profile,
            user_realm_id: user_realm_id.unwrap_or_default(),
            user_realm_key: user_realm_key.unwrap_or_else(SecretKey::generate),
            local_symkey: SecretKey::generate(),
            time_provider: time_provider.unwrap_or_default(),
        }
    }

    /// generates a new device for a given user
    pub fn from_existing_device_for_user(
        device: &LocalDevice,
        device_label: DeviceLabel,
    ) -> LocalDevice {
        LocalDevice {
            // generate device specific fields
            device_label,
            device_id: DeviceID::default(),
            signing_key: SigningKey::generate(),
            local_symkey: SecretKey::generate(),

            // the other fields are the same as the existing device
            organization_addr: device.organization_addr.clone(),
            user_id: device.user_id,
            human_handle: device.human_handle.clone(),
            private_key: device.private_key.clone(),
            initial_profile: device.initial_profile,
            user_realm_id: device.user_realm_id,
            user_realm_key: device.user_realm_key.clone(),

            // this is not serialized
            // it's derived from the previous device to
            // assure that mocked devices work properly
            time_provider: device.time_provider.new_child(),
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

    pub fn root_verify_key(&self) -> &VerifyKey {
        self.organization_addr.root_verify_key()
    }

    pub fn organization_id(&self) -> &OrganizationID {
        self.organization_addr.organization_id()
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
    type Error = anyhow::Error;

    fn try_from(data: LocalDeviceData) -> Result<Self, Self::Error> {
        let organization_addr = {
            let server_addr =
                ParsecAddr::from_http_url(&data.server_url).context("Invalid server URL")?;
            ParsecOrganizationAddr::new(server_addr, data.organization_id, data.root_verify_key)
        };
        Ok(Self {
            organization_addr,
            user_id: data.user_id,
            device_id: data.device_id,
            device_label: data.device_label,
            human_handle: data.human_handle,
            signing_key: data.signing_key,
            private_key: data.private_key,
            initial_profile: data.initial_profile,
            user_realm_id: data.user_realm_id,
            user_realm_key: data.user_realm_key,
            local_symkey: data.local_symkey,
            time_provider: TimeProvider::default(),
        })
    }
}

impl From<LocalDevice> for LocalDeviceData {
    fn from(obj: LocalDevice) -> Self {
        let server_url = {
            let server_addr = ParsecAddr::new(
                obj.organization_addr.hostname().to_string(),
                Some(obj.organization_addr.port()),
                obj.organization_addr.use_ssl(),
            );
            server_addr.to_http_url(None).to_string()
        };
        Self {
            ty: Default::default(),
            organization_id: obj.organization_addr.organization_id().clone(),
            root_verify_key: obj.organization_addr.root_verify_key().clone(),
            server_url,
            user_id: obj.user_id,
            device_id: obj.device_id,
            device_label: obj.device_label,
            human_handle: obj.human_handle,
            signing_key: obj.signing_key,
            private_key: obj.private_key,
            initial_profile: obj.initial_profile,
            user_realm_id: obj.user_realm_id,
            user_realm_key: obj.user_realm_key,
            local_symkey: obj.local_symkey,
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/local_device.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
