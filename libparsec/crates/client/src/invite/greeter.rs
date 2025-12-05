// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use authenticated_cmds::latest::invite_greeter_step;
use libparsec_client_connection::{
    protocol::authenticated_cmds, AuthenticatedCmds, ConnectionError,
};
use libparsec_platform_async::event::EventListener;
use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_platform_async::select2_biased;
use libparsec_types::prelude::*;

use crate::invite::common::{Throttle, WAIT_PEER_MAX_ATTEMPTS};
use crate::utils::create_user_and_device_certificates;
use crate::{
    greater_timestamp, EventBus, EventTooMuchDriftWithServerClock, GreaterTimestampOffset,
};

/*
 * new_user_invitation
 */

pub enum InvitationEmailSentStatus {
    Success,
    ServerUnavailable,
    RecipientRefused,
}

#[derive(Debug, thiserror::Error)]
pub enum NewUserInvitationError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Not allowed to invite a user")]
    NotAllowed,
    #[error("A non-revoked user already exists with this email")]
    AlreadyMember,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn new_user_invitation(
    cmds: &AuthenticatedCmds,
    claimer_email: EmailAddress,
    send_email: bool,
) -> Result<(InvitationToken, InvitationEmailSentStatus), NewUserInvitationError> {
    use authenticated_cmds::latest::invite_new_user::{
        InvitationEmailSentStatus as ApiInvitationEmailSentStatus, Rep, Req,
    };

    let req = Req {
        claimer_email,
        send_email,
    };
    let rep = cmds.send(req).await?;

    match rep {
        Rep::Ok { token, email_sent } => {
            let email_sent = match email_sent {
                ApiInvitationEmailSentStatus::Success => InvitationEmailSentStatus::Success,
                ApiInvitationEmailSentStatus::ServerUnavailable => {
                    InvitationEmailSentStatus::ServerUnavailable
                }
                ApiInvitationEmailSentStatus::RecipientRefused => {
                    InvitationEmailSentStatus::RecipientRefused
                }
            };
            Ok((token, email_sent))
        }
        Rep::AuthorNotAllowed => Err(NewUserInvitationError::NotAllowed),
        Rep::ClaimerEmailAlreadyEnrolled => Err(NewUserInvitationError::AlreadyMember),
        rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    }
}

/*
 * new_device_invitation
 */

#[derive(Debug, thiserror::Error)]
pub enum NewDeviceInvitationError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn new_device_invitation(
    cmds: &AuthenticatedCmds,
    send_email: bool,
) -> Result<(InvitationToken, InvitationEmailSentStatus), NewDeviceInvitationError> {
    use authenticated_cmds::latest::invite_new_device::{
        InvitationEmailSentStatus as ApiInvitationEmailSentStatus, Rep, Req,
    };

    let req = Req { send_email };
    let rep = cmds.send(req).await?;

    match rep {
        Rep::Ok { token, email_sent } => {
            let email_sent = match email_sent {
                ApiInvitationEmailSentStatus::Success => InvitationEmailSentStatus::Success,
                ApiInvitationEmailSentStatus::ServerUnavailable => {
                    InvitationEmailSentStatus::ServerUnavailable
                }
                ApiInvitationEmailSentStatus::RecipientRefused => {
                    InvitationEmailSentStatus::RecipientRefused
                }
            };
            Ok((token, email_sent))
        }
        rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    }
}

/*
 * new_shamir_invitation
 */

#[derive(Debug, thiserror::Error)]
pub enum NewShamirRecoveryInvitationError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Author not part of the user's current recipients")]
    NotAllowed,
    #[error("Provided user not found")]
    UserNotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn new_shamir_recovery_invitation(
    cmds: &AuthenticatedCmds,
    claimer_user_id: UserID,
    send_email: bool,
) -> Result<(InvitationToken, InvitationEmailSentStatus), NewShamirRecoveryInvitationError> {
    use authenticated_cmds::latest::invite_new_shamir_recovery::{
        InvitationEmailSentStatus as ApiInvitationEmailSentStatus, Rep, Req,
    };

    let req = Req {
        send_email,
        claimer_user_id,
    };
    let rep = cmds.send(req).await?;

    match rep {
        Rep::Ok { token, email_sent } => {
            let email_sent = match email_sent {
                ApiInvitationEmailSentStatus::Success => InvitationEmailSentStatus::Success,
                ApiInvitationEmailSentStatus::ServerUnavailable => {
                    InvitationEmailSentStatus::ServerUnavailable
                }
                ApiInvitationEmailSentStatus::RecipientRefused => {
                    InvitationEmailSentStatus::RecipientRefused
                }
            };
            Ok((token, email_sent))
        }
        Rep::AuthorNotAllowed => Err(NewShamirRecoveryInvitationError::NotAllowed),
        Rep::UserNotFound => Err(NewShamirRecoveryInvitationError::UserNotFound),
        rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    }
}

/*
 * delete_invitation
 */

#[derive(Debug, thiserror::Error)]
pub enum CancelInvitationError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Invitation not found")]
    NotFound,
    #[error("Author not allowed")]
    NotAllowed,
    #[error("Invitation already cancelled")]
    AlreadyCancelled,
    #[error("Invitation already completed")]
    Completed,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn cancel_invitation(
    cmds: &AuthenticatedCmds,
    token: InvitationToken,
) -> Result<(), CancelInvitationError> {
    use authenticated_cmds::latest::invite_cancel::{Rep, Req};

    let req = Req { token };
    let rep = cmds.send(req).await?;

    match rep {
        Rep::Ok => Ok(()),
        Rep::AuthorNotAllowed => Err(CancelInvitationError::NotAllowed),
        Rep::InvitationAlreadyCancelled => Err(CancelInvitationError::AlreadyCancelled),
        Rep::InvitationCompleted => Err(CancelInvitationError::Completed),
        Rep::InvitationNotFound => Err(CancelInvitationError::NotFound),
        rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    }
}

/*
 * list_invitation
 */

#[derive(Debug, thiserror::Error)]
pub enum ListInvitationsError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub use authenticated_cmds::latest::invite_list::InviteListItem;

pub async fn list_invitations(
    cmds: &AuthenticatedCmds,
) -> Result<Vec<InviteListItem>, ListInvitationsError> {
    use authenticated_cmds::latest::invite_list::{Rep, Req};

    let rep = cmds.send(Req).await?;

    match rep {
        Rep::Ok { invitations } => Ok(invitations),
        rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GreetInProgressError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
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
    #[error("Not allowed to greet this invitation")]
    GreeterNotAllowed,
    #[error("Greeting attempt cancelled by {origin:?} because {reason:?} on {timestamp}")]
    GreetingAttemptCancelled {
        origin: GreeterOrClaimer,
        reason: CancelledGreetingAttemptReason,
        timestamp: DateTime,
    },
    #[error(transparent)]
    CorruptedSharedSecretKey(CryptoError),
    #[error(transparent)]
    CorruptedInviteUserData(DataError),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error("Operation cancelled")]
    Cancelled,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

// Cancel greeting attempt helper

async fn cancel_greeting_attempt(
    cmds: &AuthenticatedCmds,
    greeting_attempt: GreetingAttemptID,
    reason: CancelledGreetingAttemptReason,
) -> Result<(), GreetInProgressError> {
    use authenticated_cmds::latest::invite_greeter_cancel_greeting_attempt::{Rep, Req};

    let req = Req {
        greeting_attempt,
        reason,
    };
    let rep = cmds.send(req).await?;

    match rep {
        Rep::Ok => Ok(()),
        // Expected errors
        Rep::InvitationCompleted => Err(GreetInProgressError::AlreadyDeleted),
        Rep::InvitationCancelled => Err(GreetInProgressError::AlreadyDeleted),
        Rep::AuthorNotAllowed => Err(GreetInProgressError::GreeterNotAllowed),
        Rep::GreetingAttemptAlreadyCancelled {
            origin,
            reason,
            timestamp,
        } => Err(GreetInProgressError::GreetingAttemptCancelled {
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
    cmds: &AuthenticatedCmds,
    greeting_attempt: GreetingAttemptID,
    reason: CancelledGreetingAttemptReason,
) {
    if let Err(err) = cancel_greeting_attempt(cmds, greeting_attempt, reason).await {
        // Already cancelled, no need to log a warning
        if let GreetInProgressError::GreetingAttemptCancelled { .. } = &err {
            return;
        }
        log::warn!(
            "Greeter failed to cancel greeting attempt {greeting_attempt:?} with reason {reason:?}: {err:?}"
        );
    }
}

// Greeter step helper

async fn run_greeter_step_until_ready(
    cmds: &AuthenticatedCmds,
    greeting_attempt: GreetingAttemptID,
    greeter_step: invite_greeter_step::GreeterStep,
    time_provider: &TimeProvider,
) -> Result<invite_greeter_step::ClaimerStep, GreetInProgressError> {
    let mut throttle = Throttle::new(time_provider);
    let req = invite_greeter_step::Req {
        greeting_attempt,
        greeter_step,
    };

    // Loop over the requests
    loop {
        // Throttle the requests
        throttle.throttle().await;

        // Send the request
        let rep = cmds.send(req.clone()).await?;

        // Handle the response
        return match rep {
            invite_greeter_step::Rep::NotReady => continue,
            invite_greeter_step::Rep::Ok { claimer_step } => Ok(claimer_step),
            // Expected errors
            invite_greeter_step::Rep::InvitationCompleted => {
                Err(GreetInProgressError::AlreadyDeleted)
            }
            invite_greeter_step::Rep::InvitationCancelled => {
                Err(GreetInProgressError::AlreadyDeleted)
            }
            invite_greeter_step::Rep::AuthorNotAllowed => {
                Err(GreetInProgressError::GreeterNotAllowed)
            }
            invite_greeter_step::Rep::GreetingAttemptCancelled {
                origin,
                reason,
                timestamp,
            } => Err(GreetInProgressError::GreetingAttemptCancelled {
                origin,
                reason,
                timestamp,
            }),
            // Unexpected errors
            invite_greeter_step::Rep::UnknownStatus { .. }
            | invite_greeter_step::Rep::GreetingAttemptNotFound
            | invite_greeter_step::Rep::GreetingAttemptNotJoined
            | invite_greeter_step::Rep::StepMismatch
            | invite_greeter_step::Rep::StepTooAdvanced => {
                Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
            }
        };
    }
}

// GreetCancellerCtx

#[derive(Debug)]
pub struct GreetCancellerCtx {
    greeting_attempt_id: GreetingAttemptID,
    cmds: Arc<AuthenticatedCmds>,
}

impl GreetCancellerCtx {
    fn new(greeting_attempt_id: GreetingAttemptID, cmds: Arc<AuthenticatedCmds>) -> Self {
        Self {
            greeting_attempt_id,
            cmds,
        }
    }

    pub async fn cancel(self) -> Result<(), GreetInProgressError> {
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

// GreetInitialCtx

#[derive(Debug)]
struct BaseGreetInitialCtx {
    token: InvitationToken,
    device: Arc<LocalDevice>,
    cmds: Arc<AuthenticatedCmds>,
    event_bus: EventBus,
}

impl BaseGreetInitialCtx {
    fn new(
        device: Arc<LocalDevice>,
        cmds: Arc<AuthenticatedCmds>,
        event_bus: EventBus,
        token: InvitationToken,
    ) -> Self {
        Self {
            token,
            device,
            cmds,
            event_bus,
        }
    }

    async fn _do_wait_peer(
        self,
        mut register_greeting_attempt: Arc<AsyncMutex<Option<GreetingAttemptID>>>,
    ) -> Result<BaseGreetInProgress1Ctx, GreetInProgressError> {
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
                Err(GreetInProgressError::GreetingAttemptCancelled {
                    origin: GreeterOrClaimer::Claimer,
                    reason: CancelledGreetingAttemptReason::AutomaticallyCancelled,
                    ..
                }) if attempt < WAIT_PEER_MAX_ATTEMPTS => continue,
                Err(err) => return Err(err),
            };
            // Move self into the next context
            return Ok(BaseGreetInProgress1Ctx {
                token: self.token,
                greeting_attempt,
                device: self.device,
                greeter_sas,
                claimer_sas,
                shared_secret_key,
                cmds: self.cmds,
                event_bus: self.event_bus,
            });
        }
        unreachable!()
    }

    async fn _do_wait_peer_single_attempt(
        &self,
        register_greeting_attempt: &mut Arc<AsyncMutex<Option<GreetingAttemptID>>>,
    ) -> Result<(GreetingAttemptID, SASCode, SASCode, SecretKey), GreetInProgressError> {
        let greeting_attempt = {
            use authenticated_cmds::latest::invite_greeter_start_greeting_attempt::{Rep, Req};
            let mut register_greeting_attempt_guard = register_greeting_attempt.lock().await;
            let rep = self.cmds.send(Req { token: self.token }).await?;

            let greeting_attempt = match rep {
                Rep::Ok { greeting_attempt } => Ok(greeting_attempt),
                Rep::InvitationCompleted => Err(GreetInProgressError::AlreadyDeleted),
                Rep::InvitationCancelled => Err(GreetInProgressError::AlreadyDeleted),
                Rep::InvitationNotFound => Err(GreetInProgressError::NotFound),
                Rep::AuthorNotAllowed => Err(GreetInProgressError::GreeterNotAllowed),
                bad_rep @ Rep::UnknownStatus { .. } => {
                    Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                }
            }?;
            // Register the new greeting attempt
            register_greeting_attempt_guard.replace(greeting_attempt);
            greeting_attempt
        };

        let greeter_private_key = PrivateKey::generate();

        let claimer_public_key = {
            let claimer_step = run_greeter_step_until_ready(
                &self.cmds,
                greeting_attempt,
                invite_greeter_step::GreeterStep::Number0WaitPeer {
                    public_key: greeter_private_key.public_key(),
                },
                &self.device.time_provider,
            )
            .await?;
            let result: Result<_, GreetInProgressError> = match claimer_step {
                invite_greeter_step::ClaimerStep::Number0WaitPeer { public_key } => Ok(public_key),
                _ => Err(anyhow::anyhow!("Unexpected claimer step: {:?}", claimer_step).into()),
            };
            result?
        };

        let shared_secret_key = greeter_private_key
            .generate_shared_secret_key(&claimer_public_key, SharedSecretKeyRole::Claimer)
            .map_err(GreetInProgressError::CorruptedSharedSecretKey)?;
        let greeter_nonce: Bytes = generate_sas_code_nonce().into();

        let claimer_hashed_nonce = {
            let claimer_step = run_greeter_step_until_ready(
                &self.cmds,
                greeting_attempt,
                invite_greeter_step::GreeterStep::Number1GetHashedNonce,
                &self.device.time_provider,
            )
            .await?;
            let result: Result<_, GreetInProgressError> = match claimer_step {
                invite_greeter_step::ClaimerStep::Number1SendHashedNonce { hashed_nonce } => {
                    Ok(hashed_nonce)
                }
                _ => Err(anyhow::anyhow!("Unexpected claimer step: {:?}", claimer_step).into()),
            };
            result?
        };

        {
            let claimer_step = run_greeter_step_until_ready(
                &self.cmds,
                greeting_attempt,
                invite_greeter_step::GreeterStep::Number2SendNonce {
                    greeter_nonce: greeter_nonce.clone(),
                },
                &self.device.time_provider,
            )
            .await?;
            let result: Result<_, GreetInProgressError> = match claimer_step {
                invite_greeter_step::ClaimerStep::Number2GetNonce => Ok(()),
                _ => Err(anyhow::anyhow!("Unexpected claimer step: {:?}", claimer_step).into()),
            };
            result?
        };

        let claimer_nonce = {
            let claimer_step = run_greeter_step_until_ready(
                &self.cmds,
                greeting_attempt,
                invite_greeter_step::GreeterStep::Number3GetNonce,
                &self.device.time_provider,
            )
            .await?;
            let result: Result<_, GreetInProgressError> = match claimer_step {
                invite_greeter_step::ClaimerStep::Number3SendNonce { claimer_nonce } => {
                    Ok(claimer_nonce)
                }
                _ => Err(anyhow::anyhow!("Unexpected claimer step: {:?}", claimer_step).into()),
            };
            result?
        };

        if HashDigest::from_data(&claimer_nonce) != claimer_hashed_nonce {
            cancel_greeting_attempt_and_warn_on_error(
                &self.cmds,
                greeting_attempt,
                CancelledGreetingAttemptReason::InvalidNonceHash,
            )
            .await;
            return Err(GreetInProgressError::NonceMismatch);
        }

        let (claimer_sas, greeter_sas) =
            SASCode::generate_sas_codes(&claimer_nonce, greeter_nonce.as_ref(), &shared_secret_key);

        Ok((
            greeting_attempt,
            greeter_sas,
            claimer_sas,
            shared_secret_key,
        ))
    }

    async fn do_wait_peer(self) -> Result<BaseGreetInProgress1Ctx, GreetInProgressError> {
        let register_greeting_attempt = Arc::new(AsyncMutex::new(None));
        self._do_wait_peer(register_greeting_attempt).await
    }

    pub async fn do_wait_peer_with_canceller_event(
        self,
        cancel_requested: EventListener,
    ) -> Result<BaseGreetInProgress1Ctx, GreetInProgressError> {
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
                .map(|greeting_attempt_id| GreetCancellerCtx::new(greeting_attempt_id, cmds))
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

        Err(GreetInProgressError::Cancelled)
    }
}

#[derive(Debug)]
pub struct UserGreetInitialCtx(BaseGreetInitialCtx);

impl UserGreetInitialCtx {
    pub fn new(
        device: Arc<LocalDevice>,
        cmds: Arc<AuthenticatedCmds>,
        event_bus: EventBus,
        token: InvitationToken,
    ) -> Self {
        UserGreetInitialCtx(BaseGreetInitialCtx::new(device, cmds, event_bus, token))
    }

    pub async fn do_wait_peer(self) -> Result<UserGreetInProgress1Ctx, GreetInProgressError> {
        self.0.do_wait_peer().await.map(UserGreetInProgress1Ctx)
    }

    pub async fn do_wait_peer_with_canceller_event(
        self,
        cancel_requested: EventListener,
    ) -> Result<UserGreetInProgress1Ctx, GreetInProgressError> {
        self.0
            .do_wait_peer_with_canceller_event(cancel_requested)
            .await
            .map(UserGreetInProgress1Ctx)
    }
}

#[derive(Debug)]
pub struct DeviceGreetInitialCtx(BaseGreetInitialCtx);

impl DeviceGreetInitialCtx {
    pub fn new(
        device: Arc<LocalDevice>,
        cmds: Arc<AuthenticatedCmds>,
        event_bus: EventBus,
        token: InvitationToken,
    ) -> Self {
        Self(BaseGreetInitialCtx::new(device, cmds, event_bus, token))
    }

    pub async fn do_wait_peer(self) -> Result<DeviceGreetInProgress1Ctx, GreetInProgressError> {
        self.0.do_wait_peer().await.map(DeviceGreetInProgress1Ctx)
    }

    pub async fn do_wait_peer_with_canceller_event(
        self,
        cancel_requested: EventListener,
    ) -> Result<DeviceGreetInProgress1Ctx, GreetInProgressError> {
        self.0
            .do_wait_peer_with_canceller_event(cancel_requested)
            .await
            .map(DeviceGreetInProgress1Ctx)
    }
}

#[derive(Debug)]
pub struct ShamirRecoveryGreetInitialCtx {
    base: BaseGreetInitialCtx,
    pub share_data: ShamirRecoveryShareData,
}

impl ShamirRecoveryGreetInitialCtx {
    pub fn new(
        device: Arc<LocalDevice>,
        cmds: Arc<AuthenticatedCmds>,
        event_bus: EventBus,
        token: InvitationToken,
        share_data: ShamirRecoveryShareData,
    ) -> Self {
        Self {
            base: BaseGreetInitialCtx::new(device, cmds, event_bus, token),
            share_data,
        }
    }

    pub async fn do_wait_peer(
        self,
    ) -> Result<ShamirRecoveryGreetInProgress1Ctx, GreetInProgressError> {
        self.base
            .do_wait_peer()
            .await
            .map(|base| ShamirRecoveryGreetInProgress1Ctx {
                base,
                share_data: self.share_data,
            })
    }

    pub async fn do_wait_peer_with_canceller_event(
        self,
        cancel_requested: EventListener,
    ) -> Result<ShamirRecoveryGreetInProgress1Ctx, GreetInProgressError> {
        self.base
            .do_wait_peer_with_canceller_event(cancel_requested)
            .await
            .map(|base| ShamirRecoveryGreetInProgress1Ctx {
                base,
                share_data: self.share_data,
            })
    }
}

// GreetInProgress1Ctx

#[derive(Debug)]
struct BaseGreetInProgress1Ctx {
    token: InvitationToken,
    greeting_attempt: GreetingAttemptID,
    device: Arc<LocalDevice>,
    greeter_sas: SASCode,
    claimer_sas: SASCode,
    shared_secret_key: SecretKey,
    cmds: Arc<AuthenticatedCmds>,
    event_bus: EventBus,
}

impl BaseGreetInProgress1Ctx {
    async fn do_wait_peer_trust(self) -> Result<BaseGreetInProgress2Ctx, GreetInProgressError> {
        let claimer_step = run_greeter_step_until_ready(
            &self.cmds,
            self.greeting_attempt,
            invite_greeter_step::GreeterStep::Number4WaitPeerTrust,
            &self.device.time_provider,
        )
        .await?;
        match claimer_step {
            invite_greeter_step::ClaimerStep::Number4SignifyTrust => {}
            _ => return Err(anyhow::anyhow!("Unexpected claimer step: {:?}", claimer_step).into()),
        };

        Ok(BaseGreetInProgress2Ctx {
            token: self.token,
            greeting_attempt: self.greeting_attempt,
            device: self.device,
            claimer_sas: self.claimer_sas,
            shared_secret_key: self.shared_secret_key,
            cmds: self.cmds,
            event_bus: self.event_bus,
        })
    }
}

#[derive(Debug)]
pub struct UserGreetInProgress1Ctx(BaseGreetInProgress1Ctx);

impl UserGreetInProgress1Ctx {
    pub fn greeter_sas(&self) -> &SASCode {
        &self.0.greeter_sas
    }

    pub fn canceller_ctx(&self) -> GreetCancellerCtx {
        GreetCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_wait_peer_trust(self) -> Result<UserGreetInProgress2Ctx, GreetInProgressError> {
        self.0
            .do_wait_peer_trust()
            .await
            .map(UserGreetInProgress2Ctx)
    }
}

#[derive(Debug)]
pub struct DeviceGreetInProgress1Ctx(BaseGreetInProgress1Ctx);

impl DeviceGreetInProgress1Ctx {
    pub fn greeter_sas(&self) -> &SASCode {
        &self.0.greeter_sas
    }

    pub fn canceller_ctx(&self) -> GreetCancellerCtx {
        GreetCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_wait_peer_trust(
        self,
    ) -> Result<DeviceGreetInProgress2Ctx, GreetInProgressError> {
        self.0
            .do_wait_peer_trust()
            .await
            .map(DeviceGreetInProgress2Ctx)
    }
}

#[derive(Debug)]
pub struct ShamirRecoveryGreetInProgress1Ctx {
    base: BaseGreetInProgress1Ctx,
    share_data: ShamirRecoveryShareData,
}

impl ShamirRecoveryGreetInProgress1Ctx {
    pub fn greeter_sas(&self) -> &SASCode {
        &self.base.greeter_sas
    }

    pub fn canceller_ctx(&self) -> GreetCancellerCtx {
        GreetCancellerCtx::new(self.base.greeting_attempt, self.base.cmds.clone())
    }

    pub async fn do_wait_peer_trust(
        self,
    ) -> Result<ShamirRecoveryGreetInProgress2Ctx, GreetInProgressError> {
        self.base
            .do_wait_peer_trust()
            .await
            .map(|base| ShamirRecoveryGreetInProgress2Ctx {
                base,
                share_data: self.share_data,
            })
    }
}

// GreetInProgress2Ctx

#[derive(Debug)]
struct BaseGreetInProgress2Ctx {
    token: InvitationToken,
    greeting_attempt: GreetingAttemptID,
    device: Arc<LocalDevice>,
    claimer_sas: SASCode,
    shared_secret_key: SecretKey,
    cmds: Arc<AuthenticatedCmds>,
    event_bus: EventBus,
}

impl BaseGreetInProgress2Ctx {
    fn generate_claimer_sas_choices(&self, size: usize) -> Vec<SASCode> {
        SASCode::generate_sas_code_candidates(&self.claimer_sas, size)
    }

    async fn do_deny_trust(&self) -> Result<(), GreetInProgressError> {
        cancel_greeting_attempt(
            &self.cmds,
            self.greeting_attempt,
            CancelledGreetingAttemptReason::InvalidSasCode,
        )
        .await
    }

    async fn do_signify_trust(self) -> Result<BaseGreetInProgress3Ctx, GreetInProgressError> {
        let claimer_step = run_greeter_step_until_ready(
            &self.cmds,
            self.greeting_attempt,
            invite_greeter_step::GreeterStep::Number5SignifyTrust,
            &self.device.time_provider,
        )
        .await?;
        match claimer_step {
            invite_greeter_step::ClaimerStep::Number5WaitPeerTrust => {}
            _ => return Err(anyhow::anyhow!("Unexpected claimer step: {:?}", claimer_step).into()),
        };

        Ok(BaseGreetInProgress3Ctx {
            token: self.token,
            greeting_attempt: self.greeting_attempt,
            device: self.device,
            shared_secret_key: self.shared_secret_key,
            cmds: self.cmds,
            event_bus: self.event_bus,
        })
    }
}

#[derive(Debug)]
pub struct UserGreetInProgress2Ctx(BaseGreetInProgress2Ctx);

impl UserGreetInProgress2Ctx {
    pub fn claimer_sas(&self) -> &SASCode {
        &self.0.claimer_sas
    }

    pub fn generate_claimer_sas_choices(&self, size: usize) -> Vec<SASCode> {
        self.0.generate_claimer_sas_choices(size)
    }

    pub fn canceller_ctx(&self) -> GreetCancellerCtx {
        GreetCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_deny_trust(self) -> Result<(), GreetInProgressError> {
        self.0.do_deny_trust().await
    }

    pub async fn do_signify_trust(self) -> Result<UserGreetInProgress3Ctx, GreetInProgressError> {
        self.0.do_signify_trust().await.map(UserGreetInProgress3Ctx)
    }
}

#[derive(Debug)]
pub struct DeviceGreetInProgress2Ctx(BaseGreetInProgress2Ctx);

impl DeviceGreetInProgress2Ctx {
    pub fn claimer_sas(&self) -> &SASCode {
        &self.0.claimer_sas
    }

    pub fn generate_claimer_sas_choices(&self, size: usize) -> Vec<SASCode> {
        self.0.generate_claimer_sas_choices(size)
    }

    pub fn canceller_ctx(&self) -> GreetCancellerCtx {
        GreetCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }
    pub async fn do_deny_trust(self) -> Result<(), GreetInProgressError> {
        self.0.do_deny_trust().await
    }

    pub async fn do_signify_trust(self) -> Result<DeviceGreetInProgress3Ctx, GreetInProgressError> {
        self.0
            .do_signify_trust()
            .await
            .map(DeviceGreetInProgress3Ctx)
    }
}

#[derive(Debug)]
pub struct ShamirRecoveryGreetInProgress2Ctx {
    base: BaseGreetInProgress2Ctx,
    share_data: ShamirRecoveryShareData,
}

impl ShamirRecoveryGreetInProgress2Ctx {
    pub fn claimer_sas(&self) -> &SASCode {
        &self.base.claimer_sas
    }

    pub fn generate_claimer_sas_choices(&self, size: usize) -> Vec<SASCode> {
        self.base.generate_claimer_sas_choices(size)
    }

    pub fn canceller_ctx(&self) -> GreetCancellerCtx {
        GreetCancellerCtx::new(self.base.greeting_attempt, self.base.cmds.clone())
    }
    pub async fn do_deny_trust(self) -> Result<(), GreetInProgressError> {
        self.base.do_deny_trust().await
    }

    pub async fn do_signify_trust(
        self,
    ) -> Result<ShamirRecoveryGreetInProgress3Ctx, GreetInProgressError> {
        self.base
            .do_signify_trust()
            .await
            .map(|base| ShamirRecoveryGreetInProgress3Ctx {
                base,
                share_data: self.share_data,
            })
    }
}

// GreetInProgress3Ctx

#[derive(Debug)]
struct BaseGreetInProgress3Ctx {
    token: InvitationToken,
    greeting_attempt: GreetingAttemptID,
    device: Arc<LocalDevice>,
    shared_secret_key: SecretKey,
    cmds: Arc<AuthenticatedCmds>,
    event_bus: EventBus,
}

#[derive(Debug)]
struct BaseGreetInProgress3WithPayloadCtx {
    token: InvitationToken,
    greeting_attempt: GreetingAttemptID,
    device: Arc<LocalDevice>,
    shared_secret_key: SecretKey,
    cmds: Arc<AuthenticatedCmds>,
    event_bus: EventBus,
    payload: Bytes,
}

impl BaseGreetInProgress3Ctx {
    async fn do_get_claim_requests(
        self,
    ) -> Result<BaseGreetInProgress3WithPayloadCtx, GreetInProgressError> {
        let claimer_payload = {
            let claimer_step = run_greeter_step_until_ready(
                &self.cmds,
                self.greeting_attempt,
                invite_greeter_step::GreeterStep::Number6GetPayload,
                &self.device.time_provider,
            )
            .await?;
            let result: Result<_, GreetInProgressError> = match claimer_step {
                invite_greeter_step::ClaimerStep::Number6SendPayload { claimer_payload } => {
                    Ok(claimer_payload)
                }
                _ => Err(anyhow::anyhow!("Unexpected claimer step: {:?}", claimer_step).into()),
            };
            result?
        };

        Ok(BaseGreetInProgress3WithPayloadCtx {
            token: self.token,
            greeting_attempt: self.greeting_attempt,
            device: self.device,
            shared_secret_key: self.shared_secret_key,
            cmds: self.cmds,
            event_bus: self.event_bus,
            payload: claimer_payload,
        })
    }
}

#[derive(Debug)]
pub struct UserGreetInProgress3Ctx(BaseGreetInProgress3Ctx);

impl UserGreetInProgress3Ctx {
    pub fn canceller_ctx(&self) -> GreetCancellerCtx {
        GreetCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_get_claim_requests(
        self,
    ) -> Result<UserGreetInProgress4Ctx, GreetInProgressError> {
        let ctx = self.0.do_get_claim_requests().await?;

        let data = match InviteUserData::decrypt_and_load(&ctx.payload, &ctx.shared_secret_key) {
            Ok(data) => data,
            Err(err) => {
                let reason = match err {
                    DataError::Decryption => CancelledGreetingAttemptReason::UndecipherablePayload,
                    DataError::BadSerialization { .. } => {
                        CancelledGreetingAttemptReason::UndeserializablePayload
                    }
                    _ => CancelledGreetingAttemptReason::InconsistentPayload,
                };
                cancel_greeting_attempt_and_warn_on_error(&ctx.cmds, ctx.greeting_attempt, reason)
                    .await;
                return Err(GreetInProgressError::CorruptedInviteUserData(err));
            }
        };

        Ok(UserGreetInProgress4Ctx {
            token: ctx.token,
            greeting_attempt: ctx.greeting_attempt,
            device: ctx.device,
            requested_device_label: data.requested_device_label,
            requested_human_handle: data.requested_human_handle,
            public_key: data.public_key,
            verify_key: data.verify_key,
            shared_secret_key: ctx.shared_secret_key,
            cmds: ctx.cmds,
            event_bus: ctx.event_bus,
        })
    }
}

#[derive(Debug)]
pub struct DeviceGreetInProgress3Ctx(BaseGreetInProgress3Ctx);

impl DeviceGreetInProgress3Ctx {
    pub fn canceller_ctx(&self) -> GreetCancellerCtx {
        GreetCancellerCtx::new(self.0.greeting_attempt, self.0.cmds.clone())
    }

    pub async fn do_get_claim_requests(
        self,
    ) -> Result<DeviceGreetInProgress4Ctx, GreetInProgressError> {
        let ctx = self.0.do_get_claim_requests().await?;

        let data = match InviteDeviceData::decrypt_and_load(&ctx.payload, &ctx.shared_secret_key) {
            Ok(data) => data,
            Err(err) => {
                let reason = match err {
                    DataError::Decryption => CancelledGreetingAttemptReason::UndecipherablePayload,
                    DataError::BadSerialization { .. } => {
                        CancelledGreetingAttemptReason::UndeserializablePayload
                    }
                    _ => CancelledGreetingAttemptReason::InconsistentPayload,
                };
                cancel_greeting_attempt_and_warn_on_error(&ctx.cmds, ctx.greeting_attempt, reason)
                    .await;
                return Err(GreetInProgressError::CorruptedInviteUserData(err));
            }
        };

        Ok(DeviceGreetInProgress4Ctx {
            token: ctx.token,
            greeting_attempt: ctx.greeting_attempt,
            device: ctx.device,
            requested_device_label: data.requested_device_label,
            verify_key: data.verify_key,
            shared_secret_key: ctx.shared_secret_key,
            cmds: ctx.cmds,
            event_bus: ctx.event_bus,
        })
    }
}

#[derive(Debug)]
pub struct ShamirRecoveryGreetInProgress3Ctx {
    base: BaseGreetInProgress3Ctx,
    share_data: ShamirRecoveryShareData,
}

impl ShamirRecoveryGreetInProgress3Ctx {
    pub fn canceller_ctx(&self) -> GreetCancellerCtx {
        GreetCancellerCtx::new(self.base.greeting_attempt, self.base.cmds.clone())
    }

    pub async fn do_send_share(self) -> Result<(), GreetInProgressError> {
        // Get claim request
        let ctx = self.base.do_get_claim_requests().await?;
        let _data = match InviteShamirRecoveryData::decrypt_and_load(
            &ctx.payload,
            &ctx.shared_secret_key,
        ) {
            Ok(data) => data,
            Err(err) => {
                let reason = match err {
                    DataError::Decryption => CancelledGreetingAttemptReason::UndecipherablePayload,
                    DataError::BadSerialization { .. } => {
                        CancelledGreetingAttemptReason::UndeserializablePayload
                    }
                    _ => CancelledGreetingAttemptReason::InconsistentPayload,
                };
                cancel_greeting_attempt_and_warn_on_error(&ctx.cmds, ctx.greeting_attempt, reason)
                    .await;
                return Err(GreetInProgressError::CorruptedInviteUserData(err));
            }
        };

        // Build the confirmation containing the weighted share
        let greeter_payload = InviteShamirRecoveryConfirmation {
            weighted_share: self.share_data.weighted_share.clone(),
        }
        .dump_and_encrypt(&ctx.shared_secret_key)
        .into();

        let claimer_step = run_greeter_step_until_ready(
            &ctx.cmds,
            ctx.greeting_attempt,
            invite_greeter_step::GreeterStep::Number7SendPayload { greeter_payload },
            &ctx.device.time_provider,
        )
        .await?;
        match claimer_step {
            invite_greeter_step::ClaimerStep::Number7GetPayload => {}
            _ => return Err(anyhow::anyhow!("Unexpected claimer step: {:?}", claimer_step).into()),
        };

        let claimer_step = run_greeter_step_until_ready(
            &ctx.cmds,
            ctx.greeting_attempt,
            invite_greeter_step::GreeterStep::Number8WaitPeerAcknowledgment,
            &ctx.device.time_provider,
        )
        .await?;
        match claimer_step {
            invite_greeter_step::ClaimerStep::Number8Acknowledge => {}
            _ => return Err(anyhow::anyhow!("Unexpected claimer step: {:?}", claimer_step).into()),
        };

        // Note that we don't call invite_complete here, as it is done on the claimer's side,
        // as we cannot know if the recovery process is over yet
        // See "invite_complete command" section in RFC 1011
        // https://github.com/Scille/parsec-cloud/blob/master/docs/rfcs/1011-non-blocking-invite-transport.md#invite_complete-command

        Ok(())
    }
}

/// Helper to prepare the creation of a new user.
fn create_new_signed_user_certificates(
    author: &LocalDevice,
    device_label: DeviceLabel,
    human_handle: HumanHandle,
    profile: UserProfile,
    public_key: PublicKey,
    verify_key: VerifyKey,
    timestamp: DateTime,
) -> (Bytes, Bytes, Bytes, Bytes, InviteUserConfirmation) {
    let (
        user_id,
        device_id,
        user_certificate_bytes,
        redacted_user_certificate_bytes,
        device_certificate_bytes,
        redacted_device_certificate_bytes,
    ) = create_user_and_device_certificates(
        author,
        device_label.clone(),
        human_handle.clone(),
        profile,
        public_key,
        verify_key,
        timestamp,
    );

    let invite_user_confirmation = InviteUserConfirmation {
        user_id,
        device_id,
        device_label,
        human_handle,
        profile,
        root_verify_key: author.root_verify_key().to_owned(),
    };

    (
        user_certificate_bytes,
        redacted_user_certificate_bytes,
        device_certificate_bytes,
        redacted_device_certificate_bytes,
        invite_user_confirmation,
    )
}

fn create_new_signed_device_certificates(
    author: &LocalDevice,
    device_label: DeviceLabel,
    verify_key: VerifyKey,
    timestamp: DateTime,
) -> (Bytes, Bytes, DeviceID) {
    let device_id = DeviceID::default();

    let device_certificate = DeviceCertificate {
        author: CertificateSigner::User(author.device_id),
        timestamp,
        purpose: DevicePurpose::Standard,
        user_id: author.user_id,
        device_id,
        device_label: MaybeRedacted::Real(device_label),
        verify_key: verify_key.clone(),
        algorithm: SigningKeyAlgorithm::Ed25519,
    };

    let redacted_device_certificate = DeviceCertificate {
        author: CertificateSigner::User(author.device_id),
        timestamp,
        purpose: DevicePurpose::Standard,
        user_id: author.user_id,
        device_id,
        device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(device_id)),
        verify_key,
        algorithm: SigningKeyAlgorithm::Ed25519,
    };

    let device_certificate_bytes = device_certificate.dump_and_sign(&author.signing_key);
    let redacted_device_certificate_bytes =
        redacted_device_certificate.dump_and_sign(&author.signing_key);

    (
        device_certificate_bytes.into(),
        redacted_device_certificate_bytes.into(),
        device_id,
    )
}

// GreetInProgress4Ctx

#[derive(Debug)]
pub struct UserGreetInProgress4Ctx {
    pub token: InvitationToken,
    pub greeting_attempt: GreetingAttemptID,
    pub requested_device_label: DeviceLabel,
    pub requested_human_handle: HumanHandle,
    device: Arc<LocalDevice>,
    public_key: PublicKey,
    verify_key: VerifyKey,
    shared_secret_key: SecretKey,
    cmds: Arc<AuthenticatedCmds>,
    event_bus: EventBus,
}

impl UserGreetInProgress4Ctx {
    pub fn canceller_ctx(&self) -> GreetCancellerCtx {
        GreetCancellerCtx::new(self.greeting_attempt, self.cmds.clone())
    }

    pub async fn do_create_new_user(
        self,
        device_label: DeviceLabel,
        human_handle: HumanHandle,
        profile: UserProfile,
    ) -> Result<(), GreetInProgressError> {
        let mut timestamp = self.device.time_provider.now();
        let invite_user_confirmation = loop {
            let (
                user_certificate,
                redacted_user_certificate,
                device_certificate,
                redacted_device_certificate,
                invite_user_confirmation,
            ) = create_new_signed_user_certificates(
                &self.device,
                device_label.clone(),
                human_handle.clone(),
                profile,
                self.public_key.clone(),
                self.verify_key.clone(),
                timestamp,
            );

            {
                use authenticated_cmds::latest::user_create::{Rep, Req};

                let rep = self
                    .cmds
                    .send(Req {
                        user_certificate,
                        device_certificate,
                        redacted_user_certificate,
                        redacted_device_certificate,
                    })
                    .await?;

                match rep {
                    Rep::Ok => Ok(()),
                    Rep::RequireGreaterTimestamp {
                        strictly_greater_than,
                    } => {
                        timestamp = greater_timestamp(
                            &self.device.time_provider,
                            GreaterTimestampOffset::User,
                            strictly_greater_than,
                        );
                        continue;
                    }
                    Rep::ActiveUsersLimitReached => {
                        Err(GreetInProgressError::ActiveUsersLimitReached)
                    }
                    Rep::HumanHandleAlreadyTaken => {
                        Err(GreetInProgressError::HumanHandleAlreadyTaken)
                    }
                    Rep::UserAlreadyExists => Err(GreetInProgressError::UserAlreadyExists),
                    Rep::AuthorNotAllowed => Err(GreetInProgressError::UserCreateNotAllowed),
                    Rep::TimestampOutOfBallpark {
                        server_timestamp,
                        client_timestamp,
                        ballpark_client_early_offset,
                        ballpark_client_late_offset,
                    } => {
                        let event = EventTooMuchDriftWithServerClock {
                            server_timestamp,
                            ballpark_client_early_offset,
                            ballpark_client_late_offset,
                            client_timestamp,
                        };
                        self.event_bus.send(&event);

                        Err(GreetInProgressError::TimestampOutOfBallpark {
                            server_timestamp,
                            client_timestamp,
                            ballpark_client_early_offset,
                            ballpark_client_late_offset,
                        })
                    }
                    bad_rep @ (Rep::UnknownStatus { .. } | Rep::InvalidCertificate) => {
                        Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                    }
                }?;
            }

            break invite_user_confirmation;
        };

        // From now on the user has been created on the server, but greeter
        // is not aware of it yet. If something goes wrong, we can end up with
        // the greeter losing it private keys.
        // This is considered acceptable given 1) the error window is small and
        // 2) if this occurs the inviter can revoke the user and retry the
        // enrollment process to fix this

        let greeter_payload = invite_user_confirmation
            .dump_and_encrypt(&self.shared_secret_key)
            .into();
        let claimer_step = run_greeter_step_until_ready(
            &self.cmds,
            self.greeting_attempt,
            invite_greeter_step::GreeterStep::Number7SendPayload { greeter_payload },
            &self.device.time_provider,
        )
        .await?;
        match claimer_step {
            invite_greeter_step::ClaimerStep::Number7GetPayload => {}
            _ => return Err(anyhow::anyhow!("Unexpected claimer step: {:?}", claimer_step).into()),
        };

        let claimer_step = run_greeter_step_until_ready(
            &self.cmds,
            self.greeting_attempt,
            invite_greeter_step::GreeterStep::Number8WaitPeerAcknowledgment,
            &self.device.time_provider,
        )
        .await?;
        match claimer_step {
            invite_greeter_step::ClaimerStep::Number8Acknowledge => {}
            _ => return Err(anyhow::anyhow!("Unexpected claimer step: {:?}", claimer_step).into()),
        };
        // Note that we call invite_complete here, as it not is done on the claimer's side.
        // See "invite_complete command" section in RFC 1011
        // https://github.com/Scille/parsec-cloud/blob/master/docs/rfcs/1011-non-blocking-invite-transport.md#invite_complete-command

        {
            use authenticated_cmds::latest::invite_complete::{Rep, Req};

            let rep = self.cmds.send(Req { token: self.token }).await?;

            match rep {
                Rep::Ok => Ok(()),
                Rep::InvitationAlreadyCompleted => Err(GreetInProgressError::AlreadyDeleted),
                Rep::InvitationCancelled => Err(GreetInProgressError::AlreadyDeleted),
                Rep::InvitationNotFound => Err(GreetInProgressError::NotFound),
                Rep::AuthorNotAllowed => Err(GreetInProgressError::GreeterNotAllowed),
                bad_rep @ Rep::UnknownStatus { .. } => {
                    Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                }
            }?
        };

        Ok(())
    }
}

#[derive(Debug)]
pub struct DeviceGreetInProgress4Ctx {
    pub token: InvitationToken,
    pub greeting_attempt: GreetingAttemptID,
    pub requested_device_label: DeviceLabel,
    device: Arc<LocalDevice>,
    verify_key: VerifyKey,
    shared_secret_key: SecretKey,
    cmds: Arc<AuthenticatedCmds>,
    #[allow(dead_code)]
    event_bus: EventBus,
}

impl DeviceGreetInProgress4Ctx {
    pub fn canceller_ctx(&self) -> GreetCancellerCtx {
        GreetCancellerCtx::new(self.greeting_attempt, self.cmds.clone())
    }

    pub async fn do_create_new_device(
        self,
        device_label: DeviceLabel,
    ) -> Result<(), GreetInProgressError> {
        let mut timestamp = self.device.time_provider.now();
        let device_id = loop {
            let (device_certificate_bytes, redacted_device_certificate_bytes, device_id) =
                create_new_signed_device_certificates(
                    &self.device,
                    device_label.clone(),
                    self.verify_key.clone(),
                    timestamp,
                );

            {
                use authenticated_cmds::latest::device_create::{Rep, Req};

                let rep = self
                    .cmds
                    .send(Req {
                        device_certificate: device_certificate_bytes,
                        redacted_device_certificate: redacted_device_certificate_bytes,
                    })
                    .await?;

                match rep {
                    Rep::Ok => Ok(()),
                    Rep::RequireGreaterTimestamp {
                        strictly_greater_than,
                    } => {
                        timestamp = greater_timestamp(
                            &self.device.time_provider,
                            GreaterTimestampOffset::User,
                            strictly_greater_than,
                        );
                        continue;
                    }
                    Rep::DeviceAlreadyExists => Err(GreetInProgressError::DeviceAlreadyExists),
                    Rep::TimestampOutOfBallpark {
                        server_timestamp,
                        client_timestamp,
                        ballpark_client_early_offset,
                        ballpark_client_late_offset,
                    } => Err(GreetInProgressError::TimestampOutOfBallpark {
                        server_timestamp,
                        client_timestamp,
                        ballpark_client_early_offset,
                        ballpark_client_late_offset,
                    }),
                    bad_rep @ (Rep::UnknownStatus { .. } | Rep::InvalidCertificate) => {
                        Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                    }
                }?;

                break device_id;
            }
        };

        // From now on the device has been created on the server, but greeter
        // is not aware of it yet. If something goes wrong, we can end up with
        // the greeter losing it private keys.
        // This is considered acceptable given 1) the error window is small and
        // 2) if this occurs the inviter can revoke the device and retry the
        // enrollment process to fix this

        let greeter_payload = InviteDeviceConfirmation {
            user_id: self.device.user_id,
            device_id,
            device_label,
            human_handle: self.device.human_handle.clone(),
            profile: self.device.initial_profile,
            private_key: self.device.private_key.clone(),
            user_realm_id: self.device.user_realm_id,
            user_realm_key: self.device.user_realm_key.clone(),
            root_verify_key: self.device.root_verify_key().clone(),
        }
        .dump_and_encrypt(&self.shared_secret_key)
        .into();

        let claimer_step = run_greeter_step_until_ready(
            &self.cmds,
            self.greeting_attempt,
            invite_greeter_step::GreeterStep::Number7SendPayload { greeter_payload },
            &self.device.time_provider,
        )
        .await?;
        match claimer_step {
            invite_greeter_step::ClaimerStep::Number7GetPayload => {}
            _ => return Err(anyhow::anyhow!("Unexpected claimer step: {:?}", claimer_step).into()),
        };

        let claimer_step = run_greeter_step_until_ready(
            &self.cmds,
            self.greeting_attempt,
            invite_greeter_step::GreeterStep::Number8WaitPeerAcknowledgment,
            &self.device.time_provider,
        )
        .await?;
        match claimer_step {
            invite_greeter_step::ClaimerStep::Number8Acknowledge => {}
            _ => return Err(anyhow::anyhow!("Unexpected claimer step: {:?}", claimer_step).into()),
        };
        // Note that we call invite_complete here, as it not is done on the claimer's side.
        // See "invite_complete command" section in RFC 1011
        // https://github.com/Scille/parsec-cloud/blob/master/docs/rfcs/1011-non-blocking-invite-transport.md#invite_complete-command
        {
            use authenticated_cmds::latest::invite_complete::{Rep, Req};

            let rep = self.cmds.send(Req { token: self.token }).await?;

            match rep {
                Rep::Ok => Ok(()),
                Rep::InvitationAlreadyCompleted => Err(GreetInProgressError::AlreadyDeleted),
                Rep::InvitationCancelled => Err(GreetInProgressError::AlreadyDeleted),
                Rep::InvitationNotFound => Err(GreetInProgressError::NotFound),
                Rep::AuthorNotAllowed => Err(GreetInProgressError::GreeterNotAllowed),
                bad_rep @ Rep::UnknownStatus { .. } => {
                    Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                }
            }?
        };

        Ok(())
    }
}
