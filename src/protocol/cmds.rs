// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    import_exception,
    prelude::*,
    types::{PyBytes, PyType},
};

use libparsec::protocol::{authenticated_cmds, invited_cmds};

use crate::protocol::*;

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(Clone)]
pub struct AuthenticatedAnyCmdReq(pub authenticated_cmds::AnyCmdReq);

#[pymethods]
impl AuthenticatedAnyCmdReq {
    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>, py: Python) -> PyResult<PyObject> {
        use authenticated_cmds::AnyCmdReq;
        let cmd = AnyCmdReq::load(&buf).map_err(ProtocolError::new_err)?;
        Ok(match cmd {
            AnyCmdReq::BlockRead(x) => BlockReadReq(x).into_py(py),
            AnyCmdReq::BlockCreate(x) => BlockCreateReq(x).into_py(py),
            AnyCmdReq::DeviceCreate(x) => DeviceCreateReq(x).into_py(py),
            AnyCmdReq::EventsListen(x) => EventsListenReq(x).into_py(py),
            AnyCmdReq::EventsSubscribe(x) => EventsSubscribeReq(x).into_py(py),
            AnyCmdReq::HumanFind(x) => HumanFindReq(x).into_py(py),
            AnyCmdReq::InviteList(x) => InviteListReq(x).into_py(py),
            AnyCmdReq::Invite1GreeterWaitPeer(x) => Invite1GreeterWaitPeerReq(x).into_py(py),
            AnyCmdReq::Invite2aGreeterGetHashedNonce(x) => {
                Invite2aGreeterGetHashedNonceReq(x).into_py(py)
            }
            AnyCmdReq::Invite2bGreeterSendNonce(x) => Invite2bGreeterSendNonceReq(x).into_py(py),
            AnyCmdReq::Invite3aGreeterWaitPeerTrust(x) => {
                Invite3aGreeterWaitPeerTrustReq(x).into_py(py)
            }
            AnyCmdReq::Invite3bGreeterSignifyTrust(x) => {
                Invite3bGreeterSignifyTrustReq(x).into_py(py)
            }
            AnyCmdReq::Invite4GreeterCommunicate(x) => Invite4GreeterCommunicateReq(x).into_py(py),
            AnyCmdReq::InviteDelete(x) => InviteDeleteReq(x).into_py(py),
            AnyCmdReq::InviteNew(x) => InviteNewReq(x).into_py(py),
            AnyCmdReq::MessageGet(x) => MessageGetReq(x).into_py(py),
            AnyCmdReq::OrganizationStats(x) => OrganizationStatsReq(x).into_py(py),
            AnyCmdReq::OrganizationConfig(x) => OrganizationConfigReq(x).into_py(py),
            AnyCmdReq::Ping(x) => AuthenticatedPingReq(x).into_py(py),
            AnyCmdReq::RealmCreate(x) => RealmCreateReq(x).into_py(py),
            AnyCmdReq::RealmStatus(x) => RealmStatusReq(x).into_py(py),
            AnyCmdReq::RealmStats(x) => RealmStatsReq(x).into_py(py),
            AnyCmdReq::RealmGetRoleCertificates(x) => RealmGetRoleCertificatesReq(x).into_py(py),
            AnyCmdReq::RealmUpdateRoles(x) => RealmUpdateRolesReq(x).into_py(py),
            AnyCmdReq::RealmStartReencryptionMaintenance(x) => {
                RealmStartReencryptionMaintenanceReq(x).into_py(py)
            }
            AnyCmdReq::RealmFinishReencryptionMaintenance(x) => {
                RealmFinishReencryptionMaintenanceReq(x).into_py(py)
            }
            AnyCmdReq::UserGet(x) => UserGetReq(x).into_py(py),
            AnyCmdReq::UserCreate(x) => UserCreateReq(x).into_py(py),
            AnyCmdReq::UserRevoke(x) => UserRevokeReq(x).into_py(py),
            AnyCmdReq::VlobCreate(x) => VlobCreateReq(x).into_py(py),
            AnyCmdReq::VlobRead(x) => VlobReadReq(x).into_py(py),
            AnyCmdReq::VlobUpdate(x) => VlobUpdateReq(x).into_py(py),
            AnyCmdReq::VlobPollChanges(x) => VlobPollChangesReq(x).into_py(py),
            AnyCmdReq::VlobListVersions(x) => VlobListVersionsReq(x).into_py(py),
            AnyCmdReq::VlobMaintenanceGetReencryptionBatch(x) => {
                VlobMaintenanceGetReencryptionBatchReq(x).into_py(py)
            }
            AnyCmdReq::VlobMaintenanceSaveReencryptionBatch(x) => {
                VlobMaintenanceSaveReencryptionBatchReq(x).into_py(py)
            }
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub struct InvitedAnyCmdReq(pub invited_cmds::AnyCmdReq);

#[pymethods]
impl InvitedAnyCmdReq {
    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>, py: Python) -> PyResult<PyObject> {
        use invited_cmds::AnyCmdReq;
        let cmd = AnyCmdReq::load(&buf).map_err(ProtocolError::new_err)?;
        Ok(match cmd {
            AnyCmdReq::Invite1ClaimerWaitPeer(x) => Invite1ClaimerWaitPeerReq(x).into_py(py),
            AnyCmdReq::Invite2aClaimerSendHashedNonce(x) => {
                Invite2aClaimerSendHashedNonceReq(x).into_py(py)
            }
            AnyCmdReq::Invite2bClaimerSendNonce(x) => Invite2bClaimerSendNonceReq(x).into_py(py),
            AnyCmdReq::Invite3aClaimerSignifyTrust(x) => {
                Invite3aClaimerSignifyTrustReq(x).into_py(py)
            }
            AnyCmdReq::Invite3bClaimerWaitPeerTrust(x) => {
                Invite3bClaimerWaitPeerTrustReq(x).into_py(py)
            }
            AnyCmdReq::Invite4ClaimerCommunicate(x) => Invite4ClaimerCommunicateReq(x).into_py(py),
            AnyCmdReq::InviteInfo(x) => InviteInfoReq(x).into_py(py),
            AnyCmdReq::Ping(x) => InvitedPingReq(x).into_py(py),
        })
    }
}
