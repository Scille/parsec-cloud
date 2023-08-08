// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

pub use libparsec_types::prelude::*;

use crate::{
    handle::{register_handle, take_and_close_handle, Handle, HandleItem},
    ClientConfig, ClientEvent, OnEventCallbackPlugged,
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
    #[error("Invalid bootstrap token")]
    InvalidToken,
    #[error("Bootstrap token already used")]
    AlreadyUsedToken,
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    BadTimestamp {
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
            libparsec_client::BootstrapOrganizationError::InvalidToken => {
                BootstrapOrganizationError::InvalidToken
            }
            libparsec_client::BootstrapOrganizationError::AlreadyUsedToken => {
                BootstrapOrganizationError::AlreadyUsedToken
            }
            libparsec_client::BootstrapOrganizationError::BadTimestamp {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => BootstrapOrganizationError::BadTimestamp {
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
    // On web we run on the JS runtime which is monothreaded, hence everything is !Send
    #[cfg(target_arch = "wasm32")] on_event_callback: Arc<dyn Fn(ClientEvent)>,

    bootstrap_organization_addr: BackendOrganizationBootstrapAddr,
    save_strategy: DeviceSaveStrategy,
    human_handle: Option<HumanHandle>,
    device_label: Option<DeviceLabel>,
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

pub use libparsec_client::{ClaimInProgressError, ClaimerRetrieveInfoError};

pub async fn claimer_retrieve_info(
    config: ClientConfig,

    // Access to the event bus is done through this callback.
    // Ad-hoc code should be added to the binding system to handle this (hence
    // why this is passed as a parameter instead of as part of `ClientConfig`:
    // we can have a simple `if func_name == "client_login"` that does a special
    // cooking of it last param.
    #[cfg(not(target_arch = "wasm32"))] _on_event_callback: Arc<dyn Fn(ClientEvent) + Send + Sync>,
    // On web we run on the JS runtime which is monothreaded, hence everything is !Send
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

pub enum UserOrDeviceClaimInitialInfo {
    User {
        handle: Handle,
        claimer_email: String,
        greeter_user_id: UserID,
        greeter_human_handle: Option<HumanHandle>,
    },
    Device {
        handle: Handle,
        greeter_user_id: UserID,
        greeter_human_handle: Option<HumanHandle>,
    },
}

pub async fn claimer_user_initial_ctx_do_wait_peer(
    handle: Handle,
) -> Result<UserClaimInProgress1Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match x {
        HandleItem::UserClaimInitial(ctx) => Some(ctx),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    let ctx = ctx.do_wait_peer().await?;
    let greeter_sas_choices = ctx.generate_greeter_sas_choices(4);
    let greeter_sas = ctx.greeter_sas().to_owned();

    let new_handle = register_handle(HandleItem::UserClaimInProgress1(ctx));

    Ok(UserClaimInProgress1Info {
        handle: new_handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

pub async fn claimer_device_initial_ctx_do_wait_peer(
    handle: Handle,
) -> Result<DeviceClaimInProgress1Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match x {
        HandleItem::DeviceClaimInitial(ctx) => Some(ctx),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    let ctx = ctx.do_wait_peer().await?;
    let greeter_sas_choices = ctx.generate_greeter_sas_choices(4);
    let greeter_sas = ctx.greeter_sas().to_owned();

    let new_handle = register_handle(HandleItem::DeviceClaimInProgress1(ctx));

    Ok(DeviceClaimInProgress1Info {
        handle: new_handle,
        greeter_sas,
        greeter_sas_choices,
    })
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

pub async fn claimer_user_in_progress_2_do_signify_trust(
    handle: Handle,
) -> Result<UserClaimInProgress2Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match x {
        HandleItem::UserClaimInProgress1(ctx) => Some(ctx),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    let ctx = ctx.do_signify_trust().await?;
    let claimer_sas = ctx.claimer_sas().to_owned();

    let new_handle = register_handle(HandleItem::UserClaimInProgress2(ctx));

    Ok(UserClaimInProgress2Info {
        handle: new_handle,
        claimer_sas,
    })
}

pub async fn claimer_device_in_progress_2_do_signify_trust(
    handle: Handle,
) -> Result<DeviceClaimInProgress2Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match x {
        HandleItem::DeviceClaimInProgress1(ctx) => Some(ctx),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    let ctx = ctx.do_signify_trust().await?;
    let claimer_sas = ctx.claimer_sas().to_owned();

    let new_handle = register_handle(HandleItem::DeviceClaimInProgress2(ctx));

    Ok(DeviceClaimInProgress2Info {
        handle: new_handle,
        claimer_sas,
    })
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
    handle: Handle,
) -> Result<UserClaimInProgress3Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match x {
        HandleItem::UserClaimInProgress2(ctx) => Some(ctx),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    let ctx = ctx.do_wait_peer_trust().await?;

    let new_handle = register_handle(HandleItem::UserClaimInProgress3(ctx));

    Ok(UserClaimInProgress3Info { handle: new_handle })
}

pub async fn claimer_device_in_progress_2_do_wait_peer_trust(
    handle: Handle,
) -> Result<DeviceClaimInProgress3Info, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match x {
        HandleItem::DeviceClaimInProgress2(ctx) => Some(ctx),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    let ctx = ctx.do_wait_peer_trust().await?;

    let new_handle = register_handle(HandleItem::DeviceClaimInProgress3(ctx));

    Ok(DeviceClaimInProgress3Info { handle: new_handle })
}

pub struct UserClaimInProgress3Info {
    pub handle: Handle,
}
pub struct DeviceClaimInProgress3Info {
    pub handle: Handle,
}

pub async fn claimer_user_in_progress_3_do_claim(
    handle: Handle,
    requested_device_label: Option<DeviceLabel>,
    requested_human_handle: Option<HumanHandle>,
) -> Result<UserClaimFinalizeInfo, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match x {
        HandleItem::UserClaimInProgress3(ctx) => Some(ctx),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    let ctx = ctx
        .do_claim_user(requested_device_label, requested_human_handle)
        .await?;

    let new_handle = register_handle(HandleItem::UserClaimFinalize(ctx));

    Ok(UserClaimFinalizeInfo { handle: new_handle })
}

pub async fn claimer_device_in_progress_3_do_claim(
    handle: Handle,
    requested_device_label: Option<DeviceLabel>,
) -> Result<DeviceClaimFinalizeInfo, ClaimInProgressError> {
    let ctx = take_and_close_handle(handle, |x| match x {
        HandleItem::DeviceClaimInProgress3(ctx) => Some(ctx),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    let ctx = ctx.do_claim_device(requested_device_label).await?;

    let new_handle = register_handle(HandleItem::DeviceClaimFinalize(ctx));

    Ok(DeviceClaimFinalizeInfo { handle: new_handle })
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
        HandleItem::UserClaimFinalize(ctx) => Some(ctx),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

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
        HandleItem::DeviceClaimFinalize(ctx) => Some(ctx),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    let access = {
        let key_file = ctx.get_default_key_file();
        save_strategy.into_access(key_file)
    };

    let available_device = ctx.save_local_device(&access).await?;

    Ok(available_device)
}
