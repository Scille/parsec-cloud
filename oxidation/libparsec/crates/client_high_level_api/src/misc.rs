// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub use libparsec_types::{
    AvailableDevice, DeviceInfo, DeviceLabel, OrganizationConfig, OrganizationStats, UserID,
    UserInfo,
};

use crate::{ClientHandle, DeviceAccessParams};

pub enum ClientRevokeUserError {}
#[allow(unused)]
pub async fn client_revoke_user(
    client: ClientHandle,
    user_id: UserID,
) -> Result<(), ClientRevokeUserError> {
    unimplemented!();
}

// Instead of human_find/get_user_info/get_user_devices_info we introduce
// simpler methods that would leverage the new server api allowing to download
// all device/user certificates at once and ensure the list is kept up to date
// This has the added benefit of allowing those function to be sync and to
// return something even if the server is not reachable !

#[allow(unused)]
pub fn client_get_users(client: ClientHandle) -> Vec<UserInfo> {
    unimplemented!();
}

#[allow(unused)]
pub fn client_get_self_devices(client: ClientHandle) -> Vec<DeviceInfo> {
    unimplemented!();
}

pub enum ClientGetOrganizationStatsError {}
#[allow(unused)]
pub async fn client_get_organization_stats(
    client: ClientHandle,
) -> Result<OrganizationStats, ClientGetOrganizationStatsError> {
    unimplemented!();
}

#[allow(unused)]
pub fn client_get_organization_config(client: ClientHandle) -> OrganizationConfig {
    unimplemented!();
}

pub enum ServerConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
}
#[allow(unused)]
pub fn client_get_server_connection_status(client: ClientHandle) -> ServerConnectionStatus {
    unimplemented!();
}

// No function to get monitor state, we rely on events instead

pub type RecoveryDeviceFileName = String;
pub type RecoveryDeviceFileContent = Vec<u8>;
pub type RecoveryDevicePassphrase = String;
pub enum ClientExportRecoveryDeviceError {}
#[allow(unused)]
pub async fn client_export_recovery_device(
    client: ClientHandle,
) -> Result<
    (
        RecoveryDeviceFileName,
        RecoveryDeviceFileContent,
        RecoveryDevicePassphrase,
    ),
    ClientExportRecoveryDeviceError,
> {
    unimplemented!();
}

pub enum ImportRecoveryDeviceError {}
#[allow(unused)]
pub async fn import_recovery_device(
    ciphered: &[u8],
    passphrase: &RecoveryDevicePassphrase,
    device_label: &DeviceLabel,
    save_device_params: DeviceAccessParams,
    login_after_import: bool,
) -> Result<(AvailableDevice, Option<ClientHandle>), ImportRecoveryDeviceError> {
    unimplemented!();
}
