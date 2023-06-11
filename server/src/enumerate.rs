// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyBytes, PyTuple, PyType},
    IntoPy, PyObject, PyResult, Python,
};

use crate::{ProtocolErrorFields, ProtocolResult};

// #[non_exhaustive] macro must be set for every enum like type,
// because we would like to call `is` in `python`, then
// a static reference should be returned instead of a new object

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    InvitationStatus,
    libparsec::low_level::types::InvitationStatus,
    [
        "IDLE",
        idle,
        libparsec::low_level::types::InvitationStatus::Idle
    ],
    [
        "READY",
        ready,
        libparsec::low_level::types::InvitationStatus::Ready
    ],
    [
        "DELETED",
        deleted,
        libparsec::low_level::types::InvitationStatus::Deleted
    ]
);

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    InvitationType,
    libparsec::low_level::types::InvitationType,
    [
        "DEVICE",
        device,
        libparsec::low_level::types::InvitationType::Device
    ],
    [
        "USER",
        user,
        libparsec::low_level::types::InvitationType::User
    ]
);

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    RealmRole,
    libparsec::low_level::types::RealmRole,
    [
        "OWNER",
        owner,
        libparsec::low_level::types::RealmRole::Owner
    ],
    [
        "MANAGER",
        manager,
        libparsec::low_level::types::RealmRole::Manager
    ],
    [
        "CONTRIBUTOR",
        contributor,
        libparsec::low_level::types::RealmRole::Contributor
    ],
    [
        "READER",
        reader,
        libparsec::low_level::types::RealmRole::Reader
    ]
);

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    UserProfile,
    libparsec::low_level::types::UserProfile,
    [
        "ADMIN",
        admin,
        libparsec::low_level::types::UserProfile::Admin
    ],
    [
        "STANDARD",
        standard,
        libparsec::low_level::types::UserProfile::Standard
    ],
    [
        "OUTSIDER",
        outsider,
        libparsec::low_level::types::UserProfile::Outsider
    ]
);

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    DeviceFileType,
    libparsec::low_level::types::DeviceFileType,
    [
        "PASSWORD",
        password,
        libparsec::low_level::types::DeviceFileType::Password
    ],
    [
        "SMARTCARD",
        smartcard,
        libparsec::low_level::types::DeviceFileType::Smartcard
    ],
    [
        "RECOVERY",
        recovery,
        libparsec::low_level::types::DeviceFileType::Recovery
    ]
);

#[pymethods]
impl DeviceFileType {
    pub fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump().map_err(|e| {
                ProtocolErrorFields(
                    libparsec::low_level::protocol::ProtocolError::EncodingError {
                        exc: e.to_string(),
                    },
                )
            })?,
        ))
    }

    #[classmethod]
    pub fn load(_cls: &PyType, bytes: &[u8]) -> PyResult<&'static PyObject> {
        Ok(Self::convert(
            libparsec::low_level::types::DeviceFileType::load(bytes)
                .map_err(|_| PyValueError::new_err("Failed to deserialize"))?,
        ))
    }
}
