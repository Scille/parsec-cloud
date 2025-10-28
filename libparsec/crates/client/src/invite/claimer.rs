// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;
use std::num::NonZeroU8;
use std::path::Path;
use std::{path::PathBuf, sync::Arc};

use invited_cmds::latest::invite_claimer_step;
use libparsec_client_connection::AuthenticatedCmds;
use libparsec_client_connection::{protocol::invited_cmds, ConnectionError, InvitedCmds};
use libparsec_platform_async::event::{Event, EventListener};
use libparsec_platform_async::future::{join_all, select_all};
use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_platform_async::{select2_biased, spawn};
use libparsec_platform_device_loader::{AvailableDevice, DeviceSaveStrategy};
use libparsec_protocol::authenticated_cmds;
use libparsec_protocol::invited_cmds::latest::invite_info::{
    InvitationCreatedBy as InviteInfoInvitationCreatedBy, ShamirRecoveryRecipient,
    UserGreetingAdministrator, UserOnlineStatus,
};
use libparsec_types::prelude::*;

use crate::client::{register_new_device, RegisterNewDeviceError};
use crate::invite::common::{Throttle, WAIT_PEER_MAX_ATTEMPTS};
use crate::ClientConfig;

#[derive(Debug, thiserror::Error)]
pub enum ClaimerRetrieveInfoError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Invitation not found")]
    NotFound,
    #[error("Invitation already used or deleted")]
    AlreadyUsedOrDeleted,
    #[error("Organization has expired")]
    OrganizationExpired,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for ClaimerRetrieveInfoError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            ConnectionError::InvitationAlreadyUsedOrDeleted => Self::AlreadyUsedOrDeleted,
            ConnectionError::ExpiredOrganization => Self::OrganizationExpired,
            ConnectionError::BadAuthenticationInfo => Self::NotFound,
            err => Self::Internal(err.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClaimInProgressError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Organization has expired")]
    OrganizationExpired,
    #[error("Organization or invitation not found")]
    NotFound,
    #[error("Invitation already used or deleted")]
    AlreadyUsedOrDeleted,
    #[error("Claim operation reset by peer")]
    PeerReset,
    #[error("Active users limit reached")]
    ActiveUsersLimitReached,
    #[error("The provided user is not allowed to greet this invitation")]
    GreeterNotAllowed,
    #[error("Greeting attempt cancelled by {origin} because of {reason} on {timestamp}")]
    GreetingAttemptCancelled {
        origin: GreeterOrClaimer,
        reason: CancelledGreetingAttemptReason,
        timestamp: DateTime,
    },
    #[error(transparent)]
    CorruptedSharedSecretKey(CryptoError),
    #[error(transparent)]
    CorruptedConfirmation(DataError),
    #[error("Operation cancelled")]
    Cancelled,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for ClaimInProgressError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            ConnectionError::InvitationAlreadyUsedOrDeleted => Self::AlreadyUsedOrDeleted,
            ConnectionError::ExpiredOrganization => Self::OrganizationExpired,
            ConnectionError::BadAuthenticationInfo => Self::NotFound,
            err => Self::Internal(err.into()),
        }
    }
}

// Cancel greeting attempt helper

async fn cancel_greeting_attempt(
    cmds: &InvitedCmds,
    greeting_attempt: GreetingAttemptID,
    reason: CancelledGreetingAttemptReason,
) -> Result<(), ClaimInProgressError> {
    use invited_cmds::latest::invite_claimer_cancel_greeting_attempt::{Rep, Req};

    let req = Req {
        greeting_attempt,
        reason,
    };
    let rep = cmds.send(req).await?;

    match rep {
        Rep::Ok => Ok(()),
        // Expected errors
        Rep::GreeterNotAllowed => Err(ClaimInProgressError::GreeterNotAllowed),
        Rep::GreeterRevoked => Err(ClaimInProgressError::GreeterNotAllowed),
        Rep::GreetingAttemptAlreadyCancelled {
            origin,
            reason,
            timestamp,
        } => Err(ClaimInProgressError::GreetingAttemptCancelled {
            origin,
            reason,
            timestamp,
        }),
        // Unexpected errors
        Rep::UnknownStatus { .. }
        | Rep::GreetingAttemptNotFound
        | Rep::GreetingAttemptNotJoined => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    }
}

// Cancel the greeting attempt and log a warning if an error occurs.
// This is used when the peer has already detected an issue and tries to communicate
// the reason of this issue to the other peer. If this request fails, there is
// no reason to try again or deal with the error in a specific way. Instead, the caller
// simply propagates the error that originally caused the greeting attempt to be cancelled.
// The greeting attempt will then be automatically cancelled when a new one is started.
// Only the reason of the cancellation is lost, which is not a big deal as it was only
// meant to be informative and to improve the user experience.
async fn cancel_greeting_attempt_and_warn_on_error(
    cmds: &InvitedCmds,
    greeting_attempt: GreetingAttemptID,
    reason: CancelledGreetingAttemptReason,
) {
    if let Err(err) = cancel_greeting_attempt(cmds, greeting_attempt, reason).await {
        // Already cancelled, no need to log a warning
        if let ClaimInProgressError::GreetingAttemptCancelled { .. } = &err {
            return;
        }
        log::warn!(
            "Claimer failed to cancel greeting attempt {greeting_attempt:?} with reason {reason:?}: {err:?}"
        );
    }
}

// Claimer step helper

async fn _run_claimer_step(
    cmds: &InvitedCmds,
    req: invite_claimer_step::Req,
) -> Result<Option<invite_claimer_step::GreeterStep>, ClaimInProgressError> {
    let rep = cmds.send(req).await?;

    // Handle the response
    match rep {
        invite_claimer_step::Rep::NotReady => Ok(None),
        invite_claimer_step::Rep::Ok { greeter_step } => Ok(Some(greeter_step)),
        // Expected errors
        invite_claimer_step::Rep::GreeterNotAllowed => Err(ClaimInProgressError::GreeterNotAllowed),
        invite_claimer_step::Rep::GreeterRevoked => Err(ClaimInProgressError::GreeterNotAllowed),
        invite_claimer_step::Rep::GreetingAttemptCancelled {
            origin,
            reason,
            timestamp,
        } => Err(ClaimInProgressError::GreetingAttemptCancelled {
            origin,
            reason,
            timestamp,
        }),
        // Unexpected errors
        invite_claimer_step::Rep::UnknownStatus { .. }
        | invite_claimer_step::Rep::GreetingAttemptNotFound
        | invite_claimer_step::Rep::GreetingAttemptNotJoined
        | invite_claimer_step::Rep::StepMismatch
        | invite_claimer_step::Rep::StepTooAdvanced => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    }
}

async fn run_claimer_step(
    cmds: &InvitedCmds,
    greeting_attempt: GreetingAttemptID,
    claimer_step: invite_claimer_step::ClaimerStep,
) -> Result<Option<invite_claimer_step::GreeterStep>, ClaimInProgressError> {
    let req = invite_claimer_step::Req {
        greeting_attempt,
        claimer_step,
    };
    _run_claimer_step(cmds, req.clone()).await
}

async fn run_claimer_step_until_ready(
    cmds: &InvitedCmds,
    greeting_attempt: GreetingAttemptID,
    claimer_step: invite_claimer_step::ClaimerStep,
    time_provider: &TimeProvider,
) -> Result<invite_claimer_step::GreeterStep, ClaimInProgressError> {
    let mut throttle = Throttle::new(time_provider);
    let req = invite_claimer_step::Req {
        greeting_attempt,
        claimer_step,
    };

    // Loop over the requests
    loop {
        // Throttle the requests
        throttle.throttle().await;

        // Send the request
        let rep = _run_claimer_step(cmds, req.clone()).await?;

        // Handle the response
        return match rep {
            None => continue,
            Some(greeter_step) => Ok(greeter_step),
        };
    }
}

#[derive(Debug)]
pub enum AnyClaimRetrievedInfoCtx {
    User(UserClaimListAdministratorsCtx),
    Device(DeviceClaimInitialCtx),
    ShamirRecovery(ShamirRecoveryClaimPickRecipientCtx),
}

/// Retrieve information for the corresponding Parsec invitation address.
///
/// The optional time provider is used to throttle the requests, and will
/// become the time provider for the freshly created local device at the
/// end of the invitation process.
pub async fn claimer_retrieve_info(
    config: Arc<ClientConfig>,
    addr: ParsecInvitationAddr,
    time_provider: Option<TimeProvider>,
) -> Result<AnyClaimRetrievedInfoCtx, ClaimerRetrieveInfoError> {
    use invited_cmds::latest::invite_info::{InvitationType, Rep, Req};
    let time_provider = time_provider.unwrap_or_default();

    let cmds = Arc::new(
        InvitedCmds::new(&config.config_dir, addr, config.proxy.clone())
            .map_err(|e| anyhow::anyhow!("Error while configuring connection to server: {e}"))?,
    );

    let rep = cmds.send(Req).await?;

    match rep {
        Rep::Ok(claimer) => match claimer {
            InvitationType::User {
                claimer_email,
                created_by,
                administrators,
            } => Ok(AnyClaimRetrievedInfoCtx::User(
                UserClaimListAdministratorsCtx::new(
                    config,
                    cmds,
                    claimer_email,
                    created_by,
                    administrators,
                    time_provider,
                ),
            )),
            InvitationType::Device {
                claimer_user_id,
                claimer_human_handle,
                created_by: _created_by,
            } => Ok(AnyClaimRetrievedInfoCtx::Device(
                DeviceClaimInitialCtx::new(
                    config,
                    cmds,
                    claimer_user_id,
                    claimer_human_handle,
                    time_provider,
                ),
            )),
            InvitationType::ShamirRecovery {
                claimer_user_id,
                claimer_human_handle,
                created_by,
                shamir_recovery_created_on,
                recipients,
                threshold,
            } => Ok(AnyClaimRetrievedInfoCtx::ShamirRecovery(
                ShamirRecoveryClaimPickRecipientCtx {
                    config,
                    cmds,
                    claimer_user_id,
                    claimer_human_handle,
                    invitation_created_by: created_by,
                    shamir_recovery_created_on,
                    recipients,
                    threshold,
                    retrieved_shares: HashMap::new(),
                    time_provider,
                },
            )),
        },
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

// UserClaimListAdministratorsCtx

#[derive(Debug)]
pub struct UserClaimListAdministratorsCtx {
    config: Arc<ClientConfig>,
    cmds: Arc<InvitedCmds>,
    claimer_email: EmailAddress,
    created_by: InviteInfoInvitationCreatedBy,
    administrators: Vec<UserGreetingAdministrator>,
    time_provider: TimeProvider,
}

impl UserClaimListAdministratorsCtx {
    pub fn new(
        config: Arc<ClientConfig>,
        cmds: Arc<InvitedCmds>,
        claimer_email: EmailAddress,
        created_by: InviteInfoInvitationCreatedBy,
        administrators: Vec<UserGreetingAdministrator>,
        time_provider: TimeProvider,
    ) -> Self {
        Self {
            config,
            cmds,
            claimer_email,
            created_by,
            administrators,
            time_provider,
        }
    }

    pub fn claimer_email(&self) -> &EmailAddress {
        &self.claimer_email
    }

    pub fn created_by(&self) -> &InviteInfoInvitationCreatedBy {
        &self.created_by
    }

    pub fn administrators(&self) -> &[UserGreetingAdministrator] {
        &self.administrators
    }

    pub fn preferred_greeter(&self) -> Option<UserGreetingAdministrator> {
        match self.created_by {
            InviteInfoInvitationCreatedBy::ExternalService { .. } => None,
            InviteInfoInvitationCreatedBy::User { user_id, .. } => self
                .administrators
                .iter()
                .find(|administrator| administrator.user_id == user_id)
                .cloned(),
        }
    }

    pub fn list_initial_ctxs(self) -> Vec<UserClaimInitialCtx> {
        self.administrators
            .iter()
            .map(|administrator| {
                UserClaimInitialCtx::new(
                    self.config.clone(),
                    self.cmds.clone(),
                    self.claimer_email.clone(),
                    administrator.clone(),
                    self.time_provider.clone(),
                )
            })
            .collect()
    }
}

// ShamirRecoveryClaimPickRecipientCtx

#[derive(Debug, thiserror::Error)]
pub enum ShamirRecoveryClaimPickRecipientError {
    #[error("Recipient not found")]
    RecipientNotFound,
    #[error("Recipient already picked")]
    RecipientAlreadyPicked,
    #[error("Recipient revoked")]
    RecipientRevoked,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub enum ShamirRecoveryClaimMaybeRecoverDeviceCtx {
    RecoverDevice(ShamirRecoveryClaimRecoverDeviceCtx),
    PickRecipient(ShamirRecoveryClaimPickRecipientCtx),
}

#[derive(Debug, thiserror::Error)]
pub enum ShamirRecoveryClaimAddShareError {
    #[error("Recipient not found")]
    RecipientNotFound,
    #[error(transparent)]
    CorruptedSecret(DataError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct ShamirRecoveryClaimPickRecipientCtx {
    config: Arc<ClientConfig>,
    cmds: Arc<InvitedCmds>,
    claimer_user_id: UserID,
    claimer_human_handle: HumanHandle,
    invitation_created_by: InviteInfoInvitationCreatedBy,
    shamir_recovery_created_on: DateTime,
    recipients: Vec<ShamirRecoveryRecipient>,
    threshold: NonZeroU8,
    retrieved_shares: HashMap<UserID, Vec<ShamirShare>>,
    time_provider: TimeProvider,
}

impl ShamirRecoveryClaimPickRecipientCtx {
    pub fn recipients(&self) -> &[ShamirRecoveryRecipient] {
        &self.recipients
    }

    pub fn threshold(&self) -> NonZeroU8 {
        self.threshold
    }

    pub fn claimer_user_id(&self) -> UserID {
        self.claimer_user_id
    }

    pub fn claimer_human_handle(&self) -> &HumanHandle {
        &self.claimer_human_handle
    }

    pub fn invitation_created_by(&self) -> &InviteInfoInvitationCreatedBy {
        &self.invitation_created_by
    }

    pub fn shamir_recovery_created_on(&self) -> DateTime {
        self.shamir_recovery_created_on
    }

    pub fn recipients_without_a_share(&self) -> Vec<&ShamirRecoveryRecipient> {
        self.recipients
            .iter()
            .filter(|r| !self.retrieved_shares.contains_key(&r.user_id))
            .collect()
    }

    pub fn is_recoverable(&self) -> bool {
        self.recipients
            .iter()
            .filter(|r| r.revoked_on.is_none())
            .map(|r| r.shares.get())
            .sum::<u8>()
            >= self.threshold.get()
    }

    pub fn retrieved_shares(&self) -> HashMap<UserID, NonZeroU8> {
        self.retrieved_shares
            .iter()
            .filter_map(|(k, v)| {
                u8::try_from(v.len())
                    .and_then(NonZeroU8::try_from)
                    .ok()
                    .map(|x| (*k, x))
            })
            .collect()
    }

    pub fn pick_recipient(
        &self,
        recipient_user_id: UserID,
    ) -> Result<ShamirRecoveryClaimInitialCtx, ShamirRecoveryClaimPickRecipientError> {
        let recipient = self
            .recipients
            .iter()
            .find(|r| r.user_id == recipient_user_id)
            .ok_or(ShamirRecoveryClaimPickRecipientError::RecipientNotFound)?;

        if recipient.revoked_on.is_some() {
            return Err(ShamirRecoveryClaimPickRecipientError::RecipientRevoked);
        }

        let greeter_user_id = recipient.user_id;
        let greeter_human_handle = recipient.human_handle.clone();

        if self.retrieved_shares.contains_key(&greeter_user_id) {
            return Err(ShamirRecoveryClaimPickRecipientError::RecipientAlreadyPicked);
        }

        Ok(ShamirRecoveryClaimInitialCtx::new(
            self.config.clone(),
            self.cmds.clone(),
            greeter_user_id,
            greeter_human_handle,
            self.time_provider.clone(),
        ))
    }

    pub fn add_share(
        self,
        share_ctx: ShamirRecoveryClaimShare,
    ) -> Result<ShamirRecoveryClaimMaybeRecoverDeviceCtx, ShamirRecoveryClaimAddShareError> {
        let mut retrieved_shares = self.retrieved_shares;

        self.recipients
            .iter()
            .find(|r| r.user_id == share_ctx.recipient)
            .ok_or(ShamirRecoveryClaimAddShareError::RecipientNotFound)?;

        // Note that we do not check if the share is already present
        // This is to avoid handling an extra error that has no real value,
        // since adding shares can be seen as an idempotent operation.
        retrieved_shares.insert(share_ctx.recipient, share_ctx.weighted_share);
        if retrieved_shares
            .values()
            .map(|shares| shares.len())
            .sum::<usize>()
            >= self.threshold.get() as usize
        {
            let secret = ShamirRecoverySecret::decrypt_and_load_from_shares(
                self.threshold,
                retrieved_shares.values().flatten(),
            )
            .map_err(ShamirRecoveryClaimAddShareError::CorruptedSecret)?;
            Ok(ShamirRecoveryClaimMaybeRecoverDeviceCtx::RecoverDevice(
                ShamirRecoveryClaimRecoverDeviceCtx {
                    config: self.config,
                    cmds: self.cmds,
                    claimer_user_id: self.claimer_user_id,
                    claimer_human_handle: self.claimer_human_handle,
                    secret,
                    time_provider: self.time_provider,
                },
            ))
        } else {
            Ok(ShamirRecoveryClaimMaybeRecoverDeviceCtx::PickRecipient(
                Self {
                    retrieved_shares,
                    ..self
                },
            ))
        }
    }
}

// ShamirRecoveryClaimRecoverDeviceCtx

#[derive(Debug, thiserror::Error)]
pub enum ShamirRecoveryClaimRecoverDeviceError {
    #[error("Organization has expired")]
    OrganizationExpired,
    #[error("Organization or invitation not found")]
    NotFound,
    #[error("Invitation already used")]
    AlreadyUsed,
    #[error("Ciphered data not found")]
    CipheredDataNotFound,
    #[error("Corrupted ciphered data: {0}")]
    CorruptedCipheredData(&'static str),
    #[error("Error while registering new device: {0}")]
    RegisterNewDeviceError(RegisterNewDeviceError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub enum ShamirRecoveryClaimMaybeFinalizeCtx {
    Offline(ShamirRecoveryClaimRecoverDeviceCtx),
    Finalize(ShamirRecoveryClaimFinalizeCtx),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ShamirRecoveryClaimRecoverDeviceCtx {
    config: Arc<ClientConfig>,
    cmds: Arc<InvitedCmds>,
    claimer_user_id: UserID,
    claimer_human_handle: HumanHandle,
    secret: ShamirRecoverySecret,
    time_provider: TimeProvider,
}

impl ShamirRecoveryClaimRecoverDeviceCtx {
    pub fn claimer_user_id(&self) -> UserID {
        self.claimer_user_id
    }

    pub fn claimer_human_handle(&self) -> &HumanHandle {
        &self.claimer_human_handle
    }

    pub async fn recover_device(
        self,
        requested_device_label: DeviceLabel,
    ) -> Result<ShamirRecoveryClaimMaybeFinalizeCtx, ShamirRecoveryClaimRecoverDeviceError> {
        let ciphered_data = {
            use invited_cmds::latest::invite_shamir_recovery_reveal::{Rep, Req};

            let rep = self
                .cmds
                .send(Req {
                    reveal_token: self.secret.reveal_token,
                })
                .await;

            match rep {
                // Results
                // Important note: a `NoResponse`` connection error leads to a result containing the current ctx
                // This allows the caller to retry the operation without losing the current state.
                // On the contrary, other errors are deemed irrecoverable and are turned into a specific error type.
                Ok(Rep::Ok { ciphered_data }) => Ok(ciphered_data),
                Err(ConnectionError::NoResponse(_)) => {
                    return Ok(ShamirRecoveryClaimMaybeFinalizeCtx::Offline(self))
                }
                // Errors
                Ok(Rep::BadRevealToken) => {
                    Err(ShamirRecoveryClaimRecoverDeviceError::CipheredDataNotFound)
                }
                Ok(Rep::BadInvitationType) => {
                    Err(anyhow::anyhow!("Unexpected bad invitation type response").into())
                }
                Ok(Rep::UnknownStatus { .. }) => {
                    Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
                }
                Err(ConnectionError::InvitationAlreadyUsedOrDeleted) => {
                    Err(ShamirRecoveryClaimRecoverDeviceError::AlreadyUsed)
                }
                Err(ConnectionError::ExpiredOrganization) => {
                    Err(ShamirRecoveryClaimRecoverDeviceError::OrganizationExpired)
                }
                Err(ConnectionError::BadAuthenticationInfo) => {
                    Err(ShamirRecoveryClaimRecoverDeviceError::NotFound)
                }
                Err(err) => Err(ShamirRecoveryClaimRecoverDeviceError::Internal(err.into())),
            }?
        };

        let mut recovery_device =
            LocalDevice::decrypt_and_load(&ciphered_data, &self.secret.data_key)
                .map_err(ShamirRecoveryClaimRecoverDeviceError::CorruptedCipheredData)?;

        // When using the tested, the recovery device organization address is set to a placeholder address.
        // This is because the organization address is not known yet when the testbed is initialized.
        // In this case, we replace the placeholder address with the actual organization address.
        if cfg!(test)
            && recovery_device
                .organization_addr
                .to_string()
                .starts_with("parsec3://parsec.invalid/PlaceholderOrg")
        {
            recovery_device.organization_addr = ParsecOrganizationAddr::new(
                self.cmds.addr(),
                self.cmds.addr().organization_id().clone(),
                recovery_device.organization_addr.root_verify_key().clone(),
            );
        }

        let recovery_device = Arc::new(recovery_device);

        let new_local_device =
            LocalDevice::from_existing_device_for_user(&recovery_device, requested_device_label);

        let recovery_cmds = AuthenticatedCmds::new(
            &self.config.config_dir,
            recovery_device.clone(),
            self.config.proxy.clone(),
        )?;

        match register_new_device(
            &recovery_cmds,
            &new_local_device,
            DevicePurpose::Standard,
            &recovery_device,
        )
        .await
        {
            // Success
            Ok(_) => Ok(()),
            // Offline, let the caller retry the operation later
            Err(RegisterNewDeviceError::Offline(_)) => {
                return Ok(ShamirRecoveryClaimMaybeFinalizeCtx::Offline(self))
            }
            // Unrecoverable error, propagate it
            Err(e) => Err(ShamirRecoveryClaimRecoverDeviceError::RegisterNewDeviceError(e)),
        }?;

        Ok(ShamirRecoveryClaimMaybeFinalizeCtx::Finalize(
            ShamirRecoveryClaimFinalizeCtx {
                config: self.config,
                new_local_device: Arc::new(new_local_device),
                token: self.cmds.addr().token(),
            },
        ))
    }
}

// ClaimCancellerCtx

#[derive(Debug)]
pub struct ClaimCancellerCtx {
    greeting_attempt_id: GreetingAttemptID,
    cmds: Arc<InvitedCmds>,
}

impl ClaimCancellerCtx {
    fn new(greeting_attempt_id: GreetingAttemptID, cmds: Arc<InvitedCmds>) -> Self {
        Self {
            greeting_attempt_id,
            cmds,
        }
    }

    pub async fn cancel(self) -> Result<(), ClaimInProgressError> {
        cancel_greeting_attempt(
            &self.cmds,
            self.greeting_attempt_id,
            CancelledGreetingAttemptReason::ManuallyCancelled,
        )
        .await
    }

    pub async fn cancel_and_warn_on_error(self) {
        cancel_greeting_attempt_and_warn_on_error(
            &self.cmds,
            self.greeting_attempt_id,
            CancelledGreetingAttemptReason::ManuallyCancelled,
        )
        .await
    }
}

#[derive(Debug)]
struct BaseClaimInitialCtx {
    config: Arc<ClientConfig>,
    cmds: Arc<InvitedCmds>,
    greeter_user_id: UserID,
    greeter_human_handle: HumanHandle,
    time_provider: TimeProvider,
}

impl BaseClaimInitialCtx {
    fn new(
        config: Arc<ClientConfig>,
        cmds: Arc<InvitedCmds>,
        greeter_user_id: UserID,
        greeter_human_handle: HumanHandle,
        time_provider: TimeProvider,
    ) -> Self {
        Self {
            config,
            cmds,
            greeter_user_id,
            greeter_human_handle,
            time_provider,
        }
    }

    async fn _do_wait_peer(
        self,
        mut register_greeting_attempt: Arc<AsyncMutex<Option<GreetingAttemptID>>>,
    ) -> Result<BaseClaimInProgress1Ctx, ClaimInProgressError> {
        // Loop over wait peer attempts
        for attempt in 0.. {
            let (greeting_attempt, greeter_sas, claimer_sas, shared_secret_key) = match self
                ._do_wait_peer_single_attempt(&mut register_greeting_attempt)
                .await
            {
                Ok(x) => x,
                // If the attempt was automatically cancelled by the other peer, try again (at most 8 times).
                // Previous attempts are automatically cancelled when a new start greeting attempt is made.
                // This way, the peers can synchronize themselves more easily during the wait-peer phase,
                // without requiring the front-end to deal with it.
                Err(ClaimInProgressError::GreetingAttemptCancelled {
                    origin: GreeterOrClaimer::Greeter,
                    reason: CancelledGreetingAttemptReason::AutomaticallyCancelled,
                    ..
                }) if attempt < WAIT_PEER_MAX_ATTEMPTS => continue,
                Err(err) => return Err(err),
            };
            // Move self into the next context
            return Ok(BaseClaimInProgress1Ctx {
                config: self.config,
                cmds: self.cmds,
                greeter_user_id: self.greeter_user_id,
                greeter_human_handle: self.greeter_human_handle,
                greeting_attempt,
                greeter_sas,
                claimer_sas,
                shared_secret_key,
                time_provider: self.time_provider,
            });
        }
        unreachable!()
    }

    async fn _do_wait_peer_single_attempt(
        &self,
        register_greeting_attempt: &mut Arc<AsyncMutex<Option<GreetingAttemptID>>>,
    ) -> Result<(GreetingAttemptID, SASCode, SASCode, SecretKey), ClaimInProgressError> {
        let greeting_attempt = {
            use invited_cmds::latest::invite_claimer_start_greeting_attempt::{Rep, Req};
            let mut register_greeting_attempt_guard = register_greeting_attempt.lock().await;
            let rep = self
                .cmds
                .send(Req {
                    greeter: self.greeter_user_id,
                })
                .await?;

            let greeting_attempt = match rep {
                Rep::Ok { greeting_attempt } => Ok(greeting_attempt),
                Rep::GreeterNotFound => Err(ClaimInProgressError::NotFound),
                Rep::GreeterNotAllowed => Err(ClaimInProgressError::GreeterNotAllowed),
                Rep::GreeterRevoked => Err(ClaimInProgressError::GreeterNotAllowed),
                bad_rep @ Rep::UnknownStatus { .. } => {
                    Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                }
            }?;
            // Register the new greeting attempt
            register_greeting_attempt_guard.replace(greeting_attempt);
            greeting_attempt
        };

        let claimer_private_key = PrivateKey::generate();

        let greeter_public_key = {
            let greeter_step = run_claimer_step_until_ready(
                &self.cmds,
                greeting_attempt,
                invite_claimer_step::ClaimerStep::Number0WaitPeer {
                    public_key: claimer_private_key.public_key(),
                },
                &self.time_provider,
            )
            .await?;
            let result: Result<_, ClaimInProgressError> = match greeter_step {
                invite_claimer_step::GreeterStep::Number0WaitPeer { public_key } => Ok(public_key),
                _ => Err(anyhow::anyhow!("Unexpected greeter step: {:?}", greeter_step).into()),
            };
            result?
        };

        let claimer_nonce = generate_sas_code_nonce();
        let hashed_nonce = HashDigest::from_data(&claimer_nonce);
        let shared_secret_key = claimer_private_key
            .generate_shared_secret_key(&greeter_public_key, SharedSecretKeyRole::Greeter)
            .map_err(ClaimInProgressError::CorruptedSharedSecretKey)?;
        {
            let greeter_step = run_claimer_step_until_ready(
                &self.cmds,
                greeting_attempt,
                invite_claimer_step::ClaimerStep::Number1SendHashedNonce { hashed_nonce },
                &self.time_provider,
            )
            .await?;
            match greeter_step {
                invite_claimer_step::GreeterStep::Number1GetHashedNonce => {}
                _ => {
                    return Err(
                        anyhow::anyhow!("Unexpected greeter step: {:?}", greeter_step).into(),
                    )
                }
            };
        };

        let greeter_nonce = {
            let greeter_step = run_claimer_step_until_ready(
                &self.cmds,
                greeting_attempt,
                invite_claimer_step::ClaimerStep::Number2GetNonce,
                &self.time_provider,
            )
            .await?;
            let result: Result<_, ClaimInProgressError> = match greeter_step {
                invite_claimer_step::GreeterStep::Number2SendNonce { greeter_nonce } => {
                    Ok(greeter_nonce)
                }
                _ => Err(anyhow::anyhow!("Unexpected greeter step: {:?}", greeter_step).into()),
            };
            result?
        };

        let (claimer_sas, greeter_sas) =
            SASCode::generate_sas_codes(&claimer_nonce, &greeter_nonce, &shared_secret_key);

        {
            let greeter_step = run_claimer_step_until_ready(
                &self.cmds,
                greeting_attempt,
                invite_claimer_step::ClaimerStep::Number3SendNonce {
                    claimer_nonce: claimer_nonce.into(),
                },
                &self.time_provider,
            )
            .await?;
            match greeter_step {
                invite_claimer_step::GreeterStep::Number3GetNonce => {}
                _ => {
                    return Err(
                        anyhow::anyhow!("Unexpected greeter step: {:?}", greeter_step).into(),
                    )
                }
            };
        };

        Ok((
            greeting_attempt,
            greeter_sas,
            claimer_sas,
            shared_secret_key,
        ))
    }

    async fn do_wait_peer(self) -> Result<BaseClaimInProgress1Ctx, ClaimInProgressError> {
        let register_greeting_attempt = Arc::new(AsyncMutex::new(None));
        self._do_wait_peer(register_greeting_attempt).await
    }

    pub async fn do_wait_peer_with_canceller_event(
        self,
        cancel_requested: EventListener,
    ) -> Result<BaseClaimInProgress1Ctx, ClaimInProgressError> {
        let cmds = self.cmds.clone();

        // This mutex is used to expose the greeting attempt ID obtained during
        // the `start_greeting_attempt` command. This way, this ID can retrieved
        // and used for cancellation if needed.
        // We use an `AsyncMutex` here, which is allowing us to take the lock
        // while sending the `start_greeting_attempt` request to the server
        // so a concurrent coroutine is able to wait for it to succeed and then
        // return the greeting attempt ID for a future cancellation.
        let register_greeting_attempt = Arc::new(AsyncMutex::new(None));
        let maybe_greeting_attempt = register_greeting_attempt.clone();

        let wait_cancellation = async {
            cancel_requested.await;
            maybe_greeting_attempt
                .lock()
                .await
                .map(|greeting_attempt_id| ClaimCancellerCtx::new(greeting_attempt_id, cmds))
        };

        // It's important to run `do_wait_peer()` and `wait_cancellation()` concurrently
        // since the both lock the greeting attempt async mutex.
        let maybe_canceller_ctx = select2_biased!(
            res = self._do_wait_peer(register_greeting_attempt) => return res,
            maybe_canceller_ctx = wait_cancellation => maybe_canceller_ctx,
        );

        if let Some(canceller_ctx) = maybe_canceller_ctx {
            canceller_ctx.cancel_and_warn_on_error().await;
        }

        Err(ClaimInProgressError::Cancelled)
    }
}

#[derive(Debug)]
pub struct UserClaimInitialCtx {
    base: BaseClaimInitialCtx,
    claimer_email: EmailAddress,
    last_greeting_attempt_joined_on: Option<DateTime>,
    online_status: UserOnlineStatus,
}

impl UserClaimInitialCtx {
    pub fn new(
        config: Arc<ClientConfig>,
        cmds: Arc<InvitedCmds>,
        claimer_email: EmailAddress,
        administrator: UserGreetingAdministrator,
        time_provider: TimeProvider,
    ) -> Self {
        Self {
            base: BaseClaimInitialCtx::new(
                config,
                cmds,
                administrator.user_id,
                administrator.human_handle,
                time_provider,
            ),
            claimer_email,
            last_greeting_attempt_joined_on: administrator.last_greeting_attempt_joined_on,
            online_status: administrator.online_status,
        }
    }

    pub fn greeter_user_id(&self) -> UserID {
        self.base.greeter_user_id
    }

    pub fn greeter_human_handle(&self) -> &HumanHandle {
        &self.base.greeter_human_handle
    }

    pub fn claimer_email(&self) -> &EmailAddress {
        &self.claimer_email
    }

    pub fn last_greeting_attempt_joined_on(&self) -> Option<DateTime> {
        self.last_greeting_attempt_joined_on
    }

    pub fn online_status(&self) -> UserOnlineStatus {
        self.online_status
    }

    pub async fn do_wait_peer(self) -> Result<UserClaimInProgress1Ctx, ClaimInProgressError> {
        self.base.do_wait_peer().await.map(UserClaimInProgress1Ctx)
    }

    pub async fn do_wait_peer_with_canceller_event(
        self,
        cancel_requested: EventListener,
    ) -> Result<UserClaimInProgress1Ctx, ClaimInProgressError> {
        self.base
            .do_wait_peer_with_canceller_event(cancel_requested)
            .await
            .map(UserClaimInProgress1Ctx)
    }

    async fn _do_wait_multiple_peer(
        initial_ctxs: Vec<Self>,
        canceller_event: Arc<Event>,
    ) -> Result<UserClaimInProgress1Ctx, ClaimInProgressError> {
        let mut wait_peer_handles = initial_ctxs
            .into_iter()
            .map(|ctx| Box::pin(ctx.do_wait_peer_with_canceller_event(canceller_event.listen())))
            .collect::<Vec<_>>();

        loop {
            // Wait for the next handle to finish
            let (result, _, new_wait_peer_handles) = select_all(wait_peer_handles).await;

            // Update peer handles
            wait_peer_handles = new_wait_peer_handles;

            // Some errors can be ignored if other greeters are still available
            if let Err(
                ClaimInProgressError::GreeterNotAllowed
                | ClaimInProgressError::GreetingAttemptCancelled { .. },
            ) = &result
            {
                if !wait_peer_handles.is_empty() {
                    continue;
                }
            }

            // Cancel the remaining greeting attempts
            canceller_event.notify(usize::MAX);
            join_all(wait_peer_handles).await;

            return result;
        }
    }

    pub async fn do_wait_multiple_peer(
        initial_ctxs: Vec<Self>,
    ) -> Result<UserClaimInProgress1Ctx, ClaimInProgressError> {
        let canceller_event = Arc::new(Event::new());
        Self::_do_wait_multiple_peer(initial_ctxs, canceller_event).await
    }

    pub async fn do_wait_multiple_peer_with_canceller_event(
        initial_ctxs: Vec<Self>,
        cancel_requested: EventListener,
    ) -> Result<UserClaimInProgress1Ctx, ClaimInProgressError> {
        let canceller_event = Arc::new(Event::new());
        let cloned_event = canceller_event.clone();
        let forward_cancellation = spawn(async move {
            cancel_requested.await;
            canceller_event.notify(usize::MAX);
        });
        let result = Self::_do_wait_multiple_peer(initial_ctxs, cloned_event).await;
        forward_cancellation.abort();
        result
    }
}

#[derive(Debug)]
pub struct DeviceClaimInitialCtx(BaseClaimInitialCtx);

impl DeviceClaimInitialCtx {
    pub fn new(
        config: Arc<ClientConfig>,
        cmds: Arc<InvitedCmds>,
        greeter_user_id: UserID,
        greeter_human_handle: HumanHandle,
        time_provider: TimeProvider,
    ) -> Self {
        Self(BaseClaimInitialCtx::new(
            config,
            cmds,
            greeter_user_id,
            greeter_human_handle,
            time_provider,
        ))
    }

    pub fn greeter_user_id(&self) -> UserID {
        self.0.greeter_user_id
    }

    pub fn greeter_human_handle(&self) -> &HumanHandle {
        &self.0.greeter_human_handle
    }

    pub async fn do_wait_peer(self) -> Result<DeviceClaimInProgress1Ctx, ClaimInProgressError> {
        self.0.do_wait_peer().await.map(DeviceClaimInProgress1Ctx)
    }

    pub async fn do_wait_peer_with_canceller_event(
        self,
        cancel_requested: EventListener,
    ) -> Result<DeviceClaimInProgress1Ctx, ClaimInProgressError> {
        self.0
            .do_wait_peer_with_canceller_event(cancel_requested)
            .await
            .map(DeviceClaimInProgress1Ctx)
    }
}

#[derive(Debug)]
pub struct ShamirRecoveryClaimInitialCtx(BaseClaimInitialCtx);

impl ShamirRecoveryClaimInitialCtx {
    pub fn new(
        config: Arc<ClientConfig>,
        cmds: Arc<InvitedCmds>,
        greeter_user_id: UserID,
        greeter_human_handle: HumanHandle,
        time_provider: TimeProvider,
    ) -> Self {
        Self(BaseClaimInitialCtx::new(
            config,
            cmds,
            greeter_user_id,
            greeter_human_handle,
            time_provider,
        ))
    }

    pub fn greeter_user_id(&self) -> UserID {
        self.0.greeter_user_id
    }

    pub fn greeter_human_handle(&self) -> &HumanHandle {
        &self.0.greeter_human_handle
    }

    pub async fn do_wait_peer(
        self,
    ) -> Result<ShamirRecoveryClaimInProgress1Ctx, ClaimInProgressError> {
        self.0
            .do_wait_peer()
            .await
            .map(ShamirRecoveryClaimInProgress1Ctx)
    }

    pub async fn do_wait_peer_with_canceller_event(
        self,
        cancel_requested: EventListener,
    ) -> Result<ShamirRecoveryClaimInProgress1Ctx, ClaimInProgressError> {
        self.0
            .do_wait_peer_with_canceller_event(cancel_requested)
            .await
            .map(ShamirRecoveryClaimInProgress1Ctx)
    }
}

#[derive(Debug)]
struct BaseClaimInProgress1Ctx {
    config: Arc<ClientConfig>,
    cmds: Arc<InvitedCmds>,
    greeter_user_id: UserID,
    greeter_human_handle: HumanHandle,
    greeting_attempt: GreetingAttemptID,
    greeter_sas: SASCode,
    claimer_sas: SASCode,
    shared_secret_key: SecretKey,
    time_provider: TimeProvider,
}

impl BaseClaimInProgress1Ctx {
    fn generate_greeter_sas_choices(&self, size: usize) -> Vec<SASCode> {
        SASCode::generate_sas_code_candidates(&self.greeter_sas, size)
    }

    async fn do_deny_trust(&self) -> Result<(), ClaimInProgressError> {
        cancel_greeting_attempt(
            &self.cmds,
            self.greeting_attempt,
            CancelledGreetingAttemptReason::InvalidSasCode,
        )
        .await
    }

    async fn do_signify_trust(self) -> Result<BaseClaimInProgress2Ctx, ClaimInProgressError> {
        let greeter_step = run_claimer_step_until_ready(
            &self.cmds,
            self.greeting_attempt,
            invite_claimer_step::ClaimerStep::Number4SignifyTrust,
            &self.time_provider,
        )
        .await?;
        match greeter_step {
            invite_claimer_step::GreeterStep::Number4WaitPeerTrust => {}
            _ => return Err(anyhow::anyhow!("Unexpected greeter step: {:?}", greeter_step).into()),
        };

        Ok(BaseClaimInProgress2Ctx {
            config: self.config,
            cmds: self.cmds,
            greeter_user_id: self.greeter_user_id,
            greeting_attempt: self.greeting_attempt,
            claimer_sas: self.claimer_sas,
            shared_secret_key: self.shared_secret_key,
            time_provider: self.time_provider,
        })
    }
}

#[derive(Debug)]
pub struct UserClaimInProgress1Ctx(BaseClaimInProgress1Ctx);

impl UserClaimInProgress1Ctx {
    pub fn greeter_user_id(&self) -> UserID {
        self.0.greeter_user_id
    }

    pub fn greeter_human_handle(&self) -> &HumanHandle {
        &self.0.greeter_human_handle
    }

    pub fn greeter_sas(&self) -> &SASCode {
        &self.0.greeter_sas
    }

    pub fn generate_greeter_sas_choices(&self, size: usize) -> Vec<SASCode> {
        self.0.generate_greeter_sas_choices(size)
    }

    pub fn canceller_ctx(&self) -> ClaimCancellerCtx {
        ClaimCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_deny_trust(self) -> Result<(), ClaimInProgressError> {
        self.0.do_deny_trust().await
    }

    pub async fn do_signify_trust(self) -> Result<UserClaimInProgress2Ctx, ClaimInProgressError> {
        self.0.do_signify_trust().await.map(UserClaimInProgress2Ctx)
    }
}

#[derive(Debug)]
pub struct DeviceClaimInProgress1Ctx(BaseClaimInProgress1Ctx);

impl DeviceClaimInProgress1Ctx {
    pub fn greeter_user_id(&self) -> UserID {
        self.0.greeter_user_id
    }

    pub fn greeter_human_handle(&self) -> &HumanHandle {
        &self.0.greeter_human_handle
    }

    pub fn greeter_sas(&self) -> &SASCode {
        &self.0.greeter_sas
    }

    pub fn generate_greeter_sas_choices(&self, size: usize) -> Vec<SASCode> {
        self.0.generate_greeter_sas_choices(size)
    }

    pub fn canceller_ctx(&self) -> ClaimCancellerCtx {
        ClaimCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_deny_trust(self) -> Result<(), ClaimInProgressError> {
        self.0.do_deny_trust().await
    }

    pub async fn do_signify_trust(self) -> Result<DeviceClaimInProgress2Ctx, ClaimInProgressError> {
        self.0
            .do_signify_trust()
            .await
            .map(DeviceClaimInProgress2Ctx)
    }
}

#[derive(Debug)]
pub struct ShamirRecoveryClaimInProgress1Ctx(BaseClaimInProgress1Ctx);

impl ShamirRecoveryClaimInProgress1Ctx {
    pub fn greeter_user_id(&self) -> UserID {
        self.0.greeter_user_id
    }

    pub fn greeter_human_handle(&self) -> &HumanHandle {
        &self.0.greeter_human_handle
    }

    pub fn greeter_sas(&self) -> &SASCode {
        &self.0.greeter_sas
    }

    pub fn generate_greeter_sas_choices(&self, size: usize) -> Vec<SASCode> {
        self.0.generate_greeter_sas_choices(size)
    }

    pub fn canceller_ctx(&self) -> ClaimCancellerCtx {
        ClaimCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_deny_trust(self) -> Result<(), ClaimInProgressError> {
        self.0.do_deny_trust().await
    }

    pub async fn do_signify_trust(
        self,
    ) -> Result<ShamirRecoveryClaimInProgress2Ctx, ClaimInProgressError> {
        self.0
            .do_signify_trust()
            .await
            .map(ShamirRecoveryClaimInProgress2Ctx)
    }
}

#[derive(Debug)]
struct BaseClaimInProgress2Ctx {
    config: Arc<ClientConfig>,
    cmds: Arc<InvitedCmds>,
    greeter_user_id: UserID,
    greeting_attempt: GreetingAttemptID,
    claimer_sas: SASCode,
    shared_secret_key: SecretKey,
    time_provider: TimeProvider,
}

impl BaseClaimInProgress2Ctx {
    async fn do_wait_peer_trust(self) -> Result<BaseClaimInProgress3Ctx, ClaimInProgressError> {
        let greeter_step = run_claimer_step_until_ready(
            &self.cmds,
            self.greeting_attempt,
            invite_claimer_step::ClaimerStep::Number5WaitPeerTrust,
            &self.time_provider,
        )
        .await?;
        match greeter_step {
            invite_claimer_step::GreeterStep::Number5SignifyTrust => {}
            _ => return Err(anyhow::anyhow!("Unexpected greeter step: {:?}", greeter_step).into()),
        };

        Ok(BaseClaimInProgress3Ctx {
            config: self.config,
            cmds: self.cmds,
            greeter_user_id: self.greeter_user_id,
            greeting_attempt: self.greeting_attempt,
            shared_secret_key: self.shared_secret_key,
            time_provider: self.time_provider,
        })
    }
}

#[derive(Debug)]
pub struct UserClaimInProgress2Ctx(BaseClaimInProgress2Ctx);

impl UserClaimInProgress2Ctx {
    pub fn claimer_sas(&self) -> &SASCode {
        &self.0.claimer_sas
    }

    pub fn canceller_ctx(&self) -> ClaimCancellerCtx {
        ClaimCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_wait_peer_trust(self) -> Result<UserClaimInProgress3Ctx, ClaimInProgressError> {
        self.0
            .do_wait_peer_trust()
            .await
            .map(UserClaimInProgress3Ctx)
    }
}

#[derive(Debug)]
pub struct DeviceClaimInProgress2Ctx(BaseClaimInProgress2Ctx);

impl DeviceClaimInProgress2Ctx {
    pub fn claimer_sas(&self) -> &SASCode {
        &self.0.claimer_sas
    }

    pub fn canceller_ctx(&self) -> ClaimCancellerCtx {
        ClaimCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_wait_peer_trust(
        self,
    ) -> Result<DeviceClaimInProgress3Ctx, ClaimInProgressError> {
        self.0
            .do_wait_peer_trust()
            .await
            .map(DeviceClaimInProgress3Ctx)
    }
}

#[derive(Debug)]
pub struct ShamirRecoveryClaimInProgress2Ctx(BaseClaimInProgress2Ctx);

impl ShamirRecoveryClaimInProgress2Ctx {
    pub fn claimer_sas(&self) -> &SASCode {
        &self.0.claimer_sas
    }

    pub fn canceller_ctx(&self) -> ClaimCancellerCtx {
        ClaimCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_wait_peer_trust(
        self,
    ) -> Result<ShamirRecoveryClaimInProgress3Ctx, ClaimInProgressError> {
        self.0
            .do_wait_peer_trust()
            .await
            .map(ShamirRecoveryClaimInProgress3Ctx)
    }
}

#[derive(Debug)]
struct BaseClaimInProgress3Ctx {
    config: Arc<ClientConfig>,
    cmds: Arc<InvitedCmds>,
    greeter_user_id: UserID,
    greeting_attempt: GreetingAttemptID,
    shared_secret_key: SecretKey,
    time_provider: TimeProvider,
}

impl BaseClaimInProgress3Ctx {
    async fn do_claim(&self, payload: Bytes) -> Result<Bytes, ClaimInProgressError> {
        {
            let greeter_step = run_claimer_step_until_ready(
                &self.cmds,
                self.greeting_attempt,
                invite_claimer_step::ClaimerStep::Number6SendPayload {
                    claimer_payload: payload,
                },
                &self.time_provider,
            )
            .await?;
            match greeter_step {
                invite_claimer_step::GreeterStep::Number6GetPayload => {}
                _ => {
                    return Err(
                        anyhow::anyhow!("Unexpected greeter step: {:?}", greeter_step).into(),
                    )
                }
            };
        }

        let greeter_payload = {
            let greeter_step = run_claimer_step_until_ready(
                &self.cmds,
                self.greeting_attempt,
                invite_claimer_step::ClaimerStep::Number7GetPayload,
                &self.time_provider,
            )
            .await?;
            let result: Result<_, ClaimInProgressError> = match greeter_step {
                invite_claimer_step::GreeterStep::Number7SendPayload { greeter_payload } => {
                    Ok(greeter_payload)
                }
                _ => Err(anyhow::anyhow!("Unexpected greeter step: {:?}", greeter_step).into()),
            };
            result?
        };

        Ok(greeter_payload)
    }

    async fn do_acknowledge(&self) -> Result<(), ClaimInProgressError> {
        // Only perform the acknowledge step once
        // This is for two reasons:
        // 1. We don't need to wait for the greeter to actually receive it
        // 2. The invitation might get marked as completed by the greeter
        //    once it has received our acknowledgement. That means that there
        //    are no guarantees that will be able to get the 8th greeter step
        //    if we poll for it.
        let greeter_step = run_claimer_step(
            &self.cmds,
            self.greeting_attempt,
            invite_claimer_step::ClaimerStep::Number8Acknowledge,
        )
        .await?;
        match greeter_step {
            None => {}
            Some(invite_claimer_step::GreeterStep::Number8WaitPeerAcknowledgment) => {}
            _ => return Err(anyhow::anyhow!("Unexpected greeter step: {:?}", greeter_step).into()),
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct UserClaimInProgress3Ctx(BaseClaimInProgress3Ctx);

impl UserClaimInProgress3Ctx {
    pub fn canceller_ctx(&self) -> ClaimCancellerCtx {
        ClaimCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_claim_user(
        self,
        requested_device_label: DeviceLabel,
        requested_human_handle: HumanHandle,
    ) -> Result<UserClaimFinalizeCtx, ClaimInProgressError> {
        // User&device keys are generated here and kept in memory until the end of
        // the enrollment process. This mean we can lost it if something goes wrong.
        // This has no impact until step 4 (somewhere between data exchange and
        // confirmation exchange steps) where greeter upload our certificates in
        // the server.
        // This is considered acceptable given 1) the error window is small and
        // 2) if this occurs the inviter can revoke the user and retry the
        // enrollment process to fix this
        let private_key = PrivateKey::generate();
        let signing_key = SigningKey::generate();

        let payload = InviteUserData {
            requested_device_label,
            requested_human_handle,
            public_key: private_key.public_key(),
            verify_key: signing_key.verify_key(),
        }
        .dump_and_encrypt(&self.0.shared_secret_key)
        .into();

        let payload = self.0.do_claim(payload).await?;

        let InviteUserConfirmation {
            user_id,
            device_id,
            device_label,
            human_handle,
            profile,
            root_verify_key,
        } = match InviteUserConfirmation::decrypt_and_load(&payload, &self.0.shared_secret_key) {
            Ok(data) => data,
            Err(err) => {
                let reason = match err {
                    DataError::Decryption => CancelledGreetingAttemptReason::UndecipherablePayload,
                    DataError::BadSerialization { .. } => {
                        CancelledGreetingAttemptReason::UndeserializablePayload
                    }
                    _ => CancelledGreetingAttemptReason::InconsistentPayload,
                };
                cancel_greeting_attempt_and_warn_on_error(
                    &self.0.cmds,
                    self.0.greeting_attempt,
                    reason,
                )
                .await;
                return Err(ClaimInProgressError::CorruptedConfirmation(err));
            }
        };

        let addr = self.0.cmds.addr();

        let organization_addr =
            ParsecOrganizationAddr::new(addr, addr.organization_id().clone(), root_verify_key);

        let new_local_device = Arc::new(LocalDevice::generate_new_device(
            organization_addr,
            profile,
            human_handle,
            device_label,
            Some(user_id),
            Some(device_id),
            Some(signing_key),
            Some(private_key),
            Some(self.0.time_provider.clone()),
            None,
            None,
        ));

        self.0.do_acknowledge().await?;

        Ok(UserClaimFinalizeCtx {
            config: self.0.config,
            new_local_device,
        })
    }
}

#[derive(Debug)]
pub struct DeviceClaimInProgress3Ctx(BaseClaimInProgress3Ctx);

impl DeviceClaimInProgress3Ctx {
    pub fn canceller_ctx(&self) -> ClaimCancellerCtx {
        ClaimCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_claim_device(
        self,
        requested_device_label: DeviceLabel,
    ) -> Result<DeviceClaimFinalizeCtx, ClaimInProgressError> {
        // Device key is generated here and kept in memory until the end of
        // the enrollment process. This mean we can lost it if something goes wrong.
        // This has no impact until step 4 (somewhere between data exchange and
        // confirmation exchange steps) where greeter upload our certificate in
        // the server.
        // This is considered acceptable given 1) the error window is small and
        // 2) if this occurs the inviter can revoke the device and retry the
        // enrollment process to fix this
        let signing_key = SigningKey::generate();

        let payload = InviteDeviceData {
            requested_device_label,
            verify_key: signing_key.verify_key(),
        }
        .dump_and_encrypt(&self.0.shared_secret_key)
        .into();

        let payload = self.0.do_claim(payload).await?;

        let InviteDeviceConfirmation {
            user_id,
            device_id,
            device_label,
            human_handle,
            profile,
            private_key,
            user_realm_id,
            user_realm_key,
            root_verify_key,
            ..
        } = match InviteDeviceConfirmation::decrypt_and_load(&payload, &self.0.shared_secret_key) {
            Ok(data) => data,
            Err(err) => {
                let reason = match err {
                    DataError::Decryption => CancelledGreetingAttemptReason::UndecipherablePayload,
                    DataError::BadSerialization { .. } => {
                        CancelledGreetingAttemptReason::UndeserializablePayload
                    }
                    _ => CancelledGreetingAttemptReason::InconsistentPayload,
                };
                cancel_greeting_attempt_and_warn_on_error(
                    &self.0.cmds,
                    self.0.greeting_attempt,
                    reason,
                )
                .await;
                return Err(ClaimInProgressError::CorruptedConfirmation(err));
            }
        };

        let addr = self.0.cmds.addr();

        let organization_addr =
            ParsecOrganizationAddr::new(addr, addr.organization_id().clone(), root_verify_key);

        let new_local_device = Arc::new(LocalDevice {
            organization_addr,
            user_id,
            device_id,
            device_label,
            human_handle,
            initial_profile: profile,
            private_key,
            signing_key,
            user_realm_id,
            user_realm_key,
            local_symkey: SecretKey::generate(),
            time_provider: self.0.time_provider.clone(),
        });

        self.0.do_acknowledge().await?;

        Ok(DeviceClaimFinalizeCtx {
            config: self.0.config,
            new_local_device,
        })
    }
}

#[derive(Debug)]
pub struct ShamirRecoveryClaimInProgress3Ctx(BaseClaimInProgress3Ctx);

impl ShamirRecoveryClaimInProgress3Ctx {
    pub fn canceller_ctx(&self) -> ClaimCancellerCtx {
        ClaimCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_recover_share(self) -> Result<ShamirRecoveryClaimShare, ClaimInProgressError> {
        let payload = InviteShamirRecoveryData
            .dump_and_encrypt(&self.0.shared_secret_key)
            .into();

        let payload = self.0.do_claim(payload).await?;

        let InviteShamirRecoveryConfirmation { weighted_share } =
            match InviteShamirRecoveryConfirmation::decrypt_and_load(
                &payload,
                &self.0.shared_secret_key,
            ) {
                Ok(data) => data,
                Err(err) => {
                    let reason = match err {
                        DataError::Decryption => {
                            CancelledGreetingAttemptReason::UndecipherablePayload
                        }
                        DataError::BadSerialization { .. } => {
                            CancelledGreetingAttemptReason::UndeserializablePayload
                        }
                        _ => CancelledGreetingAttemptReason::InconsistentPayload,
                    };
                    cancel_greeting_attempt_and_warn_on_error(
                        &self.0.cmds,
                        self.0.greeting_attempt,
                        reason,
                    )
                    .await;
                    return Err(ClaimInProgressError::CorruptedConfirmation(err));
                }
            };

        self.0.do_acknowledge().await?;

        Ok(ShamirRecoveryClaimShare {
            recipient: self.0.greeter_user_id,
            weighted_share,
        })
    }
}

#[derive(Debug)]
pub struct ShamirRecoveryClaimShare {
    pub recipient: UserID,
    pub weighted_share: Vec<ShamirShare>,
}

#[derive(Debug)]
pub struct UserClaimFinalizeCtx {
    config: Arc<ClientConfig>,
    pub new_local_device: Arc<LocalDevice>,
}

impl UserClaimFinalizeCtx {
    pub fn get_default_key_file(&self) -> PathBuf {
        libparsec_platform_device_loader::get_default_key_file(
            &self.config.config_dir,
            self.new_local_device.device_id,
        )
    }

    pub async fn save_local_device(
        self,
        strategy: &DeviceSaveStrategy,
        key_file: &Path,
    ) -> Result<AvailableDevice, anyhow::Error> {
        // Claiming a user means we are it first device, hence we know there
        // is no existing user manifest (hence our placeholder is non-speculative)
        libparsec_platform_storage::user::user_storage_non_speculative_init(
            &self.config.data_base_dir,
            &self.new_local_device,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Error while initializing device's user storage: {e}"))?;

        libparsec_platform_device_loader::save_device(
            &self.config.config_dir,
            strategy,
            &self.new_local_device,
            key_file.to_path_buf(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("Error while saving the device file: {e}"))

        // Note that we don't call invite_complete here, as it is done on the greeter's side.
        // See "invite_complete command" section in RFC 1011
        // https://github.com/Scille/parsec-cloud/blob/master/docs/rfcs/1011-non-blocking-invite-transport.md#invite_complete-command
    }
}

#[derive(Debug)]
pub struct DeviceClaimFinalizeCtx {
    pub config: Arc<ClientConfig>,
    pub new_local_device: Arc<LocalDevice>,
}

impl DeviceClaimFinalizeCtx {
    pub fn get_default_key_file(&self) -> PathBuf {
        libparsec_platform_device_loader::get_default_key_file(
            &self.config.config_dir,
            self.new_local_device.device_id,
        )
    }

    pub async fn save_local_device(
        self,
        strategy: &DeviceSaveStrategy,
        key_file: &Path,
    ) -> Result<AvailableDevice, anyhow::Error> {
        libparsec_platform_device_loader::save_device(
            &self.config.config_dir,
            strategy,
            &self.new_local_device,
            key_file.to_path_buf(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("Error while saving the device file: {e}"))

        // Note that we don't call invite_complete here, as it is done on the greeter's side.
        // See "invite_complete command" section in RFC 1011
        // https://github.com/Scille/parsec-cloud/blob/master/docs/rfcs/1011-non-blocking-invite-transport.md#invite_complete-command
    }
}

#[derive(Debug)]
pub struct ShamirRecoveryClaimFinalizeCtx {
    pub config: Arc<ClientConfig>,
    pub new_local_device: Arc<LocalDevice>,
    pub token: InvitationToken,
}

impl ShamirRecoveryClaimFinalizeCtx {
    pub fn get_default_key_file(&self) -> PathBuf {
        libparsec_platform_device_loader::get_default_key_file(
            &self.config.config_dir,
            self.new_local_device.device_id,
        )
    }

    pub async fn save_local_device(
        self,
        strategy: &DeviceSaveStrategy,
        key_file: &Path,
    ) -> Result<AvailableDevice, anyhow::Error> {
        let available_device = libparsec_platform_device_loader::save_device(
            &self.config.config_dir,
            strategy,
            &self.new_local_device,
            key_file.to_path_buf(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("Error while saving the device file: {e}"))?;

        // After the device has been successfully saved, we still have to mark the invitation as completed,
        // because the greeter cannot know if the process is complete
        // See "invite_complete command" section in RFC 1011
        // https://github.com/Scille/parsec-cloud/blob/master/docs/rfcs/1011-non-blocking-invite-transport.md#invite_complete-command
        // However, we don't want to return an error if this part fails, as the user cannot really do anything
        // about it. Instead, we log the error and return the device.
        'mark_invitation_as_completed: {
            use authenticated_cmds::latest::invite_complete::{Rep, Req};

            let cmds = match AuthenticatedCmds::new(
                &self.config.config_dir,
                self.new_local_device.clone(),
                self.config.proxy.clone(),
            ) {
                Ok(cmds) => cmds,
                Err(e) => {
                    log::error!("Error while creating authenticated commands: {e:?}");
                    break 'mark_invitation_as_completed;
                }
            };

            match cmds.send(Req { token: self.token }).await {
                Ok(Rep::Ok) => (),
                Ok(rep) => {
                    log::error!(
                        "Unexpected reply while marking the invitation as completed: {rep:?}"
                    );
                    break 'mark_invitation_as_completed;
                }
                Err(e) => {
                    log::error!("Error while marking the invitation as completed: {e:?}");
                    break 'mark_invitation_as_completed;
                }
            };
        };

        Ok(available_device)
    }
}
