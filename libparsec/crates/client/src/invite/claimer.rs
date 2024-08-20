// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use invited_cmds::latest::invite_claimer_step;
use libparsec_client_connection::{protocol::invited_cmds, ConnectionError, InvitedCmds};
use libparsec_types::prelude::*;

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

// Greeter step helper

static STEP_THROTTLE: Duration = Duration::seconds(1);

async fn run_claimer_step_until_ready(
    cmds: &InvitedCmds,
    greeting_attempt: GreetingAttemptID,
    claimer_step: invite_claimer_step::ClaimerStep,
) -> Result<invite_claimer_step::GreeterStep, ClaimInProgressError> {
    let time_provider = TimeProvider::default();
    let mut last_call = Option::None;
    let req = invite_claimer_step::Req {
        greeting_attempt,
        claimer_step,
    };

    // Loop over the requests
    loop {
        // Throttle the requests
        if let Some(last_call) = last_call {
            let duration = last_call + STEP_THROTTLE - time_provider.now();
            time_provider.sleep(duration).await;
        }
        last_call = Some(time_provider.now());

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

pub async fn claimer_retrieve_info(
    config: Arc<ClientConfig>,
    addr: ParsecInvitationAddr,
) -> Result<UserOrDeviceClaimInitialCtx, ClaimerRetrieveInfoError> {
    use invited_cmds::latest::invite_info::{Rep, Req, UserOrDevice};

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
            ))),
            UserOrDevice::Device {
                greeter_user_id,
                greeter_human_handle,
            } => Ok(UserOrDeviceClaimInitialCtx::Device(
                DeviceClaimInitialCtx::new(config, cmds, greeter_user_id, greeter_human_handle),
            )),
        },
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[derive(Debug)]
struct BaseClaimInitialCtx {
    config: Arc<ClientConfig>,
    cmds: Arc<InvitedCmds>,
    greeter_user_id: UserID,
    greeter_human_handle: HumanHandle,
}

impl BaseClaimInitialCtx {
    async fn do_wait_peer(self) -> Result<BaseClaimInProgress1Ctx, ClaimInProgressError> {
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

        Ok(BaseClaimInProgress1Ctx {
            config: self.config,
            cmds: self.cmds,
            greeting_attempt,
            greeter_sas,
            claimer_sas,
            shared_secret_key,
        })
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
    ) -> Self {
        Self {
            base: BaseClaimInitialCtx {
                config,
                cmds,
                greeter_user_id,
                greeter_human_handle,
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
    ) -> Self {
        Self(BaseClaimInitialCtx {
            config,
            cmds,
            greeter_user_id,
            greeter_human_handle,
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
}

impl BaseClaimInProgress1Ctx {
    fn generate_greeter_sas_choices(&self, size: usize) -> Vec<SASCode> {
        SASCode::generate_sas_code_candidates(&self.greeter_sas, size)
    }

    async fn do_signify_trust(self) -> Result<BaseClaimInProgress2Ctx, ClaimInProgressError> {
        let greeter_step = run_claimer_step_until_ready(
            &self.cmds,
            self.greeting_attempt,
            invite_claimer_step::ClaimerStep::Number4SignifyTrust,
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
}

impl BaseClaimInProgress2Ctx {
    async fn do_wait_peer_trust(self) -> Result<BaseClaimInProgress3Ctx, ClaimInProgressError> {
        let greeter_step = run_claimer_step_until_ready(
            &self.cmds,
            self.greeting_attempt,
            invite_claimer_step::ClaimerStep::Number5WaitPeerTrust,
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
        })
    }
}

#[derive(Debug)]
pub struct UserClaimInProgress2Ctx(BaseClaimInProgress2Ctx);

impl UserClaimInProgress2Ctx {
    pub fn claimer_sas(&self) -> &SASCode {
        &self.0.claimer_sas
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
        } = InviteUserConfirmation::decrypt_and_load(&payload, &self.0.shared_secret_key)
            .map_err(ClaimInProgressError::CorruptedConfirmation)?;

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
        } = InviteDeviceConfirmation::decrypt_and_load(&payload, &self.0.shared_secret_key)
            .map_err(ClaimInProgressError::CorruptedConfirmation)?;

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
            time_provider: Default::default(),
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
