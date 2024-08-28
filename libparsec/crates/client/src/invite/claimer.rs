// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use invited_cmds::latest::invite_claimer_step;
use libparsec_client_connection::{protocol::invited_cmds, ConnectionError, InvitedCmds};
use libparsec_types::prelude::*;

use crate::invite::common::Throttle;
use crate::ClientConfig;

#[derive(Debug, thiserror::Error)]
pub enum ClaimerRetrieveInfoError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Invitation not found")]
    NotFound,
    #[error("Invitation already used")]
    AlreadyUsed,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for ClaimerRetrieveInfoError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
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
    #[error("Invitation already used")]
    AlreadyUsed,
    #[error("Claim operation reset by peer")]
    PeerReset,
    #[error("Active users limit reached")]
    ActiveUsersLimitReached,
    #[error("The provided user is not allowed to greet this invitation")]
    GreeterNotAllowed,
    #[error("Greeting attempt cancelled by {origin:?} because {reason:?} on {timestamp}")]
    GreetingAttemptCancelled {
        origin: GreeterOrClaimer,
        reason: CancelledGreetingAttemptReason,
        timestamp: DateTime,
    },
    #[error(transparent)]
    CorruptedConfirmation(DataError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for ClaimInProgressError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            ConnectionError::InvitationAlreadyDeleted => Self::AlreadyUsed,
            ConnectionError::ExpiredOrganization => Self::OrganizationExpired,
            ConnectionError::InvitationNotFound => Self::NotFound,
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
        Rep::GreetingAttemptNotFound => Err(anyhow::anyhow!("Greeting attempt not found").into()),
        Rep::GreetingAttemptNotJoined => Err(anyhow::anyhow!("Greeting attempt not joined").into()),
        rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    }
}

// Greeter step helper

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
        let rep = cmds.send(req.clone()).await?;

        // Handle the response
        return match rep {
            invite_claimer_step::Rep::NotReady => continue,
            invite_claimer_step::Rep::Ok { greeter_step } => Ok(greeter_step),
            // Expected errors
            invite_claimer_step::Rep::GreeterNotAllowed => {
                Err(ClaimInProgressError::GreeterNotAllowed)
            }
            invite_claimer_step::Rep::GreeterRevoked => {
                Err(ClaimInProgressError::GreeterNotAllowed)
            }
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
            invite_claimer_step::Rep::GreetingAttemptNotFound => {
                Err(anyhow::anyhow!("Greeting attempt not found").into())
            }
            invite_claimer_step::Rep::GreetingAttemptNotJoined => {
                Err(anyhow::anyhow!("Greeting attempt not joined").into())
            }
            invite_claimer_step::Rep::StepMismatch => {
                Err(anyhow::anyhow!("Greeting attempt failed due to step mismatch").into())
            }
            invite_claimer_step::Rep::StepTooAdvanced => {
                Err(anyhow::anyhow!("Greeting attempt failed due to step too advanced").into())
            }
            rep @ invite_claimer_step::Rep::UnknownStatus { .. } => {
                Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
            }
        };
    }
}

#[derive(Debug)]
pub enum UserOrDeviceClaimInitialCtx {
    User(UserClaimInitialCtx),
    Device(DeviceClaimInitialCtx),
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
) -> Result<UserOrDeviceClaimInitialCtx, ClaimerRetrieveInfoError> {
    use invited_cmds::latest::invite_info::{Rep, Req, UserOrDevice};
    let time_provider = time_provider.unwrap_or_default();

    let cmds = Arc::new(
        InvitedCmds::new(&config.config_dir, addr, config.proxy.clone())
            .map_err(|e| anyhow::anyhow!("Error while configuring connection to server: {e}"))?,
    );

    let rep = cmds.send(Req).await?;

    match rep {
        Rep::Ok(claimer) => match claimer {
            UserOrDevice::User {
                claimer_email,
                greeter_user_id,
                greeter_human_handle,
            } => Ok(UserOrDeviceClaimInitialCtx::User(UserClaimInitialCtx::new(
                config,
                cmds,
                claimer_email,
                greeter_user_id,
                greeter_human_handle,
                time_provider,
            ))),
            UserOrDevice::Device {
                greeter_user_id,
                greeter_human_handle,
            } => Ok(UserOrDeviceClaimInitialCtx::Device(
                DeviceClaimInitialCtx::new(
                    config,
                    cmds,
                    greeter_user_id,
                    greeter_human_handle,
                    time_provider,
                ),
            )),
        },
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

// ClaimCancellerCtx

#[derive(Debug)]
pub struct ClaimCancellerCtx {
    greeting_attempt: GreetingAttemptID,
    cmds: Arc<InvitedCmds>,
}

impl ClaimCancellerCtx {
    pub async fn cancel(self) -> Result<(), ClaimInProgressError> {
        cancel_greeting_attempt(
            &self.cmds,
            self.greeting_attempt,
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
    async fn do_wait_peer(self) -> Result<BaseClaimInProgress1Ctx, ClaimInProgressError> {
        // Wait for the other peer
        let mut result = self._do_wait_peer().await;
        // If the attempt was automatically cancelled by the other peer, try again once.
        // Previous attempts are automatically cancelled when a new start greeting attempt is made.
        // This way, the peers can synchronize themselves more easily during the wait-peer phase,
        // without requiring the front-end to deal with it.
        if let Err(ClaimInProgressError::GreetingAttemptCancelled {
            origin: GreeterOrClaimer::Greeter,
            reason: CancelledGreetingAttemptReason::AutomaticallyCancelled,
            ..
        }) = result
        {
            result = self._do_wait_peer().await
        }
        // Move self into the next context
        let (greeting_attempt, greeter_sas, claimer_sas, shared_secret_key) = result?;
        Ok(BaseClaimInProgress1Ctx {
            config: self.config,
            cmds: self.cmds,
            greeting_attempt,
            greeter_sas,
            claimer_sas,
            shared_secret_key,
            time_provider: self.time_provider,
        })
    }

    async fn _do_wait_peer(
        &self,
    ) -> Result<(GreetingAttemptID, SASCode, SASCode, SecretKey), ClaimInProgressError> {
        let greeting_attempt = {
            use invited_cmds::latest::invite_claimer_start_greeting_attempt::{Rep, Req};
            let rep = self
                .cmds
                .send(Req {
                    greeter: self.greeter_user_id,
                })
                .await?;

            match rep {
                Rep::Ok { greeting_attempt } => Ok(greeting_attempt),
                Rep::GreeterNotFound => Err(ClaimInProgressError::NotFound),
                Rep::GreeterNotAllowed => Err(ClaimInProgressError::GreeterNotAllowed),
                Rep::GreeterRevoked => Err(ClaimInProgressError::GreeterNotAllowed),
                bad_rep @ Rep::UnknownStatus { .. } => {
                    Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                }
            }?
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

        let claimer_nonce = generate_nonce();
        let hashed_nonce = HashDigest::from_data(&claimer_nonce);
        let shared_secret_key = claimer_private_key.generate_shared_secret_key(&greeter_public_key);
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
}

#[derive(Debug)]
pub struct UserClaimInitialCtx {
    base: BaseClaimInitialCtx,
    pub claimer_email: String,
}

impl UserClaimInitialCtx {
    pub fn new(
        config: Arc<ClientConfig>,
        cmds: Arc<InvitedCmds>,
        claimer_email: String,
        greeter_user_id: UserID,
        greeter_human_handle: HumanHandle,
        time_provider: TimeProvider,
    ) -> Self {
        Self {
            base: BaseClaimInitialCtx {
                config,
                cmds,
                greeter_user_id,
                greeter_human_handle,
                time_provider,
            },
            claimer_email,
        }
    }

    pub fn greeter_user_id(&self) -> &UserID {
        &self.base.greeter_user_id
    }

    pub fn greeter_human_handle(&self) -> &HumanHandle {
        &self.base.greeter_human_handle
    }

    pub async fn do_wait_peer(self) -> Result<UserClaimInProgress1Ctx, ClaimInProgressError> {
        self.base.do_wait_peer().await.map(UserClaimInProgress1Ctx)
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
        Self(BaseClaimInitialCtx {
            config,
            cmds,
            greeter_user_id,
            greeter_human_handle,
            time_provider,
        })
    }

    pub fn greeter_user_id(&self) -> &UserID {
        &self.0.greeter_user_id
    }

    pub fn greeter_human_handle(&self) -> &HumanHandle {
        &self.0.greeter_human_handle
    }

    pub async fn do_wait_peer(self) -> Result<DeviceClaimInProgress1Ctx, ClaimInProgressError> {
        self.0.do_wait_peer().await.map(DeviceClaimInProgress1Ctx)
    }
}

#[derive(Debug)]
struct BaseClaimInProgress1Ctx {
    config: Arc<ClientConfig>,
    cmds: Arc<InvitedCmds>,
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
    pub fn greeter_sas(&self) -> &SASCode {
        &self.0.greeter_sas
    }

    pub fn generate_greeter_sas_choices(&self, size: usize) -> Vec<SASCode> {
        self.0.generate_greeter_sas_choices(size)
    }

    pub fn canceller_ctx(&self) -> ClaimCancellerCtx {
        ClaimCancellerCtx {
            greeting_attempt: self.0.greeting_attempt,
            cmds: self.0.cmds.clone(),
        }
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
    pub fn greeter_sas(&self) -> &SASCode {
        &self.0.greeter_sas
    }

    pub fn generate_greeter_sas_choices(&self, size: usize) -> Vec<SASCode> {
        self.0.generate_greeter_sas_choices(size)
    }

    pub fn canceller_ctx(&self) -> ClaimCancellerCtx {
        ClaimCancellerCtx {
            greeting_attempt: self.0.greeting_attempt,
            cmds: self.0.cmds.clone(),
        }
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
struct BaseClaimInProgress2Ctx {
    config: Arc<ClientConfig>,
    cmds: Arc<InvitedCmds>,
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
        ClaimCancellerCtx {
            greeting_attempt: self.0.greeting_attempt,
            cmds: self.0.cmds.clone(),
        }
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
        ClaimCancellerCtx {
            greeting_attempt: self.0.greeting_attempt,
            cmds: self.0.cmds.clone(),
        }
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
struct BaseClaimInProgress3Ctx {
    config: Arc<ClientConfig>,
    cmds: Arc<InvitedCmds>,
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
        let greeter_step = run_claimer_step_until_ready(
            &self.cmds,
            self.greeting_attempt,
            invite_claimer_step::ClaimerStep::Number8Acknowledge,
            &self.time_provider,
        )
        .await?;
        match greeter_step {
            invite_claimer_step::GreeterStep::Number8WaitPeerAcknowledgment => {}
            _ => return Err(anyhow::anyhow!("Unexpected greeter step: {:?}", greeter_step).into()),
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct UserClaimInProgress3Ctx(BaseClaimInProgress3Ctx);

impl UserClaimInProgress3Ctx {
    pub fn canceller_ctx(&self) -> ClaimCancellerCtx {
        ClaimCancellerCtx {
            greeting_attempt: self.0.greeting_attempt,
            cmds: self.0.cmds.clone(),
        }
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
                if cancel_greeting_attempt(&self.0.cmds, self.0.greeting_attempt, reason)
                    .await
                    .is_err()
                {
                    // TODO: Warn about the error before discarding it
                };
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
        ClaimCancellerCtx {
            greeting_attempt: self.0.greeting_attempt,
            cmds: self.0.cmds.clone(),
        }
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
                if cancel_greeting_attempt(&self.0.cmds, self.0.greeting_attempt, reason)
                    .await
                    .is_err()
                {
                    // TODO: Warn about the error before discarding it
                };
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
pub struct UserClaimFinalizeCtx {
    config: Arc<ClientConfig>,
    pub new_local_device: Arc<LocalDevice>,
}

impl UserClaimFinalizeCtx {
    pub fn get_default_key_file(&self) -> PathBuf {
        libparsec_platform_device_loader::get_default_key_file(
            &self.config.config_dir,
            &self.new_local_device.device_id,
        )
    }

    pub async fn save_local_device(
        self,
        access: &DeviceAccessStrategy,
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
            access,
            &self.new_local_device,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Error while saving the device file: {e}"))
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
            &self.new_local_device.device_id,
        )
    }

    pub async fn save_local_device(
        self,
        access: &DeviceAccessStrategy,
    ) -> Result<AvailableDevice, anyhow::Error> {
        libparsec_platform_device_loader::save_device(
            &self.config.config_dir,
            access,
            &self.new_local_device,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Error while saving the device file: {e}"))
    }
}
