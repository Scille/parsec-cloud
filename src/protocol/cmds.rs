// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};

use libparsec::protocol::{authenticated_cmds, invited_cmds};

use crate::protocol::{
    AuthenticatedPingReq, BlockCreateReq, BlockReadReq, InvitedPingReq, MessageGetReq,
    OrganizationConfigReq, OrganizationStatsReq, RealmCreateReq,
    RealmFinishReencryptionMaintenanceReq, RealmGetRoleCertificatesReq,
    RealmStartReencryptionMaintenanceReq, RealmStatsReq, RealmStatusReq, RealmUpdateRolesReq,
    VlobCreateReq, VlobListVersionsReq, VlobMaintenanceGetReencryptionBatchReq,
    VlobMaintenanceSaveReencryptionBatchReq, VlobPollChangesReq, VlobReadReq, VlobUpdateReq,
};

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct AuthenticatedAnyCmdReq(pub authenticated_cmds::AnyCmdReq);

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
        Ok(
            match AnyCmdReq::load(&buf).map_err(ProtocolError::new_err)? {
                AnyCmdReq::BlockRead(x) => BlockReadReq(x).into_py(py),
                AnyCmdReq::BlockCreate(x) => BlockCreateReq(x).into_py(py),
                AnyCmdReq::MessageGet(x) => MessageGetReq(x).into_py(py),
                AnyCmdReq::OrganizationStats(x) => OrganizationStatsReq(x).into_py(py),
                AnyCmdReq::OrganizationConfig(x) => OrganizationConfigReq(x).into_py(py),
                AnyCmdReq::RealmCreate(x) => RealmCreateReq(x).into_py(py),
                AnyCmdReq::RealmStatus(x) => RealmStatusReq(x).into_py(py),
                AnyCmdReq::RealmStats(x) => RealmStatsReq(x).into_py(py),
                AnyCmdReq::RealmGetRoleCertificates(x) => {
                    RealmGetRoleCertificatesReq(x).into_py(py)
                }
                AnyCmdReq::RealmUpdateRoles(x) => RealmUpdateRolesReq(x).into_py(py),
                AnyCmdReq::RealmStartReencryptionMaintenance(x) => {
                    RealmStartReencryptionMaintenanceReq(x).into_py(py)
                }
                AnyCmdReq::RealmFinishReencryptionMaintenance(x) => {
                    RealmFinishReencryptionMaintenanceReq(x).into_py(py)
                }
                AnyCmdReq::Ping(x) => AuthenticatedPingReq(x).into_py(py),
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
                _ => unimplemented!(),
            },
        )
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct InvitedAnyCmdReq(pub invited_cmds::AnyCmdReq);

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
        Ok(
            match AnyCmdReq::load(&buf).map_err(ProtocolError::new_err)? {
                AnyCmdReq::Ping(x) => InvitedPingReq(x).into_py(py),
                _ => unimplemented!(),
            },
        )
    }
}
