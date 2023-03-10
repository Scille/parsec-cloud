// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{pyclass, pymethods, PyResult};
use std::{collections::HashMap, num::NonZeroU64, sync::Arc};

use libparsec::{
    client_connection,
    protocol::{authenticated_cmds, IntegerBetween1And100},
    types::Maybe,
};

use crate::{
    addrs::BackendOrganizationAddr,
    api_crypto::{PublicKey, SigningKey},
    binding_utils::BytesWrapper,
    enumerate::{InvitationDeletedReason, InvitationType},
    ids::{
        BlockID, DeviceID, EnrollmentID, InvitationToken, RealmID, SequesterServiceID, UserID,
        VlobID,
    },
    protocol::*,
    runtime::FutureIntoCoroutine,
    time::DateTime,
};

#[pyclass]
pub(crate) struct AuthenticatedCmds(pub Arc<client_connection::AuthenticatedCmds>);

#[pymethods]
impl AuthenticatedCmds {
    #[new]
    fn new(
        addr: BackendOrganizationAddr,
        device_id: DeviceID,
        signing_key: SigningKey,
    ) -> PyResult<Self> {
        let auth_cmds =
            client_connection::generate_authenticated_client(signing_key.0, device_id.0, addr.0);
        Ok(Self(Arc::new(auth_cmds)))
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

            let req = authenticated_cmds::v2::block_create::Req {
                block,
                block_id,
                realm_id,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::block_create,
                BlockCreateRep,
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

            let req = authenticated_cmds::v2::block_read::Req { block_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::block_read,
                BlockReadRep,
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
            let req = authenticated_cmds::v2::device_create::Req {
                device_certificate,
                redacted_device_certificate,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::device_create,
                DeviceCreateRep,
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
            let req = authenticated_cmds::v2::events_listen::Req { wait };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::events_listen,
                EventsListenRep,
                NoEvents,
                Ok,
                Cancelled,
                UnknownStatus
            )
        })
    }

    fn events_subscribe(&self) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::v2::events_subscribe::Req;

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::events_subscribe,
                EventsSubscribeRep,
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
            let page = NonZeroU64::try_from(page).map_err(InvalidMessageError::new_err)?;
            let per_page =
                IntegerBetween1And100::try_from(per_page).map_err(InvalidMessageError::new_err)?;

            let req = authenticated_cmds::v2::human_find::Req {
                omit_non_human,
                omit_revoked,
                page,
                per_page,
                query,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::human_find,
                HumanFindRep,
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

            let req = authenticated_cmds::v2::invite_1_greeter_wait_peer::Req {
                greeter_public_key,
                token,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::invite_1_greeter_wait_peer,
                Invite1GreeterWaitPeerRep,
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

            let req = authenticated_cmds::v2::invite_2a_greeter_get_hashed_nonce::Req { token };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::invite_2a_greeter_get_hashed_nonce,
                Invite2aGreeterGetHashedNonceRep,
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

            let req = authenticated_cmds::v2::invite_2b_greeter_send_nonce::Req {
                greeter_nonce,
                token,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::invite_2b_greeter_send_nonce,
                Invite2bGreeterSendNonceRep,
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

            let req = authenticated_cmds::v2::invite_3a_greeter_wait_peer_trust::Req { token };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::invite_3a_greeter_wait_peer_trust,
                Invite3aGreeterWaitPeerTrustRep,
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

            let req = authenticated_cmds::v2::invite_3b_greeter_signify_trust::Req { token };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::invite_3b_greeter_signify_trust,
                Invite3bGreeterSignifyTrustRep,
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

            let req = authenticated_cmds::v2::invite_4_greeter_communicate::Req { token, payload };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::invite_4_greeter_communicate,
                Invite4GreeterCommunicateRep,
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
        reason: InvitationDeletedReason,
    ) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let token = token.0;
            let reason = reason.0;

            let req = authenticated_cmds::v2::invite_delete::Req { token, reason };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::invite_delete,
                InviteDeleteRep,
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
            let req = authenticated_cmds::v2::invite_list::Req;

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::invite_list,
                InviteListRep,
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
            let req = authenticated_cmds::v2::invite_new::Req(match r#type.0 {
                libparsec::types::InvitationType::Device => {
                    authenticated_cmds::v2::invite_new::UserOrDevice::Device { send_email }
                }
                libparsec::types::InvitationType::User => {
                    authenticated_cmds::v2::invite_new::UserOrDevice::User {
                        send_email,
                        claimer_email: claimer_email.expect("Missing claimer_email_argument"),
                    }
                }
            });

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::invite_new,
                InviteNewRep,
                NotAllowed,
                AlreadyMember,
                NotAvailable,
                Ok,
                UnknownStatus
            )
        })
    }

    fn message_get(&self, offset: u64) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::v2::message_get::Req { offset };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::message_get,
                MessageGetRep,
                Ok,
                UnknownStatus
            )
        })
    }

    fn organization_config(&self) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::v2::organization_config::Req;

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::organization_config,
                OrganizationConfigRep,
                Ok,
                UnknownStatus,
                NotFound
            )
        })
    }

    fn organization_stats(&self) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = authenticated_cmds::v2::organization_stats::Req;

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::organization_stats,
                OrganizationStatsRep,
                NotFound,
                NotAllowed,
                Ok,
                UnknownStatus
            )
        })
    }

    #[args(ping = "String::new()")]
    fn ping(&self, ping: String) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        let req = authenticated_cmds::v2::ping::Req { ping };

        FutureIntoCoroutine::from_raw(async move {
            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::ping,
                AuthenticatedPingRep,
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

            let req = authenticated_cmds::v2::pki_enrollment_accept::Req {
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
                authenticated_cmds::v2::pki_enrollment_accept,
                PkiEnrollmentAcceptRep,
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
            let req = authenticated_cmds::v2::pki_enrollment_list::Req;

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::pki_enrollment_list,
                PkiEnrollmentListRep,
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

            let req = authenticated_cmds::v2::pki_enrollment_reject::Req { enrollment_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::pki_enrollment_reject,
                PkiEnrollmentRejectRep,
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
            let req = authenticated_cmds::v2::realm_create::Req { role_certificate };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::realm_create,
                RealmCreateRep,
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

            let req = authenticated_cmds::v2::realm_finish_reencryption_maintenance::Req {
                encryption_revision,
                realm_id,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::realm_finish_reencryption_maintenance,
                RealmFinishReencryptionMaintenanceRep,
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

            let req = authenticated_cmds::v2::realm_get_role_certificates::Req { realm_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::realm_get_role_certificates,
                RealmGetRoleCertificatesRep,
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

            let req = authenticated_cmds::v2::realm_start_reencryption_maintenance::Req {
                encryption_revision,
                per_participant_message,
                realm_id,
                timestamp,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::realm_start_reencryption_maintenance,
                RealmStartReencryptionMaintenanceRep,
                BadEncryptionRevision,
                BadTimestamp,
                InMaintenance,
                MaintenanceError,
                NotAllowed,
                NotFound,
                Ok,
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

            let req = authenticated_cmds::v2::realm_stats::Req { realm_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::realm_stats,
                RealmStatsRep,
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

            let req = authenticated_cmds::v2::realm_status::Req { realm_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::realm_status,
                RealmStatusRep,
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
            let req = authenticated_cmds::v2::realm_update_roles::Req {
                recipient_message,
                role_certificate,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::realm_update_roles,
                RealmUpdateRolesRep,
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
            let req = authenticated_cmds::v2::user_create::Req {
                device_certificate,
                redacted_device_certificate,
                redacted_user_certificate,
                user_certificate,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::user_create,
                UserCreateRep,
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

            let req = authenticated_cmds::v2::user_get::Req { user_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::user_get,
                UserGetRep,
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
            let req = authenticated_cmds::v2::user_revoke::Req {
                revoked_user_certificate,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::user_revoke,
                UserRevokeRep,
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

            let req = authenticated_cmds::v2::vlob_create::Req {
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
                authenticated_cmds::v2::vlob_create,
                VlobCreateRep,
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

            let req = authenticated_cmds::v2::vlob_list_versions::Req { vlob_id };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::vlob_list_versions,
                VlobListVersionsRep,
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

            let req = authenticated_cmds::v2::vlob_maintenance_get_reencryption_batch::Req {
                encryption_revision,
                realm_id,
                size,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::vlob_maintenance_get_reencryption_batch,
                VlobMaintenanceGetReencryptionBatchRep,
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

            let req = authenticated_cmds::v2::vlob_maintenance_save_reencryption_batch::Req {
                encryption_revision,
                realm_id,
                batch,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::vlob_maintenance_save_reencryption_batch,
                VlobMaintenanceSaveReencryptionBatchRep,
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

            let req = authenticated_cmds::v2::vlob_poll_changes::Req {
                last_checkpoint,
                realm_id,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::vlob_poll_changes,
                VlobPollChangesRep,
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

            let req = authenticated_cmds::v2::vlob_read::Req {
                encryption_revision,
                timestamp,
                version,
                vlob_id,
            };

            crate::binding_utils::send_command!(
                auth_cmds,
                req,
                authenticated_cmds::v2::vlob_read,
                VlobReadRep,
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

            let req = authenticated_cmds::v2::vlob_update::Req {
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
                authenticated_cmds::v2::vlob_update,
                VlobUpdateRep,
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
