// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, num::NonZeroU8, sync::Arc};

pub use libparsec_client::{
    ClientCancelInvitationError, ClientNewDeviceInvitationError,
    ClientNewShamirRecoveryInvitationError, ClientNewUserInvitationError,
    ClientStartShamirRecoveryInvitationGreetError, InvitationEmailSentStatus, ListInvitationsError,
    ShamirRecoveryClaimAddShareError, ShamirRecoveryClaimPickRecipientError,
    ShamirRecoveryClaimRecoverDeviceError, UserClaimInitialCtx,
};
use libparsec_platform_device_loader::{RemoteOperationServer, SaveDeviceError};
pub use libparsec_protocol::authenticated_cmds::latest::invite_list::InvitationCreatedBy as InviteListInvitationCreatedBy;
pub use libparsec_protocol::invited_cmds::latest::invite_info::{
    InvitationCreatedBy as InviteInfoInvitationCreatedBy, ShamirRecoveryRecipient,
    UserGreetingAdministrator, UserOnlineStatus,
};
use libparsec_types::prelude::*;

use crate::ParsecInvitationAddrAndRedirectionURL;
use crate::{
    handle::{borrow_from_handle, register_handle, take_and_close_handle, Handle, HandleItem},
    listen_canceller, AvailableDevice, ClientConfig, DeviceSaveStrategy, OnEventCallbackPlugged,
};

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
    #[error("Invalid sequester authority verify key: {0}")]
    InvalidSequesterAuthorityVerifyKey(anyhow::Error),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error("Cannot save device: no space is available")]
    SaveDeviceNoSpaceAvailable,
    #[error("Cannot save device: invalid path: {0}")]
    SaveDeviceInvalidPath(anyhow::Error),
    #[error("Cannot save device: no response from {server} server: {error}")]
    // We don't use `ConnectionError` here since this type only corresponds to
    // an answer from the Parsec server and here any arbitrary server may have
    // been (unsuccessfully) requested (e.g. OpenBao server).
    SaveDeviceRemoteOpaqueKeyUploadOffline {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    /// Note only a subset of save strategies requires server access to
    /// upload an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("Cannot save device: {server} server opaque key upload failed: {error}")]
    SaveDeviceRemoteOpaqueKeyUploadFailed {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
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
    bootstrap_organization_addr: ParsecOrganizationBootstrapAddr,
    save_strategy: DeviceSaveStrategy,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    // Note we do the validation code parsing directly within the function. This is because:
    // - The authority key is expected to come from an arbitrary file provided by the user,
    //   so having incorrect data is a to-be-expected error.
    // - Using a `SequesterVerifyKeyDer` as parameter would mean the parsing is done in the
    //   bindings, where a Javascript type error is thrown, however this behavior is only
    //   supposed to occur for unexpected errors (e.g. passing a dummy value as DeviceID,
    //   since the GUI is only supposed to pass a DevicesID that have previously been
    //   obtained from libparsec).
    sequester_authority_verify_key_pem: Option<&str>,
) -> Result<AvailableDevice, BootstrapOrganizationError> {
    let save_strategy = save_strategy.convert_with_side_effects()?;

    let sequester_authority_verify_key = match sequester_authority_verify_key_pem {
        None => None,
        Some(raw) => {
            let key = SequesterVerifyKeyDer::load_pem(raw).map_err(|e| {
                BootstrapOrganizationError::InvalidSequesterAuthorityVerifyKey(e.into())
            })?;
            Some(key)
        }
    };

    let config: Arc<libparsec_client::ClientConfig> = config.into();
    let on_event_callback = super::get_on_event_callback();
    let events_plugged = OnEventCallbackPlugged::new(
        // Pass invalid handle, since it's not needed by the possible raised events during a bootstrap
        Handle::from(0u32),
        on_event_callback,
    );
    // TODO: connect event_bus to on_event_callback

    let finalize_ctx = {
        let org_id = bootstrap_organization_addr.organization_id().clone();
        libparsec_client::bootstrap_organization(
            config.clone(),
            events_plugged.event_bus,
            bootstrap_organization_addr,
            human_handle,
            device_label,
            sequester_authority_verify_key,
        )
        .await
        .inspect(|_ctx| log::debug!("Organization {org_id} is now bootstrapped"))
        .inspect_err(|e| log::error!("Failed to bootstrap organization {org_id}: {e}"))
    }?;

    let key_file = libparsec_platform_device_loader::get_default_key_file(
        &config.config_dir,
        finalize_ctx.new_local_device.device_id,
    );

    let available_device = finalize_ctx
        .save_local_device(&save_strategy, &key_file)
        .await
        .inspect_err(|e| log::error!("Error while saving device: {e}"))
        .map_err(|err| match err {
            SaveDeviceError::RemoteOpaqueKeyUploadOffline { server, error } => {
                BootstrapOrganizationError::SaveDeviceRemoteOpaqueKeyUploadOffline { server, error }
            }
            SaveDeviceError::RemoteOpaqueKeyUploadFailed { server, error } => {
                BootstrapOrganizationError::SaveDeviceRemoteOpaqueKeyUploadFailed { server, error }
            }
            SaveDeviceError::Internal(err) => {
                BootstrapOrganizationError::Internal(err.context("Cannot save device"))
            }
            SaveDeviceError::NoSpaceAvailable => {
                BootstrapOrganizationError::SaveDeviceNoSpaceAvailable
            }
            SaveDeviceError::InvalidPath => {
                BootstrapOrganizationError::SaveDeviceInvalidPath(anyhow::anyhow!("invalid path"))
            }
        })?;

    Ok(available_device)
}

/*
 * Invitation claimer
 */

pub use libparsec_client::ClaimerRetrieveInfoError;
pub use libparsec_client::{ClaimFinalizeError, ClaimInProgressError};

pub async fn claimer_retrieve_info(
    config: ClientConfig,
    addr: ParsecInvitationAddr,
) -> Result<AnyClaimRetrievedInfo, ClaimerRetrieveInfoError> {
    let config: Arc<libparsec_client::ClientConfig> = config.into();
    // TODO
    // let on_event_callback = super::get_on_event_callback();
    // let events_plugged = Arc::new(OnEventCallbackPlugged::new(on_event_callback));
    let ctx = libparsec_client::claimer_retrieve_info(config, addr, None).await?;

    match ctx {
        libparsec_client::AnyClaimRetrievedInfoCtx::User(ctx) => {
            let claimer_email = ctx.claimer_email().to_owned();
            let created_by = ctx.created_by().to_owned();
            let administrators = ctx.administrators().to_owned();
            let preferred_greeter = ctx.preferred_greeter().to_owned();
            let handle = register_handle(HandleItem::UserClaimListAdministrators(ctx));
            Ok(AnyClaimRetrievedInfo::User {
                handle,
                claimer_email,
                created_by,
                administrators,
                preferred_greeter,
            })
        }
        libparsec_client::AnyClaimRetrievedInfoCtx::Device(ctx) => {
            let greeter_user_id = ctx.greeter_user_id().to_owned();
            let greeter_human_handle = ctx.greeter_human_handle().to_owned();
            let handle = register_handle(HandleItem::DeviceClaimInitial(ctx));
            Ok(AnyClaimRetrievedInfo::Device {
                handle,
                greeter_user_id,
                greeter_human_handle,
            })
        }
        libparsec_client::AnyClaimRetrievedInfoCtx::ShamirRecovery(ctx) => {
            let claimer_user_id = ctx.claimer_user_id().to_owned();
            let claimer_human_handle = ctx.claimer_human_handle().to_owned();
            let invitation_created_by = ctx.invitation_created_by().to_owned();
            let shamir_recovery_created_on = ctx.shamir_recovery_created_on().to_owned();
            let recipients = ctx.recipients().to_owned();
            let threshold = ctx.threshold().to_owned();
            let is_recoverable = ctx.is_recoverable();
            let handle = register_handle(HandleItem::ShamirRecoveryClaimPickRecipient(ctx));
            Ok(AnyClaimRetrievedInfo::ShamirRecovery {
                handle,
                claimer_user_id,
                claimer_human_handle,
                invitation_created_by,
                shamir_recovery_created_on,
                recipients,
                threshold,
                is_recoverable,
            })
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClaimerGreeterAbortOperationError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

enum EitherCancellerCtx {
    GreetCancellerCtx(libparsec_client::GreetCancellerCtx),
    ClaimCancellerCtx(libparsec_client::ClaimCancellerCtx),
}

pub async fn claimer_greeter_abort_operation(
    handle: Handle,
) -> Result<(), ClaimerGreeterAbortOperationError> {
    let result = take_and_close_handle(handle, |x| match *x {
        HandleItem::ShamirRecoveryClaimPickRecipient(_) => Ok(None),
        HandleItem::ShamirRecoveryClaimShare(_) => Ok(None),
        HandleItem::ShamirRecoveryClaimRecoverDevice(_) => Ok(None),
        // Nothing to cancel for inital ctxs since the `wait_peer` operation has not started yet
        HandleItem::UserClaimInitial(_) => Ok(None),
        HandleItem::DeviceClaimInitial(_) => Ok(None),
        HandleItem::ShamirRecoveryClaimInitial(_) => Ok(None),
        HandleItem::UserClaimInProgress1(x) => Ok(Some(EitherCancellerCtx::ClaimCancellerCtx(
            x.canceller_ctx(),
        ))),
        HandleItem::DeviceClaimInProgress1(x) => Ok(Some(EitherCancellerCtx::ClaimCancellerCtx(
            x.canceller_ctx(),
        ))),
        HandleItem::ShamirRecoveryClaimInProgress1(x) => Ok(Some(
            EitherCancellerCtx::ClaimCancellerCtx(x.canceller_ctx()),
        )),
        HandleItem::UserClaimInProgress2(x) => Ok(Some(EitherCancellerCtx::ClaimCancellerCtx(
            x.canceller_ctx(),
        ))),
        HandleItem::DeviceClaimInProgress2(x) => Ok(Some(EitherCancellerCtx::ClaimCancellerCtx(
            x.canceller_ctx(),
        ))),
        HandleItem::ShamirRecoveryClaimInProgress2(x) => Ok(Some(
            EitherCancellerCtx::ClaimCancellerCtx(x.canceller_ctx()),
        )),
        HandleItem::UserClaimInProgress3(x) => Ok(Some(EitherCancellerCtx::ClaimCancellerCtx(
            x.canceller_ctx(),
        ))),
        HandleItem::DeviceClaimInProgress3(x) => Ok(Some(EitherCancellerCtx::ClaimCancellerCtx(
            x.canceller_ctx(),
        ))),
        HandleItem::ShamirRecoveryClaimInProgress3(x) => Ok(Some(
            EitherCancellerCtx::ClaimCancellerCtx(x.canceller_ctx()),
        )),
        HandleItem::UserClaimFinalize(_) => Ok(None),
        HandleItem::DeviceClaimFinalize(_) => Ok(None),
        HandleItem::ShamirRecoveryClaimFinalize(_) => Ok(None),
        // Nothing to cancel for inital ctxs since the `wait_peer` operation has not started yet
        HandleItem::UserGreetInitial(_) => Ok(None),
        HandleItem::DeviceGreetInitial(_) => Ok(None),
        HandleItem::ShamirRecoveryGreetInitial(_) => Ok(None),
        HandleItem::UserGreetInProgress1(x) => Ok(Some(EitherCancellerCtx::GreetCancellerCtx(
            x.canceller_ctx(),
        ))),
        HandleItem::DeviceGreetInProgress1(x) => Ok(Some(EitherCancellerCtx::GreetCancellerCtx(
            x.canceller_ctx(),
        ))),
        HandleItem::ShamirRecoveryGreetInProgress1(x) => Ok(Some(
            EitherCancellerCtx::GreetCancellerCtx(x.canceller_ctx()),
        )),
        HandleItem::UserGreetInProgress2(x) => Ok(Some(EitherCancellerCtx::GreetCancellerCtx(
            x.canceller_ctx(),
        ))),
        HandleItem::DeviceGreetInProgress2(x) => Ok(Some(EitherCancellerCtx::GreetCancellerCtx(
            x.canceller_ctx(),
        ))),
        HandleItem::ShamirRecoveryGreetInProgress2(x) => Ok(Some(
            EitherCancellerCtx::GreetCancellerCtx(x.canceller_ctx()),
        )),
        HandleItem::UserGreetInProgress3(x) => Ok(Some(EitherCancellerCtx::GreetCancellerCtx(
            x.canceller_ctx(),
        ))),
        HandleItem::DeviceGreetInProgress3(x) => Ok(Some(EitherCancellerCtx::GreetCancellerCtx(
            x.canceller_ctx(),
        ))),
        HandleItem::ShamirRecoveryGreetInProgress3(x) => Ok(Some(
            EitherCancellerCtx::GreetCancellerCtx(x.canceller_ctx()),
        )),
        HandleItem::UserGreetInProgress4(x) => Ok(Some(EitherCancellerCtx::GreetCancellerCtx(
            x.canceller_ctx(),
        ))),
        HandleItem::DeviceGreetInProgress4(x) => Ok(Some(EitherCancellerCtx::GreetCancellerCtx(
            x.canceller_ctx(),
        ))),
        _ => Err(x),
    });
    match result {
        Ok(None) => Ok(()),
        Ok(Some(EitherCancellerCtx::GreetCancellerCtx(ctx))) => ctx.cancel().await.map_err(|x| {
            anyhow::anyhow!("Error while cancelling the greeting attempt: {:?}", x).into()
        }),
        Ok(Some(EitherCancellerCtx::ClaimCancellerCtx(ctx))) => ctx.cancel().await.map_err(|x| {
            anyhow::anyhow!("Error while cancelling the greeting attempt: {:?}", x).into()
        }),
        Err(invalid) => Err(invalid).map_err(|x| x.into()),
    }
}

pub enum AnyClaimRetrievedInfo {
    User {
        handle: Handle,
        claimer_email: EmailAddress,
        created_by: InviteInfoInvitationCreatedBy,
        administrators: Vec<UserGreetingAdministrator>,
        preferred_greeter: Option<UserGreetingAdministrator>,
    },
    Device {
        handle: Handle,
        greeter_user_id: UserID,
        greeter_human_handle: HumanHandle,
    },
    ShamirRecovery {
        handle: Handle,
        claimer_user_id: UserID,
        claimer_human_handle: HumanHandle,
        invitation_created_by: InviteInfoInvitationCreatedBy,
        shamir_recovery_created_on: DateTime,
        recipients: Vec<ShamirRecoveryRecipient>,
        threshold: NonZeroU8,
        is_recoverable: bool,
    },
}

pub struct UserClaimInitialInfo {
    pub handle: Handle,
    pub greeter_user_id: UserID,
    pub greeter_human_handle: HumanHandle,
    pub online_status: UserOnlineStatus,
    pub last_greeting_attempt_joined_on: Option<DateTime>,
}

pub async fn claimer_user_wait_all_peers(
    canceller: Handle,
    handle: Handle,
) -> Result<UserClaimInProgress1Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::UserClaimListAdministrators(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let initial_ctxs = ctx.list_initial_ctxs();

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;

    let ctx = UserClaimInitialCtx::do_wait_multiple_peer_with_canceller_event(
        initial_ctxs,
        cancel_requested,
    )
    .await?;
    let greeter_sas_choices = ctx.generate_greeter_sas_choices(4);
    let greeter_sas = ctx.greeter_sas().to_owned();
    let greeter_user_id = ctx.greeter_user_id().to_owned();
    let greeter_human_handle = ctx.greeter_human_handle().to_owned();

    let new_handle = register_handle(HandleItem::UserClaimInProgress1(ctx));

    Ok(UserClaimInProgress1Info {
        handle: new_handle,
        greeter_user_id,
        greeter_human_handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

#[derive(Debug, thiserror::Error)]
pub enum UserClaimListInitialInfosError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub fn claimer_user_list_initial_info(
    handle: Handle,
) -> Result<Vec<UserClaimInitialInfo>, UserClaimListInitialInfosError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::UserClaimListAdministrators(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    Ok(ctx
        .list_initial_ctxs()
        .into_iter()
        .map(|ctx| {
            let greeter_user_id = ctx.greeter_user_id().to_owned();
            let greeter_human_handle = ctx.greeter_human_handle().to_owned();
            let online_status = ctx.online_status().to_owned();
            let last_greeting_attempt_joined_on = ctx.last_greeting_attempt_joined_on().to_owned();
            let new_handle = register_handle(HandleItem::UserClaimInitial(ctx));
            UserClaimInitialInfo {
                handle: new_handle,
                greeter_user_id,
                greeter_human_handle,
                online_status,
                last_greeting_attempt_joined_on,
            }
        })
        .collect())
}

pub struct ShamirRecoveryClaimInitialInfo {
    pub handle: Handle,
    pub greeter_user_id: UserID,
    pub greeter_human_handle: HumanHandle,
}

pub fn claimer_shamir_recovery_pick_recipient(
    handle: Handle,
    recipient: UserID,
) -> Result<ShamirRecoveryClaimInitialInfo, ShamirRecoveryClaimPickRecipientError> {
    let ctx = borrow_from_handle(handle, |x| match x {
        HandleItem::ShamirRecoveryClaimPickRecipient(ctx) => Some(ctx.pick_recipient(recipient)),
        _ => None,
    })??;
    let greeter_user_id = ctx.greeter_user_id().to_owned();
    let greeter_human_handle = ctx.greeter_human_handle().to_owned();
    let new_handle = register_handle(HandleItem::ShamirRecoveryClaimInitial(ctx));

    Ok(ShamirRecoveryClaimInitialInfo {
        handle: new_handle,
        greeter_user_id,
        greeter_human_handle,
    })
}

pub async fn claimer_user_initial_do_wait_peer(
    canceller: Handle,
    handle: Handle,
) -> Result<UserClaimInProgress1Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::UserClaimInitial(ctx) => Ok(ctx),
        _ => Err(x),
    })?;

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    let ctx = ctx
        .do_wait_peer_with_canceller_event(cancel_requested)
        .await?;
    let greeter_user_id = ctx.greeter_user_id().to_owned();
    let greeter_human_handle = ctx.greeter_human_handle().to_owned();
    let greeter_sas_choices = ctx.generate_greeter_sas_choices(4);
    let greeter_sas = ctx.greeter_sas().to_owned();

    let new_handle = register_handle(HandleItem::UserClaimInProgress1(ctx));

    Ok(UserClaimInProgress1Info {
        handle: new_handle,
        greeter_user_id,
        greeter_human_handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

pub async fn claimer_device_initial_do_wait_peer(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceClaimInProgress1Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::DeviceClaimInitial(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    let ctx = ctx
        .do_wait_peer_with_canceller_event(cancel_requested)
        .await?;
    let greeter_user_id = ctx.greeter_user_id().to_owned();
    let greeter_human_handle = ctx.greeter_human_handle().to_owned();
    let greeter_sas_choices = ctx.generate_greeter_sas_choices(4);
    let greeter_sas = ctx.greeter_sas().to_owned();

    let new_handle = register_handle(HandleItem::DeviceClaimInProgress1(ctx));

    Ok(DeviceClaimInProgress1Info {
        handle: new_handle,
        greeter_user_id,
        greeter_human_handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

pub async fn claimer_shamir_recovery_initial_do_wait_peer(
    canceller: Handle,
    handle: Handle,
) -> Result<ShamirRecoveryClaimInProgress1Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::ShamirRecoveryClaimInitial(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    let ctx = ctx
        .do_wait_peer_with_canceller_event(cancel_requested)
        .await?;
    let greeter_user_id = ctx.greeter_user_id().to_owned();
    let greeter_human_handle = ctx.greeter_human_handle().to_owned();
    let greeter_sas_choices = ctx.generate_greeter_sas_choices(4);
    let greeter_sas = ctx.greeter_sas().to_owned();

    let new_handle = register_handle(HandleItem::ShamirRecoveryClaimInProgress1(ctx));

    Ok(ShamirRecoveryClaimInProgress1Info {
        handle: new_handle,
        greeter_user_id,
        greeter_human_handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

pub async fn claimer_user_in_progress_1_do_deny_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<(), ClaimInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match *x {
            HandleItem::UserClaimInProgress1(ctx) => Ok(ctx),
            _ => Err(x),
        })?;

        ctx.do_deny_trust().await?;
        Ok(())
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => Err(ClaimInProgressError::Cancelled),
    )
}

pub async fn claimer_device_in_progress_1_do_deny_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<(), ClaimInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match *x {
            HandleItem::DeviceClaimInProgress1(ctx) => Ok(ctx),
            _ => Err(x),
        })?;

        ctx.do_deny_trust().await?;
        Ok(())
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => Err(ClaimInProgressError::Cancelled),
    )
}

pub async fn claimer_shamir_recovery_in_progress_1_do_deny_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<(), ClaimInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match *x {
            HandleItem::ShamirRecoveryClaimInProgress1(ctx) => Ok(ctx),
            _ => Err(x),
        })?;

        ctx.do_deny_trust().await?;
        Ok(())
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => Err(ClaimInProgressError::Cancelled),
    )
}

pub struct UserClaimInProgress1Info {
    pub handle: Handle,
    pub greeter_user_id: UserID,
    pub greeter_human_handle: HumanHandle,
    pub greeter_sas: SASCode,
    pub greeter_sas_choices: Vec<SASCode>,
}
pub struct DeviceClaimInProgress1Info {
    pub handle: Handle,
    pub greeter_user_id: UserID,
    pub greeter_human_handle: HumanHandle,
    pub greeter_sas: SASCode,
    pub greeter_sas_choices: Vec<SASCode>,
}
pub struct ShamirRecoveryClaimInProgress1Info {
    pub handle: Handle,
    pub greeter_user_id: UserID,
    pub greeter_human_handle: HumanHandle,
    pub greeter_sas: SASCode,
    pub greeter_sas_choices: Vec<SASCode>,
}

pub async fn claimer_user_in_progress_1_do_signify_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<UserClaimInProgress2Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::UserClaimInProgress1(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx.do_signify_trust().await?;
        let claimer_sas = ctx.claimer_sas().to_owned();

        let new_handle = register_handle(HandleItem::UserClaimInProgress2(ctx));

        Ok(UserClaimInProgress2Info {
            handle: new_handle,
            claimer_sas,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(ClaimInProgressError::Cancelled)
        },
    )
}

pub async fn claimer_device_in_progress_1_do_signify_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceClaimInProgress2Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::DeviceClaimInProgress1(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx.do_signify_trust().await?;
        let claimer_sas = ctx.claimer_sas().to_owned();

        let new_handle = register_handle(HandleItem::DeviceClaimInProgress2(ctx));

        Ok(DeviceClaimInProgress2Info {
            handle: new_handle,
            claimer_sas,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(ClaimInProgressError::Cancelled)
        },
    )
}

pub async fn claimer_shamir_recovery_in_progress_1_do_signify_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<ShamirRecoveryClaimInProgress2Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::ShamirRecoveryClaimInProgress1(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx.do_signify_trust().await?;
        let claimer_sas = ctx.claimer_sas().to_owned();

        let new_handle = register_handle(HandleItem::ShamirRecoveryClaimInProgress2(ctx));

        Ok(ShamirRecoveryClaimInProgress2Info {
            handle: new_handle,
            claimer_sas,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(ClaimInProgressError::Cancelled)
        },
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
pub struct ShamirRecoveryClaimInProgress2Info {
    pub handle: Handle,
    pub claimer_sas: SASCode,
}

pub async fn claimer_user_in_progress_2_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<UserClaimInProgress3Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::UserClaimInProgress2(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx.do_wait_peer_trust().await?;

        let new_handle = register_handle(HandleItem::UserClaimInProgress3(ctx));

        Ok(UserClaimInProgress3Info { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(ClaimInProgressError::Cancelled)
        },
    )
}

pub async fn claimer_device_in_progress_2_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceClaimInProgress3Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::DeviceClaimInProgress2(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx.do_wait_peer_trust().await?;

        let new_handle = register_handle(HandleItem::DeviceClaimInProgress3(ctx));

        Ok(DeviceClaimInProgress3Info { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(ClaimInProgressError::Cancelled)
        },
    )
}

pub async fn claimer_shamir_recovery_in_progress_2_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<ShamirRecoveryClaimInProgress3Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::ShamirRecoveryClaimInProgress2(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx.do_wait_peer_trust().await?;

        let new_handle = register_handle(HandleItem::ShamirRecoveryClaimInProgress3(ctx));

        Ok(ShamirRecoveryClaimInProgress3Info { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(ClaimInProgressError::Cancelled)
        },
    )
}

pub struct UserClaimInProgress3Info {
    pub handle: Handle,
}
pub struct DeviceClaimInProgress3Info {
    pub handle: Handle,
}
pub struct ShamirRecoveryClaimInProgress3Info {
    pub handle: Handle,
}

pub async fn claimer_user_in_progress_3_do_claim(
    canceller: Handle,
    handle: Handle,
    requested_device_label: DeviceLabel,
    requested_human_handle: HumanHandle,
) -> Result<UserClaimFinalizeInfo, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::UserClaimInProgress3(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx
            .do_claim_user(requested_device_label, requested_human_handle)
            .await?;

        let new_handle = register_handle(HandleItem::UserClaimFinalize(ctx));

        Ok(UserClaimFinalizeInfo { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(ClaimInProgressError::Cancelled)
        },
    )
}

pub async fn claimer_device_in_progress_3_do_claim(
    canceller: Handle,
    handle: Handle,
    requested_device_label: DeviceLabel,
) -> Result<DeviceClaimFinalizeInfo, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::DeviceClaimInProgress3(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx.do_claim_device(requested_device_label).await?;

        let new_handle = register_handle(HandleItem::DeviceClaimFinalize(ctx));

        Ok(DeviceClaimFinalizeInfo { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(ClaimInProgressError::Cancelled)
        },
    )
}

pub async fn claimer_shamir_recovery_in_progress_3_do_claim(
    canceller: Handle,
    handle: Handle,
) -> Result<ShamirRecoveryClaimShareInfo, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::ShamirRecoveryClaimInProgress3(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx.do_recover_share().await?;

        let new_handle = register_handle(HandleItem::ShamirRecoveryClaimShare(ctx));

        Ok(ShamirRecoveryClaimShareInfo { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(ClaimInProgressError::Cancelled)
        },
    )
}

pub struct UserClaimFinalizeInfo {
    pub handle: Handle,
}
pub struct DeviceClaimFinalizeInfo {
    pub handle: Handle,
}

pub struct ShamirRecoveryClaimShareInfo {
    pub handle: Handle,
}

pub enum ShamirRecoveryClaimMaybeRecoverDeviceInfo {
    PickRecipient {
        handle: Handle,
        claimer_user_id: UserID,
        claimer_human_handle: HumanHandle,
        shamir_recovery_created_on: DateTime,
        recipients: Vec<ShamirRecoveryRecipient>,
        threshold: NonZeroU8,
        recovered_shares: HashMap<UserID, NonZeroU8>,
        is_recoverable: bool,
    },
    RecoverDevice {
        handle: Handle,
        claimer_user_id: UserID,
        claimer_human_handle: HumanHandle,
    },
}

pub enum ShamirRecoveryClaimMaybeFinalizeInfo {
    Offline { handle: Handle },
    Finalize { handle: Handle },
}

pub fn claimer_shamir_recovery_add_share(
    pick_recipient_handle: Handle,
    share_handle: Handle,
) -> Result<ShamirRecoveryClaimMaybeRecoverDeviceInfo, ShamirRecoveryClaimAddShareError> {
    let ctx = take_and_close_handle(pick_recipient_handle, |x| match *x {
        HandleItem::ShamirRecoveryClaimPickRecipient(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let share = take_and_close_handle(share_handle, |x| match *x {
        HandleItem::ShamirRecoveryClaimShare(ctx) => Ok(ctx),
        _ => Err(x),
    })?;

    match ctx.add_share(share)? {
        libparsec_client::ShamirRecoveryClaimMaybeRecoverDeviceCtx::PickRecipient(ctx) => {
            let claimer_user_id = ctx.claimer_user_id().to_owned();
            let claimer_human_handle = ctx.claimer_human_handle().to_owned();
            let shamir_recovery_created_on = ctx.shamir_recovery_created_on().to_owned();
            let recipients = ctx.recipients().to_owned();
            let threshold = ctx.threshold().to_owned();
            let recovered_shares = ctx.retrieved_shares().to_owned();
            let is_recoverable = ctx.is_recoverable();
            let new_handle = register_handle(HandleItem::ShamirRecoveryClaimPickRecipient(ctx));
            Ok(ShamirRecoveryClaimMaybeRecoverDeviceInfo::PickRecipient {
                handle: new_handle,
                claimer_user_id,
                claimer_human_handle,
                shamir_recovery_created_on,
                recipients,
                threshold,
                recovered_shares,
                is_recoverable,
            })
        }
        libparsec_client::ShamirRecoveryClaimMaybeRecoverDeviceCtx::RecoverDevice(ctx) => {
            let claimer_user_id = ctx.claimer_user_id().to_owned();
            let claimer_human_handle = ctx.claimer_human_handle().to_owned();
            let new_handle = register_handle(HandleItem::ShamirRecoveryClaimRecoverDevice(ctx));
            Ok(ShamirRecoveryClaimMaybeRecoverDeviceInfo::RecoverDevice {
                handle: new_handle,
                claimer_user_id,
                claimer_human_handle,
            })
        }
    }
}

pub async fn claimer_shamir_recovery_recover_device(
    handle: Handle,
    requested_device_label: DeviceLabel,
) -> Result<ShamirRecoveryClaimMaybeFinalizeInfo, ShamirRecoveryClaimRecoverDeviceError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::ShamirRecoveryClaimRecoverDevice(ctx) => Ok(ctx),
        _ => Err(x),
    })?;

    match ctx.recover_device(requested_device_label).await? {
        libparsec_client::ShamirRecoveryClaimMaybeFinalizeCtx::Offline(ctx) => {
            let new_handle = register_handle(HandleItem::ShamirRecoveryClaimRecoverDevice(ctx));
            Ok(ShamirRecoveryClaimMaybeFinalizeInfo::Offline { handle: new_handle })
        }
        libparsec_client::ShamirRecoveryClaimMaybeFinalizeCtx::Finalize(ctx) => {
            let new_handle = register_handle(HandleItem::ShamirRecoveryClaimFinalize(ctx));
            Ok(ShamirRecoveryClaimMaybeFinalizeInfo::Finalize { handle: new_handle })
        }
    }
}

pub async fn claimer_user_finalize_save_local_device(
    handle: Handle,
    save_strategy: DeviceSaveStrategy,
) -> Result<AvailableDevice, ClaimFinalizeError> {
    let save_strategy = save_strategy
        .convert_with_side_effects()
        .map_err(ClaimFinalizeError::Internal)?;

    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::UserClaimFinalize(ctx) => Ok(ctx),
        _ => Err(x),
    })
    .map_err(ClaimFinalizeError::Internal)?;

    let key_file = ctx.get_default_key_file();

    let available_device = ctx.save_local_device(&save_strategy, &key_file).await?;

    Ok(available_device)
}

pub async fn claimer_device_finalize_save_local_device(
    handle: Handle,
    save_strategy: DeviceSaveStrategy,
) -> Result<AvailableDevice, ClaimFinalizeError> {
    let save_strategy = save_strategy
        .convert_with_side_effects()
        .map_err(ClaimFinalizeError::Internal)?;

    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::DeviceClaimFinalize(ctx) => Ok(ctx),
        _ => Err(x),
    })
    .map_err(ClaimFinalizeError::Internal)?;

    let key_file = ctx.get_default_key_file();

    let available_device = ctx.save_local_device(&save_strategy, &key_file).await?;

    Ok(available_device)
}

pub async fn claimer_shamir_recovery_finalize_save_local_device(
    handle: Handle,
    save_strategy: DeviceSaveStrategy,
) -> Result<AvailableDevice, ClaimFinalizeError> {
    let save_strategy = save_strategy
        .convert_with_side_effects()
        .map_err(ClaimFinalizeError::Internal)?;

    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::ShamirRecoveryClaimFinalize(ctx) => Ok(ctx),
        _ => Err(x),
    })
    .map_err(ClaimFinalizeError::Internal)?;

    let key_file = ctx.get_default_key_file();

    let available_device = ctx.save_local_device(&save_strategy, &key_file).await?;

    Ok(available_device)
}

/*
 * Invitation greeter
 */

pub struct NewInvitationInfo {
    pub addr: ParsecInvitationAddrAndRedirectionURL,
    pub token: AccessToken,
    pub email_sent_status: InvitationEmailSentStatus,
}

pub async fn client_new_user_invitation(
    client: Handle,
    claimer_email: EmailAddress,
    send_email: bool,
) -> Result<NewInvitationInfo, ClientNewUserInvitationError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let (token, email_sent_status) = client
        .new_user_invitation(claimer_email, send_email)
        .await?;
    let addr = ParsecInvitationAddr::new(
        client.organization_addr(),
        client.organization_id().to_owned(),
        InvitationType::User,
        token,
    );
    Ok(NewInvitationInfo {
        addr: (addr.clone(), addr.to_http_redirection_url()),
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

    let addr = ParsecInvitationAddr::new(
        client.organization_addr(),
        client.organization_id().to_owned(),
        InvitationType::Device,
        token,
    );
    Ok(NewInvitationInfo {
        addr: (addr.clone(), addr.to_http_redirection_url()),
        token,
        email_sent_status,
    })
}

pub async fn client_new_shamir_recovery_invitation(
    client: Handle,
    claimer_user_id: UserID,
    send_email: bool,
) -> Result<NewInvitationInfo, ClientNewShamirRecoveryInvitationError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let (token, email_sent_status) = client
        .new_shamir_recovery_invitation(claimer_user_id, send_email)
        .await?;

    let addr = ParsecInvitationAddr::new(
        client.organization_addr(),
        client.organization_id().to_owned(),
        InvitationType::ShamirRecovery,
        token,
    );
    Ok(NewInvitationInfo {
        addr: (addr.clone(), addr.to_http_redirection_url()),
        token,
        email_sent_status,
    })
}

pub async fn client_cancel_invitation(
    client: Handle,
    token: AccessToken,
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
        addr: ParsecInvitationAddrAndRedirectionURL,
        token: AccessToken,
        created_on: DateTime,
        created_by: InviteListInvitationCreatedBy,
        claimer_email: EmailAddress,
        status: InvitationStatus,
    },
    Device {
        addr: ParsecInvitationAddrAndRedirectionURL,
        token: AccessToken,
        created_on: DateTime,
        created_by: InviteListInvitationCreatedBy,
        status: InvitationStatus,
    },
    ShamirRecovery {
        addr: ParsecInvitationAddrAndRedirectionURL,
        token: AccessToken,
        created_on: DateTime,
        created_by: InviteListInvitationCreatedBy,
        claimer_user_id: UserID,
        shamir_recovery_created_on: DateTime,
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
                created_by,
                status,
                token,
            } => {
                let addr = ParsecInvitationAddr::new(
                    client.organization_addr(),
                    client.organization_id().to_owned(),
                    InvitationType::User,
                    token,
                );
                InviteListItem::User {
                    addr: (addr.clone(), addr.to_http_redirection_url()),
                    claimer_email,
                    created_on,
                    created_by,
                    status,
                    token,
                }
            }
            libparsec_client::InviteListItem::Device {
                created_on,
                created_by,
                status,
                token,
            } => {
                let addr = ParsecInvitationAddr::new(
                    client.organization_addr(),
                    client.organization_id().to_owned(),
                    InvitationType::Device,
                    token,
                );
                InviteListItem::Device {
                    addr: (addr.clone(), addr.to_http_redirection_url()),
                    created_on,
                    created_by,
                    status,
                    token,
                }
            }
            libparsec_client::InviteListItem::ShamirRecovery {
                created_on,
                created_by,
                status,
                token,
                claimer_user_id,
                shamir_recovery_created_on,
            } => {
                let addr = ParsecInvitationAddr::new(
                    client.organization_addr(),
                    client.organization_id().to_owned(),
                    InvitationType::ShamirRecovery,
                    token,
                );
                InviteListItem::ShamirRecovery {
                    addr: (addr.clone(), addr.to_http_redirection_url()),
                    created_on,
                    created_by,
                    status,
                    token,
                    claimer_user_id,
                    shamir_recovery_created_on,
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
    token: AccessToken,
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
    token: AccessToken,
) -> Result<DeviceGreetInitialInfo, ClientStartInvitationGreetError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let ctx = client.start_device_invitation_greet(token);

    let handle = register_handle(HandleItem::DeviceGreetInitial(ctx));

    Ok(DeviceGreetInitialInfo { handle })
}

pub async fn client_start_shamir_recovery_invitation_greet(
    client: Handle,
    token: AccessToken,
) -> Result<ShamirRecoveryGreetInitialInfo, ClientStartShamirRecoveryInvitationGreetError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let ctx = client.start_shamir_recovery_invitation_greet(token).await?;

    let handle = register_handle(HandleItem::ShamirRecoveryGreetInitial(ctx));

    Ok(ShamirRecoveryGreetInitialInfo { handle })
}

pub use libparsec_client::GreetInProgressError;

pub struct UserGreetInitialInfo {
    pub handle: Handle,
}
pub struct DeviceGreetInitialInfo {
    pub handle: Handle,
}

pub struct ShamirRecoveryGreetInitialInfo {
    pub handle: Handle,
}

pub async fn greeter_user_initial_do_wait_peer(
    canceller: Handle,
    handle: Handle,
) -> Result<UserGreetInProgress1Info, GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match *x {
            HandleItem::UserGreetInitial(ctx) => Ok(ctx),
            _ => Err(x),
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
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}

pub async fn greeter_device_initial_do_wait_peer(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceGreetInProgress1Info, GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match *x {
            HandleItem::DeviceGreetInitial(ctx) => Ok(ctx),
            _ => Err(x),
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
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}

pub async fn greeter_shamir_recovery_initial_do_wait_peer(
    canceller: Handle,
    handle: Handle,
) -> Result<ShamirRecoveryGreetInProgress1Info, GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match *x {
            HandleItem::ShamirRecoveryGreetInitial(ctx) => Ok(ctx),
            _ => Err(x),
        })?;

        let ctx = ctx.do_wait_peer().await?;
        let greeter_sas = ctx.greeter_sas().to_owned();

        let new_handle = register_handle(HandleItem::ShamirRecoveryGreetInProgress1(ctx));

        Ok(ShamirRecoveryGreetInProgress1Info {
            handle: new_handle,
            greeter_sas,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
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

pub struct ShamirRecoveryGreetInProgress1Info {
    pub handle: Handle,
    pub greeter_sas: SASCode,
}

pub async fn greeter_user_in_progress_1_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<UserGreetInProgress2Info, GreetInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::UserGreetInProgress1(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
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
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(GreetInProgressError::Cancelled)
        },
    )
}

pub async fn greeter_device_in_progress_1_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceGreetInProgress2Info, GreetInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::DeviceGreetInProgress1(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
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
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(GreetInProgressError::Cancelled)
        },
    )
}

pub async fn greeter_shamir_recovery_in_progress_1_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<ShamirRecoveryGreetInProgress2Info, GreetInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::ShamirRecoveryGreetInProgress1(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx.do_wait_peer_trust().await?;
        let claimer_sas = ctx.claimer_sas().to_owned();
        let claimer_sas_choices = ctx.generate_claimer_sas_choices(4);

        let new_handle = register_handle(HandleItem::ShamirRecoveryGreetInProgress2(ctx));

        Ok(ShamirRecoveryGreetInProgress2Info {
            handle: new_handle,
            claimer_sas,
            claimer_sas_choices,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(GreetInProgressError::Cancelled)
        },
    )
}

pub async fn greeter_user_in_progress_2_do_deny_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<(), GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match *x {
            HandleItem::UserGreetInProgress2(ctx) => Ok(ctx),
            _ => Err(x),
        })?;

        ctx.do_deny_trust().await?;
        Ok(())
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}

pub async fn greeter_device_in_progress_2_do_deny_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<(), GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match *x {
            HandleItem::DeviceGreetInProgress2(ctx) => Ok(ctx),
            _ => Err(x),
        })?;

        ctx.do_deny_trust().await?;
        Ok(())
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => Err(GreetInProgressError::Cancelled),
    )
}

pub async fn greeter_shamir_recovery_in_progress_2_do_deny_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<(), GreetInProgressError> {
    let work = async {
        let ctx = take_and_close_handle(handle, |x| match *x {
            HandleItem::ShamirRecoveryGreetInProgress2(ctx) => Ok(ctx),
            _ => Err(x),
        })?;

        ctx.do_deny_trust().await?;
        Ok(())
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
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

pub struct ShamirRecoveryGreetInProgress2Info {
    pub handle: Handle,
    pub claimer_sas: SASCode,
    pub claimer_sas_choices: Vec<SASCode>,
}

pub async fn greeter_user_in_progress_2_do_signify_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<UserGreetInProgress3Info, GreetInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::UserGreetInProgress2(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx.do_signify_trust().await?;

        let new_handle = register_handle(HandleItem::UserGreetInProgress3(ctx));

        Ok(UserGreetInProgress3Info { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(GreetInProgressError::Cancelled)
        },
    )
}

pub async fn greeter_device_in_progress_2_do_signify_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceGreetInProgress3Info, GreetInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::DeviceGreetInProgress2(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx.do_signify_trust().await?;

        let new_handle = register_handle(HandleItem::DeviceGreetInProgress3(ctx));

        Ok(DeviceGreetInProgress3Info { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(GreetInProgressError::Cancelled)
        },
    )
}

pub async fn greeter_shamir_recovery_in_progress_2_do_signify_trust(
    canceller: Handle,
    handle: Handle,
) -> Result<ShamirRecoveryGreetInProgress3Info, GreetInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::ShamirRecoveryGreetInProgress2(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx.do_signify_trust().await?;

        let new_handle = register_handle(HandleItem::ShamirRecoveryGreetInProgress3(ctx));

        Ok(ShamirRecoveryGreetInProgress3Info { handle: new_handle })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(GreetInProgressError::Cancelled)
        },
    )
}

pub struct UserGreetInProgress3Info {
    pub handle: Handle,
}

pub struct DeviceGreetInProgress3Info {
    pub handle: Handle,
}

pub struct ShamirRecoveryGreetInProgress3Info {
    pub handle: Handle,
}

pub async fn greeter_user_in_progress_3_do_get_claim_requests(
    canceller: Handle,
    handle: Handle,
) -> Result<UserGreetInProgress4Info, GreetInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::UserGreetInProgress3(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
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
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(GreetInProgressError::Cancelled)
        },
    )
}

pub async fn greeter_device_in_progress_3_do_get_claim_requests(
    canceller: Handle,
    handle: Handle,
) -> Result<DeviceGreetInProgress4Info, GreetInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::DeviceGreetInProgress3(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        let ctx = ctx.do_get_claim_requests().await?;
        let requested_device_label = ctx.requested_device_label.clone();

        let new_handle = register_handle(HandleItem::DeviceGreetInProgress4(ctx));

        Ok(DeviceGreetInProgress4Info {
            handle: new_handle,
            requested_device_label,
        })
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(GreetInProgressError::Cancelled)
        },
    )
}

pub async fn greeter_shamir_recovery_in_progress_3_do_get_claim_requests(
    canceller: Handle,
    handle: Handle,
) -> Result<(), GreetInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::ShamirRecoveryGreetInProgress3(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        ctx.do_send_share().await?;
        Ok(())
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(GreetInProgressError::Cancelled)
        },
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
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::UserGreetInProgress4(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        ctx.do_create_new_user(device_label, human_handle, profile)
            .await?;

        Ok(())
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(GreetInProgressError::Cancelled)
        },
    )
}

pub async fn greeter_device_in_progress_4_do_create(
    canceller: Handle,
    handle: Handle,
    device_label: DeviceLabel,
) -> Result<(), GreetInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match *x {
        HandleItem::DeviceGreetInProgress4(ctx) => Ok(ctx),
        _ => Err(x),
    })?;
    let canceller_ctx = ctx.canceller_ctx();

    let work = async {
        ctx.do_create_new_device(device_label).await?;
        Ok(())
    };

    let (cancel_requested, _canceller_guard) = listen_canceller(canceller)?;
    libparsec_platform_async::select2_biased!(
        res = work => res,
        _ = cancel_requested => {
            canceller_ctx.cancel_and_warn_on_error().await;
            Err(GreetInProgressError::Cancelled)
        },
    )
}
