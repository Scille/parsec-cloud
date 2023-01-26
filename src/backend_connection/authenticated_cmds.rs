// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use pyo3::{
    pyclass, pyclass_init::PyObjectInit, pymethods, types::PyType, PyClassInitializer, PyErr,
    PyObject, PyResult, PyTypeInfo, Python,
};

use libparsec::{client_connection, protocol::authenticated_cmds};

use crate::{
    addrs::BackendOrganizationAddr,
    api_crypto::SigningKey,
    ids::DeviceID,
    protocol::*,
    protocol::{AuthenticatedPingRep, BlockCreateReq},
    runtime::FutureIntoCoroutine,
};

use super::CommandErrorExc;

#[pyclass]
#[derive(Clone)]
pub(crate) struct AuthenticatedCmdsType(authenticated_cmds::v2::AnyCmdReq);

#[pymethods]
impl AuthenticatedCmdsType {
    #[classmethod]
    #[pyo3(name = "BLOCK_CREATE")]
    fn block_create(_cls: &PyType, req: BlockCreateReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::BlockCreate(req.0))
    }

    #[classmethod]
    #[pyo3(name = "BLOCK_READ")]
    fn block_read(_cls: &PyType, req: BlockReadReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::BlockRead(req.0))
    }

    #[classmethod]
    #[pyo3(name = "DEVICE_CREATE")]
    fn device_create(_cls: &PyType, req: DeviceCreateReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::DeviceCreate(req.0))
    }

    #[classmethod]
    #[pyo3(name = "EVENTS_LISTEN")]
    fn events_listen(_cls: &PyType, req: EventsListenReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::EventsListen(req.0))
    }

    #[classmethod]
    #[pyo3(name = "EVENTS_SUBSCRIBE")]
    fn events_subscribe(_cls: &PyType, req: EventsSubscribeReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::EventsSubscribe(req.0))
    }

    #[classmethod]
    #[pyo3(name = "HUMAN_FIND")]
    fn human_find(_cls: &PyType, req: HumanFindReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::HumanFind(req.0))
    }

    #[classmethod]
    #[pyo3(name = "INVITE1_GREETER_WAIT_PEER")]
    fn invite1_greeter_wait_peer(_cls: &PyType, req: Invite1GreeterWaitPeerReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::Invite1GreeterWaitPeer(
            req.0,
        ))
    }

    #[classmethod]
    #[pyo3(name = "INVITE2A_GREETER_GET_HASHED_NONCE")]
    fn invite2a_greeter_get_hashed_nonce(
        _cls: &PyType,
        req: Invite2aGreeterGetHashedNonceReq,
    ) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::Invite2aGreeterGetHashedNonce(req.0))
    }

    #[classmethod]
    #[pyo3(name = "INVITE2B_GREETER_SEND_NONCE")]
    fn invite2b_greeter_send_nonce(_cls: &PyType, req: Invite2bGreeterSendNonceReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::Invite2bGreeterSendNonce(
            req.0,
        ))
    }

    #[classmethod]
    #[pyo3(name = "INVITE3A_GREETER_WAIT_PEER_TRUST")]
    fn invite3a_greeter_wait_peer_trust(
        _cls: &PyType,
        req: Invite3aGreeterWaitPeerTrustReq,
    ) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::Invite3aGreeterWaitPeerTrust(req.0))
    }

    #[classmethod]
    #[pyo3(name = "INVITE3B_GREETER_SIGNIFY_TRUST")]
    fn invite3b_greeter_signify_trust(_cls: &PyType, req: Invite3bGreeterSignifyTrustReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::Invite3bGreeterSignifyTrust(req.0))
    }

    #[classmethod]
    #[pyo3(name = "INVITE4GREETER_COMMUNICATE")]
    fn invite4greeter_communicate(_cls: &PyType, req: Invite4GreeterCommunicateReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::Invite4GreeterCommunicate(req.0))
    }

    #[classmethod]
    #[pyo3(name = "INVITE_DELETE")]
    fn invite_delete(_cls: &PyType, req: InviteDeleteReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::InviteDelete(req.0))
    }

    #[classmethod]
    #[pyo3(name = "INVITE_LIST")]
    fn invite_list(_cls: &PyType, req: InviteListReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::InviteList(req.0))
    }

    #[classmethod]
    #[pyo3(name = "INVITE_NEW")]
    fn invite_new(_cls: &PyType, req: InviteNewReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::InviteNew(req.0))
    }

    #[classmethod]
    #[pyo3(name = "MESSAGE_GET")]
    fn message_get(_cls: &PyType, req: MessageGetReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::MessageGet(req.0))
    }

    #[classmethod]
    #[pyo3(name = "ORGANIZATION_STATS")]
    fn organization_stats(_cls: &PyType, req: OrganizationStatsReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::OrganizationStats(req.0))
    }

    #[classmethod]
    #[pyo3(name = "PKI_ENROLLMENT_ACCEPT")]
    fn pki_enrollment_accept(_cls: &PyType, req: PkiEnrollmentAcceptReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::PkiEnrollmentAccept(
            req.0,
        ))
    }

    #[classmethod]
    #[pyo3(name = "PKI_ENROLLMENT_LIST")]
    fn pki_enrollment_list(_cls: &PyType, req: PkiEnrollmentListReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::PkiEnrollmentList(req.0))
    }

    #[classmethod]
    #[pyo3(name = "PKI_ENROLLMENT_REJECT")]
    fn pki_enrollment_reject(_cls: &PyType, req: PkiEnrollmentRejectReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::PkiEnrollmentReject(
            req.0,
        ))
    }

    #[classmethod]
    #[pyo3(name = "REALM_CREATE")]
    fn realm_create(_cls: &PyType, req: RealmCreateReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::RealmCreate(req.0))
    }

    #[classmethod]
    #[pyo3(name = "REALM_FINISH_REENCRYPTION_MAINTENANCE")]
    fn realm_finish_reencryption_maintenance(
        _cls: &PyType,
        req: RealmFinishReencryptionMaintenanceReq,
    ) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::RealmFinishReencryptionMaintenance(req.0))
    }

    #[classmethod]
    #[pyo3(name = "REALM_GET_ROLE_CERTIFICATES")]
    fn realm_get_role_certificates(_cls: &PyType, req: RealmGetRoleCertificatesReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::RealmGetRoleCertificates(
            req.0,
        ))
    }

    #[classmethod]
    #[pyo3(name = "REALM_START_REENCRYPTION_MAINTENANCE")]
    fn realm_start_reencryption_maintenance(
        _cls: &PyType,
        req: RealmStartReencryptionMaintenanceReq,
    ) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::RealmStartReencryptionMaintenance(req.0))
    }

    #[classmethod]
    #[pyo3(name = "REALM_STATS")]
    fn realm_stats(_cls: &PyType, req: RealmStatsReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::RealmStats(req.0))
    }

    #[classmethod]
    #[pyo3(name = "REALM_STATUS")]
    fn realm_status(_cls: &PyType, req: RealmStatusReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::RealmStatus(req.0))
    }

    #[classmethod]
    #[pyo3(name = "REALM_UPDATE_ROLES")]
    fn realm_update_roles(_cls: &PyType, req: RealmUpdateRolesReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::RealmUpdateRoles(req.0))
    }

    #[classmethod]
    #[pyo3(name = "USER_CREATE")]
    fn user_create(_cls: &PyType, req: UserCreateReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::UserCreate(req.0))
    }

    #[classmethod]
    #[pyo3(name = "USER_GET")]
    fn user_get(_cls: &PyType, req: UserGetReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::UserGet(req.0))
    }

    #[classmethod]
    #[pyo3(name = "USER_REVOKE")]
    fn user_revoke(_cls: &PyType, req: UserRevokeReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::UserRevoke(req.0))
    }

    #[classmethod]
    #[pyo3(name = "VLOB_CREATE")]
    fn vlob_create(_cls: &PyType, req: VlobCreateReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::VlobCreate(req.0))
    }

    #[classmethod]
    #[pyo3(name = "VLOB_LIST_VERSIONS")]
    fn vlob_list_versions(_cls: &PyType, req: VlobListVersionsReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::VlobListVersions(req.0))
    }

    #[classmethod]
    #[pyo3(name = "VLOB_MAINTENANCE_GET_REENCRYPTION_BATCH")]
    fn vlob_maintenance_get_reencryption_batch(
        _cls: &PyType,
        req: VlobMaintenanceGetReencryptionBatchReq,
    ) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::VlobMaintenanceGetReencryptionBatch(req.0))
    }

    #[classmethod]
    #[pyo3(name = "VLOB_MAINTENANCE_SAVE_REENCRYPTION_BATCH")]
    fn vlob_maintenance_save_reencryption_batch(
        _cls: &PyType,
        req: VlobMaintenanceSaveReencryptionBatchReq,
    ) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::VlobMaintenanceSaveReencryptionBatch(req.0))
    }

    #[classmethod]
    #[pyo3(name = "VLOB_POLL_CHANGES")]
    fn vlob_poll_changes(_cls: &PyType, req: VlobPollChangesReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::VlobPollChanges(req.0))
    }

    #[classmethod]
    #[pyo3(name = "VLOB_READ")]
    fn vlob_read(_cls: &PyType, req: VlobReadReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::VlobRead(req.0))
    }

    #[classmethod]
    #[pyo3(name = "VLOB_UPDATE")]
    fn vlob_update(_cls: &PyType, req: VlobUpdateReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::VlobUpdate(req.0))
    }

    #[classmethod]
    #[pyo3(name = "ORGANIZATION_CONFIG")]
    fn organization_config(_cls: &PyType, req: OrganizationConfigReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::OrganizationConfig(req.0))
    }

    #[classmethod]
    #[pyo3(name = "PING")]
    fn ping(_cls: &PyType, req: AuthenticatedPingReq) -> Self {
        Self(authenticated_cmds::v2::AnyCmdReq::Ping(req.0))
    }
}

#[pyclass]
pub(crate) struct AuthenticatedCmds(Arc<client_connection::AuthenticatedCmds>);

#[pymethods]
impl AuthenticatedCmds {
    #[new]
    fn new(
        server_url: BackendOrganizationAddr,
        device_id: DeviceID,
        signing_key: SigningKey,
    ) -> PyResult<Self> {
        let auth_cmds =
            client_connection::client::generate_client(signing_key.0, device_id.0, server_url.0);
        Ok(Self(Arc::new(auth_cmds)))
    }

    fn send_command(&self, cmd: AuthenticatedCmdsType) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from(async move {
            match cmd.0 {
                authenticated_cmds::v2::AnyCmdReq::BlockCreate(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        block_create,
                        BlockCreateRep,
                        Ok,
                        NotFound,
                        Timeout,
                        AlreadyExists,
                        InMaintenance,
                        NotAllowed,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::BlockRead(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        block_read,
                        BlockReadRep,
                        Ok,
                        NotFound,
                        Timeout,
                        InMaintenance,
                        NotAllowed,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::DeviceCreate(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        device_create,
                        DeviceCreateRep,
                        Ok,
                        BadUserId,
                        InvalidCertification,
                        InvalidData,
                        AlreadyExists,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::EventsListen(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        events_listen,
                        EventsListenRep,
                        NoEvents,
                        Ok,
                        Cancelled,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::EventsSubscribe(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        events_subscribe,
                        EventsSubscribeRep,
                        Ok,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::HumanFind(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        human_find,
                        HumanFindRep,
                        Ok,
                        NotAllowed,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::Invite1GreeterWaitPeer(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        invite_1_greeter_wait_peer,
                        Invite1GreeterWaitPeerRep,
                        Ok,
                        NotFound,
                        InvalidState,
                        UnknownStatus,
                        AlreadyDeleted
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::Invite2aGreeterGetHashedNonce(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        invite_2a_greeter_get_hashed_nonce,
                        Invite2aGreeterGetHashedNonceRep,
                        Ok,
                        NotFound,
                        InvalidState,
                        UnknownStatus,
                        AlreadyDeleted
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::Invite2bGreeterSendNonce(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        invite_2b_greeter_send_nonce,
                        Invite2bGreeterSendNonceRep,
                        Ok,
                        NotFound,
                        InvalidState,
                        UnknownStatus,
                        AlreadyDeleted
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::Invite3aGreeterWaitPeerTrust(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        invite_3a_greeter_wait_peer_trust,
                        Invite3aGreeterWaitPeerTrustRep,
                        Ok,
                        NotFound,
                        InvalidState,
                        UnknownStatus,
                        AlreadyDeleted
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::Invite3bGreeterSignifyTrust(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        invite_3b_greeter_signify_trust,
                        Invite3bGreeterSignifyTrustRep,
                        Ok,
                        NotFound,
                        InvalidState,
                        UnknownStatus,
                        AlreadyDeleted
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::Invite4GreeterCommunicate(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        invite_4_greeter_communicate,
                        Invite4GreeterCommunicateRep,
                        Ok,
                        NotFound,
                        InvalidState,
                        UnknownStatus,
                        AlreadyDeleted
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::InviteDelete(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        invite_delete,
                        InviteDeleteRep,
                        Ok,
                        NotFound,
                        UnknownStatus,
                        AlreadyDeleted
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::InviteList(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        invite_list,
                        InviteListRep,
                        Ok,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::InviteNew(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        invite_new,
                        InviteNewRep,
                        NotAllowed,
                        AlreadyMember,
                        NotAvailable,
                        Ok,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::MessageGet(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        message_get,
                        MessageGetRep,
                        Ok,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::OrganizationStats(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        organization_stats,
                        OrganizationStatsRep,
                        NotFound,
                        NotAllowed,
                        Ok,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::Ping(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        ping,
                        AuthenticatedPingRep,
                        Ok,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::PkiEnrollmentAccept(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        pki_enrollment_accept,
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
                }
                authenticated_cmds::v2::AnyCmdReq::PkiEnrollmentList(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        pki_enrollment_list,
                        PkiEnrollmentListRep,
                        Ok,
                        NotAllowed,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::PkiEnrollmentReject(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        pki_enrollment_reject,
                        PkiEnrollmentRejectRep,
                        Ok,
                        NotFound,
                        NoLongerAvailable,
                        NotAllowed,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::RealmCreate(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        realm_create,
                        RealmCreateRep,
                        Ok,
                        NotFound,
                        BadTimestamp,
                        InvalidCertification,
                        InvalidData,
                        AlreadyExists,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::RealmFinishReencryptionMaintenance(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        realm_finish_reencryption_maintenance,
                        RealmFinishReencryptionMaintenanceRep,
                        Ok,
                        MaintenanceError,
                        NotInMaintenance,
                        NotAllowed,
                        BadEncryptionRevision,
                        NotFound,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::RealmGetRoleCertificates(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        realm_get_role_certificates,
                        RealmGetRoleCertificatesRep,
                        Ok,
                        NotAllowed,
                        NotFound,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::RealmStartReencryptionMaintenance(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        realm_start_reencryption_maintenance,
                        RealmStartReencryptionMaintenanceRep,
                        BadEncryptionRevision,
                        BadTimestamp,
                        InMaintenance,
                        MaintenanceError,
                        NotAllowed,
                        NotFound,
                        Ok,
                        ParticipantMismatch,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::RealmStats(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        realm_stats,
                        RealmStatsRep,
                        Ok,
                        NotAllowed,
                        NotFound,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::RealmStatus(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        realm_status,
                        RealmStatusRep,
                        Ok,
                        NotAllowed,
                        NotFound,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::RealmUpdateRoles(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        realm_update_roles,
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
                        IncompatibleProfile
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::UserCreate(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        user_create,
                        UserCreateRep,
                        Ok,
                        ActiveUsersLimitReached,
                        AlreadyExists,
                        InvalidCertification,
                        InvalidData,
                        NotAllowed,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::UserGet(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        user_get,
                        UserGetRep,
                        Ok,
                        NotFound,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::UserRevoke(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        user_revoke,
                        UserRevokeRep,
                        Ok,
                        InvalidCertification,
                        UnknownStatus,
                        NotAllowed,
                        NotFound,
                        AlreadyRevoked
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::VlobCreate(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        vlob_create,
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
                        Timeout
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::VlobListVersions(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        vlob_list_versions,
                        VlobListVersionsRep,
                        Ok,
                        NotAllowed,
                        InMaintenance,
                        UnknownStatus,
                        NotFound
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::VlobMaintenanceGetReencryptionBatch(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        vlob_maintenance_get_reencryption_batch,
                        VlobMaintenanceGetReencryptionBatchRep,
                        Ok,
                        BadEncryptionRevision,
                        MaintenanceError,
                        NotAllowed,
                        NotFound,
                        NotInMaintenance,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::VlobMaintenanceSaveReencryptionBatch(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        vlob_maintenance_save_reencryption_batch,
                        VlobMaintenanceSaveReencryptionBatchRep,
                        Ok,
                        BadEncryptionRevision,
                        MaintenanceError,
                        NotInMaintenance,
                        NotAllowed,
                        NotFound,
                        UnknownStatus
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::VlobPollChanges(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        vlob_poll_changes,
                        VlobPollChangesRep,
                        Ok,
                        UnknownStatus,
                        InMaintenance,
                        NotAllowed,
                        NotFound
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::VlobRead(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        vlob_read,
                        VlobReadRep,
                        Ok,
                        UnknownStatus,
                        BadVersion,
                        BadEncryptionRevision,
                        InMaintenance,
                        NotAllowed,
                        NotFound
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::VlobUpdate(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        vlob_update,
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
                        Timeout
                    )
                }
                authenticated_cmds::v2::AnyCmdReq::OrganizationConfig(req) => {
                    crate::binding_utils::send_command!(
                        auth_cmds,
                        req,
                        organization_config,
                        OrganizationConfigRep,
                        Ok,
                        UnknownStatus,
                        NotFound
                    )
                }
            }
        })
    }
}
