// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, num::NonZeroU8, path::Path, sync::Arc};

use libparsec_client::ServerConfig;
pub use libparsec_client::{
    ClientAcceptTosError, ClientCreateWorkspaceError, ClientDeleteShamirRecoveryError,
    ClientForgetAllCertificatesError, ClientGetCurrentSelfProfileError,
    ClientGetOrganizationBootstrapDateError, ClientGetSelfShamirRecoveryError, ClientGetTosError,
    ClientGetUserDeviceError, ClientGetUserInfoError, ClientListFrozenUsersError,
    ClientListShamirRecoveriesForOthersError, ClientListUserDevicesError, ClientListUsersError,
    ClientListWorkspaceUsersError, ClientOrganizationInfoError, ClientRenameWorkspaceError,
    ClientRevokeUserError, ClientSetupShamirRecoveryError, ClientShareWorkspaceError,
    ClientUserUpdateProfileError, DeviceInfo, InvalidityReason, OrganizationInfo,
    OtherShamirRecoveryInfo, PKIInfoItem, PkiEnrollmentAcceptError, PkiEnrollmentInfoError,
    PkiEnrollmentListError, PkiEnrollmentListItem, PkiEnrollmentRejectError,
    SelfShamirRecoveryInfo, Tos, UserInfo, WorkspaceInfo, WorkspaceUserAccessInfo,
};
pub use libparsec_client_connection::ConnectionError;
use libparsec_platform_async::event::{Event, EventListener};
use libparsec_platform_device_loader::RemoteOperationServer;
use libparsec_types::prelude::*;
pub use libparsec_types::RealmRole;

use crate::{
    handle::{
        borrow_from_handle, filter_close_handles, iter_opened_handles, register_handle_with_init,
        take_and_close_handle, FilterCloseHandle, Handle, HandleItem,
    },
    ClientConfig, ClientEvent, DeviceAccessStrategy, OnEventCallbackPlugged,
};

fn borrow_client(client: Handle) -> anyhow::Result<Arc<libparsec_client::Client>> {
    borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })
}

/*
 * List started clients
 */

/// List started clients by handle and device ID.
///
/// In theory `DeviceID` is not guaranteed to be globally unique since its value
/// is choosen client-side during the device creation.
///
/// However in practice the device ID *is* randomly picked so we can use it
/// as a unique identifier (instead of having to also compare the server URL and
/// organization ID).
pub fn list_started_clients() -> Vec<(Handle, DeviceID)> {
    let mut clients = vec![];
    iter_opened_handles(|handle, item| {
        if let HandleItem::Client { client, .. } = item {
            clients.push((handle, client.device_id()));
        }
    });
    clients
}

/*
 * Wait for device available
 */

#[derive(Debug, thiserror::Error)]
pub enum WaitForDeviceAvailableError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn wait_for_device_available(
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))] config_dir: &Path,
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))] device_id: DeviceID,
) -> Result<(), WaitForDeviceAvailableError> {
    // On web there is no other processes able to use the devices we have access of.
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Wait for the lock, take it, then release it right away
        libparsec_platform_ipc::lock_device_for_use(config_dir, device_id)
            .await
            .map_err(WaitForDeviceAvailableError::Internal)?;
    }

    Ok(())
}

/*
 * Start
 */

#[derive(Debug, thiserror::Error)]
pub enum ClientStartError {
    #[error("Device already used by another process")]
    DeviceUsedByAnotherProcess,
    #[error("Cannot load device file: invalid path ({})", .0)]
    LoadDeviceInvalidPath(anyhow::Error),
    #[error("Cannot load device file: invalid data")]
    LoadDeviceInvalidData,
    #[error("Cannot load device file: decryption failed")]
    LoadDeviceDecryptionFailed,
    /// Client start is a fully offline operation, except for some device
    /// access strategies (e.g. account vault), where the ciphertext key
    /// protecting the device is itself encrypted by an opaque key that
    /// must first be remotely fetched.
    #[error("Cannot load device file: no response from {server} server: {error}")]
    // We don't use `ConnectionError` here since this type only corresponds to
    // an answer from the Parsec server and here any arbitrary server may have
    // been (unsuccessfully) requested (e.g. OpenBao server).
    LoadDeviceRemoteOpaqueKeyFetchOffline {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    /// Note only a subset of load strategies requires server access to
    /// fetch an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("Cannot load device file: {server} server opaque key fetch failed: {error}")]
    LoadDeviceRemoteOpaqueKeyFetchFailed {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<libparsec_platform_device_loader::LoadDeviceError> for ClientStartError {
    fn from(value: libparsec_platform_device_loader::LoadDeviceError) -> Self {
        use libparsec_platform_device_loader::LoadDeviceError;
        match value {
            e @ LoadDeviceError::StorageNotAvailable => Self::LoadDeviceInvalidPath(e.into()),
            LoadDeviceError::InvalidPath(err) => Self::LoadDeviceInvalidPath(err),
            LoadDeviceError::InvalidData => Self::LoadDeviceInvalidData,
            LoadDeviceError::DecryptionFailed => Self::LoadDeviceDecryptionFailed,
            LoadDeviceError::Internal(e) => Self::Internal(e),
            LoadDeviceError::RemoteOpaqueKeyFetchOffline { server, error } => {
                Self::LoadDeviceRemoteOpaqueKeyFetchOffline { server, error }
            }
            LoadDeviceError::RemoteOpaqueKeyFetchFailed { server, error } => {
                Self::LoadDeviceRemoteOpaqueKeyFetchFailed { server, error }
            }
        }
    }
}

pub async fn client_start(
    config: ClientConfig,
    access: DeviceAccessStrategy,
) -> Result<Handle, ClientStartError> {
    log::trace!("Starting client_start");
    let access = access
        .convert_with_side_effects()
        .map_err(ClientStartError::Internal)?;

    let config: Arc<libparsec_client::ClientConfig> = config.into();

    // 1) Load the device

    let device = libparsec_platform_device_loader::load_device(&config.config_dir, &access).await?;

    // 2) Actually start the client

    client_start_from_local_device(config, device).await
}

pub(super) async fn client_start_from_local_device(
    config: Arc<libparsec_client::ClientConfig>,
    device: Arc<LocalDevice>,
) -> Result<Handle, ClientStartError> {
    // 1) Make sure another client is not running this device

    // 1.1) Is our own process already running this device ?

    enum RegisterFailed {
        AlreadyRegistered(Handle),
        ConcurrentRegister(EventListener),
    }

    log::debug!("Starting client for device: {}", device.device_id);

    let initializing = loop {
        let outcome = register_handle_with_init(
            HandleItem::StartingClient {
                device_id: device.device_id,
                to_wake_on_done: vec![],
            },
            |handle, item| match item {
                HandleItem::Client { client, .. } if client.device_id() == device.device_id => {
                    Err(RegisterFailed::AlreadyRegistered(handle))
                }
                HandleItem::StartingClient {
                    device_id: x_device_id,
                    to_wake_on_done,
                } if *x_device_id == device.device_id => {
                    let event = Event::new();
                    let listener = event.listen();
                    to_wake_on_done.push(event);
                    Err(RegisterFailed::ConcurrentRegister(listener))
                }
                _ => Ok(()),
            },
        );

        match outcome {
            Ok(initializing) => break initializing,
            Err(RegisterFailed::AlreadyRegistered(handle)) => {
                // Go idempotent here
                return Ok(handle);
            }
            // Wait for concurrent operation to finish before retrying
            Err(RegisterFailed::ConcurrentRegister(listener)) => listener.await,
        }
    };

    // 1.2) Is another process running this device ?

    // On web there is no other processes able to use the devices we have access of.
    #[cfg(not(target_arch = "wasm32"))]
    let device_in_use_guard = {
        use libparsec_platform_ipc::{try_lock_device_for_use, TryLockDeviceForUseError};

        try_lock_device_for_use(&config.config_dir, device.device_id).map_err(|err| match err {
            TryLockDeviceForUseError::AlreadyInUse => ClientStartError::DeviceUsedByAnotherProcess,
            TryLockDeviceForUseError::Internal(err) => err.into(),
        })?
    };

    // 2) Actually start the client

    let on_event_callback = super::get_on_event_callback();
    let on_event = OnEventCallbackPlugged::new(initializing.handle(), on_event_callback.clone());
    let client = libparsec_client::Client::start(config, on_event.event_bus.clone(), device)
        .await
        .map_err(ClientStartError::Internal)?;
    let device_id = client.device_id();

    let handle = initializing.initialized(move |item: &mut HandleItem| {
        let starting = std::mem::replace(
            item,
            HandleItem::Client {
                client,
                on_event,
                #[cfg(not(target_arch = "wasm32"))]
                device_in_use_guard,
            },
        );
        match starting {
            HandleItem::StartingClient {
                to_wake_on_done, ..
            } => {
                for event in to_wake_on_done {
                    event.notify(usize::MAX);
                }
            }
            _ => unreachable!(),
        }
    });

    log::debug!("Client started for device: {}", device_id);

    on_event_callback(handle, ClientEvent::ClientStarted { device_id });

    Ok(handle)
}

/*
 * Stop
 */

#[derive(Debug, thiserror::Error)]
pub enum ClientStopError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn client_stop(client: Handle) -> Result<(), ClientStopError> {
    let client_handle = client;

    // Stopping the client is divided into two steps:
    // - First actually stopping the client
    // - And only then cleaning up the handles related to the client
    //
    // The order is important here: if we remove the client handle before
    // stopping the client, then we open the door for a concurrent start
    // of a client using the same device (which is not allowed).

    // 1. Stop the client

    let client = borrow_client(client_handle)?;
    let device_id = client.device_id();

    // Notes:
    // - Stop is idempotent.
    // - We may have concurrent (or even subsequent given the client handle is still
    //   usable !) operations currently using the client, this is fine as those
    //   operations will simply receive a `Stopped` error.
    // - Stopping the client also stop all it related workspace ops.
    client.stop().await;

    // 2. Cleanup the handles related to the client

    let (on_event, device_in_use_guard) = take_and_close_handle(client_handle, |x| match *x {
        HandleItem::Client {
            on_event,
            #[cfg(not(target_arch = "wasm32"))]
            device_in_use_guard,
            ..
        } => {
            #[cfg(target_arch = "wasm32")]
            let device_in_use_guard = ();
            Ok((on_event, device_in_use_guard))
        }
        // Note we consider an error if the handle is in `HandleItem::StartingClient` state
        // this is because at that point this is not a legit use of the handle given it
        // has never been yet provided to the caller in the first place !
        // On top of that it simplifies the start logic (given it guarantees nothing will
        // concurrently close the handle)
        _ => Err(x),
    })?;

    // Wait until after the client is closed to disconnect the event bus to ensure
    // we don't miss any event fired during client teardown
    drop(on_event);

    // Also wait until after the client is closed to allow other process to use our device.
    // (This is a noop on web, as no other processes are able to use the devices we have access of).
    #[cfg_attr(target_arch = "wasm32", allow(dropping_copy_types))]
    drop(device_in_use_guard);

    // Finally cleanup the handles related to the client's workspaces and mountpoints
    // (Note the mountpoints are automatically unmounted when the handle item is dropped).
    loop {
        let mut maybe_wait = None;
        filter_close_handles(client_handle, |x| match x {
            HandleItem::Workspace {
                client: x_client, ..
            }
            | HandleItem::WorkspaceHistory {
                client: Some(x_client),
                ..
            }
            | HandleItem::Mountpoint {
                client: x_client, ..
            } if *x_client == client_handle => FilterCloseHandle::Close,

            // If something is still starting, it will most likely won't go very far
            // (all workspace ops now are stopped), but we have to wait for it anyway
            HandleItem::StartingMountpoint {
                client: x_client,
                to_wake_on_done,
                ..
            }
            | HandleItem::StartingClientWorkspaceHistory {
                client: x_client,
                to_wake_on_done,
                ..
            }
            | HandleItem::StartingWorkspace {
                client: x_client,
                to_wake_on_done,
                ..
            } if *x_client == client_handle => {
                if maybe_wait.is_none() {
                    let event = Event::new();
                    let listener = event.listen();
                    to_wake_on_done.push(event);
                    maybe_wait = Some(listener);
                }
                FilterCloseHandle::Keep
            }
            _ => FilterCloseHandle::Keep,
        });
        // Note there is no risk (in theory it least !) to end up in an infinite
        // loop here: the client's handle is closed so no new workspace start
        // commands can be issued and we only process the remaining ones.
        match maybe_wait {
            Some(listener) => listener.await,
            None => break,
        }
    }

    let on_event_callback = super::get_on_event_callback();
    on_event_callback(client_handle, ClientEvent::ClientStopped { device_id });

    Ok(())
}

/*
 * Client forget all certificates
 */

pub async fn client_forget_all_certificates(
    client: Handle,
) -> Result<(), ClientForgetAllCertificatesError> {
    let client = borrow_client(client)?;

    client.forget_all_certificates().await
}

/*
 * Client info
 */

pub struct ClientInfo {
    pub organization_addr: ParsecOrganizationAddr,
    pub organization_id: OrganizationID,
    pub device_id: DeviceID,
    pub user_id: UserID,
    pub device_label: DeviceLabel,
    pub human_handle: HumanHandle,
    pub current_profile: UserProfile,
    pub server_config: ServerConfig,
    pub is_server_online: bool,
    pub is_organization_expired: bool,
    pub must_accept_tos: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ClientInfoError {
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn client_info(client: Handle) -> Result<ClientInfo, ClientInfoError> {
    let client = borrow_client(client)?;

    Ok(ClientInfo {
        organization_addr: client.organization_addr().clone(),
        organization_id: client.organization_id().clone(),
        device_id: client.device_id(),
        user_id: client.user_id(),
        device_label: client.device_label().to_owned(),
        human_handle: client.human_handle().to_owned(),
        current_profile: client
            .get_current_self_profile()
            .await
            .map_err(|e| match e {
                ClientGetCurrentSelfProfileError::Stopped => ClientInfoError::Stopped,
                ClientGetCurrentSelfProfileError::Internal(e) => {
                    e.context("Cannot retrieve profile").into()
                }
            })?,
        server_config: client.server_config(),
        is_server_online: client.is_server_online(),
        is_organization_expired: client.is_organization_expired(),
        must_accept_tos: client.must_accept_tos(),
    })
}

/*
 * Get Terms of Service (TOS)
 */

pub async fn client_get_tos(client: Handle) -> Result<Tos, ClientGetTosError> {
    let client = borrow_client(client)?;

    client.get_tos().await
}

/*
 * Accept Terms of Service (TOS)
 */

pub async fn client_accept_tos(
    client: Handle,
    tos_updated_on: DateTime,
) -> Result<(), ClientAcceptTosError> {
    let client = borrow_client(client)?;

    client.accept_tos(tos_updated_on).await
}

/*
 * Revoke user
 */

pub async fn client_revoke_user(client: Handle, user: UserID) -> Result<(), ClientRevokeUserError> {
    let client = borrow_client(client)?;

    client.revoke_user(user).await
}

/*
 * List users
 */

// TODO: order by user name (asc/desc), offset/limit
pub async fn client_list_users(
    client: Handle,
    skip_revoked: bool,
    // offset: Option<usize>,
    // limit: Option<usize>,
) -> Result<Vec<UserInfo>, ClientListUsersError> {
    let client = borrow_client(client)?;

    client.list_users(skip_revoked, None, None).await
}

/*
 * Get user info
 */

pub async fn client_get_user_info(
    client: Handle,
    user_id: UserID,
) -> Result<UserInfo, ClientGetUserInfoError> {
    let client = borrow_client(client)?;

    client.get_user_info(user_id).await
}

/*
 * List frozen users
 */

pub async fn client_list_frozen_users(
    client: Handle,
) -> Result<Vec<UserID>, ClientListFrozenUsersError> {
    let client = borrow_client(client)?;

    client.list_frozen_users().await
}

/*
 * List user devices
 */

pub async fn client_list_user_devices(
    client: Handle,
    user: UserID,
) -> Result<Vec<DeviceInfo>, ClientListUserDevicesError> {
    let client = borrow_client(client)?;

    client.list_user_devices(user).await
}

/*
 * Get user device
 */

pub async fn client_get_user_device(
    client: Handle,
    device: DeviceID,
) -> Result<(UserInfo, DeviceInfo), ClientGetUserDeviceError> {
    let client = borrow_client(client)?;

    client.get_user_device(device).await
}

/*
 * List workspace's users
 */

pub async fn client_list_workspace_users(
    client: Handle,
    realm_id: VlobID,
) -> Result<Vec<WorkspaceUserAccessInfo>, ClientListWorkspaceUsersError> {
    let client = borrow_client(client)?;

    client.list_workspace_users(realm_id).await
}

/*
 * List workspaces
 */

#[derive(Debug, thiserror::Error)]
pub enum ClientListWorkspacesError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn client_list_workspaces(
    client: Handle,
) -> Result<Vec<WorkspaceInfo>, ClientListWorkspacesError> {
    let client = borrow_client(client)?;

    let workspaces = client.list_workspaces().await;

    Ok(workspaces)
}

/*
 * Create workspace
 */

pub async fn client_create_workspace(
    client: Handle,
    name: EntryName,
) -> Result<VlobID, ClientCreateWorkspaceError> {
    let client = borrow_client(client)?;

    client.create_workspace(name).await
}

/*
 * Client rename workspace
 */

pub async fn client_rename_workspace(
    client: Handle,
    realm_id: VlobID,
    new_name: EntryName,
) -> Result<(), ClientRenameWorkspaceError> {
    let client = borrow_client(client)?;

    client.rename_workspace(realm_id, new_name).await
}

/*
 * Client share workspace
 */

pub async fn client_share_workspace(
    client: Handle,
    realm_id: VlobID,
    recipient: UserID,
    role: Option<RealmRole>,
) -> Result<(), ClientShareWorkspaceError> {
    let client = borrow_client(client)?;

    client.share_workspace(realm_id, recipient, role).await
}

/*
 * Setup shamir recovery
 */

pub async fn client_setup_shamir_recovery(
    client: Handle,
    per_recipient_shares: HashMap<UserID, NonZeroU8>,
    threshold: NonZeroU8,
) -> Result<(), ClientSetupShamirRecoveryError> {
    let client = borrow_client(client)?;

    client
        .setup_shamir_recovery(per_recipient_shares, threshold)
        .await
}

/*
 * Delete shamir recovery
 */

pub async fn client_delete_shamir_recovery(
    client: Handle,
) -> Result<(), ClientDeleteShamirRecoveryError> {
    let client = borrow_client(client)?;

    client.delete_shamir_recovery().await
}

/*
 * Get self shamir recovery
 */

pub async fn client_get_self_shamir_recovery(
    client: Handle,
) -> Result<SelfShamirRecoveryInfo, ClientGetSelfShamirRecoveryError> {
    let client = borrow_client(client)?;

    client.get_self_shamir_recovery().await
}

/*
 * List shamir recoveries for others
 */

pub async fn client_list_shamir_recoveries_for_others(
    client: Handle,
) -> Result<Vec<OtherShamirRecoveryInfo>, ClientListShamirRecoveriesForOthersError> {
    let client = borrow_client(client)?;

    client.list_shamir_recoveries_for_others().await
}

pub async fn client_update_user_profile(
    client: Handle,
    user: UserID,
    new_profile: UserProfile,
) -> Result<(), ClientUserUpdateProfileError> {
    let client = borrow_client(client)?;

    client.update_user_profile(user, new_profile).await
}

/*
 * Organization info
 */
pub async fn client_organization_info(
    client: Handle,
) -> Result<OrganizationInfo, ClientOrganizationInfoError> {
    let client = borrow_client(client)?;

    client.organization_info().await
}

pub async fn client_get_organization_bootstrap_date(
    client: Handle,
) -> Result<DateTime, ClientGetOrganizationBootstrapDateError> {
    let client = borrow_client(client)?;
    client.get_organization_bootstrap_date().await
}

pub async fn client_pki_list_enrollments(
    client: Handle,
) -> Result<Vec<PkiEnrollmentListItem>, PkiEnrollmentListError> {
    let client = borrow_client(client)?;
    client.pki_list_enrollments().await
}

#[derive(Debug, thiserror::Error)]
pub enum PkiGetAddrError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn client_pki_get_addr(
    client: Handle,
) -> Result<ParsecPkiEnrollmentAddr, PkiGetAddrError> {
    let client = borrow_client(client)?;
    Ok(client.pki_get_addr().await)
}

pub async fn client_pki_enrollment_reject(
    client: Handle,
    enrollment_id: PKIEnrollmentID,
) -> Result<(), PkiEnrollmentRejectError> {
    let client = borrow_client(client)?;
    client.pki_enrollment_reject(enrollment_id).await
}

pub async fn client_pki_enrollment_accept(
    client: Handle,
    profile: UserProfile,
    enrollment_id: PKIEnrollmentID,
    human_handle: &HumanHandle,
    cert_ref: &X509CertificateReference,
    submit_payload: PkiEnrollmentSubmitPayload,
) -> Result<(), PkiEnrollmentAcceptError> {
    let client = borrow_client(client)?;
    client
        .pki_enrollment_accept(
            profile,
            enrollment_id,
            human_handle,
            cert_ref,
            &submit_payload,
        )
        .await
}
