// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::{Path, PathBuf};

pub use libparsec_client_types::AvailableDevice;
pub use libparsec_crypto::SequesterVerifyKeyDer;
pub use libparsec_protocol::authenticated_cmds::v3::invite_list::InviteListItem;
pub use libparsec_types::BackendAddr;
pub use libparsec_types::{BackendOrganizationBootstrapAddr, DeviceLabel, HumanHandle};

use crate::ClientEvent;

//
// Client login
//

// TODO: use u32 when binding support it
pub type ClientHandle = i32;

pub enum DeviceAccessParams {
    Password { path: PathBuf, password: String },
    Smartcard { path: PathBuf },
    // Future API that will be use for parsec-web
    // ServerSide{
    //     url: BackendOrganizationAddr,
    //     email: String,
    //     password: String,
    // }
}

#[derive(Debug, Clone, Copy)]
pub enum WorkspaceStorageCacheSize {
    Default,
    // TODO: support arbitrary int size in bindings
    Custom { size: i32 },
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    // On web, `config_dir`&`data_base_dir` are converted into String and
    // used as database name when using IndexedDB API
    pub config_dir: PathBuf,
    pub data_base_dir: PathBuf,
    pub mountpoint_base_dir: PathBuf, // Ignored on web
    pub preferred_org_creation_backend_addr: BackendAddr,
    pub workspace_storage_cache_size: WorkspaceStorageCacheSize,
}

pub enum ClientLoginError {
    DeviceAlreadyLoggedIn,
    AccessMethodNotAvailable, // e.g. smartcard on web
    DecryptionFailed,         // e.g. bad password
    DeviceInvalidFormat,      // e.g. try to load a dummy file
}
#[allow(unused)]
pub async fn client_login(
    load_device_params: DeviceAccessParams,
    config: ClientConfig,
    // Access to the event bus is done through this callback.
    // Ad-hoc code should be added to the binding system to handle this (hence
    // why this is passed as a parameter instead of as part of `ClientConfig`:
    // we can have a simple `if func_name == "client_login"` that does a special
    // cooking of it last param.
    #[cfg(not(target_arch = "wasm32"))] on_event_callback: Box<dyn FnMut(ClientEvent) + Send>,
    // On web we run on the JS runtime which is monothreaded, hence everything is !Send
    #[cfg(target_arch = "wasm32")] on_event_callback: Box<dyn FnMut(ClientEvent)>,
) -> Result<ClientHandle, ClientLoginError> {
    unimplemented!();
}

pub enum ClientLogoutError {
    InvalidHandle,
}

#[allow(unused)]
pub async fn client_logout(client: ClientHandle) -> Result<(), ClientLogoutError> {
    unimplemented!()
}

//
// Local device stuff
//

#[allow(unused)]
pub async fn list_available_devices(config_dir: &Path) -> Vec<AvailableDevice> {
    #[cfg(target_arch = "wasm32")]
    {
        vec![]
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        unimplemented!();
    }
}

pub enum ChangeDeviceProtectionError {
    AccessMethodNotAvailable, // e.g. smartcard on web
    DecryptionFailed,         // e.g. bad password
    DeviceInvalidFormat,      // e.g. try to load a dummy file
    SaveFailed,
}
#[allow(unused)]
pub async fn change_device_protection(
    load_device_params: DeviceAccessParams,
    save_device_params: DeviceAccessParams,
) -> Result<(), ChangeDeviceProtectionError> {
    unimplemented!();
}
