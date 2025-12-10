// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, num::NonZeroU8, sync::Arc};

pub use libparsec_client::{
    AsyncEnrollmentUntrusted, ClientAcceptAsyncEnrollmentError, ClientListAsyncEnrollmentsError,
    ClientRejectAsyncEnrollmentError, SubmitAsyncEnrollmentError,
};
pub use libparsec_platform_async::future::join_all;
use libparsec_platform_device_loader::{RemoteOperationServer, SaveDeviceError};
pub use libparsec_protocol::authenticated_cmds::latest::invite_list::InvitationCreatedBy as InviteListInvitationCreatedBy;
pub use libparsec_protocol::invited_cmds::latest::invite_info::{
    InvitationCreatedBy as InviteInfoInvitationCreatedBy, ShamirRecoveryRecipient,
    UserGreetingAdministrator, UserOnlineStatus,
};
use libparsec_types::prelude::*;

use crate::{
    handle::{borrow_from_handle, register_handle, take_and_close_handle, Handle, HandleItem},
    listen_canceller, AvailableDevice, ClientConfig, DeviceSaveStrategy, OnEventCallbackPlugged,
};

mod strategy {
    use std::{path::PathBuf, sync::Arc};

    use libparsec_crypto::{Password, SecretKey};
    use libparsec_openbao::{OpenBaoCmds, OpenBaoFetchOpaqueKeyError, OpenBaoUploadOpaqueKeyError};
    use libparsec_platform_async::{pretend_future_is_send_on_web, PinBoxFutureResult};
    use libparsec_types::prelude::*;

    use crate::handle::{borrow_from_handle, Handle, HandleItem};

    /*
     * AsyncEnrollmentIdentityStrategy
     */

    #[derive(Debug, Clone)]
    pub enum AsyncEnrollmentIdentityStrategy {
        OpenBao {
            openbao_server_url: String,
            openbao_transit_mount_path: String,
            openbao_secret_mount_path: String,
            openbao_entity_id: String,
            openbao_auth_token: String,
        },
        PKI {
            certificate_reference: X509CertificateReference,
        },
    }

    impl AsyncEnrollmentIdentityStrategy {
        pub fn convert(
            self,
        ) -> anyhow::Result<Box<dyn libparsec_client::AcceptAsyncEnrollmentIdentityStrategy>>
        {
            match self {
                AsyncEnrollmentIdentityStrategy::OpenBao {
                    openbao_server_url,
                    openbao_transit_mount_path,
                    openbao_secret_mount_path,
                    openbao_entity_id,
                    openbao_auth_token,
                } => {
                    let client = libparsec_client_connection::build_client_with_proxy(
                        libparsec_client_connection::ProxyConfig::default(),
                    )?;

                    let cmds = Arc::new(OpenBaoCmds::new(
                        client,
                        openbao_server_url,
                        openbao_secret_mount_path,
                        // openbao_transit_mount_path,  // TODO
                        openbao_entity_id,
                        openbao_auth_token,
                    ));

                    Ok(Box::new(AsyncEnrollmentOpenBaoIdentityStrategy { cmds }))
                }
                AsyncEnrollmentIdentityStrategy::PKI {
                    certificate_reference,
                } => Ok(Box::new(AsyncEnrollmentPKIIdentityStrategy {
                    certificate_reference,
                })),
            }
        }
    }

    #[derive(Debug)]
    struct AsyncEnrollmentOpenBaoIdentityStrategy {
        cmds: Arc<OpenBaoCmds>,
    }

    impl libparsec_client::AcceptAsyncEnrollmentIdentityStrategy
        for AsyncEnrollmentOpenBaoIdentityStrategy
    {
        fn _verify_submit_payload(
            &self,
            payload: &[u8],
            payload_signature: libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_list::SubmitPayloadSignature,
        ) -> Result<
            AsyncEnrollmentSubmitPayload,
            libparsec_client::AsyncEnrollmentVerifySubmitPayloadError,
        > {
            todo!()
        }

        fn sign_accept_payload(&self, payload: &[u8]) -> libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_accept::AcceptPayloadSignature{
            todo!()
        }
    }

    impl libparsec_client::SubmitAsyncEnrollmentIdentityStrategy
        for AsyncEnrollmentOpenBaoIdentityStrategy
    {
        fn sign(&self, payload: &[u8]) -> libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_submit::SubmitPayloadSignature{
            todo!()
        }

        fn human_handle(&self) -> &HumanHandle {
            todo!()
        }

        fn generate_ciphertext_key(
            &self,
        ) -> PinBoxFutureResult<
            (SecretKey, AsyncEnrollmentLocalPendingIdentitySystem),
            libparsec_client::SubmitAsyncEnrollmentError,
        > {
            todo!()
        }
    }

    #[derive(Debug)]
    struct AsyncEnrollmentPKIIdentityStrategy {
        certificate_reference: X509CertificateReference,
    }

    impl libparsec_client::AcceptAsyncEnrollmentIdentityStrategy
        for AsyncEnrollmentPKIIdentityStrategy
    {
        fn _verify_submit_payload(
            &self,
            payload: &[u8],
            payload_signature: libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_list::SubmitPayloadSignature,
        ) -> Result<
            AsyncEnrollmentSubmitPayload,
            libparsec_client::AsyncEnrollmentVerifySubmitPayloadError,
        > {
            todo!()
        }

        fn sign_accept_payload(&self, payload: &[u8]) -> libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_accept::AcceptPayloadSignature{
            todo!()
        }
    }

    impl libparsec_client::SubmitAsyncEnrollmentIdentityStrategy
        for AsyncEnrollmentPKIIdentityStrategy
    {
        fn sign(&self, payload: &[u8]) -> libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_submit::SubmitPayloadSignature{
            todo!()
        }

        fn human_handle(&self) -> &HumanHandle {
            todo!()
        }

        fn generate_ciphertext_key(
            &self,
        ) -> PinBoxFutureResult<
            (SecretKey, AsyncEnrollmentLocalPendingIdentitySystem),
            libparsec_client::SubmitAsyncEnrollmentError,
        > {
            todo!()
        }
    }
}

pub use strategy::AsyncEnrollmentIdentityStrategy;

pub async fn client_list_async_enrollments(
    client: Handle,
) -> Result<Vec<AsyncEnrollmentUntrusted>, ClientListAsyncEnrollmentsError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    client.list_async_enrollments().await
}

pub async fn client_reject_async_enrollment(
    client: Handle,
    enrollment_id: AsyncEnrollmentID,
) -> Result<(), ClientRejectAsyncEnrollmentError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    client.reject_async_enrollment(enrollment_id).await
}

pub async fn client_accept_async_enrollment(
    client: Handle,
    profile: UserProfile,
    enrollment_id: AsyncEnrollmentID,
    identity_strategy: AsyncEnrollmentIdentityStrategy,
) -> Result<(), ClientAcceptAsyncEnrollmentError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let identity_strategy = identity_strategy.convert()?;

    client
        .accept_async_enrollment(profile, enrollment_id, identity_strategy.as_ref())
        .await
}

pub async fn submit_async_enrollment(
    config: ClientConfig,
    addr: ParsecAnonymousAddr,
    force: bool,
    requested_device_label: DeviceLabel,
    identity_strategy: AsyncEnrollmentIdentityStrategy,
) -> Result<AsyncEnrollmentID, SubmitAsyncEnrollmentError> {
    let identity_strategy = identity_strategy.convert()?;
    let config: Arc<libparsec_client::ClientConfig> = config.into();

    libparsec_client::submit_async_enrollment(
        config,
        addr,
        force,
        requested_device_label,
        identity_strategy.as_ref(),
    )
    .await
}

pub async fn submitter_list_async_enrollments(
    config: ClientConfig,
) -> Result<Vec<()>, SubmitterListAsyncEnrollmentsError> {
    todo!()
}

pub enum AsyncEnrollementStatus {
    Cancelled {},
    Accepted,
}

pub async fn submitter_retrieve_async_enrollment_info(
) -> Result<(), SubmitterRetrieveAsyncEnrollmentInfoError> {
    todo!()
}

pub async fn submitter_async_enrollment_finalize_save_local_device(
    handle: Handle,
    save_strategy: DeviceSaveStrategy,
) {
    todo!()
}

pub async fn submitter_async_enrollment_finalize_destroy(
    handle: Handle,
    save_strategy: DeviceSaveStrategy,
) {
    todo!()
}
