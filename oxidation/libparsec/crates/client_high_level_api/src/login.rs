// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_platform_device_loader::load_device_with_password;
use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};
use thiserror::Error;

use libparsec_core::LoggedCore;
use libparsec_platform_async::Mutex;
use libparsec_types::DeviceID;

pub use libparsec_client_types::{AvailableDevice, DeviceFileType};
pub use libparsec_crypto::SequesterVerifyKeyDer;
pub use libparsec_protocol::authenticated_cmds::v3::invite_list::InviteListItem;
pub use libparsec_types::BackendAddr;
pub use libparsec_types::{BackendOrganizationBootstrapAddr, DeviceLabel, HumanHandle};

use crate::ClientEvent;

static CORE: OnceCell<Mutex<Vec<Option<LoggedCore>>>> = OnceCell::new();

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
    match load_device_params {
        DeviceAccessParams::Password { path, password } => {
            let path = path
                .into_os_string()
                .into_string()
                .map_err(|_| ClientLoginError::DeviceInvalidFormat)?;
            let mut core = CORE.get_or_init(Default::default).lock().await;

            let index = core.len() as i32;

            let device = load_device_with_password(&path, &password)
                .map_err(|_| ClientLoginError::DeviceInvalidFormat)?;

            // Store in memory
            core.push(Some(LoggedCore { device }));

            Ok(index)
        }
        DeviceAccessParams::Smartcard { .. } => Err(ClientLoginError::AccessMethodNotAvailable),
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ClientGetterError {
    #[error("The device is disconnected")]
    Disconnected,
    #[error("The handle provided is invalid: {handle:?}")]
    InvalidHandle { handle: ClientHandle },
}

pub async fn client_get_device_id(handle: ClientHandle) -> Result<DeviceID, ClientGetterError> {
    if let Some(logged_core) = CORE
        .get_or_init(Default::default)
        .lock()
        .await
        .get(handle as usize)
    {
        return if let Some(logged_core) = logged_core {
            Ok(logged_core.device_id().clone())
        } else {
            Err(ClientGetterError::Disconnected)
        };
    }

    Err(ClientGetterError::InvalidHandle { handle })
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ClientLogoutError {
    #[error("The handle provided is invalid: {handle:?}")]
    InvalidHandle { handle: ClientHandle },
}

#[allow(unused)]
pub async fn client_logout(handle: ClientHandle) -> Result<(), ClientLogoutError> {
    if let Some(logged_core) = CORE
        .get_or_init(Default::default)
        .lock()
        .await
        .get_mut(handle as usize)
    {
        *logged_core = None;
        return Ok(());
    }

    Err(ClientLogoutError::InvalidHandle { handle })
}

//
// Local device stuff
//

#[allow(unused)]
pub async fn client_list_available_devices(config_dir: &Path) -> Vec<AvailableDevice> {
    libparsec_platform_device_loader::list_available_devices(config_dir).await
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
