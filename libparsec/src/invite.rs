// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

pub use libparsec_client::{
    ClientCancelInvitationError, ClientNewDeviceInvitationError, ClientNewUserInvitationError,
    InvitationEmailSentStatus, ListInvitationsError,
};
pub use libparsec_types::prelude::*;

use crate::{
    handle::{borrow_from_handle, register_handle, take_and_close_handle, Handle, HandleItem},
    listen_canceller, ClientConfig, ClientEvent, OnEventCallbackPlugged,
};

#[derive(Debug, Clone)]
pub enum DeviceSaveStrategy {
    Password { password: Password },
    Smartcard,
}

impl DeviceSaveStrategy {
    pub fn into_access(self, key_file: PathBuf) -> DeviceAccessStrategy {
        match self {
            DeviceSaveStrategy::Password { password } => {
                DeviceAccessStrategy::Password { key_file, password }
            }
            DeviceSaveStrategy::Smartcard => DeviceAccessStrategy::Smartcard { key_file },
        }
    }
}

/*
 * Bootstrap organization
 */

#[derive(Debug, thiserror::Error)]
pub enum BootstrapOrganizationError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Organization has expired")]
    OrganizationExpired,
    #[error("Invalid bootstrap token")]
    InvalidToken,
    #[error("Bootstrap token already used")]
    AlreadyUsedToken,
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error("Cannot save device: {0}")]
    SaveDeviceError(anyhow::Error),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<libparsec_client::BootstrapOrganizationError> for BootstrapOrganizationError {
    fn from(value: libparsec_client::BootstrapOrganizationError) -> Self {
        match value {
            libparsec_client::BootstrapOrganizationError::Offline => {
                BootstrapOrganizationError::Offline
            }
            libparsec_client::BootstrapOrganizationError::OrganizationExpired => {
                BootstrapOrganizationError::OrganizationExpired
            }
            libparsec_client::BootstrapOrganizationError::InvalidToken => {
                BootstrapOrganizationError::InvalidToken
            }
            libparsec_client::BootstrapOrganizationError::AlreadyUsedToken => {
                BootstrapOrganizationError::AlreadyUsedToken
            }
            libparsec_client::BootstrapOrganizationError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => BootstrapOrganizationError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
            libparsec_client::BootstrapOrganizationError::Internal(e) => {
                BootstrapOrganizationError::Internal(e)
            }
        }
    }
}

pub async fn bootstrap_organization(
    config: ClientConfig,

    // Access to the event bus is done through this callback.
    // Ad-hoc code should be added to the binding system to handle this (hence
    // why this is passed as a parameter instead of as part of `ClientConfig`:
    // we can have a simple `if func_name == "client_login"` that does a special
    // cooking of it last param.
    #[cfg(not(target_arch = "wasm32"))] on_event_callback: Arc<dyn Fn(ClientEvent) + Send + Sync>,
    // On web we run on the JS runtime which is mono-threaded, hence everything is !Send
    #[cfg(target_arch = "wasm32")] on_event_callback: Arc<dyn Fn(ClientEvent)>,

    bootstrap_organization_addr: BackendOrganizationBootstrapAddr,
    save_strategy: DeviceSaveStrategy,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    sequester_authority_verify_key: Option<SequesterVerifyKeyDer>,
) -> Result<AvailableDevice, BootstrapOrganizationError> {
    let config: Arc<libparsec_client::ClientConfig> = config.into();
    let events_plugged = OnEventCallbackPlugged::new(on_event_callback);
    // TODO: connect event_bus to on_event_callback

    let finalize_ctx = libparsec_client::bootstrap_organization(
        config.clone(),
        events_plugged.event_bus,
        bootstrap_organization_addr,
        human_handle,
        device_label,
        sequester_authority_verify_key,
    )
    .await?;

    let access = {
        let key_file = libparsec_platform_device_loader::get_default_key_file(
            &config.config_dir,
            &finalize_ctx.new_local_device,
        );
        save_strategy.into_access(key_file)
    };

    let available_device = finalize_ctx
        .save_local_device(&access)
        .await
        .map_err(BootstrapOrganizationError::SaveDeviceError)?;

    Ok(available_device)
}

/*
 * Invitation claimer
 */

#[derive(Debug, thiserror::Error)]
pub enum ClaimInProgressError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Organization has expired")]
    OrganizationExpired,
    #[error("Invitation not found")]
    NotFound,
    #[error("Invitation already used")]
    AlreadyUsed,
    #[error("Claim operation reset by peer")]
    PeerReset,
    #[error("Active users limit reached")]
    ActiveUsersLimitReached,
    #[error(transparent)]
    CorruptedConfirmation(DataError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    // Additional error
    #[error("Operation cancelled")]
    Cancelled,
}

impl From<libparsec_client::ClaimInProgressError> for ClaimInProgressError {
    fn from(value: libparsec_client::ClaimInProgressError) -> Self {
        match value {
            libparsec_client::ClaimInProgressError::Offline => ClaimInProgressError::Offline,
            libparsec_client::ClaimInProgressError::OrganizationExpired => {
                ClaimInProgressError::OrganizationExpired
            }
            libparsec_client::ClaimInProgressError::NotFound => ClaimInProgressError::NotFound,
            libparsec_client::ClaimInProgressError::AlreadyUsed => {
                ClaimInProgressError::AlreadyUsed
            }
            libparsec_client::ClaimInProgressError::PeerReset => ClaimInProgressError::PeerReset,
            libparsec_client::ClaimInProgressError::ActiveUsersLimitReached => {
                ClaimInProgressError::ActiveUsersLimitReached
            }
            libparsec_client::ClaimInProgressError::CorruptedConfirmation(x) => {
                ClaimInProgressError::CorruptedConfirmation(x)
            }
            libparsec_client::ClaimInProgressError::Internal(x) => {
                ClaimInProgressError::Internal(x)
            }
        }
    }
}

pub use libparsec_client::ClaimerRetrieveInfoError;

pub async fn claimer_retrieve_info(
    config: ClientConfig,

    // Access to the event bus is done through this callback.
    // Ad-hoc code should be added to the binding system to handle this (hence
    // why this is passed as a parameter instead of as part of `ClientConfig`:
    // we can have a simple `if func_name == "client_login"` that does a special
    // cooking of it last param.
    #[cfg(not(target_arch = "wasm32"))] _on_event_callback: Arc<dyn Fn(ClientEvent) + Send + Sync>,
    // On web we run on the JS runtime which is mono-threaded, hence everything is !Send
    #[cfg(target_arch = "wasm32")] _on_event_callback: Arc<dyn Fn(ClientEvent)>,

    addr: BackendInvitationAddr,
) -> Result<UserOrDeviceClaimInitialInfo, ClaimerRetrieveInfoError> {
    let config: Arc<libparsec_client::ClientConfig> = config.into();
    // TODO
    // let events_plugged = Arc::new(OnEventCallbackPlugged::new(on_event_callback));

    let ctx = libparsec_client::claimer_retrieve_info(config, addr).await?;

    match ctx {
        libparsec_client::UserOrDeviceClaimInitialCtx::User(ctx) => {
            let claimer_email = ctx.claimer_email.clone();
            let greeter_user_id = ctx.greeter_user_id().to_owned();
            let greeter_human_handle = ctx.greeter_human_handle().to_owned();
            let handle = register_handle(HandleItem::UserClaimInitial(ctx));
            Ok(UserOrDeviceClaimInitialInfo::User {
                handle,
                claimer_email,
                greeter_user_id,
                greeter_human_handle,
            })
        }
        libparsec_client::UserOrDeviceClaimInitialCtx::Device(ctx) => {
            let greeter_user_id = ctx.greeter_user_id().to_owned();
            let greeter_human_handle = ctx.greeter_human_handle().to_owned();
            let handle = register_handle(HandleItem::DeviceClaimInitial(ctx));
            Ok(UserOrDeviceClaimInitialInfo::Device {
                handle,
                greeter_user_id,
                greeter_human_handle,
            })
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClaimerGreeterAbortOperationError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub fn claimer_greeter_abort_operation(
    handle: Handle,
) -> Result<(), ClaimerGreeterAbortOperationError> {
    take_and_close_handle(handle, |x| match x {
        HandleItem::UserClaimInitial(_)
        | HandleItem::DeviceClaimInitial(_)
        | HandleItem::UserClaimInProgress1(_)
        | HandleItem::DeviceClaimInProgress1(_)
        | HandleItem::UserClaimInProgress2(_)
        | HandleItem::DeviceClaimInProgress2(_)
        | HandleItem::UserClaimInProgress3(_)
        | HandleItem::DeviceClaimInProgress3(_)
        | HandleItem::UserClaimFinalize(_)
        | HandleItem::DeviceClaimFinalize(_)
        | HandleItem::UserGreetInitial(_)
        | HandleItem::DeviceGreetInitial(_)
        | HandleItem::UserGreetInProgress1(_)
        | HandleItem::DeviceGreetInProgress1(_)
        | HandleItem::UserGreetInProgress2(_)
        | HandleItem::DeviceGreetInProgress2(_)
        | HandleItem::UserGreetInProgress3(_)
        | HandleItem::DeviceGreetInProgress3(_)
        | HandleItem::UserGreetInProgress4(_)
        | HandleItem::DeviceGreetInProgress4(_) => Ok(()),
        invalid => Err(invalid),
    })
    .map_err(|err| err.into())
}

pub enum UserOrDeviceClaimInitialInfo {
    User {
        handle: Handle,
        claimer_email: String,
        greeter_user_id: UserID,
        greeter_human_handle: HumanHandle,
    },
    Device {
        handle: Handle,
        greeter_user_id: UserID,
        greeter_human_handle: HumanHandle,
    },
}

pub async fn claimer_user_initial_do_wait_peer(
    canceller: Handle,
    handle: Handle,
) -> Result<UserClaimInProgress1Info, ClaimInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::UserClaimInitial(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_wait_peer().await?;
        let greeter_sas_choices = ctx.generate_greeter_sas_choices(4);
        let greeter_sas = ctx.greeter_sas().to_owned();

        let new_handle = register_handle(HandleItem::UserClaimInProgress1(ctx));

        Ok(UserClaimInProgress1Info {
            handle: new_handle,
            greeter_sas,
            greeter_sas_choices,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(ClaimInProgressError::Cancelled),
    )
}

pub async fn claimer_device_initial_do_wait_peer(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceClaimInProgress1Info, ClaimInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::DeviceClaimInitial(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_wait_peer().await?;
        let greeter_sas_choices = ctx.generate_greeter_sas_choices(4);
        let greeter_sas = ctx.greeter_sas().to_owned();

        let new_handle = register_handle(HandleItem::DeviceClaimInProgress1(ctx));

        Ok(DeviceClaimInProgress1Info {
            handle: new_handle,
            greeter_sas,
            greeter_sas_choices,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(ClaimInProgressError::Cancelled),
    )
}

pub struct UserClaimInProgress1Info {
    pub handle: Handle,
    pub greeter_sas: SASCode,
    pub greeter_sas_choices: Vec<SASCode>,
}
pub struct DeviceClaimInProgress1Info {
    pub handle: Handle,
    pub greeter_sas: SASCode,
    pub greeter_sas_choices: Vec<SASCode>,
}

pub async fn claimer_user_in_progress_1_do_signify_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<UserClaimInProgress2Info, ClaimInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::UserClaimInProgress1(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_signify_trust().await?;
        let claimer_sas = ctx.claimer_sas().to_owned();

        let new_handle = register_handle(HandleItem::UserClaimInProgress2(ctx));

        Ok(UserClaimInProgress2Info {
            handle: new_handle,
            claimer_sas,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(ClaimInProgressError::Cancelled),
    )
}

pub async fn claimer_device_in_progress_1_do_signify_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceClaimInProgress2Info, ClaimInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::DeviceClaimInProgress1(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_signify_trust().await?;
        let claimer_sas = ctx.claimer_sas().to_owned();

        let new_handle = register_handle(HandleItem::DeviceClaimInProgress2(ctx));

        Ok(DeviceClaimInProgress2Info {
            handle: new_handle,
            claimer_sas,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(ClaimInProgressError::Cancelled),
    )
}

pub struct UserClaimInProgress2Info {
    pub handle: Handle,
    pub claimer_sas: SASCode,
}
pub struct DeviceClaimInProgress2Info {
    pub handle: Handle,
    pub claimer_sas: SASCode,
}

pub async fn claimer_user_in_progress_2_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<UserClaimInProgress3Info, ClaimInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::UserClaimInProgress2(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_wait_peer_trust().await?;

        let new_handle = register_handle(HandleItem::UserClaimInProgress3(ctx));

        Ok(UserClaimInProgress3Info { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(ClaimInProgressError::Cancelled),
    )
}

pub async fn claimer_device_in_progress_2_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceClaimInProgress3Info, ClaimInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::DeviceClaimInProgress2(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_wait_peer_trust().await?;

        let new_handle = register_handle(HandleItem::DeviceClaimInProgress3(ctx));

        Ok(DeviceClaimInProgress3Info { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(ClaimInProgressError::Cancelled),
    )
}

pub struct UserClaimInProgress3Info {
    pub handle: Handle,
}
pub struct DeviceClaimInProgress3Info {
    pub handle: Handle,
}

pub async fn claimer_user_in_progress_3_do_claim(
    canceller: Handle,
    handle: Handle,
    requested_device_label: DeviceLabel,
    requested_human_handle: HumanHandle,
) -> Result<UserClaimFinalizeInfo, ClaimInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::UserClaimInProgress3(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx
            .do_claim_user(requested_device_label, requested_human_handle)
            .await?;

        let new_handle = register_handle(HandleItem::UserClaimFinalize(ctx));

        Ok(UserClaimFinalizeInfo { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(ClaimInProgressError::Cancelled),
    )
}

pub async fn claimer_device_in_progress_3_do_claim(
    canceller: Handle,
    handle: Handle,
    requested_device_label: DeviceLabel,
) -> Result<DeviceClaimFinalizeInfo, ClaimInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::DeviceClaimInProgress3(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_claim_device(requested_device_label).await?;

        let new_handle = register_handle(HandleItem::DeviceClaimFinalize(ctx));

        Ok(DeviceClaimFinalizeInfo { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(ClaimInProgressError::Cancelled),
    )
}

pub struct UserClaimFinalizeInfo {
    pub handle: Handle,
}
pub struct DeviceClaimFinalizeInfo {
    pub handle: Handle,
}

pub async fn claimer_user_finalize_save_local_device(
    handle: Handle,
    save_strategy: DeviceSaveStrategy,
) -> Result<AvailableDevice, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match x {
        HandleItem::UserClaimFinalize(ctx) => Ok(ctx),
        invalid => Err(invalid),
    })?;

    let access = {
        let key_file = ctx.get_default_key_file();
        save_strategy.into_access(key_file)
    };

    let available_device = ctx.save_local_device(&access).await?;

    Ok(available_device)
}

pub async fn claimer_device_finalize_save_local_device(
    handle: Handle,
    save_strategy: DeviceSaveStrategy,
) -> Result<AvailableDevice, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match x {
        HandleItem::DeviceClaimFinalize(ctx) => Ok(ctx),
        invalid => Err(invalid),
    })?;

    let access = {
        let key_file = ctx.get_default_key_file();
        save_strategy.into_access(key_file)
    };

    let available_device = ctx.save_local_device(&access).await?;

    Ok(available_device)
}

/*
 * Invitation greeter
 */

pub struct NewInvitationInfo {
    pub addr: BackendInvitationAddr,
    pub token: InvitationToken,
    pub email_sent_status: InvitationEmailSentStatus,
}

pub async fn client_new_user_invitation(
    client: Handle,
    claimer_email: String,
    send_email: bool,
) -> Result<NewInvitationInfo, ClientNewUserInvitationError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let (token, email_sent_status) = client
        .new_user_invitation(claimer_email, send_email)
        .await?;

    Ok(NewInvitationInfo {
        addr: BackendInvitationAddr::new(
            client.organization_addr(),
            client.organization_id().to_owned(),
            InvitationType::User,
            token,
        ),
        token,
        email_sent_status,
    })
}

pub async fn client_new_device_invitation(
    client: Handle,
    send_email: bool,
) -> Result<NewInvitationInfo, ClientNewDeviceInvitationError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let (token, email_sent_status) = client.new_device_invitation(send_email).await?;

    Ok(NewInvitationInfo {
        addr: BackendInvitationAddr::new(
            client.organization_addr(),
            client.organization_id().to_owned(),
            InvitationType::Device,
            token,
        ),
        token,
        email_sent_status,
    })
}

pub async fn client_cancel_invitation(
    client: Handle,
    token: InvitationToken,
) -> Result<(), ClientCancelInvitationError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    client.cancel_invitation(token).await
}

// pub use libparsec_client::InviteListItem;

pub enum InviteListItem {
    User {
        addr: BackendInvitationAddr,
        token: InvitationToken,
        created_on: DateTime,
        claimer_email: String,
        status: InvitationStatus,
    },
    Device {
        addr: BackendInvitationAddr,
        token: InvitationToken,
        created_on: DateTime,
        status: InvitationStatus,
    },
}

pub async fn client_list_invitations(
    client: Handle,
) -> Result<Vec<InviteListItem>, ListInvitationsError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let items = client
        .list_invitations()
        .await?
        .into_iter()
        .map(|item| match item {
            libparsec_client::InviteListItem::User {
                claimer_email,
                created_on,
                status,
                token,
            } => {
                let addr = BackendInvitationAddr::new(
                    client.organization_addr(),
                    client.organization_id().to_owned(),
                    InvitationType::User,
                    token,
                );
                InviteListItem::User {
                    addr,
                    claimer_email,
                    created_on,
                    status,
                    token,
                }
            }
            libparsec_client::InviteListItem::Device {
                created_on,
                status,
                token,
            } => {
                let addr = BackendInvitationAddr::new(
                    client.organization_addr(),
                    client.organization_id().to_owned(),
                    InvitationType::Device,
                    token,
                );
                InviteListItem::Device {
                    addr,
                    created_on,
                    status,
                    token,
                }
            }
        })
        .collect();

    Ok(items)
}

#[derive(Debug, thiserror::Error)]
pub enum ClientStartInvitationGreetError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn client_start_user_invitation_greet(
    client: Handle,
    token: InvitationToken,
) -> Result<UserGreetInitialInfo, ClientStartInvitationGreetError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let ctx = client.start_user_invitation_greet(token);

    let handle = register_handle(HandleItem::UserGreetInitial(ctx));

    Ok(UserGreetInitialInfo { handle })
}

pub async fn client_start_device_invitation_greet(
    client: Handle,
    token: InvitationToken,
) -> Result<DeviceGreetInitialInfo, ClientStartInvitationGreetError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let ctx = client.start_device_invitation_greet(token);

    let handle = register_handle(HandleItem::DeviceGreetInitial(ctx));

    Ok(DeviceGreetInitialInfo { handle })
}

#[derive(Debug, thiserror::Error)]
pub enum GreetInProgressError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Invitation not found")]
    NotFound,
    #[error("Invitation already used or cancelled")]
    AlreadyDeleted,
    #[error("Greet operation reset by peer")]
    PeerReset,
    #[error("Active users limit reached")]
    ActiveUsersLimitReached,
    #[error("Claimer's nonce and hashed nonce don't match")]
    NonceMismatch,
    #[error("Human handle (i.e. email address) already taken")]
    HumanHandleAlreadyTaken,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Device already exists")]
    DeviceAlreadyExists,
    #[error("Not allowed to create a user")]
    UserCreateNotAllowed,
    #[error(transparent)]
    CorruptedInviteUserData(DataError),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    // Additional error
    #[error("Operation cancelled")]
    Cancelled,
}

impl From<libparsec_client::GreetInProgressError> for GreetInProgressError {
    fn from(value: libparsec_client::GreetInProgressError) -> Self {
        match value {
            libparsec_client::GreetInProgressError::Offline => GreetInProgressError::Offline,
            libparsec_client::GreetInProgressError::NotFound => GreetInProgressError::NotFound,
            libparsec_client::GreetInProgressError::AlreadyDeleted => {
                GreetInProgressError::AlreadyDeleted
            }
            libparsec_client::GreetInProgressError::PeerReset => GreetInProgressError::PeerReset,
            libparsec_client::GreetInProgressError::ActiveUsersLimitReached => {
                GreetInProgressError::ActiveUsersLimitReached
            }
            libparsec_client::GreetInProgressError::NonceMismatch => {
                GreetInProgressError::NonceMismatch
            }
            libparsec_client::GreetInProgressError::HumanHandleAlreadyTaken => {
                GreetInProgressError::HumanHandleAlreadyTaken
            }
            libparsec_client::GreetInProgressError::UserAlreadyExists => {
                GreetInProgressError::UserAlreadyExists
            }
            libparsec_client::GreetInProgressError::DeviceAlreadyExists => {
                GreetInProgressError::DeviceAlreadyExists
            }
            libparsec_client::GreetInProgressError::UserCreateNotAllowed => {
                GreetInProgressError::UserCreateNotAllowed
            }
            libparsec_client::GreetInProgressError::CorruptedInviteUserData(err) => {
                GreetInProgressError::CorruptedInviteUserData(err)
            }
            libparsec_client::GreetInProgressError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => GreetInProgressError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
            libparsec_client::GreetInProgressError::Internal(err) => {
                GreetInProgressError::Internal(err)
            }
        }
    }
}

pub struct UserGreetInitialInfo {
    pub handle: Handle,
}
pub struct DeviceGreetInitialInfo {
    pub handle: Handle,
}

pub async fn greeter_user_initial_do_wait_peer(
    canceller: Handle,
    handle: Handle,
) -> Result<UserGreetInProgress1Info, GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::UserGreetInitial(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_wait_peer().await?;
        let greeter_sas = ctx.greeter_sas().to_owned();

        let new_handle = register_handle(HandleItem::UserGreetInProgress1(ctx));

        Ok(UserGreetInProgress1Info {
            handle: new_handle,
            greeter_sas,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}

pub async fn greeter_device_initial_do_wait_peer(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceGreetInProgress1Info, GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::DeviceGreetInitial(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_wait_peer().await?;
        let greeter_sas = ctx.greeter_sas().to_owned();

        let new_handle = register_handle(HandleItem::DeviceGreetInProgress1(ctx));

        Ok(DeviceGreetInProgress1Info {
            handle: new_handle,
            greeter_sas,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}

pub struct UserGreetInProgress1Info {
    pub handle: Handle,
    pub greeter_sas: SASCode,
}

pub struct DeviceGreetInProgress1Info {
    pub handle: Handle,
    pub greeter_sas: SASCode,
}

pub async fn greeter_user_in_progress_1_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<UserGreetInProgress2Info, GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::UserGreetInProgress1(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_wait_peer_trust().await?;
        let claimer_sas = ctx.claimer_sas().to_owned();
        let claimer_sas_choices = ctx.generate_claimer_sas_choices(4);

        let new_handle = register_handle(HandleItem::UserGreetInProgress2(ctx));

        Ok(UserGreetInProgress2Info {
            handle: new_handle,
            claimer_sas,
            claimer_sas_choices,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}

pub async fn greeter_device_in_progress_1_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceGreetInProgress2Info, GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::DeviceGreetInProgress1(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_wait_peer_trust().await?;
        let claimer_sas = ctx.claimer_sas().to_owned();
        let claimer_sas_choices = ctx.generate_claimer_sas_choices(4);

        let new_handle = register_handle(HandleItem::DeviceGreetInProgress2(ctx));

        Ok(DeviceGreetInProgress2Info {
            handle: new_handle,
            claimer_sas,
            claimer_sas_choices,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}

pub struct UserGreetInProgress2Info {
    pub handle: Handle,
    pub claimer_sas: SASCode,
    pub claimer_sas_choices: Vec<SASCode>,
}

pub struct DeviceGreetInProgress2Info {
    pub handle: Handle,
    pub claimer_sas: SASCode,
    pub claimer_sas_choices: Vec<SASCode>,
}

pub async fn greeter_user_in_progress_2_do_signify_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<UserGreetInProgress3Info, GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::UserGreetInProgress2(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_signify_trust().await?;

        let new_handle = register_handle(HandleItem::UserGreetInProgress3(ctx));

        Ok(UserGreetInProgress3Info { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}

pub async fn greeter_device_in_progress_2_do_signify_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceGreetInProgress3Info, GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::DeviceGreetInProgress2(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_signify_trust().await?;

        let new_handle = register_handle(HandleItem::DeviceGreetInProgress3(ctx));

        Ok(DeviceGreetInProgress3Info { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}

pub struct UserGreetInProgress3Info {
    pub handle: Handle,
}

pub struct DeviceGreetInProgress3Info {
    pub handle: Handle,
}

pub async fn greeter_user_in_progress_3_do_get_claim_requests(
    canceller: Handle,
    handle: Handle,
) -> Result<UserGreetInProgress4Info, GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::UserGreetInProgress3(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_get_claim_requests().await?;
        let requested_human_handle = ctx.requested_human_handle.clone();
        let requested_device_label = ctx.requested_device_label.clone();

        let new_handle = register_handle(HandleItem::UserGreetInProgress4(ctx));

        Ok(UserGreetInProgress4Info {
            handle: new_handle,
            requested_human_handle,
            requested_device_label,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}

pub async fn greeter_device_in_progress_3_do_get_claim_requests(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceGreetInProgress4Info, GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::DeviceGreetInProgress3(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        let ctx = ctx.do_get_claim_requests().await?;
        let requested_device_label = ctx.requested_device_label.clone();

        let new_handle = register_handle(HandleItem::DeviceGreetInProgress4(ctx));

        Ok(DeviceGreetInProgress4Info {
            handle: new_handle,
            requested_device_label,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}

pub struct UserGreetInProgress4Info {
    pub handle: Handle,
    pub requested_device_label: DeviceLabel,
    pub requested_human_handle: HumanHandle,
}

pub struct DeviceGreetInProgress4Info {
    pub handle: Handle,
    pub requested_device_label: DeviceLabel,
}

pub async fn greeter_user_in_progress_4_do_create(
    canceller: Handle,
    handle: Handle,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    profile: UserProfile,
) -> Result<(), GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::UserGreetInProgress4(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        ctx.do_create_new_user(device_label, human_handle, profile)
            .await?;

        Ok(())
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}

pub async fn greeter_device_in_progress_4_do_create(
    canceller: Handle,
    handle: Handle,
    device_label: DeviceLabel,
) -> Result<(), GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match x {
            HandleItem::DeviceGreetInProgress4(ctx) => Ok(ctx),
            invalid => Err(invalid),
        })?;

        ctx.do_create_new_device(device_label).await?;

        Ok(())
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}
