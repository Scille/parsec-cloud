// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyBytes, PyTuple, PyType},
    IntoPy, PyObject, PyResult, Python,
};

use libparsec::types;

use crate::protocol::{ProtocolErrorFields, ProtocolResult};

// #[non_exhaustive] macro must be set for every enum like type,
// because we would like to call `is` in `python`, then
// a static reference should be returned instead of a new object

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct InvitationStatus(pub libparsec::types::InvitationStatus);

crate::binding_utils::gen_proto!(InvitationStatus, __repr__);
crate::binding_utils::gen_proto!(InvitationStatus, __copy__);
crate::binding_utils::gen_proto!(InvitationStatus, __deepcopy__);
crate::binding_utils::gen_proto!(InvitationStatus, __richcmp__, eq);
crate::binding_utils::gen_proto!(InvitationStatus, __hash__);

crate::binding_utils::impl_enum_field!(
    InvitationStatus,
    ["IDLE", idle, libparsec::types::InvitationStatus::Idle],
    ["READY", ready, libparsec::types::InvitationStatus::Ready],
    [
        "DELETED",
        deleted,
        libparsec::types::InvitationStatus::Deleted
    ]
);

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct InvitationType(pub libparsec::types::InvitationType);

crate::binding_utils::gen_proto!(InvitationType, __repr__);
crate::binding_utils::gen_proto!(InvitationType, __copy__);
crate::binding_utils::gen_proto!(InvitationType, __deepcopy__);
crate::binding_utils::gen_proto!(InvitationType, __richcmp__, eq);
crate::binding_utils::gen_proto!(InvitationType, __hash__);

crate::binding_utils::impl_enum_field!(
    InvitationType,
    ["DEVICE", device, libparsec::types::InvitationType::Device],
    ["USER", user, libparsec::types::InvitationType::User]
);

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct RealmRole(pub libparsec::types::RealmRole);

crate::binding_utils::gen_proto!(RealmRole, __repr__);
crate::binding_utils::gen_proto!(RealmRole, __copy__);
crate::binding_utils::gen_proto!(RealmRole, __deepcopy__);
crate::binding_utils::gen_proto!(RealmRole, __richcmp__, eq);
crate::binding_utils::gen_proto!(RealmRole, __hash__);

crate::binding_utils::impl_enum_field!(
    RealmRole,
    ["OWNER", owner, libparsec::types::RealmRole::Owner],
    ["MANAGER", manager, libparsec::types::RealmRole::Manager],
    [
        "CONTRIBUTOR",
        contributor,
        libparsec::types::RealmRole::Contributor
    ],
    ["READER", reader, libparsec::types::RealmRole::Reader]
);

impl RealmRole {
    pub(crate) fn from_role(profile: libparsec::types::RealmRole) -> &'static PyObject {
        match profile {
            libparsec::types::RealmRole::Owner => RealmRole::owner(),
            libparsec::types::RealmRole::Manager => RealmRole::manager(),
            libparsec::types::RealmRole::Contributor => RealmRole::contributor(),
            libparsec::types::RealmRole::Reader => RealmRole::reader(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct UserProfile(pub libparsec::types::UserProfile);

crate::binding_utils::gen_proto!(UserProfile, __repr__);
crate::binding_utils::gen_proto!(UserProfile, __copy__);
crate::binding_utils::gen_proto!(UserProfile, __deepcopy__);
crate::binding_utils::gen_proto!(UserProfile, __richcmp__, eq);
crate::binding_utils::gen_proto!(UserProfile, __hash__);

crate::binding_utils::impl_enum_field!(
    UserProfile,
    ["ADMIN", admin, libparsec::types::UserProfile::Admin],
    [
        "STANDARD",
        standard,
        libparsec::types::UserProfile::Standard
    ],
    [
        "OUTSIDER",
        outsider,
        libparsec::types::UserProfile::Outsider
    ]
);

impl UserProfile {
    pub(crate) fn from_profile(profile: libparsec::types::UserProfile) -> &'static PyObject {
        match profile {
            libparsec::types::UserProfile::Admin => UserProfile::admin(),
            libparsec::types::UserProfile::Standard => UserProfile::standard(),
            libparsec::types::UserProfile::Outsider => UserProfile::outsider(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct DeviceFileType(pub types::DeviceFileType);

crate::binding_utils::impl_enum_field!(
    DeviceFileType,
    ["PASSWORD", password, types::DeviceFileType::Password],
    ["SMARTCARD", smartcard, types::DeviceFileType::Smartcard],
    ["RECOVERY", recovery, types::DeviceFileType::Recovery]
);

#[pymethods]
impl DeviceFileType {
    pub fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }

    #[classmethod]
    pub fn load(_cls: &PyType, bytes: &[u8]) -> PyResult<Self> {
        Ok(Self(types::DeviceFileType::load(bytes).map_err(|_| {
            PyValueError::new_err("Failed to deserialize")
        })?))
    }
}

crate::binding_utils::gen_proto!(DeviceFileType, __hash__);
crate::binding_utils::gen_proto!(DeviceFileType, __repr__);
crate::binding_utils::gen_proto!(DeviceFileType, __copy__);
crate::binding_utils::gen_proto!(DeviceFileType, __deepcopy__);
crate::binding_utils::gen_proto!(DeviceFileType, __richcmp__, eq);
