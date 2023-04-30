// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{exceptions::PyValueError, pyclass, pymethods, PyResult};
use std::{collections::HashMap, num::NonZeroU64, path::PathBuf, sync::Arc};

use libparsec::{
    client_connection,
    protocol::authenticated_cmds::v2 as authenticated_cmds,
    types::{IntegerBetween1And100, Maybe},
};

use crate::{
    addrs::BackendOrganizationAddr,
    api_crypto::PublicKey,
    binding_utils::BytesWrapper,
    enumerate::InvitationType,
    ids::{BlockID, EnrollmentID, InvitationToken, RealmID, SequesterServiceID, UserID, VlobID},
    local_device::LocalDevice,
    protocol::{authenticated_cmds::v2 as authenticated_cmds_wrapper, ReencryptionBatchEntry},
    runtime::FutureIntoCoroutine,
    time::DateTime,
};

#[pyclass]
pub(crate) struct AuthenticatedCmds(pub Arc<client_connection::AuthenticatedCmds>);

#[pymethods]
impl AuthenticatedCmds {
    #[new]
    fn new(device: &LocalDevice) -> PyResult<Self> {
        // Config dir is only used as discriminant for the testbed, which is never used in Python
        let dummy_config_dir = PathBuf::from("");
        let proxy_config = client_connection::ProxyConfig::new_from_env();
        client_connection::AuthenticatedCmds::new(&dummy_config_dir, device.0.clone(), proxy_config)
            .map_err(|e| {
                PyValueError::new_err(format!("Fail to generate an authenticated client: {e}"))
            })
            .map(|client| Self(Arc::new(client)))
    }

    #[getter]
    fn addr(&self) -> BackendOrganizationAddr {
        BackendOrganizationAddr(self.0.addr().clone())
    }

    fn block_create(
        &self,
        block_id: BlockID,
        realm_id: RealmID,
        block: BytesWrapper,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(block);

        FutureIntoCoroutine::from_raw(async move {
            let block_id = block_id.0;
            let realm_id = realm_id.0;

            let req = authenticated_cmds::block_create::Req {
                block,
                block_id,
                realm_id,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::block_create,
                authenticated_cmds_wrapper::block_create,
                Ok,
                NotFound,
                Timeout,
                AlreadyExists,
                InMaintenance,
                NotAllowed,
                UnknownStatus
            )
        })
    }

    fn block_read(&self, block_id: BlockID) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let block_id = block_id.0;

            let req = authenticated_cmds::block_read::Req { block_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::block_read,
                authenticated_cmds_wrapper::block_read,
                Ok,
                NotFound,
                Timeout,
                InMaintenance,
                NotAllowed,
                UnknownStatus
            )
        })
    }

    fn device_create(
        &self,
        device_certificate: BytesWrapper,
        redacted_device_certificate: BytesWrapper,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(device_certificate, redacted_device_certificate);

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::device_create::Req {
                device_certificate,
                redacted_device_certificate,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::device_create,
                authenticated_cmds_wrapper::device_create,
                Ok,
                BadUserId,
                InvalidCertification,
                InvalidData,
                AlreadyExists,
                UnknownStatus
            )
        })
    }

    fn events_listen(&self, wait: bool) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::events_listen::Req { wait };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::events_listen,
                authenticated_cmds_wrapper::events_listen,
                Ok,
                NoEvents,
                Cancelled,
                UnknownStatus
            )
        })
    }

    fn events_subscribe(&self) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::events_subscribe::Req;

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::events_subscribe,
                authenticated_cmds_wrapper::events_subscribe,
                Ok,
                UnknownStatus
            )
        })
    }

    fn human_find(
        &self,
        query: Option<String>,
        page: u64,
        per_page: u64,
        omit_revoked: bool,
        omit_non_human: bool,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let page = NonZeroU64::try_from(page).map_err(PyValueError::new_err)?;
            let per_page =
                IntegerBetween1And100::try_from(per_page).map_err(PyValueError::new_err)?;

            let req = authenticated_cmds::human_find::Req {
                omit_non_human,
                omit_revoked,
                page,
                per_page,
                query,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::human_find,
                authenticated_cmds_wrapper::human_find,
                Ok,
                NotAllowed,
                UnknownStatus
            )
        })
    }

    fn invite_1_greeter_wait_peer(
        &self,
        token: InvitationToken,
        greeter_public_key: PublicKey,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let greeter_public_key = greeter_public_key.0;
            let token = token.0;

            let req = authenticated_cmds::invite_1_greeter_wait_peer::Req {
                greeter_public_key,
                token,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::invite_1_greeter_wait_peer,
                authenticated_cmds_wrapper::invite_1_greeter_wait_peer,
                Ok,
                NotFound,
                InvalidState,
                UnknownStatus,
                AlreadyDeleted
            )
        })
    }

    fn invite_2a_greeter_get_hashed_nonce(&self, token: InvitationToken) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let token = token.0;

            let req = authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Req { token };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::invite_2a_greeter_get_hashed_nonce,
                authenticated_cmds_wrapper::invite_2a_greeter_get_hashed_nonce,
                Ok,
                NotFound,
                InvalidState,
                UnknownStatus,
                AlreadyDeleted
            )
        })
    }

    fn invite_2b_greeter_send_nonce(
        &self,
        token: InvitationToken,
        greeter_nonce: BytesWrapper,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(greeter_nonce);

        FutureIntoCoroutine::from_raw(async move {
            let token = token.0;

            let req = authenticated_cmds::invite_2b_greeter_send_nonce::Req {
                greeter_nonce,
                token,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::invite_2b_greeter_send_nonce,
                authenticated_cmds_wrapper::invite_2b_greeter_send_nonce,
                Ok,
                NotFound,
                InvalidState,
                UnknownStatus,
                AlreadyDeleted
            )
        })
    }

    fn invite_3a_greeter_wait_peer_trust(&self, token: InvitationToken) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let token = token.0;

            let req = authenticated_cmds::invite_3a_greeter_wait_peer_trust::Req { token };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::invite_3a_greeter_wait_peer_trust,
                authenticated_cmds_wrapper::invite_3a_greeter_wait_peer_trust,
                Ok,
                NotFound,
                InvalidState,
                UnknownStatus,
                AlreadyDeleted
            )
        })
    }

    fn invite_3b_greeter_signify_trust(&self, token: InvitationToken) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let token = token.0;

            let req = authenticated_cmds::invite_3b_greeter_signify_trust::Req { token };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::invite_3b_greeter_signify_trust,
                authenticated_cmds_wrapper::invite_3b_greeter_signify_trust,
                Ok,
                NotFound,
                InvalidState,
                UnknownStatus,
                AlreadyDeleted
            )
        })
    }

    fn invite_4_greeter_communicate(
        &self,
        token: InvitationToken,
        payload: BytesWrapper,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(payload);

        FutureIntoCoroutine::from_raw(async move {
            let token = token.0;

            let req = authenticated_cmds::invite_4_greeter_communicate::Req { token, payload };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::invite_4_greeter_communicate,
                authenticated_cmds_wrapper::invite_4_greeter_communicate,
                Ok,
                NotFound,
                InvalidState,
                UnknownStatus,
                AlreadyDeleted
            )
        })
    }

    fn invite_delete(
        &self,
        token: InvitationToken,
        reason: authenticated_cmds_wrapper::invite_delete::InvitationDeletedReason,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let token = token.0;
            let reason = reason.0;

            let req = authenticated_cmds::invite_delete::Req { token, reason };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::invite_delete,
                authenticated_cmds_wrapper::invite_delete,
                Ok,
                NotFound,
                UnknownStatus,
                AlreadyDeleted
            )
        })
    }

    fn invite_list(&self) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::invite_list::Req;

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::invite_list,
                authenticated_cmds_wrapper::invite_list,
                Ok,
                UnknownStatus
            )
        })
    }

    #[args(send_email = "false", claimer_email = "None")]
    fn invite_new(
        &self,
        r#type: InvitationType,
        send_email: bool,
        claimer_email: Option<String>,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::invite_new::Req(match r#type.0 {
                libparsec::types::InvitationType::Device => {
                    authenticated_cmds::invite_new::UserOrDevice::Device { send_email }
                }
                libparsec::types::InvitationType::User => {
                    authenticated_cmds::invite_new::UserOrDevice::User {
                        send_email,
                        claimer_email: claimer_email.expect("Missing claimer_email_argument"),
                    }
                }
            });

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::invite_new,
                authenticated_cmds_wrapper::invite_new,
                Ok,
                NotAllowed,
                AlreadyMember,
                NotAvailable,
                UnknownStatus
            )
        })
    }

    fn message_get(&self, offset: u64) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::message_get::Req { offset };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::message_get,
                authenticated_cmds_wrapper::message_get,
                Ok,
                UnknownStatus
            )
        })
    }

    fn organization_config(&self) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::organization_config::Req;

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::organization_config,
                authenticated_cmds_wrapper::organization_config,
                Ok,
                UnknownStatus,
                NotFound
            )
        })
    }

    fn organization_stats(&self) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::organization_stats::Req;

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::organization_stats,
                authenticated_cmds_wrapper::organization_stats,
                Ok,
                NotFound,
                NotAllowed,
                UnknownStatus
            )
        })
    }

    #[args(ping = "String::new()")]
    fn ping(&self, ping: String) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        let req = authenticated_cmds::ping::Req { ping };

        FutureIntoCoroutine::from_raw(async move {
            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::ping,
                authenticated_cmds_wrapper::ping,
                Ok,
                UnknownStatus
            )
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn pki_enrollment_accept(
        &self,
        enrollment_id: EnrollmentID,
        accepter_der_x509_certificate: BytesWrapper,
        accept_payload_signature: BytesWrapper,
        accept_payload: BytesWrapper,
        user_certificate: BytesWrapper,
        device_certificate: BytesWrapper,
        redacted_user_certificate: BytesWrapper,
        redacted_device_certificate: BytesWrapper,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(
            accepter_der_x509_certificate,
            accept_payload_signature,
            accept_payload,
            user_certificate,
            device_certificate,
            redacted_user_certificate,
            redacted_device_certificate
        );

        FutureIntoCoroutine::from_raw(async move {
            let enrollment_id = enrollment_id.0;

            let req = authenticated_cmds::pki_enrollment_accept::Req {
                accept_payload,
                accept_payload_signature,
                accepter_der_x509_certificate,
                device_certificate,
                enrollment_id,
                redacted_device_certificate,
                redacted_user_certificate,
                user_certificate,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::pki_enrollment_accept,
                authenticated_cmds_wrapper::pki_enrollment_accept,
                Ok,
                InvalidData,
                InvalidCertification,
                InvalidPayloadData,
                NotAllowed,
                NotFound,
                NoLongerAvailable,
                AlreadyExists,
                ActiveUsersLimitReached,
                UnknownStatus
            )
        })
    }

    fn pki_enrollment_list(&self) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::pki_enrollment_list::Req;

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::pki_enrollment_list,
                authenticated_cmds_wrapper::pki_enrollment_list,
                Ok,
                NotAllowed,
                UnknownStatus
            )
        })
    }

    fn pki_enrollment_reject(&self, enrollment_id: EnrollmentID) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let enrollment_id = enrollment_id.0;

            let req = authenticated_cmds::pki_enrollment_reject::Req { enrollment_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::pki_enrollment_reject,
                authenticated_cmds_wrapper::pki_enrollment_reject,
                Ok,
                NotFound,
                NoLongerAvailable,
                NotAllowed,
                UnknownStatus
            )
        })
    }

    fn realm_create(&self, role_certificate: BytesWrapper) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(role_certificate);

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::realm_create::Req { role_certificate };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::realm_create,
                authenticated_cmds_wrapper::realm_create,
                Ok,
                NotFound,
                BadTimestamp,
                InvalidCertification,
                InvalidData,
                AlreadyExists,
                UnknownStatus,
                "handle_bad_timestamp"
            )
        })
    }

    fn realm_finish_reencryption_maintenance(
        &self,
        realm_id: RealmID,
        encryption_revision: u64,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let realm_id = realm_id.0;

            let req = authenticated_cmds::realm_finish_reencryption_maintenance::Req {
                encryption_revision,
                realm_id,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::realm_finish_reencryption_maintenance,
                authenticated_cmds_wrapper::realm_finish_reencryption_maintenance,
                Ok,
                MaintenanceError,
                NotInMaintenance,
                NotAllowed,
                BadEncryptionRevision,
                NotFound,
                UnknownStatus
            )
        })
    }

    fn realm_get_role_certificates(&self, realm_id: RealmID) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let realm_id = realm_id.0;

            let req = authenticated_cmds::realm_get_role_certificates::Req { realm_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::realm_get_role_certificates,
                authenticated_cmds_wrapper::realm_get_role_certificates,
                Ok,
                NotAllowed,
                NotFound,
                UnknownStatus
            )
        })
    }

    fn realm_start_reencryption_maintenance(
        &self,
        realm_id: RealmID,
        encryption_revision: u64,
        timestamp: DateTime,
        per_participant_message: HashMap<UserID, BytesWrapper>,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        let per_participant_message = per_participant_message
            .into_iter()
            .map(|(id, b)| {
                crate::binding_utils::unwrap_bytes!(b);
                (id.0, b)
            })
            .collect();

        FutureIntoCoroutine::from_raw(async move {
            let realm_id = realm_id.0;
            let timestamp = timestamp.0;

            let req = authenticated_cmds::realm_start_reencryption_maintenance::Req {
                encryption_revision,
                per_participant_message,
                realm_id,
                timestamp,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::realm_start_reencryption_maintenance,
                authenticated_cmds_wrapper::realm_start_reencryption_maintenance,
                Ok,
                BadEncryptionRevision,
                BadTimestamp,
                InMaintenance,
                MaintenanceError,
                NotAllowed,
                NotFound,
                ParticipantMismatch,
                UnknownStatus,
                "handle_bad_timestamp"
            )
        })
    }

    fn realm_stats(&self, realm_id: RealmID) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let realm_id = realm_id.0;

            let req = authenticated_cmds::realm_stats::Req { realm_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::realm_stats,
                authenticated_cmds_wrapper::realm_stats,
                Ok,
                NotAllowed,
                NotFound,
                UnknownStatus
            )
        })
    }

    fn realm_status(&self, realm_id: RealmID) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let realm_id = realm_id.0;

            let req = authenticated_cmds::realm_status::Req { realm_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::realm_status,
                authenticated_cmds_wrapper::realm_status,
                Ok,
                NotAllowed,
                NotFound,
                UnknownStatus
            )
        })
    }

    fn realm_update_roles(
        &self,
        role_certificate: BytesWrapper,
        recipient_message: Option<BytesWrapper>,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(recipient_message, role_certificate);

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::realm_update_roles::Req {
                recipient_message,
                role_certificate,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::realm_update_roles,
                authenticated_cmds_wrapper::realm_update_roles,
                Ok,
                NotAllowed,
                NotFound,
                RequireGreaterTimestamp,
                BadTimestamp,
                InMaintenance,
                AlreadyGranted,
                UnknownStatus,
                InvalidCertification,
                InvalidData,
                UserRevoked,
                IncompatibleProfile,
                "handle_bad_timestamp"
            )
        })
    }

    fn user_create(
        &self,
        user_certificate: BytesWrapper,
        device_certificate: BytesWrapper,
        redacted_user_certificate: BytesWrapper,
        redacted_device_certificate: BytesWrapper,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(
            user_certificate,
            device_certificate,
            redacted_user_certificate,
            redacted_device_certificate
        );

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::user_create::Req {
                device_certificate,
                redacted_device_certificate,
                redacted_user_certificate,
                user_certificate,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::user_create,
                authenticated_cmds_wrapper::user_create,
                Ok,
                ActiveUsersLimitReached,
                AlreadyExists,
                InvalidCertification,
                InvalidData,
                NotAllowed,
                UnknownStatus
            )
        })
    }

    fn user_get(&self, user_id: UserID) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let user_id = user_id.0;

            let req = authenticated_cmds::user_get::Req { user_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::user_get,
                authenticated_cmds_wrapper::user_get,
                Ok,
                NotFound,
                UnknownStatus
            )
        })
    }

    fn user_revoke(&self, revoked_user_certificate: BytesWrapper) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(revoked_user_certificate);

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::user_revoke::Req {
                revoked_user_certificate,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::user_revoke,
                authenticated_cmds_wrapper::user_revoke,
                Ok,
                InvalidCertification,
                UnknownStatus,
                NotAllowed,
                NotFound,
                AlreadyRevoked
            )
        })
    }

    fn vlob_create(
        &self,
        realm_id: RealmID,
        encryption_revision: u64,
        vlob_id: VlobID,
        timestamp: DateTime,
        blob: BytesWrapper,
        sequester_blob: Option<HashMap<SequesterServiceID, BytesWrapper>>,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(blob);
        let sequester_blob = Maybe::Present(sequester_blob.map(|x| {
            x.into_iter()
                .map(|(id, b)| {
                    crate::binding_utils::unwrap_bytes!(b);
                    (id.0, b)
                })
                .collect()
        }));

        FutureIntoCoroutine::from_raw(async move {
            let realm_id = realm_id.0;
            let vlob_id = vlob_id.0;
            let timestamp = timestamp.0;

            let req = authenticated_cmds::vlob_create::Req {
                blob,
                encryption_revision,
                realm_id,
                sequester_blob,
                timestamp,
                vlob_id,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::vlob_create,
                authenticated_cmds_wrapper::vlob_create,
                Ok,
                UnknownStatus,
                AlreadyExists,
                BadTimestamp,
                BadEncryptionRevision,
                InMaintenance,
                NotAllowed,
                NotASequesteredOrganization,
                RejectedBySequesterService,
                RequireGreaterTimestamp,
                SequesterInconsistency,
                Timeout,
                "handle_bad_timestamp"
            )
        })
    }

    fn vlob_list_versions(&self, vlob_id: VlobID) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let vlob_id = vlob_id.0;

            let req = authenticated_cmds::vlob_list_versions::Req { vlob_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::vlob_list_versions,
                authenticated_cmds_wrapper::vlob_list_versions,
                Ok,
                NotAllowed,
                InMaintenance,
                UnknownStatus,
                NotFound
            )
        })
    }

    fn vlob_maintenance_get_reencryption_batch(
        &self,
        realm_id: RealmID,
        encryption_revision: u64,
        size: u64,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let realm_id = realm_id.0;

            let req = authenticated_cmds::vlob_maintenance_get_reencryption_batch::Req {
                encryption_revision,
                realm_id,
                size,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::vlob_maintenance_get_reencryption_batch,
                authenticated_cmds_wrapper::vlob_maintenance_get_reencryption_batch,
                Ok,
                BadEncryptionRevision,
                MaintenanceError,
                NotAllowed,
                NotFound,
                NotInMaintenance,
                UnknownStatus
            )
        })
    }

    fn vlob_maintenance_save_reencryption_batch(
        &self,
        realm_id: RealmID,
        encryption_revision: u64,
        batch: Vec<ReencryptionBatchEntry>,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let realm_id = realm_id.0;
            let batch = batch.into_iter().map(|x| x.0).collect();

            let req = authenticated_cmds::vlob_maintenance_save_reencryption_batch::Req {
                encryption_revision,
                realm_id,
                batch,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::vlob_maintenance_save_reencryption_batch,
                authenticated_cmds_wrapper::vlob_maintenance_save_reencryption_batch,
                Ok,
                BadEncryptionRevision,
                MaintenanceError,
                NotInMaintenance,
                NotAllowed,
                NotFound,
                UnknownStatus
            )
        })
    }

    fn vlob_poll_changes(&self, realm_id: RealmID, last_checkpoint: u64) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let realm_id = realm_id.0;

            let req = authenticated_cmds::vlob_poll_changes::Req {
                last_checkpoint,
                realm_id,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::vlob_poll_changes,
                authenticated_cmds_wrapper::vlob_poll_changes,
                Ok,
                UnknownStatus,
                InMaintenance,
                NotAllowed,
                NotFound
            )
        })
    }

    #[args(version = "None", timestamp = "None")]
    fn vlob_read(
        &self,
        encryption_revision: u64,
        vlob_id: VlobID,
        version: Option<u32>,
        timestamp: Option<DateTime>,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let vlob_id = vlob_id.0;
            let timestamp = timestamp.map(|x| x.0);

            let req = authenticated_cmds::vlob_read::Req {
                encryption_revision,
                timestamp,
                version,
                vlob_id,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::vlob_read,
                authenticated_cmds_wrapper::vlob_read,
                Ok,
                UnknownStatus,
                BadVersion,
                BadEncryptionRevision,
                InMaintenance,
                NotAllowed,
                NotFound
            )
        })
    }

    fn vlob_update(
        &self,
        encryption_revision: u64,
        vlob_id: VlobID,
        version: u32,
        timestamp: DateTime,
        blob: BytesWrapper,
        sequester_blob: Option<HashMap<SequesterServiceID, BytesWrapper>>,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(blob);
        let sequester_blob = Maybe::Present(sequester_blob.map(|x| {
            x.into_iter()
                .map(|(id, b)| {
                    crate::binding_utils::unwrap_bytes!(b);
                    (id.0, b)
                })
                .collect()
        }));

        FutureIntoCoroutine::from_raw(async move {
            let vlob_id = vlob_id.0;
            let timestamp = timestamp.0;

            let req = authenticated_cmds::vlob_update::Req {
                blob,
                sequester_blob,
                encryption_revision,
                timestamp,
                version,
                vlob_id,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::vlob_update,
                authenticated_cmds_wrapper::vlob_update,
                Ok,
                UnknownStatus,
                BadEncryptionRevision,
                BadTimestamp,
                BadVersion,
                InMaintenance,
                NotASequesteredOrganization,
                NotAllowed,
                NotFound,
                RejectedBySequesterService,
                RequireGreaterTimestamp,
                SequesterInconsistency,
                Timeout,
                "handle_bad_timestamp"
            )
        })
    }
}
