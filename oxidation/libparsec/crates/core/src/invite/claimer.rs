// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client_connection::InvitedCmds;
use libparsec_crypto::{generate_nonce, HashDigest, PrivateKey, SecretKey, SigningKey};
use libparsec_protocol::invited_cmds::v2::*;
use libparsec_types::{
    BackendOrganizationAddr, DeviceLabel, HumanHandle, InviteDeviceConfirmation, InviteDeviceData,
    InviteUserConfirmation, InviteUserData, LocalDevice, SASCode, UserID,
};

use crate::{InviteError, InviteResult};

pub async fn claimer_retrieve_info(cmds: &InvitedCmds) -> InviteResult<invite_info::UserOrDevice> {
    let rep = cmds.send(invite_info::Req).await?;

    match rep {
        invite_info::Rep::Ok(claimer) => Ok(claimer),
        invite_info::Rep::UnknownStatus { unknown_status, .. } => Err(InviteError::Custom(
            format!("Backend error during invitation retrieval: {unknown_status}"),
        )),
    }
}

#[derive(Debug)]
struct BaseClaimInitialCtx {
    greeter_user_id: UserID,
    greeter_human_handle: Option<HumanHandle>,
    cmds: InvitedCmds,
}

impl BaseClaimInitialCtx {
    async fn do_wait_peer(self) -> InviteResult<BaseClaimInProgress1Ctx> {
        let claimer_private_key = PrivateKey::generate();

        let rep = self
            .cmds
            .send(invite_1_claimer_wait_peer::Req {
                claimer_public_key: claimer_private_key.public_key(),
            })
            .await?;

        let greeter_public_key = match rep {
            invite_1_claimer_wait_peer::Rep::AlreadyDeleted => Err(InviteError::AlreadyUsed),
            invite_1_claimer_wait_peer::Rep::InvalidState => Err(InviteError::PeerReset),
            invite_1_claimer_wait_peer::Rep::NotFound => Err(InviteError::NotFound),
            invite_1_claimer_wait_peer::Rep::Ok { greeter_public_key } => Ok(greeter_public_key),
            invite_1_claimer_wait_peer::Rep::UnknownStatus { unknown_status, .. } => Err(
                InviteError::Custom(format!("Backend error during step 1: {unknown_status}")),
            ),
        }?;

        let shared_secret_key = claimer_private_key.generate_shared_secret_key(&greeter_public_key);
        let claimer_nonce = generate_nonce();

        let rep = self
            .cmds
            .send(invite_2a_claimer_send_hashed_nonce::Req {
                claimer_hashed_nonce: HashDigest::from_data(&claimer_nonce),
            })
            .await?;

        let greeter_nonce = match rep {
            invite_2a_claimer_send_hashed_nonce::Rep::AlreadyDeleted => {
                Err(InviteError::AlreadyUsed)
            }
            invite_2a_claimer_send_hashed_nonce::Rep::InvalidState => Err(InviteError::PeerReset),
            invite_2a_claimer_send_hashed_nonce::Rep::NotFound => Err(InviteError::NotFound),
            invite_2a_claimer_send_hashed_nonce::Rep::Ok { greeter_nonce } => Ok(greeter_nonce),
            invite_2a_claimer_send_hashed_nonce::Rep::UnknownStatus { unknown_status, .. } => Err(
                InviteError::Custom(format!("Backend error during step 2a: {unknown_status}")),
            ),
        }?;

        let (claimer_sas, greeter_sas) =
            SASCode::generate_sas_codes(&claimer_nonce, &greeter_nonce, &shared_secret_key);

        let rep = self
            .cmds
            .send(invite_2b_claimer_send_nonce::Req { claimer_nonce })
            .await?;

        match rep {
            invite_2b_claimer_send_nonce::Rep::AlreadyDeleted => Err(InviteError::AlreadyUsed),
            invite_2b_claimer_send_nonce::Rep::InvalidState => Err(InviteError::PeerReset),
            invite_2b_claimer_send_nonce::Rep::NotFound => Err(InviteError::NotFound),
            invite_2b_claimer_send_nonce::Rep::Ok => Ok(()),
            invite_2b_claimer_send_nonce::Rep::UnknownStatus { unknown_status, .. } => Err(
                InviteError::Custom(format!("Backend error during step 2b: {unknown_status}")),
            ),
        }?;

        Ok(BaseClaimInProgress1Ctx {
            greeter_sas,
            claimer_sas,
            shared_secret_key,
            cmds: self.cmds,
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
        cmds: InvitedCmds,
        claimer_email: String,
        greeter_user_id: UserID,
        greeter_human_handle: Option<HumanHandle>,
    ) -> Self {
        Self {
            base: BaseClaimInitialCtx {
                greeter_user_id,
                greeter_human_handle,
                cmds,
            },
            claimer_email,
        }
    }

    pub fn greeter_user_id(&self) -> &UserID {
        &self.base.greeter_user_id
    }

    pub fn greeter_human_handle(&self) -> &Option<HumanHandle> {
        &self.base.greeter_human_handle
    }

    pub async fn do_wait_peer(self) -> InviteResult<UserClaimInProgress1Ctx> {
        self.base.do_wait_peer().await.map(UserClaimInProgress1Ctx)
    }
}

#[derive(Debug)]
pub struct DeviceClaimInitialCtx(BaseClaimInitialCtx);

impl DeviceClaimInitialCtx {
    pub fn new(
        cmds: InvitedCmds,
        greeter_user_id: UserID,
        greeter_human_handle: Option<HumanHandle>,
    ) -> Self {
        Self(BaseClaimInitialCtx {
            greeter_user_id,
            greeter_human_handle,
            cmds,
        })
    }

    pub fn greeter_user_id(&self) -> &UserID {
        &self.0.greeter_user_id
    }

    pub fn greeter_human_handle(&self) -> &Option<HumanHandle> {
        &self.0.greeter_human_handle
    }

    pub async fn do_wait_peer(self) -> InviteResult<DeviceClaimInProgress1Ctx> {
        self.0.do_wait_peer().await.map(DeviceClaimInProgress1Ctx)
    }
}

#[derive(Debug)]
struct BaseClaimInProgress1Ctx {
    greeter_sas: SASCode,
    claimer_sas: SASCode,
    shared_secret_key: SecretKey,
    cmds: InvitedCmds,
}

impl BaseClaimInProgress1Ctx {
    fn generate_greeter_sas_choices(&self, size: usize) -> Vec<SASCode> {
        SASCode::generate_sas_code_candidates(&self.greeter_sas, size)
    }

    async fn do_signify_trust(self) -> InviteResult<BaseClaimInProgress2Ctx> {
        let rep = self.cmds.send(invite_3a_claimer_signify_trust::Req).await?;

        match rep {
            invite_3a_claimer_signify_trust::Rep::AlreadyDeleted => Err(InviteError::AlreadyUsed),
            invite_3a_claimer_signify_trust::Rep::InvalidState => Err(InviteError::PeerReset),
            invite_3a_claimer_signify_trust::Rep::NotFound => Err(InviteError::NotFound),
            invite_3a_claimer_signify_trust::Rep::Ok => Ok(BaseClaimInProgress2Ctx {
                claimer_sas: self.claimer_sas,
                shared_secret_key: self.shared_secret_key,
                cmds: self.cmds,
            }),
            invite_3a_claimer_signify_trust::Rep::UnknownStatus { unknown_status, .. } => Err(
                InviteError::Custom(format!("Backend error during step 3a: {unknown_status}")),
            ),
        }
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

    pub async fn do_signify_trust(self) -> InviteResult<UserClaimInProgress2Ctx> {
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

    pub async fn do_signify_trust(self) -> InviteResult<DeviceClaimInProgress2Ctx> {
        self.0
            .do_signify_trust()
            .await
            .map(DeviceClaimInProgress2Ctx)
    }
}

#[derive(Debug)]
struct BaseClaimInProgress2Ctx {
    claimer_sas: SASCode,
    shared_secret_key: SecretKey,
    cmds: InvitedCmds,
}

impl BaseClaimInProgress2Ctx {
    async fn do_wait_peer_trust(self) -> InviteResult<BaseClaimInProgress3Ctx> {
        let rep = self
            .cmds
            .send(invite_3b_claimer_wait_peer_trust::Req)
            .await?;

        match rep {
            invite_3b_claimer_wait_peer_trust::Rep::AlreadyDeleted => Err(InviteError::AlreadyUsed),
            invite_3b_claimer_wait_peer_trust::Rep::InvalidState => Err(InviteError::PeerReset),
            invite_3b_claimer_wait_peer_trust::Rep::NotFound => Err(InviteError::NotFound),
            invite_3b_claimer_wait_peer_trust::Rep::Ok => Ok(BaseClaimInProgress3Ctx {
                shared_secret_key: self.shared_secret_key,
                cmds: self.cmds,
            }),
            invite_3b_claimer_wait_peer_trust::Rep::UnknownStatus { unknown_status, .. } => Err(
                InviteError::Custom(format!("Backend error during step 3b: {unknown_status}")),
            ),
        }
    }
}

#[derive(Debug)]
pub struct UserClaimInProgress2Ctx(BaseClaimInProgress2Ctx);

impl UserClaimInProgress2Ctx {
    pub fn claimer_sas(&self) -> &SASCode {
        &self.0.claimer_sas
    }

    pub async fn do_wait_peer_trust(self) -> InviteResult<UserClaimInProgress3Ctx> {
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

    pub async fn do_wait_peer_trust(self) -> InviteResult<DeviceClaimInProgress3Ctx> {
        self.0
            .do_wait_peer_trust()
            .await
            .map(DeviceClaimInProgress3Ctx)
    }
}

#[derive(Debug)]
struct BaseClaimInProgress3Ctx {
    shared_secret_key: SecretKey,
    cmds: InvitedCmds,
}

impl BaseClaimInProgress3Ctx {
    async fn do_claim(&self, payload: Vec<u8>) -> InviteResult<Vec<u8>> {
        let rep = self
            .cmds
            .send(invite_4_claimer_communicate::Req { payload })
            .await?;

        match rep {
            invite_4_claimer_communicate::Rep::AlreadyDeleted => Err(InviteError::AlreadyUsed),
            invite_4_claimer_communicate::Rep::InvalidState => Err(InviteError::PeerReset),
            invite_4_claimer_communicate::Rep::NotFound => Err(InviteError::NotFound),
            invite_4_claimer_communicate::Rep::Ok { .. } => Ok(()),
            invite_4_claimer_communicate::Rep::UnknownStatus { unknown_status, .. } => {
                Err(InviteError::Custom(format!(
                    "Backend error during step 4 (data exchange): {unknown_status}"
                )))
            }
        }?;

        // Note the empty payload here, this is because we only want to receive our peer's
        // data, but for that we must send it something.
        let rep = self
            .cmds
            .send(invite_4_claimer_communicate::Req { payload: vec![] })
            .await?;

        match rep {
            invite_4_claimer_communicate::Rep::AlreadyDeleted => Err(InviteError::AlreadyUsed),
            invite_4_claimer_communicate::Rep::InvalidState => Err(InviteError::PeerReset),
            invite_4_claimer_communicate::Rep::NotFound => Err(InviteError::NotFound),
            invite_4_claimer_communicate::Rep::Ok { payload } => Ok(payload),
            invite_4_claimer_communicate::Rep::UnknownStatus { unknown_status, .. } => {
                Err(InviteError::Custom(format!(
                    "Backend error during step 4 (confirmation exchange): {unknown_status}"
                )))
            }
        }
    }
}

#[derive(Debug)]
pub struct UserClaimInProgress3Ctx(BaseClaimInProgress3Ctx);

impl UserClaimInProgress3Ctx {
    pub async fn do_claim_user(
        self,
        requested_device_label: Option<DeviceLabel>,
        requested_human_handle: Option<HumanHandle>,
    ) -> InviteResult<LocalDevice> {
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
        .dump_and_encrypt(&self.0.shared_secret_key);

        let payload = self.0.do_claim(payload).await?;

        let InviteUserConfirmation {
            device_id,
            device_label,
            human_handle,
            profile,
            root_verify_key,
        } = InviteUserConfirmation::decrypt_and_load(&payload, &self.0.shared_secret_key).map_err(
            |_| {
                InviteError::Custom(
                    "Invalid InviteUserConfirmation payload provided by peer".into(),
                )
            },
        )?;

        let addr = self.0.cmds.addr();

        let organization_addr = BackendOrganizationAddr::new(
            addr.clone(),
            addr.organization_id().clone(),
            root_verify_key,
        );

        Ok(LocalDevice::generate_new_device(
            organization_addr,
            Some(device_id),
            profile,
            human_handle,
            device_label,
            Some(signing_key),
            Some(private_key),
        ))
    }
}

#[derive(Debug)]
pub struct DeviceClaimInProgress3Ctx(BaseClaimInProgress3Ctx);

impl DeviceClaimInProgress3Ctx {
    pub async fn do_claim_device(
        self,
        requested_device_label: Option<DeviceLabel>,
    ) -> InviteResult<LocalDevice> {
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
        .dump_and_encrypt(&self.0.shared_secret_key);

        let payload = self.0.do_claim(payload).await?;

        let InviteDeviceConfirmation {
            device_id,
            device_label,
            human_handle,
            profile,
            private_key,
            user_manifest_id,
            user_manifest_key,
            root_verify_key,
            ..
        } = InviteDeviceConfirmation::decrypt_and_load(&payload, &self.0.shared_secret_key)
            .map_err(|_| {
                InviteError::Custom(
                    "Invalid InviteDeviceConfirmation payload provided by peer".into(),
                )
            })?;

        let addr = self.0.cmds.addr();

        let organization_addr = BackendOrganizationAddr::new(
            addr.clone(),
            addr.organization_id().clone(),
            root_verify_key,
        );

        Ok(LocalDevice {
            organization_addr,
            device_id,
            device_label,
            human_handle,
            profile,
            private_key,
            signing_key,
            user_manifest_id,
            user_manifest_key,
            local_symkey: SecretKey::generate(),
            time_provider: Default::default(),
        })
    }
}
