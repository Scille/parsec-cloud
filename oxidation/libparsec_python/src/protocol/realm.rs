// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::exceptions::PyValueError;
use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};
use std::collections::HashMap;

use libparsec::protocol::authenticated_cmds::{
    realm_create, realm_finish_reencryption_maintenance, realm_get_role_certificates,
    realm_start_reencryption_maintenance, realm_stats, realm_status, realm_update_roles,
};

use crate::ids::{DeviceID, RealmID, UserID};
use crate::time::DateTime;

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct RealmCreateReq(pub realm_create::Req);

#[pymethods]
impl RealmCreateReq {
    #[new]
    fn new(role_certificate: Vec<u8>) -> PyResult<Self> {
        Ok(Self(realm_create::Req { role_certificate }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn roles_certificate(&self) -> PyResult<&[u8]> {
        Ok(&self.0.role_certificate)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct RealmCreateRep(pub realm_create::Rep);

#[pymethods]
impl RealmCreateRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(realm_create::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "InvalidCertification")]
    fn invalid_certification(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(realm_create::Rep::InvalidCertification { reason }))
    }

    #[classmethod]
    #[pyo3(name = "InvalidData")]
    fn invalid_data(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(realm_create::Rep::InvalidData { reason }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(realm_create::Rep::NotFound { reason }))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyExists")]
    fn already_exists(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(realm_create::Rep::AlreadyExists))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        realm_create::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct RealmStatusReq(pub realm_status::Req);

#[pymethods]
impl RealmStatusReq {
    #[new]
    fn new(realm_id: RealmID) -> PyResult<Self> {
        let realm_id = realm_id.0;
        Ok(Self(realm_status::Req { realm_id }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn realm_id(&self) -> PyResult<RealmID> {
        Ok(RealmID(self.0.realm_id))
    }
}

pub fn py_to_rs_maintenance_type(
    maintenance_type: &PyAny,
) -> PyResult<realm_status::MaintenanceType> {
    use realm_status::MaintenanceType::*;
    Ok(match maintenance_type.getattr("name")?.extract::<&str>()? {
        "GARBAGE_COLLECTION" => GarbageCollection,
        "REENCRYPTION" => Reencryption,
        _ => unreachable!(),
    })
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct RealmStatusRep(pub realm_status::Rep);

#[pymethods]
impl RealmStatusRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(
        _cls: &PyType,
        in_maintenance: bool,
        maintenance_type: Option<&PyAny>,
        maintenance_started_on: Option<DateTime>,
        maintenance_started_by: Option<DeviceID>,
        encryption_revision: u64,
    ) -> PyResult<Self> {
        let maintenance_type = maintenance_type
            .map(py_to_rs_maintenance_type)
            .transpose()?;
        let maintenance_started_on = maintenance_started_on.map(|x| x.0);
        let maintenance_started_by = maintenance_started_by.map(|id| id.0);
        Ok(Self(realm_status::Rep::Ok {
            in_maintenance,
            maintenance_type,
            maintenance_started_on,
            maintenance_started_by,
            encryption_revision,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(realm_status::Rep::NotAllowed))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(realm_status::Rep::NotFound { reason }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        realm_status::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct RealmStatsReq(pub realm_stats::Req);

#[pymethods]
impl RealmStatsReq {
    #[new]
    fn new(realm_id: RealmID) -> PyResult<Self> {
        let realm_id = realm_id.0;
        Ok(Self(realm_stats::Req { realm_id }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn realm_id(&self) -> PyResult<RealmID> {
        Ok(RealmID(self.0.realm_id))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct RealmStatsRep(pub realm_stats::Rep);

#[pymethods]
impl RealmStatsRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, blocks_size: u64, vlobs_size: u64) -> PyResult<Self> {
        Ok(Self(realm_stats::Rep::Ok {
            blocks_size,
            vlobs_size,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(realm_stats::Rep::NotAllowed))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(realm_stats::Rep::NotFound { reason }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        realm_stats::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct RealmGetRoleCertificateReq(pub realm_get_role_certificates::Req);

#[pymethods]
impl RealmGetRoleCertificateReq {
    #[new]
    fn new(realm_id: RealmID) -> PyResult<Self> {
        let realm_id = realm_id.0;
        Ok(Self(realm_get_role_certificates::Req { realm_id }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn realm_id(&self) -> PyResult<RealmID> {
        Ok(RealmID(self.0.realm_id))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct RealmGetRoleCertificateRep(pub realm_get_role_certificates::Rep);

#[pymethods]
impl RealmGetRoleCertificateRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, certificates: Vec<Vec<u8>>) -> PyResult<Self> {
        Ok(Self(realm_get_role_certificates::Rep::Ok { certificates }))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(realm_get_role_certificates::Rep::NotAllowed))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(realm_get_role_certificates::Rep::NotFound { reason }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        realm_get_role_certificates::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct RealmUpdateRolesReq(pub realm_update_roles::Req);

#[pymethods]
impl RealmUpdateRolesReq {
    #[new]
    fn new(role_certificate: Vec<u8>, recipient_message: Option<Vec<u8>>) -> PyResult<Self> {
        Ok(Self(realm_update_roles::Req {
            role_certificate,
            recipient_message,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
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

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct RealmUpdateRolesRep(pub realm_update_roles::Rep);

#[pymethods]
impl RealmUpdateRolesRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(realm_update_roles::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(realm_update_roles::Rep::NotAllowed { reason }))
    }

    #[classmethod]
    #[pyo3(name = "InvalidCertification")]
    fn invalid_certification(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(realm_update_roles::Rep::InvalidCertification {
            reason,
        }))
    }

    #[classmethod]
    #[pyo3(name = "InvalidData")]
    fn invalid_data(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(realm_update_roles::Rep::InvalidData { reason }))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyGranted")]
    fn already_granted(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(realm_update_roles::Rep::AlreadyGranted))
    }

    #[classmethod]
    #[pyo3(name = "IncompatibleProfile")]
    fn incompatible_profile(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(realm_update_roles::Rep::IncompatibleProfile {
            reason,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(realm_update_roles::Rep::NotFound { reason }))
    }

    #[classmethod]
    #[pyo3(name = "InMaintenance")]
    fn in_maintenance(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(realm_update_roles::Rep::InMaintenance))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        realm_update_roles::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct RealmStartReencryptionMaintenanceReq(
    pub realm_start_reencryption_maintenance::Req,
);

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

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
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
    fn per_participant_message(&self) -> PyResult<HashMap<UserID, Vec<u8>>> {
        Ok(self
            .0
            .per_participant_message
            .clone()
            .into_iter()
            .map(|(k, v)| (UserID(k), v))
            .collect())
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct RealmStartReencryptionMaintenanceRep(
    pub realm_start_reencryption_maintenance::Rep,
);

#[pymethods]
impl RealmStartReencryptionMaintenanceRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(realm_start_reencryption_maintenance::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(realm_start_reencryption_maintenance::Rep::NotAllowed))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(realm_start_reencryption_maintenance::Rep::NotFound {
            reason,
        }))
    }

    #[classmethod]
    #[pyo3(name = "BadEncryptionRevision")]
    fn bad_encryption_revision(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(
            realm_start_reencryption_maintenance::Rep::BadEncryptionRevision,
        ))
    }

    #[classmethod]
    #[pyo3(name = "ParticipantMismatch")]
    fn participant_mismatch(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(
            realm_start_reencryption_maintenance::Rep::ParticipantMismatch { reason },
        ))
    }

    #[classmethod]
    #[pyo3(name = "MaintenanceError")]
    fn maintenance_error(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(
            realm_start_reencryption_maintenance::Rep::MaintenanceError { reason },
        ))
    }

    #[classmethod]
    #[pyo3(name = "InMaintenance")]
    fn in_maintenance(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(
            realm_start_reencryption_maintenance::Rep::InMaintenance,
        ))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        realm_start_reencryption_maintenance::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct RealmFinishReencryptionMaintenanceReq(
    pub realm_finish_reencryption_maintenance::Req,
);

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

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
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

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct RealmFinishReencryptionMaintenanceRep(
    pub realm_finish_reencryption_maintenance::Rep,
);

#[pymethods]
impl RealmFinishReencryptionMaintenanceRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(realm_finish_reencryption_maintenance::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(realm_finish_reencryption_maintenance::Rep::NotAllowed))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(realm_finish_reencryption_maintenance::Rep::NotFound {
            reason,
        }))
    }

    #[classmethod]
    #[pyo3(name = "BadEncryptionRevision")]
    fn bad_encryption_revision(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(
            realm_finish_reencryption_maintenance::Rep::BadEncryptionRevision,
        ))
    }

    #[classmethod]
    #[pyo3(name = "NotInMaintenance")]
    fn not_in_maintenance(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(
            realm_finish_reencryption_maintenance::Rep::NotInMaintenance { reason },
        ))
    }

    #[classmethod]
    #[pyo3(name = "MaintenanceError")]
    fn maintenance_error(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(
            realm_finish_reencryption_maintenance::Rep::MaintenanceError { reason },
        ))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        realm_finish_reencryption_maintenance::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
