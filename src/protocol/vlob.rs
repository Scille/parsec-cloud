// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use pyo3::{
    exceptions::PyNotImplementedError,
    prelude::*,
    types::{PyBytes, PyTuple},
};

use libparsec::protocol::authenticated_cmds::v2::{
    vlob_create, vlob_list_versions, vlob_maintenance_get_reencryption_batch,
    vlob_maintenance_save_reencryption_batch, vlob_poll_changes, vlob_read, vlob_update,
};
use libparsec::types::ProtocolRequest;

use crate::{
    binding_utils::BytesWrapper,
    ids::{DeviceID, RealmID, SequesterServiceID, VlobID},
    protocol::{
        error::{ProtocolError, ProtocolErrorFields, ProtocolResult},
        gen_rep, Bytes, ListOfBytes, OptionalDateTime, OptionalFloat, Reason,
    },
    time::DateTime,
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct ReencryptionBatchEntry(pub libparsec::types::ReencryptionBatchEntry);

crate::binding_utils::gen_proto!(ReencryptionBatchEntry, __repr__);
crate::binding_utils::gen_proto!(ReencryptionBatchEntry, __copy__);
crate::binding_utils::gen_proto!(ReencryptionBatchEntry, __deepcopy__);
crate::binding_utils::gen_proto!(ReencryptionBatchEntry, __richcmp__, eq);

#[pymethods]
impl ReencryptionBatchEntry {
    #[new]
    fn new(vlob_id: VlobID, version: u64, blob: BytesWrapper) -> PyResult<Self> {
        crate::binding_utils::unwrap_bytes!(blob);
        let vlob_id = vlob_id.0;
        Ok(Self(libparsec::types::ReencryptionBatchEntry {
            vlob_id,
            version,
            blob,
        }))
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
#[derive(Clone)]
pub(crate) struct VlobCreateReq(pub vlob_create::Req);

crate::binding_utils::gen_proto!(VlobCreateReq, __repr__);
crate::binding_utils::gen_proto!(VlobCreateReq, __copy__);
crate::binding_utils::gen_proto!(VlobCreateReq, __deepcopy__);
crate::binding_utils::gen_proto!(VlobCreateReq, __richcmp__, eq);

#[pymethods]
impl VlobCreateReq {
    #[new]
    fn new(
        realm_id: RealmID,
        encryption_revision: u64,
        vlob_id: VlobID,
        timestamp: DateTime,
        blob: BytesWrapper,
        sequester_blob: Option<HashMap<SequesterServiceID, BytesWrapper>>,
    ) -> PyResult<Self> {
        crate::binding_utils::unwrap_bytes!(blob);
        let realm_id = realm_id.0;
        let vlob_id = vlob_id.0;
        let timestamp = timestamp.0;
        let sequester_blob = libparsec::types::Maybe::Present(
            sequester_blob.map(|x| x.into_iter().map(|(k, v)| (k.0, v.into())).collect()),
        );
        Ok(Self(vlob_create::Req {
            realm_id,
            encryption_revision,
            vlob_id,
            timestamp,
            blob,
            sequester_blob,
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

    #[getter]
    fn sequester_blob<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Option<HashMap<SequesterServiceID, &'py PyBytes>>> {
        Ok(match &self.0.sequester_blob {
            libparsec::types::Maybe::Present(x) => x.as_ref().map(|x| {
                x.iter()
                    .map(|(k, v)| (SequesterServiceID(*k), PyBytes::new(py, v)))
                    .collect()
            }),
            libparsec::types::Maybe::Absent => None,
        })
    }
}

gen_rep!(
    vlob_create,
    VlobCreateRep,
    [AlreadyExists, reason: Reason],
    [NotAllowed],
    [BadEncryptionRevision],
    [InMaintenance],
    [RequireGreaterTimestamp, strictly_greater_than: DateTime],
    [
        BadTimestamp,
        reason: Reason,
        ballpark_client_early_offset: OptionalFloat,
        ballpark_client_late_offset: OptionalFloat,
        backend_timestamp: OptionalDateTime,
        client_timestamp: OptionalDateTime,
    ],
    [NotASequesteredOrganization],
    [
        SequesterInconsistency,
        sequester_authority_certificate: Bytes,
        sequester_services_certificates: ListOfBytes
    ],
    [
        RejectedBySequesterService,
        service_id: SequesterServiceID,
        service_label: String,
        reason: String
    ],
    [Timeout],
);

#[pyclass(extends=VlobCreateRep)]
pub(crate) struct VlobCreateRepOk;

#[pymethods]
impl VlobCreateRepOk {
    #[new]
    fn new() -> PyResult<(Self, VlobCreateRep)> {
        Ok((Self, VlobCreateRep(vlob_create::Rep::Ok)))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct VlobReadReq(pub vlob_read::Req);

crate::binding_utils::gen_proto!(VlobReadReq, __repr__);
crate::binding_utils::gen_proto!(VlobReadReq, __copy__);
crate::binding_utils::gen_proto!(VlobReadReq, __deepcopy__);
crate::binding_utils::gen_proto!(VlobReadReq, __richcmp__, eq);

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

gen_rep!(
    vlob_read,
    VlobReadRep,
    { .. },
    [NotFound, reason: Reason],
    [NotAllowed],
    [BadVersion],
    [BadEncryptionRevision],
    [InMaintenance],
);

#[pyclass(extends=VlobReadRep)]
pub(crate) struct VlobReadRepOk;

#[pymethods]
impl VlobReadRepOk {
    #[new]
    fn new(
        version: u32,
        blob: BytesWrapper,
        author: DeviceID,
        timestamp: DateTime,
        author_last_role_granted_on: DateTime,
    ) -> PyResult<(Self, VlobReadRep)> {
        crate::binding_utils::unwrap_bytes!(blob);
        Ok((
            Self,
            VlobReadRep(vlob_read::Rep::Ok {
                version,
                blob,
                author: author.0,
                timestamp: timestamp.0,
                author_last_role_granted_on: libparsec::types::Maybe::Present(
                    author_last_role_granted_on.0,
                ),
            }),
        ))
    }

    #[getter]
    fn version(_self: PyRef<'_, Self>) -> PyResult<u32> {
        Ok(match _self.as_ref().0 {
            vlob_read::Rep::Ok { version, .. } => version,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn blob<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(match &_self.as_ref().0 {
            vlob_read::Rep::Ok { blob, .. } => PyBytes::new(py, blob),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn author(_self: PyRef<'_, Self>) -> PyResult<DeviceID> {
        Ok(match &_self.as_ref().0 {
            vlob_read::Rep::Ok { author, .. } => DeviceID(author.clone()),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn timestamp(_self: PyRef<'_, Self>) -> PyResult<DateTime> {
        Ok(match _self.as_ref().0 {
            vlob_read::Rep::Ok { timestamp, .. } => DateTime(timestamp),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn author_last_role_granted_on(_self: PyRef<'_, Self>) -> PyResult<Option<DateTime>> {
        Ok(match &_self.as_ref().0 {
            vlob_read::Rep::Ok {
                author_last_role_granted_on,
                ..
            } => match author_last_role_granted_on {
                libparsec::types::Maybe::Present(x) => Some(DateTime(*x)),
                _ => None,
            },
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct VlobUpdateReq(pub vlob_update::Req);

crate::binding_utils::gen_proto!(VlobUpdateReq, __repr__);
crate::binding_utils::gen_proto!(VlobUpdateReq, __copy__);
crate::binding_utils::gen_proto!(VlobUpdateReq, __deepcopy__);
crate::binding_utils::gen_proto!(VlobUpdateReq, __richcmp__, eq);

#[pymethods]
impl VlobUpdateReq {
    #[new]
    fn new(
        encryption_revision: u64,
        vlob_id: VlobID,
        timestamp: DateTime,
        version: u32,
        blob: BytesWrapper,
        sequester_blob: Option<HashMap<SequesterServiceID, BytesWrapper>>,
    ) -> PyResult<Self> {
        crate::binding_utils::unwrap_bytes!(blob);
        let vlob_id = vlob_id.0;
        let timestamp = timestamp.0;
        let sequester_blob = libparsec::types::Maybe::Present(
            sequester_blob.map(|x| x.into_iter().map(|(k, v)| (k.0, v.into())).collect()),
        );
        Ok(Self(vlob_update::Req {
            encryption_revision,
            vlob_id,
            timestamp,
            version,
            blob,
            sequester_blob,
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

    #[getter]
    fn sequester_blob<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Option<HashMap<SequesterServiceID, &'py PyBytes>>> {
        Ok(match &self.0.sequester_blob {
            libparsec::types::Maybe::Present(x) => x.as_ref().map(|x| {
                x.iter()
                    .map(|(k, v)| (SequesterServiceID(*k), PyBytes::new(py, v)))
                    .collect()
            }),
            libparsec::types::Maybe::Absent => None,
        })
    }
}

gen_rep!(
    vlob_update,
    VlobUpdateRep,
    [NotFound, reason: Reason],
    [NotAllowed],
    [BadVersion],
    [BadEncryptionRevision],
    [InMaintenance],
    [RequireGreaterTimestamp, strictly_greater_than: DateTime],
    [
        BadTimestamp,
        reason: Reason,
        ballpark_client_early_offset: OptionalFloat,
        ballpark_client_late_offset: OptionalFloat,
        backend_timestamp: OptionalDateTime,
        client_timestamp: OptionalDateTime,
    ],
    [NotASequesteredOrganization],
    [
        SequesterInconsistency,
        sequester_authority_certificate: Bytes,
        sequester_services_certificates: ListOfBytes
    ],
    [
        RejectedBySequesterService,
        service_id: SequesterServiceID,
        service_label: String,
        reason: String
    ],
    [Timeout],
);

#[pyclass(extends=VlobUpdateRep)]
pub(crate) struct VlobUpdateRepOk;

#[pymethods]
impl VlobUpdateRepOk {
    #[new]
    fn new() -> PyResult<(Self, VlobUpdateRep)> {
        Ok((Self, VlobUpdateRep(vlob_update::Rep::Ok)))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct VlobPollChangesReq(pub vlob_poll_changes::Req);

crate::binding_utils::gen_proto!(VlobPollChangesReq, __repr__);
crate::binding_utils::gen_proto!(VlobPollChangesReq, __copy__);
crate::binding_utils::gen_proto!(VlobPollChangesReq, __deepcopy__);
crate::binding_utils::gen_proto!(VlobPollChangesReq, __richcmp__, eq);

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
    fn last_checkpoint(&self) -> PyResult<u64> {
        Ok(self.0.last_checkpoint)
    }
}

gen_rep!(
    vlob_poll_changes,
    VlobPollChangesRep,
    { .. },
    [NotFound, reason: Reason],
    [NotAllowed],
    [InMaintenance],
);

#[pyclass(extends=VlobPollChangesRep)]
pub(crate) struct VlobPollChangesRepOk;

#[pymethods]
impl VlobPollChangesRepOk {
    #[new]
    fn new(
        changes: HashMap<VlobID, u64>,
        current_checkpoint: u64,
    ) -> PyResult<(Self, VlobPollChangesRep)> {
        let changes = changes.into_iter().map(|(k, v)| (k.0, v)).collect();
        Ok((
            Self,
            VlobPollChangesRep(vlob_poll_changes::Rep::Ok {
                changes,
                current_checkpoint,
            }),
        ))
    }

    #[getter]
    fn changes(_self: PyRef<'_, Self>) -> PyResult<HashMap<VlobID, u64>> {
        Ok(match &_self.as_ref().0 {
            vlob_poll_changes::Rep::Ok { changes, .. } => {
                changes.iter().map(|(k, v)| (VlobID(*k), *v)).collect()
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn current_checkpoint(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            vlob_poll_changes::Rep::Ok {
                current_checkpoint, ..
            } => current_checkpoint,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct VlobListVersionsReq(pub vlob_list_versions::Req);

crate::binding_utils::gen_proto!(VlobListVersionsReq, __repr__);
crate::binding_utils::gen_proto!(VlobListVersionsReq, __copy__);
crate::binding_utils::gen_proto!(VlobListVersionsReq, __deepcopy__);
crate::binding_utils::gen_proto!(VlobListVersionsReq, __richcmp__, eq);

#[pymethods]
impl VlobListVersionsReq {
    #[new]
    fn new(vlob_id: VlobID) -> PyResult<Self> {
        let vlob_id = vlob_id.0;
        Ok(Self(vlob_list_versions::Req { vlob_id }))
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
    fn vlob_id(&self) -> PyResult<VlobID> {
        Ok(VlobID(self.0.vlob_id))
    }
}

gen_rep!(
    vlob_list_versions,
    VlobListVersionsRep,
    { .. },
    [NotFound, reason: Reason],
    [NotAllowed],
    [InMaintenance],
);

#[pyclass(extends=VlobListVersionsRep)]
pub(crate) struct VlobListVersionsRepOk;

#[pymethods]
impl VlobListVersionsRepOk {
    #[new]
    fn new(versions: HashMap<u64, (DateTime, DeviceID)>) -> PyResult<(Self, VlobListVersionsRep)> {
        let versions = versions
            .into_iter()
            .map(|(k, (dt, id))| (k, (dt.0, id.0)))
            .collect();
        Ok((
            Self,
            VlobListVersionsRep(vlob_list_versions::Rep::Ok { versions }),
        ))
    }

    #[getter]
    fn versions(_self: PyRef<'_, Self>) -> PyResult<HashMap<u64, (DateTime, DeviceID)>> {
        Ok(match &_self.as_ref().0 {
            vlob_list_versions::Rep::Ok { versions, .. } => versions
                .iter()
                .map(|(k, (dt, id))| (*k, (DateTime(*dt), DeviceID(id.clone()))))
                .collect(),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct VlobMaintenanceGetReencryptionBatchReq(
    pub vlob_maintenance_get_reencryption_batch::Req,
);

crate::binding_utils::gen_proto!(VlobMaintenanceGetReencryptionBatchReq, __repr__);
crate::binding_utils::gen_proto!(VlobMaintenanceGetReencryptionBatchReq, __copy__);
crate::binding_utils::gen_proto!(VlobMaintenanceGetReencryptionBatchReq, __deepcopy__);
crate::binding_utils::gen_proto!(VlobMaintenanceGetReencryptionBatchReq, __richcmp__, eq);

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
    fn size(&self) -> PyResult<u64> {
        Ok(self.0.size)
    }
}

gen_rep!(
    vlob_maintenance_get_reencryption_batch,
    VlobMaintenanceGetReencryptionBatchRep,
    { .. },
    [NotFound, reason: Reason],
    [NotAllowed],
    [NotInMaintenance, reason: Reason],
    [BadEncryptionRevision],
    [MaintenanceError, reason: Reason],
);

#[pyclass(extends=VlobMaintenanceGetReencryptionBatchRep)]
pub(crate) struct VlobMaintenanceGetReencryptionBatchRepOk;

#[pymethods]
impl VlobMaintenanceGetReencryptionBatchRepOk {
    #[new]
    fn new(
        batch: Vec<ReencryptionBatchEntry>,
    ) -> PyResult<(Self, VlobMaintenanceGetReencryptionBatchRep)> {
        let batch = batch.into_iter().map(|entry| entry.0).collect();
        Ok((
            Self,
            VlobMaintenanceGetReencryptionBatchRep(
                vlob_maintenance_get_reencryption_batch::Rep::Ok { batch },
            ),
        ))
    }

    #[getter]
    fn batch(_self: PyRef<'_, Self>) -> PyResult<Vec<ReencryptionBatchEntry>> {
        Ok(match &_self.as_ref().0 {
            vlob_maintenance_get_reencryption_batch::Rep::Ok { batch, .. } => batch
                .iter()
                .map(|entry| ReencryptionBatchEntry(entry.clone()))
                .collect(),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct VlobMaintenanceSaveReencryptionBatchReq(
    pub vlob_maintenance_save_reencryption_batch::Req,
);

crate::binding_utils::gen_proto!(VlobMaintenanceSaveReencryptionBatchReq, __repr__);
crate::binding_utils::gen_proto!(VlobMaintenanceSaveReencryptionBatchReq, __copy__);
crate::binding_utils::gen_proto!(VlobMaintenanceSaveReencryptionBatchReq, __deepcopy__);
crate::binding_utils::gen_proto!(VlobMaintenanceSaveReencryptionBatchReq, __richcmp__, eq);

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

gen_rep!(
    vlob_maintenance_save_reencryption_batch,
    VlobMaintenanceSaveReencryptionBatchRep,
    { .. },
    [NotFound, reason: Reason],
    [NotAllowed],
    [NotInMaintenance, reason: Reason],
    [BadEncryptionRevision],
    [MaintenanceError, reason: Reason],
);

#[pyclass(extends=VlobMaintenanceSaveReencryptionBatchRep)]
pub(crate) struct VlobMaintenanceSaveReencryptionBatchRepOk;

#[pymethods]
impl VlobMaintenanceSaveReencryptionBatchRepOk {
    #[new]
    fn new(total: u64, done: u64) -> PyResult<(Self, VlobMaintenanceSaveReencryptionBatchRep)> {
        Ok((
            Self,
            VlobMaintenanceSaveReencryptionBatchRep(
                vlob_maintenance_save_reencryption_batch::Rep::Ok { total, done },
            ),
        ))
    }

    #[getter]
    fn total(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            vlob_maintenance_save_reencryption_batch::Rep::Ok { total, .. } => total,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn done(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            vlob_maintenance_save_reencryption_batch::Rep::Ok { done, .. } => done,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}
