// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_client_types::LocalDevice;
use libparsec_crypto::{generate_nonce, HashDigest, PrivateKey, PublicKey, SecretKey, VerifyKey};
use libparsec_protocol::authenticated_cmds::v2::*;
use libparsec_types::{
    CertificateSignerOwned, DeviceCertificate, DeviceID, DeviceLabel, DeviceName, HumanHandle,
    InvitationToken, InviteDeviceConfirmation, InviteDeviceData, InviteUserConfirmation,
    InviteUserData, SASCode, UserCertificate, UserProfile,
};

use crate::{InviteError, InviteResult};

// GreetInitialCtx

// TODO: can cmds be replaced by reference if python's binding is removed ?
#[derive(Debug)]
struct BaseGreetInitialCtx {
    token: InvitationToken,
    cmds: AuthenticatedCmds,
}

impl BaseGreetInitialCtx {
    async fn do_wait_peer(&self) -> InviteResult<(SASCode, SASCode, SecretKey)> {
        let greeter_private_key = PrivateKey::generate();
        let rep = self
            .cmds
            .send(invite_1_greeter_wait_peer::Req {
                greeter_public_key: greeter_private_key.public_key(),
                token: self.token,
            })
            .await?;

        let claimer_public_key = match rep {
            invite_1_greeter_wait_peer::Rep::AlreadyDeleted => {
                return Err(InviteError::AlreadyUsed)
            }
            invite_1_greeter_wait_peer::Rep::InvalidState => return Err(InviteError::PeerReset),
            invite_1_greeter_wait_peer::Rep::NotFound => return Err(InviteError::NotFound),
            invite_1_greeter_wait_peer::Rep::Ok { claimer_public_key } => claimer_public_key,
            invite_1_greeter_wait_peer::Rep::UnknownStatus { unknown_status, .. } => {
                return Err(InviteError::Custom(format!(
                    "Backend error during step 1: {unknown_status}"
                )))
            }
        };

        let shared_secret_key = greeter_private_key.generate_shared_secret_key(&claimer_public_key);
        let greeter_nonce = generate_nonce();

        let rep = self
            .cmds
            .send(invite_2a_greeter_get_hashed_nonce::Req { token: self.token })
            .await?;

        let claimer_hashed_nonce = match rep {
            invite_2a_greeter_get_hashed_nonce::Rep::AlreadyDeleted => {
                return Err(InviteError::AlreadyUsed)
            }
            invite_2a_greeter_get_hashed_nonce::Rep::InvalidState => {
                return Err(InviteError::PeerReset)
            }
            invite_2a_greeter_get_hashed_nonce::Rep::NotFound => return Err(InviteError::NotFound),
            invite_2a_greeter_get_hashed_nonce::Rep::Ok {
                claimer_hashed_nonce,
            } => claimer_hashed_nonce,
            invite_2a_greeter_get_hashed_nonce::Rep::UnknownStatus { unknown_status, .. } => {
                return Err(InviteError::Custom(format!(
                    "Backend error during step 2a: {unknown_status}"
                )))
            }
        };

        let rep = self
            .cmds
            .send(invite_2b_greeter_send_nonce::Req {
                greeter_nonce: greeter_nonce.clone(),
                token: self.token,
            })
            .await?;

        let claimer_nonce = match rep {
            invite_2b_greeter_send_nonce::Rep::AlreadyDeleted => {
                return Err(InviteError::AlreadyUsed)
            }
            invite_2b_greeter_send_nonce::Rep::InvalidState => return Err(InviteError::PeerReset),
            invite_2b_greeter_send_nonce::Rep::NotFound => return Err(InviteError::NotFound),
            invite_2b_greeter_send_nonce::Rep::Ok { claimer_nonce } => claimer_nonce,
            invite_2b_greeter_send_nonce::Rep::UnknownStatus { unknown_status, .. } => {
                return Err(InviteError::Custom(format!(
                    "Backend error during step 2b: {unknown_status}"
                )))
            }
        };

        if HashDigest::from_data(&claimer_nonce) != claimer_hashed_nonce {
            return Err(InviteError::Custom(
                "Invitee nonce and hashed nonce doesn't match".into(),
            ));
        }

        let (claimer_sas, greeter_sas) =
            SASCode::generate_sas_codes(&claimer_nonce, &greeter_nonce, &shared_secret_key);

        Ok((claimer_sas, greeter_sas, shared_secret_key))
    }
}

#[derive(Debug)]
pub struct UserGreetInitialCtx(BaseGreetInitialCtx);

impl UserGreetInitialCtx {
    pub fn new(cmds: AuthenticatedCmds, token: InvitationToken) -> Self {
        Self(BaseGreetInitialCtx { cmds, token })
    }

    pub async fn do_wait_peer(self) -> InviteResult<UserGreetInProgress1Ctx> {
        let (claimer_sas, greeter_sas, shared_secret_key) = self.0.do_wait_peer().await?;

        Ok(UserGreetInProgress1Ctx(BaseGreetInProgress1Ctx {
            token: self.0.token,
            greeter_sas,
            claimer_sas,
            shared_secret_key,
            cmds: self.0.cmds,
        }))
    }
}

#[derive(Debug)]
pub struct DeviceGreetInitialCtx(BaseGreetInitialCtx);

impl DeviceGreetInitialCtx {
    pub fn new(cmds: AuthenticatedCmds, token: InvitationToken) -> Self {
        Self(BaseGreetInitialCtx { cmds, token })
    }

    pub async fn do_wait_peer(self) -> InviteResult<DeviceGreetInProgress1Ctx> {
        let (claimer_sas, greeter_sas, shared_secret_key) = self.0.do_wait_peer().await?;

        Ok(DeviceGreetInProgress1Ctx(BaseGreetInProgress1Ctx {
            token: self.0.token,
            greeter_sas,
            claimer_sas,
            shared_secret_key,
            cmds: self.0.cmds,
        }))
    }
}

// GreetInProgress1Ctx

#[derive(Debug)]
struct BaseGreetInProgress1Ctx {
    token: InvitationToken,
    greeter_sas: SASCode,
    claimer_sas: SASCode,
    shared_secret_key: SecretKey,
    cmds: AuthenticatedCmds,
}

impl BaseGreetInProgress1Ctx {
    async fn do_wait_peer_trust(&self) -> InviteResult<()> {
        let rep = self
            .cmds
            .send(invite_3a_greeter_wait_peer_trust::Req { token: self.token })
            .await?;

        match rep {
            invite_3a_greeter_wait_peer_trust::Rep::AlreadyDeleted => Err(InviteError::AlreadyUsed),
            invite_3a_greeter_wait_peer_trust::Rep::InvalidState => Err(InviteError::PeerReset),
            invite_3a_greeter_wait_peer_trust::Rep::NotFound => Err(InviteError::NotFound),
            invite_3a_greeter_wait_peer_trust::Rep::Ok => Ok(()),
            invite_3a_greeter_wait_peer_trust::Rep::UnknownStatus { unknown_status, .. } => Err(
                InviteError::Custom(format!("Backend error during step 3a: {unknown_status}")),
            ),
        }
    }
}

#[derive(Debug)]
pub struct UserGreetInProgress1Ctx(BaseGreetInProgress1Ctx);

impl UserGreetInProgress1Ctx {
    pub fn greeter_sas(&self) -> &SASCode {
        &self.0.greeter_sas
    }

    pub async fn do_wait_peer_trust(self) -> InviteResult<UserGreetInProgress2Ctx> {
        self.0.do_wait_peer_trust().await?;

        Ok(UserGreetInProgress2Ctx(BaseGreetInProgress2Ctx {
            token: self.0.token,
            claimer_sas: self.0.claimer_sas,
            shared_secret_key: self.0.shared_secret_key,
            cmds: self.0.cmds,
        }))
    }
}

#[derive(Debug)]
pub struct DeviceGreetInProgress1Ctx(BaseGreetInProgress1Ctx);

impl DeviceGreetInProgress1Ctx {
    pub fn greeter_sas(&self) -> &SASCode {
        &self.0.greeter_sas
    }

    pub async fn do_wait_peer_trust(self) -> InviteResult<DeviceGreetInProgress2Ctx> {
        self.0.do_wait_peer_trust().await?;

        Ok(DeviceGreetInProgress2Ctx(BaseGreetInProgress2Ctx {
            token: self.0.token,
            claimer_sas: self.0.claimer_sas,
            shared_secret_key: self.0.shared_secret_key,
            cmds: self.0.cmds,
        }))
    }
}

// GreetInProgress2Ctx

#[derive(Debug)]
struct BaseGreetInProgress2Ctx {
    token: InvitationToken,
    claimer_sas: SASCode,
    shared_secret_key: SecretKey,
    cmds: AuthenticatedCmds,
}

impl BaseGreetInProgress2Ctx {
    fn generate_claimer_sas_choices(&self, size: usize) -> Vec<SASCode> {
        SASCode::generate_sas_code_candidates(&self.claimer_sas, size)
    }

    async fn do_signify_trust(&self) -> InviteResult<()> {
        let rep = self
            .cmds
            .send(invite_3b_greeter_signify_trust::Req { token: self.token })
            .await?;

        match rep {
            invite_3b_greeter_signify_trust::Rep::AlreadyDeleted => Err(InviteError::AlreadyUsed),
            invite_3b_greeter_signify_trust::Rep::InvalidState => Err(InviteError::PeerReset),
            invite_3b_greeter_signify_trust::Rep::NotFound => Err(InviteError::NotFound),
            invite_3b_greeter_signify_trust::Rep::Ok => Ok(()),
            invite_3b_greeter_signify_trust::Rep::UnknownStatus { unknown_status, .. } => Err(
                InviteError::Custom(format!("Backend error during step 3b: {unknown_status}")),
            ),
        }
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

    pub async fn do_signify_trust(self) -> InviteResult<UserGreetInProgress3Ctx> {
        self.0.do_signify_trust().await?;

        Ok(UserGreetInProgress3Ctx(BaseGreetInProgress3Ctx {
            token: self.0.token,
            shared_secret_key: self.0.shared_secret_key,
            cmds: self.0.cmds,
        }))
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

    pub async fn do_signify_trust(self) -> InviteResult<DeviceGreetInProgress3Ctx> {
        self.0.do_signify_trust().await?;

        Ok(DeviceGreetInProgress3Ctx(BaseGreetInProgress3Ctx {
            token: self.0.token,
            shared_secret_key: self.0.shared_secret_key,
            cmds: self.0.cmds,
        }))
    }
}

// GreetInProgress3Ctx

#[derive(Debug)]
struct BaseGreetInProgress3Ctx {
    token: InvitationToken,
    shared_secret_key: SecretKey,
    cmds: AuthenticatedCmds,
}

impl BaseGreetInProgress3Ctx {
    async fn do_get_claim_requests(&self) -> InviteResult<Vec<u8>> {
        let rep = self
            .cmds
            .send(invite_4_greeter_communicate::Req {
                token: self.token,
                payload: vec![],
            })
            .await?;

        let payload = match rep {
            invite_4_greeter_communicate::Rep::AlreadyDeleted => {
                return Err(InviteError::AlreadyUsed)
            }
            invite_4_greeter_communicate::Rep::InvalidState => return Err(InviteError::PeerReset),
            invite_4_greeter_communicate::Rep::NotFound => return Err(InviteError::NotFound),
            invite_4_greeter_communicate::Rep::Ok { payload } => payload,
            invite_4_greeter_communicate::Rep::UnknownStatus { unknown_status, .. } => {
                return Err(InviteError::Custom(format!(
                    "Backend error during step 4 (data exchange): {unknown_status}"
                )))
            }
        };

        if payload.is_empty() {
            return Err(InviteError::Custom("Missing InviteUserData payload".into()));
        }

        Ok(payload)
    }
}

#[derive(Debug)]
pub struct UserGreetInProgress3Ctx(BaseGreetInProgress3Ctx);

impl UserGreetInProgress3Ctx {
    pub async fn do_get_claim_requests(self) -> InviteResult<UserGreetInProgress4Ctx> {
        let payload = self.0.do_get_claim_requests().await?;

        let data = InviteUserData::decrypt_and_load(&payload, &self.0.shared_secret_key)
            .map_err(|e| InviteError::Custom(e.to_string()))?;

        Ok(UserGreetInProgress4Ctx {
            token: self.0.token,
            requested_device_label: data.requested_device_label,
            requested_human_handle: data.requested_human_handle,
            public_key: data.public_key,
            verify_key: data.verify_key,
            shared_secret_key: self.0.shared_secret_key,
            cmds: self.0.cmds,
        })
    }
}

#[derive(Debug)]
pub struct DeviceGreetInProgress3Ctx(BaseGreetInProgress3Ctx);

impl DeviceGreetInProgress3Ctx {
    pub async fn do_get_claim_requests(self) -> InviteResult<DeviceGreetInProgress4Ctx> {
        let payload = self.0.do_get_claim_requests().await?;

        let data = InviteDeviceData::decrypt_and_load(&payload, &self.0.shared_secret_key)
            .map_err(|e| InviteError::Custom(e.to_string()))?;

        Ok(DeviceGreetInProgress4Ctx {
            token: self.0.token,
            requested_device_label: data.requested_device_label,
            verify_key: data.verify_key,
            shared_secret_key: self.0.shared_secret_key,
            cmds: self.0.cmds,
        })
    }
}

/// Helper to prepare the creation of a new user.
fn create_new_user_certificates(
    author: &LocalDevice,
    device_label: Option<DeviceLabel>,
    human_handle: Option<HumanHandle>,
    profile: UserProfile,
    public_key: PublicKey,
    verify_key: VerifyKey,
) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, InviteUserConfirmation) {
    let device_id = DeviceID::default();
    let timestamp = author.now();

    let user_certificate = UserCertificate {
        author: CertificateSignerOwned::User(author.device_id.clone()),
        timestamp,
        user_id: device_id.user_id().clone(),
        human_handle: human_handle.clone(),
        public_key: public_key.clone(),
        profile,
    };

    let redacted_user_certificate = UserCertificate {
        author: CertificateSignerOwned::User(author.device_id.clone()),
        timestamp,
        user_id: device_id.user_id().clone(),
        human_handle: None,
        public_key,
        profile,
    };

    let device_certificate = DeviceCertificate {
        author: CertificateSignerOwned::User(author.device_id.clone()),
        timestamp,
        device_id: device_id.clone(),
        device_label: device_label.clone(),
        verify_key: verify_key.clone(),
    };

    let redacted_device_certificate = DeviceCertificate {
        author: CertificateSignerOwned::User(author.device_id.clone()),
        timestamp,
        device_id: device_id.clone(),
        device_label: None,
        verify_key,
    };

    let user_certificate_bytes = user_certificate.dump_and_sign(&author.signing_key);
    let redacted_user_certificate_bytes =
        redacted_user_certificate.dump_and_sign(&author.signing_key);
    let device_certificate_bytes = device_certificate.dump_and_sign(&author.signing_key);
    let redacted_device_certificate_bytes =
        redacted_device_certificate.dump_and_sign(&author.signing_key);

    let invite_user_confirmation = InviteUserConfirmation {
        device_id,
        device_label,
        human_handle,
        profile,
        root_verify_key: author.root_verify_key().clone(),
    };

    (
        user_certificate_bytes,
        redacted_user_certificate_bytes,
        device_certificate_bytes,
        redacted_device_certificate_bytes,
        invite_user_confirmation,
    )
}

// GreetInProgress4Ctx

#[derive(Debug)]
pub struct UserGreetInProgress4Ctx {
    pub token: InvitationToken,
    pub requested_device_label: Option<DeviceLabel>,
    pub requested_human_handle: Option<HumanHandle>,
    public_key: PublicKey,
    verify_key: VerifyKey,
    shared_secret_key: SecretKey,
    cmds: AuthenticatedCmds,
}

impl UserGreetInProgress4Ctx {
    pub async fn do_create_new_user(
        self,
        author: &LocalDevice,
        device_label: Option<DeviceLabel>,
        human_handle: Option<HumanHandle>,
        profile: UserProfile,
    ) -> InviteResult<()> {
        let (
            user_certificate,
            redacted_user_certificate,
            device_certificate,
            redacted_device_certificate,
            invite_user_confirmation,
        ) = create_new_user_certificates(
            author,
            device_label,
            human_handle,
            profile,
            self.public_key,
            self.verify_key,
        );

        let rep = self
            .cmds
            .send(user_create::Req {
                user_certificate,
                device_certificate,
                redacted_user_certificate,
                redacted_device_certificate,
            })
            .await?;

        match rep {
            user_create::Rep::ActiveUsersLimitReached { .. } => {
                return Err(InviteError::ActiveUsersLimitReached)
            }
            user_create::Rep::Ok => (),
            _ => {
                return Err(InviteError::Custom(format!(
                    "Backend error during step 4 (user certificates upload): {rep:?}"
                )))
            }
        }

        // From now on the user has been created on the server, but greeter
        // is not aware of it yet. If something goes wrong, we can end up with
        // the greeter losing it private keys.
        // This is considered acceptable given 1) the error window is small and
        // 2) if this occurs the inviter can revoke the user and retry the
        // enrollment process to fix this

        let payload = invite_user_confirmation.dump_and_encrypt(&self.shared_secret_key);

        let rep = self
            .cmds
            .send(invite_4_greeter_communicate::Req {
                token: self.token,
                payload,
            })
            .await?;

        match rep {
            invite_4_greeter_communicate::Rep::AlreadyDeleted => {
                return Err(InviteError::AlreadyUsed)
            }
            invite_4_greeter_communicate::Rep::InvalidState => return Err(InviteError::PeerReset),
            invite_4_greeter_communicate::Rep::NotFound => return Err(InviteError::NotFound),
            invite_4_greeter_communicate::Rep::Ok { .. } => (),
            invite_4_greeter_communicate::Rep::UnknownStatus { unknown_status, .. } => {
                return Err(InviteError::Custom(format!(
                    "Backend error during step 4 (confirmation exchange): {unknown_status}"
                )))
            }
        }

        // Invitation deletion is not strictly necessary (enrollment has succeeded
        // anyway) so it's no big deal if something goes wrong before it can be
        // done (and it can be manually deleted from invitation list).

        let _ = self
            .cmds
            .send(invite_delete::Req {
                token: self.token,
                reason: invite_delete::InvitationDeletedReason::Finished,
            })
            .await;

        Ok(())
    }
}

#[derive(Debug)]
pub struct DeviceGreetInProgress4Ctx {
    pub token: InvitationToken,
    pub requested_device_label: Option<DeviceLabel>,
    verify_key: VerifyKey,
    shared_secret_key: SecretKey,
    cmds: AuthenticatedCmds,
}

impl DeviceGreetInProgress4Ctx {
    pub async fn do_create_new_device(
        &self,
        author: &LocalDevice,
        device_label: Option<DeviceLabel>,
    ) -> InviteResult<()> {
        let device_id = author.user_id().to_device_id(DeviceName::default());
        let timestamp = author.now();

        let device_certificate = DeviceCertificate {
            author: CertificateSignerOwned::User(author.device_id.clone()),
            timestamp,
            device_id: device_id.clone(),
            device_label: device_label.clone(),
            verify_key: self.verify_key.clone(),
        };

        let redacted_device_certificate = DeviceCertificate {
            author: CertificateSignerOwned::User(author.device_id.clone()),
            timestamp,
            device_id: device_id.clone(),
            device_label: None,
            verify_key: self.verify_key.clone(),
        };

        let device_certificate_bytes = device_certificate.dump_and_sign(&author.signing_key);
        let redacted_device_certificate_bytes =
            redacted_device_certificate.dump_and_sign(&author.signing_key);

        let rep = self
            .cmds
            .send(device_create::Req {
                device_certificate: device_certificate_bytes,
                redacted_device_certificate: redacted_device_certificate_bytes,
            })
            .await?;

        match rep {
            device_create::Rep::Ok => (),
            _ => {
                return Err(InviteError::Custom(format!(
                    "Backend error during step 4 (device certificates upload): {rep:?}"
                )))
            }
        }

        // From now on the device has been created on the server, but greeter
        // is not aware of it yet. If something goes wrong, we can end up with
        // the greeter losing it private keys.
        // This is considered acceptable given 1) the error window is small and
        // 2) if this occurs the inviter can revoke the device and retry the
        // enrollment process to fix this

        let payload = InviteDeviceConfirmation {
            device_id,
            device_label,
            human_handle: author.human_handle.clone(),
            profile: author.profile,
            private_key: author.private_key.clone(),
            user_manifest_id: author.user_manifest_id,
            user_manifest_key: author.user_manifest_key.clone(),
            root_verify_key: author.root_verify_key().clone(),
        }
        .dump_and_encrypt(&self.shared_secret_key);

        let rep = self
            .cmds
            .send(invite_4_greeter_communicate::Req {
                token: self.token,
                payload,
            })
            .await?;

        match rep {
            invite_4_greeter_communicate::Rep::AlreadyDeleted => {
                return Err(InviteError::AlreadyUsed)
            }
            invite_4_greeter_communicate::Rep::InvalidState => return Err(InviteError::PeerReset),
            invite_4_greeter_communicate::Rep::NotFound => return Err(InviteError::NotFound),
            invite_4_greeter_communicate::Rep::Ok { .. } => (),
            invite_4_greeter_communicate::Rep::UnknownStatus { unknown_status, .. } => {
                return Err(InviteError::Custom(format!(
                    "Backend error during step 4 (confirmation exchange): {unknown_status}"
                )))
            }
        }

        // Invitation deletion is not strictly necessary (enrollment has succeeded
        // anyway) so it's no big deal if something goes wrong before it can be
        // done (and it can be manually deleted from invitation list).

        let _ = self
            .cmds
            .send(invite_delete::Req {
                token: self.token,
                reason: invite_delete::InvitationDeletedReason::Finished,
            })
            .await;

        Ok(())
    }
}
