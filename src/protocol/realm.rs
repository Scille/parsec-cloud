// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::{PyNotImplementedError, PyValueError},
    prelude::*,
    types::{PyBytes, PyTuple, PyType},
};
use std::collections::HashMap;

use libparsec::protocol::authenticated_cmds::v2::{
    realm_create, realm_finish_reencryption_maintenance, realm_get_role_certificates,
    realm_start_reencryption_maintenance, realm_stats, realm_status, realm_update_roles,
};

use crate::{
    ids::{DeviceID, RealmID, UserID},
    protocol::{
        error::{ProtocolError, ProtocolErrorFields, ProtocolResult},
        gen_rep, OptionalDateTime, OptionalFloat, Reason,
    },
    time::DateTime,
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct MaintenanceType(realm_status::MaintenanceType);

crate::binding_utils::gen_proto!(MaintenanceType, __repr__);
crate::binding_utils::gen_proto!(MaintenanceType, __richcmp__, eq);

crate::binding_utils::impl_enum_field!(
    MaintenanceType,
    [
        "GARBAGE_COLLECTION",
        garbage_collection,
        realm_status::MaintenanceType::GarbageCollection
    ],
    [
        "REENCRYPTION",
        reencryption,
        realm_status::MaintenanceType::Reencryption
    ]
);

#[pyclass]
#[derive(Clone)]
pub(crate) struct RealmCreateReq(pub realm_create::Req);

crate::binding_utils::gen_proto!(RealmCreateReq, __repr__);
crate::binding_utils::gen_proto!(RealmCreateReq, __richcmp__, eq);

#[pymethods]
impl RealmCreateReq {
    #[new]
    fn new(role_certificate: Vec<u8>) -> PyResult<Self> {
        Ok(Self(realm_create::Req { role_certificate }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }

    #[getter]
    fn role_certificate(&self) -> PyResult<&[u8]> {
        Ok(&self.0.role_certificate)
    }
}

gen_rep!(
    realm_create,
    RealmCreateRep,
    [InvalidCertification, reason: Reason],
    [InvalidData, reason: Reason],
    [NotFound, reason: Reason],
    [AlreadyExists],
    [
        BadTimestamp,
        reason: Reason,
        ballpark_client_early_offset: OptionalFloat,
        ballpark_client_late_offset: OptionalFloat,
        backend_timestamp: OptionalDateTime,
        client_timestamp: OptionalDateTime,
    ],
);

#[pyclass(extends=RealmCreateRep)]
pub(crate) struct RealmCreateRepOk;

#[pymethods]
impl RealmCreateRepOk {
    #[new]
    fn new() -> PyResult<(Self, RealmCreateRep)> {
        Ok((Self, RealmCreateRep(realm_create::Rep::Ok)))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct RealmStatusReq(pub realm_status::Req);

crate::binding_utils::gen_proto!(RealmStatusReq, __repr__);
crate::binding_utils::gen_proto!(RealmStatusReq, __richcmp__, eq);

#[pymethods]
impl RealmStatusReq {
    #[new]
    fn new(realm_id: RealmID) -> PyResult<Self> {
        let realm_id = realm_id.0;
        Ok(Self(realm_status::Req { realm_id }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }

    #[getter]
    fn realm_id(&self) -> PyResult<RealmID> {
        Ok(RealmID(self.0.realm_id))
    }
}

gen_rep!(
    realm_status,
    RealmStatusRep,
    { .. },
    [NotAllowed],
    [NotFound, reason: Reason],
);

#[pyclass(extends=RealmStatusRep)]
pub(crate) struct RealmStatusRepOk;

#[pymethods]
impl RealmStatusRepOk {
    #[new]
    fn new(
        in_maintenance: bool,
        maintenance_type: Option<MaintenanceType>,
        maintenance_started_on: Option<DateTime>,
        maintenance_started_by: Option<DeviceID>,
        encryption_revision: u64,
    ) -> PyResult<(Self, RealmStatusRep)> {
        Ok((
            Self,
            RealmStatusRep(realm_status::Rep::Ok {
                in_maintenance,
                maintenance_type: maintenance_type.map(|x| x.0),
                maintenance_started_on: maintenance_started_on.map(|x| x.0),
                maintenance_started_by: maintenance_started_by.map(|x| x.0),
                encryption_revision,
            }),
        ))
    }

    #[getter]
    fn in_maintenance(_self: PyRef<'_, Self>) -> PyResult<bool> {
        Ok(match _self.as_ref().0 {
            realm_status::Rep::Ok { in_maintenance, .. } => in_maintenance,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn maintenance_type(_self: PyRef<'_, Self>) -> PyResult<Option<MaintenanceType>> {
        Ok(match &_self.as_ref().0 {
            realm_status::Rep::Ok {
                maintenance_type, ..
            } => maintenance_type.clone().map(MaintenanceType),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn maintenance_started_on(_self: PyRef<'_, Self>) -> PyResult<Option<DateTime>> {
        Ok(match _self.as_ref().0 {
            realm_status::Rep::Ok {
                maintenance_started_on,
                ..
            } => maintenance_started_on.map(DateTime),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn maintenance_started_by(_self: PyRef<'_, Self>) -> PyResult<Option<DeviceID>> {
        Ok(match &_self.as_ref().0 {
            realm_status::Rep::Ok {
                maintenance_started_by,
                ..
            } => maintenance_started_by.clone().map(DeviceID),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn encryption_revision(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            realm_status::Rep::Ok {
                encryption_revision,
                ..
            } => encryption_revision,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct RealmStatsReq(pub realm_stats::Req);

crate::binding_utils::gen_proto!(RealmStatsReq, __repr__);
crate::binding_utils::gen_proto!(RealmStatsReq, __richcmp__, eq);

#[pymethods]
impl RealmStatsReq {
    #[new]
    fn new(realm_id: RealmID) -> PyResult<Self> {
        let realm_id = realm_id.0;
        Ok(Self(realm_stats::Req { realm_id }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }

    #[getter]
    fn realm_id(&self) -> PyResult<RealmID> {
        Ok(RealmID(self.0.realm_id))
    }
}

gen_rep!(
    realm_stats,
    RealmStatsRep,
    { .. },
    [NotAllowed],
    [NotFound, reason: Reason],
);

#[pyclass(extends=RealmStatsRep)]
pub(crate) struct RealmStatsRepOk;

#[pymethods]
impl RealmStatsRepOk {
    #[new]
    fn new(blocks_size: u64, vlobs_size: u64) -> PyResult<(Self, RealmStatsRep)> {
        Ok((
            Self,
            RealmStatsRep(realm_stats::Rep::Ok {
                blocks_size,
                vlobs_size,
            }),
        ))
    }

    #[getter]
    fn blocks_size(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            realm_stats::Rep::Ok { blocks_size, .. } => blocks_size,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn vlobs_size(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            realm_stats::Rep::Ok { vlobs_size, .. } => vlobs_size,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct RealmGetRoleCertificatesReq(pub realm_get_role_certificates::Req);

crate::binding_utils::gen_proto!(RealmGetRoleCertificatesReq, __repr__);
crate::binding_utils::gen_proto!(RealmGetRoleCertificatesReq, __richcmp__, eq);

#[pymethods]
impl RealmGetRoleCertificatesReq {
    #[new]
    fn new(realm_id: RealmID) -> PyResult<Self> {
        let realm_id = realm_id.0;
        Ok(Self(realm_get_role_certificates::Req { realm_id }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }

    #[getter]
    fn realm_id(&self) -> PyResult<RealmID> {
        Ok(RealmID(self.0.realm_id))
    }
}

gen_rep!(
    realm_get_role_certificates,
    RealmGetRoleCertificatesRep,
    { .. },
    [NotAllowed],
    [NotFound, reason: Reason],
);

#[pyclass(extends=RealmGetRoleCertificatesRep)]
pub(crate) struct RealmGetRoleCertificatesRepOk;

#[pymethods]
impl RealmGetRoleCertificatesRepOk {
    #[new]
    fn new(certificates: Vec<Vec<u8>>) -> PyResult<(Self, RealmGetRoleCertificatesRep)> {
        Ok((
            Self,
            RealmGetRoleCertificatesRep(realm_get_role_certificates::Rep::Ok { certificates }),
        ))
    }

    #[getter]
    fn certificates<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyTuple> {
        Ok(match &_self.as_ref().0 {
            realm_get_role_certificates::Rep::Ok { certificates, .. } => {
                PyTuple::new(py, certificates.iter().map(|x| PyBytes::new(py, x)))
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct RealmUpdateRolesReq(pub realm_update_roles::Req);

crate::binding_utils::gen_proto!(RealmUpdateRolesReq, __repr__);
crate::binding_utils::gen_proto!(RealmUpdateRolesReq, __richcmp__, eq);

#[pymethods]
impl RealmUpdateRolesReq {
    #[new]
    fn new(role_certificate: Vec<u8>, recipient_message: Option<Vec<u8>>) -> PyResult<Self> {
        Ok(Self(realm_update_roles::Req {
            role_certificate,
            recipient_message,
        }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }

    #[getter]
    fn role_certificate(&self) -> PyResult<&[u8]> {
        Ok(&self.0.role_certificate)
    }

    #[getter]
    fn recipient_message(&self) -> PyResult<Option<&[u8]>> {
        Ok(self.0.recipient_message.as_ref().map(|m| &m[..]))
    }
}

gen_rep!(
    realm_update_roles,
    RealmUpdateRolesRep,
    [NotAllowed, reason: Reason],
    [InvalidCertification, reason: Reason],
    [InvalidData, reason: Reason],
    [AlreadyGranted],
    [IncompatibleProfile, reason: Reason],
    [NotFound, reason: Reason],
    [InMaintenance],
    [UserRevoked],
    [RequireGreaterTimestamp, strictly_greater_than: DateTime],
    [
        BadTimestamp,
        reason: Reason,
        ballpark_client_early_offset: OptionalFloat,
        ballpark_client_late_offset: OptionalFloat,
        backend_timestamp: OptionalDateTime,
        client_timestamp: OptionalDateTime,
    ],
);

#[pyclass(extends=RealmUpdateRolesRep)]
pub(crate) struct RealmUpdateRolesRepOk;

#[pymethods]
impl RealmUpdateRolesRepOk {
    #[new]
    fn new() -> PyResult<(Self, RealmUpdateRolesRep)> {
        Ok((Self, RealmUpdateRolesRep(realm_update_roles::Rep::Ok)))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct RealmStartReencryptionMaintenanceReq(
    pub realm_start_reencryption_maintenance::Req,
);

crate::binding_utils::gen_proto!(RealmStartReencryptionMaintenanceReq, __repr__);
crate::binding_utils::gen_proto!(RealmStartReencryptionMaintenanceReq, __richcmp__, eq);

#[pymethods]
impl RealmStartReencryptionMaintenanceReq {
    #[new]
    fn new(
        realm_id: RealmID,
        encryption_revision: u64,
        timestamp: DateTime,
        per_participant_message: HashMap<UserID, Vec<u8>>,
    ) -> PyResult<Self> {
        let realm_id = realm_id.0;
        let timestamp = timestamp.0;
        let per_participant_message = per_participant_message
            .into_iter()
            .map(|(k, v)| (k.0, v))
            .collect();
        Ok(Self(realm_start_reencryption_maintenance::Req {
            realm_id,
            encryption_revision,
            timestamp,
            per_participant_message,
        }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }

    #[getter]
    fn realm_id(&self) -> PyResult<RealmID> {
        Ok(RealmID(self.0.realm_id))
    }

    #[getter]
    fn encryption_revision(&self) -> PyResult<u64> {
        Ok(self.0.encryption_revision)
    }

    #[getter]
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.timestamp))
    }

    #[getter]
    fn per_participant_message<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<HashMap<UserID, &'py PyBytes>> {
        Ok(self
            .0
            .per_participant_message
            .iter()
            .map(|(k, v)| (UserID(k.clone()), PyBytes::new(py, v)))
            .collect())
    }
}

gen_rep!(
    realm_start_reencryption_maintenance,
    RealmStartReencryptionMaintenanceRep,
    [NotAllowed],
    [NotFound, reason: Reason],
    [BadEncryptionRevision],
    [ParticipantMismatch, reason: Reason],
    [MaintenanceError, reason: Reason],
    [InMaintenance],
    [
        BadTimestamp,
        reason: Reason,
        ballpark_client_early_offset: OptionalFloat,
        ballpark_client_late_offset: OptionalFloat,
        backend_timestamp: OptionalDateTime,
        client_timestamp: OptionalDateTime,
    ],
);

#[pyclass(extends=RealmStartReencryptionMaintenanceRep)]
pub(crate) struct RealmStartReencryptionMaintenanceRepOk;

#[pymethods]
impl RealmStartReencryptionMaintenanceRepOk {
    #[new]
    fn new() -> PyResult<(Self, RealmStartReencryptionMaintenanceRep)> {
        Ok((
            Self,
            RealmStartReencryptionMaintenanceRep(realm_start_reencryption_maintenance::Rep::Ok),
        ))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct RealmFinishReencryptionMaintenanceReq(
    pub realm_finish_reencryption_maintenance::Req,
);

crate::binding_utils::gen_proto!(RealmFinishReencryptionMaintenanceReq, __repr__);
crate::binding_utils::gen_proto!(RealmFinishReencryptionMaintenanceReq, __richcmp__, eq);

#[pymethods]
impl RealmFinishReencryptionMaintenanceReq {
    #[new]
    fn new(realm_id: RealmID, encryption_revision: u64) -> PyResult<Self> {
        let realm_id = realm_id.0;
        Ok(Self(realm_finish_reencryption_maintenance::Req {
            realm_id,
            encryption_revision,
        }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }

    #[getter]
    fn realm_id(&self) -> PyResult<RealmID> {
        Ok(RealmID(self.0.realm_id))
    }

    #[getter]
    fn encryption_revision(&self) -> PyResult<u64> {
        Ok(self.0.encryption_revision)
    }
}

gen_rep!(
    realm_finish_reencryption_maintenance,
    RealmFinishReencryptionMaintenanceRep,
    [NotAllowed],
    [NotFound, reason: Reason],
    [BadEncryptionRevision],
    [NotInMaintenance, reason: Reason],
    [MaintenanceError, reason: Reason],
);

#[pyclass(extends=RealmFinishReencryptionMaintenanceRep)]
pub(crate) struct RealmFinishReencryptionMaintenanceRepOk;

#[pymethods]
impl RealmFinishReencryptionMaintenanceRepOk {
    #[new]
    fn new() -> PyResult<(Self, RealmFinishReencryptionMaintenanceRep)> {
        Ok((
            Self,
            RealmFinishReencryptionMaintenanceRep(realm_finish_reencryption_maintenance::Rep::Ok),
        ))
    }
}
