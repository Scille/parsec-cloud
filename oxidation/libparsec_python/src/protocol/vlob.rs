// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use pyo3::exceptions::PyValueError;
use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::types::{PyBytes, PyType};

use libparsec::protocol::authenticated_cmds::{
    vlob_create, vlob_list_versions, vlob_maintenance_get_reencryption_batch,
    vlob_maintenance_save_reencryption_batch, vlob_poll_changes, vlob_read, vlob_update,
};

use crate::ids::{DeviceID, RealmID, VlobID};
use crate::time::DateTime;

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct VlobCreateReq(pub vlob_create::Req);

#[pymethods]
impl VlobCreateReq {
    #[new]
    fn new(
        realm_id: RealmID,
        encryption_revision: u64,
        vlob_id: VlobID,
        timestamp: DateTime,
        blob: Vec<u8>,
    ) -> PyResult<Self> {
        let realm_id = realm_id.0;
        let vlob_id = vlob_id.0;
        let timestamp = timestamp.0;
        Ok(Self(vlob_create::Req {
            realm_id,
            encryption_revision,
            vlob_id,
            timestamp,
            blob,
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
    fn vlob_id(&self) -> PyResult<VlobID> {
        Ok(VlobID(self.0.vlob_id))
    }

    #[getter]
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.timestamp))
    }

    #[getter]
    fn blob(&self) -> PyResult<&[u8]> {
        Ok(&self.0.blob)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct VlobCreateRep(pub vlob_create::Rep);

#[pymethods]
impl VlobCreateRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_create::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyExists")]
    fn already_exists(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(vlob_create::Rep::AlreadyExists { reason }))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_create::Rep::NotAllowed))
    }

    #[classmethod]
    #[pyo3(name = "BadEncryptionRevision")]
    fn bad_encryption_revision(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_create::Rep::BadEncryptionRevision))
    }

    #[classmethod]
    #[pyo3(name = "InMaintenance")]
    fn in_maintenance(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_create::Rep::InMaintenance))
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
        vlob_create::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct VlobReadReq(pub vlob_read::Req);

#[pymethods]
impl VlobReadReq {
    #[new]
    fn new(
        encryption_revision: u64,
        vlob_id: VlobID,
        version: Option<u32>,
        timestamp: Option<DateTime>,
    ) -> PyResult<Self> {
        let vlob_id = vlob_id.0;
        let timestamp = timestamp.map(|x| x.0);
        Ok(Self(vlob_read::Req {
            encryption_revision,
            vlob_id,
            version,
            timestamp,
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
    fn encryption_revision(&self) -> PyResult<u64> {
        Ok(self.0.encryption_revision)
    }

    #[getter]
    fn vlob_id(&self) -> PyResult<VlobID> {
        Ok(VlobID(self.0.vlob_id))
    }

    #[getter]
    fn version(&self) -> PyResult<Option<u32>> {
        Ok(self.0.version)
    }

    #[getter]
    fn timestamp(&self) -> PyResult<Option<DateTime>> {
        Ok(self.0.timestamp.map(DateTime))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct VlobReadRep(pub vlob_read::Rep);

#[pymethods]
impl VlobReadRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(
        _cls: &PyType,
        version: u32,
        blob: Vec<u8>,
        author: DeviceID,
        timestamp: DateTime,
        author_last_role_granted_on: Option<DateTime>,
    ) -> PyResult<Self> {
        let author = author.0;
        let timestamp = timestamp.0;
        let author_last_role_granted_on =
            libparsec::types::Maybe::Present(author_last_role_granted_on.map(|x| x.0));
        Ok(Self(vlob_read::Rep::Ok {
            version,
            blob,
            author,
            timestamp,
            author_last_role_granted_on,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(vlob_read::Rep::NotFound { reason }))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_read::Rep::NotAllowed))
    }

    #[classmethod]
    #[pyo3(name = "BadVersion")]
    fn bad_version(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_read::Rep::BadVersion))
    }

    #[classmethod]
    #[pyo3(name = "BadEncryptionRevision")]
    fn bad_encryption_revision(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_read::Rep::BadEncryptionRevision))
    }

    #[classmethod]
    #[pyo3(name = "InMaintenance")]
    fn in_maintenance(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_read::Rep::InMaintenance))
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
        vlob_read::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct VlobUpdateReq(pub vlob_update::Req);

#[pymethods]
impl VlobUpdateReq {
    #[new]
    fn new(
        encryption_revision: u64,
        vlob_id: VlobID,
        timestamp: DateTime,
        version: u32,
        blob: Vec<u8>,
    ) -> PyResult<Self> {
        let vlob_id = vlob_id.0;
        let timestamp = timestamp.0;
        Ok(Self(vlob_update::Req {
            encryption_revision,
            vlob_id,
            timestamp,
            version,
            blob,
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
    fn encryption_revision(&self) -> PyResult<u64> {
        Ok(self.0.encryption_revision)
    }

    #[getter]
    fn vlob_id(&self) -> PyResult<VlobID> {
        Ok(VlobID(self.0.vlob_id))
    }

    #[getter]
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.timestamp))
    }

    #[getter]
    fn version(&self) -> PyResult<u32> {
        Ok(self.0.version)
    }

    #[getter]
    fn blob(&self) -> PyResult<&[u8]> {
        Ok(&self.0.blob)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct VlobUpdateRep(pub vlob_update::Rep);

#[pymethods]
impl VlobUpdateRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_update::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(vlob_update::Rep::NotFound { reason }))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_update::Rep::NotAllowed))
    }

    #[classmethod]
    #[pyo3(name = "BadVersion")]
    fn bad_version(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_update::Rep::BadVersion))
    }

    #[classmethod]
    #[pyo3(name = "BadEncryptionRevision")]
    fn bad_encryption_revision(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_update::Rep::BadEncryptionRevision))
    }

    #[classmethod]
    #[pyo3(name = "InMaintenance")]
    fn in_maintenance(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_update::Rep::InMaintenance))
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
        vlob_update::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct VlobPollChangesReq(pub vlob_poll_changes::Req);

#[pymethods]
impl VlobPollChangesReq {
    #[new]
    fn new(realm_id: RealmID, last_checkpoint: u64) -> PyResult<Self> {
        let realm_id = realm_id.0;
        Ok(Self(vlob_poll_changes::Req {
            realm_id,
            last_checkpoint,
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
    fn last_checkpoint(&self) -> PyResult<u64> {
        Ok(self.0.last_checkpoint)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct VlobPollChangesRep(pub vlob_poll_changes::Rep);

#[pymethods]
impl VlobPollChangesRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, changes: HashMap<VlobID, u64>, current_checkpoint: u64) -> PyResult<Self> {
        let changes = changes.into_iter().map(|(k, v)| (k.0, v)).collect();
        Ok(Self(vlob_poll_changes::Rep::Ok {
            changes,
            current_checkpoint,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(vlob_poll_changes::Rep::NotFound { reason }))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_poll_changes::Rep::NotAllowed))
    }

    #[classmethod]
    #[pyo3(name = "InMaintenance")]
    fn in_maintenance(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_poll_changes::Rep::InMaintenance))
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
        vlob_poll_changes::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct VlobListVersionsReq(pub vlob_list_versions::Req);

#[pymethods]
impl VlobListVersionsReq {
    #[new]
    fn new(vlob_id: VlobID) -> PyResult<Self> {
        let vlob_id = vlob_id.0;
        Ok(Self(vlob_list_versions::Req { vlob_id }))
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
    fn vlob_id(&self) -> PyResult<VlobID> {
        Ok(VlobID(self.0.vlob_id))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct VlobListVersionsRep(pub vlob_list_versions::Rep);

#[pymethods]
impl VlobListVersionsRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, versions: HashMap<u64, (DateTime, DeviceID)>) -> PyResult<Self> {
        let mut _versions = HashMap::new();
        for (k, (dt, id)) in versions.into_iter() {
            _versions.insert(k, (dt.0, id.0));
        }
        let versions = _versions;
        Ok(Self(vlob_list_versions::Rep::Ok { versions }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(vlob_list_versions::Rep::NotFound { reason }))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_list_versions::Rep::NotAllowed))
    }

    #[classmethod]
    #[pyo3(name = "InMaintenance")]
    fn in_maintenance(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(vlob_list_versions::Rep::InMaintenance))
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
        vlob_list_versions::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct VlobMaintenanceGetReencryptionBatchReq(
    pub vlob_maintenance_get_reencryption_batch::Req,
);

#[pymethods]
impl VlobMaintenanceGetReencryptionBatchReq {
    #[new]
    fn new(realm_id: RealmID, encryption_revision: u64, size: u64) -> PyResult<Self> {
        let realm_id = realm_id.0;
        Ok(Self(vlob_maintenance_get_reencryption_batch::Req {
            realm_id,
            encryption_revision,
            size,
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
    fn size(&self) -> PyResult<u64> {
        Ok(self.0.size)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct VlobMaintenanceGetReencryptionBatchRep(
    pub vlob_maintenance_get_reencryption_batch::Rep,
);

#[pymethods]
impl VlobMaintenanceGetReencryptionBatchRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, batch: Vec<ReencryptionBatchEntry>) -> PyResult<Self> {
        let batch = batch.into_iter().map(|entry| entry.0).collect();
        Ok(Self(vlob_maintenance_get_reencryption_batch::Rep::Ok {
            batch,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(
            vlob_maintenance_get_reencryption_batch::Rep::NotFound { reason },
        ))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(
            vlob_maintenance_get_reencryption_batch::Rep::NotAllowed,
        ))
    }

    #[classmethod]
    #[pyo3(name = "NotInMaintenance")]
    fn not_in_maintenance(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(
            vlob_maintenance_get_reencryption_batch::Rep::NotInMaintenance { reason },
        ))
    }

    #[classmethod]
    #[pyo3(name = "BadEncryptionRevision")]
    fn bad_encryption_revision(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(
            vlob_maintenance_get_reencryption_batch::Rep::BadEncryptionRevision,
        ))
    }

    #[classmethod]
    #[pyo3(name = "MaintenanceError")]
    fn maintenance_error(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(
            vlob_maintenance_get_reencryption_batch::Rep::MaintenanceError { reason },
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
        vlob_maintenance_get_reencryption_batch::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct ReencryptionBatchEntry(pub libparsec::types::ReencryptionBatchEntry);

#[pymethods]
impl ReencryptionBatchEntry {
    #[new]
    fn new(vlob_id: VlobID, version: u64, blob: Vec<u8>) -> PyResult<Self> {
        let vlob_id = vlob_id.0;
        Ok(Self(libparsec::types::ReencryptionBatchEntry {
            vlob_id,
            version,
            blob,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    #[getter]
    fn vlob_id(&self) -> PyResult<VlobID> {
        Ok(VlobID(self.0.vlob_id))
    }

    #[getter]
    fn version(&self) -> PyResult<u64> {
        Ok(self.0.version)
    }

    #[getter]
    fn blob(&self) -> PyResult<&[u8]> {
        Ok(&self.0.blob)
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct VlobMaintenanceSaveReencryptionBatchReq(
    pub vlob_maintenance_save_reencryption_batch::Req,
);

#[pymethods]
impl VlobMaintenanceSaveReencryptionBatchReq {
    #[new]
    fn new(
        realm_id: RealmID,
        encryption_revision: u64,
        batch: Vec<ReencryptionBatchEntry>,
    ) -> PyResult<Self> {
        let realm_id = realm_id.0;
        let batch = batch.into_iter().map(|rbe| rbe.0).collect();
        Ok(Self(vlob_maintenance_save_reencryption_batch::Req {
            realm_id,
            encryption_revision,
            batch,
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
    fn batch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyTuple> {
        Ok(PyTuple::new(
            py,
            self.0
                .batch
                .clone()
                .into_iter()
                .map(|b| ReencryptionBatchEntry(b).into_py(py)),
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct VlobMaintenanceSaveReencryptionBatchRep(
    pub vlob_maintenance_save_reencryption_batch::Rep,
);

#[pymethods]
impl VlobMaintenanceSaveReencryptionBatchRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, total: u64, done: u64) -> PyResult<Self> {
        Ok(Self(vlob_maintenance_save_reencryption_batch::Rep::Ok {
            total,
            done,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(
            vlob_maintenance_save_reencryption_batch::Rep::NotFound { reason },
        ))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(
            vlob_maintenance_save_reencryption_batch::Rep::NotAllowed,
        ))
    }

    #[classmethod]
    #[pyo3(name = "NotInMaintenance")]
    fn not_in_maintenance(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(
            vlob_maintenance_save_reencryption_batch::Rep::NotInMaintenance { reason },
        ))
    }

    #[classmethod]
    #[pyo3(name = "BadEncryptionRevision")]
    fn bad_encryption_revision(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(
            vlob_maintenance_save_reencryption_batch::Rep::BadEncryptionRevision,
        ))
    }

    #[classmethod]
    #[pyo3(name = "MaintenanceError")]
    fn maintenance_error(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(
            vlob_maintenance_save_reencryption_batch::Rep::MaintenanceError { reason },
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
        vlob_maintenance_save_reencryption_batch::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
