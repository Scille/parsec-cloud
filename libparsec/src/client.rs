// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use once_cell::sync::OnceCell;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use libparsec_client::{Client, ClientConfig as InternalClientConfig, EventBus};
use libparsec_client_connection::ProxyConfig;
use libparsec_platform_device_loader::load_device_with_password;
use libparsec_types::*;

enum RegisteredClient {
    Running(Arc<Client>),
    Terminated,
}

static CLIENTS: OnceCell<Mutex<Vec<RegisteredClient>>> = OnceCell::new();

fn get_client(handle: ClientHandle) -> Option<Arc<Client>> {
    let guard = CLIENTS
        .get_or_init(Default::default)
        .lock()
        .expect("Mutex is poisoned");
    guard
        .get(handle)
        .map(|client| match client {
            RegisteredClient::Running(client) => Some(client.to_owned()),
            RegisteredClient::Terminated => None,
        })
        .flatten()
}

fn add_client(client: Client) -> usize {
    let mut guard = CLIENTS
        .get_or_init(Default::default)
        .lock()
        .expect("Mutex is poisoned");
    let handle = guard.len();
    guard.push(RegisteredClient::Running(Arc::new(client)));
    handle
}

pub type ClientHandle = usize;

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

pub struct ClientConfig {
    config_dir: PathBuf,
    data_base_dir: PathBuf,
}

pub enum ClientEvent {
    // Dummy event for tests only
    Ping(libparsec_client::EventPing),
    // Events related to server connection
    Offline(libparsec_client::EventOffline),
    Online(libparsec_client::EventOnline),
    MissedServerEvents(libparsec_client::EventMissedServerEvents),
    TooMuchDriftWithServerClock(libparsec_client::EventTooMuchDriftWithServerClock),
    ExpiredOrganization(libparsec_client::EventExpiredOrganization),
    RevokedUser(libparsec_client::EventRevokedUser),
    IncompatibleServer(libparsec_client::EventIncompatibleServer),
    // Events related to ops
    InvalidMessage(libparsec_client::EventInvalidMessage),
    // Events related to monitors
    CertificatesMonitorCrashed(libparsec_client::EventCertificatesMonitorCrashed),
    InvalidCertificate(libparsec_client::EventInvalidCertificate),
}

/*
 * client_start
 */

#[derive(Debug, thiserror::Error)]
pub enum ClientStartError {
    #[error("Device already started")]
    DeviceAlreadyLoggedIn,
    #[error("Access method not available")]
    AccessMethodNotAvailable, // e.g. smartcard on web
    #[error("Decryption failed")]
    DecryptionFailed, // e.g. bad password
    #[error("Invalid format for local device")]
    DeviceInvalidFormat, // e.g. try to load a dummy file
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn client_start(
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
) -> Result<ClientHandle, ClientStartError> {
    let config = Arc::new(InternalClientConfig {
        config_dir: config.config_dir,
        data_base_dir: config.data_base_dir,
        proxy: ProxyConfig::default(),
    });
    let event_bus = EventBus::default();
    match load_device_params {
        DeviceAccessParams::Password { path, password } => {
            let path = path
                .into_os_string()
                .into_string()
                .map_err(|_| ClientStartError::DeviceInvalidFormat)?;

            let device = load_device_with_password(&path, &password)
                .map_err(|_| ClientStartError::DeviceInvalidFormat)?;

            let client = Client::start(config, event_bus, Arc::new(device)).await?;
            let handle = add_client(client);

            Ok(handle)
        }
        DeviceAccessParams::Smartcard { .. } => Err(ClientStartError::AccessMethodNotAvailable),
    }
}

/*
 * client_stop
 */

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum ClientStopError {
    #[error("The handle provided is invalid")]
    InvalidHandle,
}

pub async fn client_stop(client: ClientHandle) -> Result<(), ClientStopError> {
    match get_client(client) {
        None => Err(ClientStopError::InvalidHandle),
        Some(client) => {
            client.stop().await;
            Ok(())
        }
    }
}
