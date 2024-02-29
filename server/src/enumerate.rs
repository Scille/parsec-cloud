// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyTuple, PyType},
    IntoPy, PyObject, PyResult, Python,
};

// #[non_exhaustive] macro must be set for every enum like type,
// because we would like to call `is` in `python`, then
// a static reference should be returned instead of a new object

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    InvitationStatus,
    libparsec_types::InvitationStatus,
    ["IDLE", idle, libparsec_types::InvitationStatus::Idle],
    ["READY", ready, libparsec_types::InvitationStatus::Ready],
    [
        "CANCELLED",
        cancelled,
        libparsec_types::InvitationStatus::Cancelled
    ],
    [
        "FINISHED",
        finished,
        libparsec_types::InvitationStatus::Finished
    ]
);

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    InvitationType,
    libparsec_types::InvitationType,
    ["DEVICE", device, libparsec_types::InvitationType::Device],
    ["USER", user, libparsec_types::InvitationType::User]
);

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    RealmRole,
    libparsec_types::RealmRole,
    ["OWNER", owner, libparsec_types::RealmRole::Owner],
    ["MANAGER", manager, libparsec_types::RealmRole::Manager],
    [
        "CONTRIBUTOR",
        contributor,
        libparsec_types::RealmRole::Contributor
    ],
    ["READER", reader, libparsec_types::RealmRole::Reader]
);

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    UserProfile,
    libparsec_types::UserProfile,
    ["ADMIN", admin, libparsec_types::UserProfile::Admin],
    ["STANDARD", standard, libparsec_types::UserProfile::Standard],
    ["OUTSIDER", outsider, libparsec_types::UserProfile::Outsider]
);
