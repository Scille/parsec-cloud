// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyBytes, PyType},
    Bound, PyObject, PyResult, Python,
};

use crate::{
    crypto::{PublicKey, VerifyKey},
    enumerate::UserProfile,
    ids::{DeviceID, DeviceLabel, HumanHandle, UserID},
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct AsyncEnrollmentSubmitPayload(pub libparsec_types::AsyncEnrollmentSubmitPayload);

crate::binding_utils::gen_proto!(AsyncEnrollmentSubmitPayload, __repr__);
crate::binding_utils::gen_proto!(AsyncEnrollmentSubmitPayload, __copy__);
crate::binding_utils::gen_proto!(AsyncEnrollmentSubmitPayload, __deepcopy__);
crate::binding_utils::gen_proto!(AsyncEnrollmentSubmitPayload, __richcmp__, eq);

#[pymethods]
impl AsyncEnrollmentSubmitPayload {
    #[new]
    fn new(
        verify_key: VerifyKey,
        public_key: PublicKey,
        requested_device_label: DeviceLabel,
        requested_human_handle: HumanHandle,
    ) -> Self {
        Self(libparsec_types::AsyncEnrollmentSubmitPayload {
            verify_key: verify_key.0,
            public_key: public_key.0,
            requested_device_label: requested_device_label.0,
            requested_human_handle: requested_human_handle.0,
        })
    }

    #[getter]
    fn verify_key(&self) -> VerifyKey {
        VerifyKey(self.0.verify_key.clone())
    }

    #[getter]
    fn public_key(&self) -> PublicKey {
        PublicKey(self.0.public_key.clone())
    }

    #[getter]
    fn requested_device_label(&self) -> DeviceLabel {
        DeviceLabel(self.0.requested_device_label.clone())
    }

    #[getter]
    fn requested_human_handle(&self) -> HumanHandle {
        HumanHandle(self.0.requested_human_handle.clone())
    }

    #[classmethod]
    fn load(_cls: Bound<'_, PyType>, raw: &[u8]) -> PyResult<Self> {
        libparsec_types::AsyncEnrollmentSubmitPayload::load(raw)
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(Self)
    }

    fn dump<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump())
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct AsyncEnrollmentAcceptPayload(pub libparsec_types::AsyncEnrollmentAcceptPayload);

crate::binding_utils::gen_proto!(AsyncEnrollmentAcceptPayload, __repr__);
crate::binding_utils::gen_proto!(AsyncEnrollmentAcceptPayload, __copy__);
crate::binding_utils::gen_proto!(AsyncEnrollmentAcceptPayload, __deepcopy__);
crate::binding_utils::gen_proto!(AsyncEnrollmentAcceptPayload, __richcmp__, eq);

#[pymethods]
impl AsyncEnrollmentAcceptPayload {
    #[new]
    fn new(
        user_id: UserID,
        device_id: DeviceID,
        device_label: DeviceLabel,
        human_handle: HumanHandle,
        profile: UserProfile,
        root_verify_key: VerifyKey,
    ) -> Self {
        Self(libparsec_types::AsyncEnrollmentAcceptPayload {
            user_id: user_id.0,
            device_id: device_id.0,
            device_label: device_label.0,
            human_handle: human_handle.0,
            profile: profile.0,
            root_verify_key: root_verify_key.0,
        })
    }

    #[getter]
    fn user_id(&self) -> UserID {
        UserID(self.0.user_id)
    }

    #[getter]
    fn device_id(&self) -> DeviceID {
        DeviceID(self.0.device_id)
    }

    #[getter]
    fn device_label(&self) -> DeviceLabel {
        DeviceLabel(self.0.device_label.clone())
    }

    #[getter]
    fn human_handle(&self) -> HumanHandle {
        HumanHandle(self.0.human_handle.clone())
    }

    #[getter]
    fn profile(&self) -> &'static PyObject {
        UserProfile::convert(self.0.profile)
    }

    #[getter]
    fn root_verify_key(&self) -> VerifyKey {
        VerifyKey(self.0.root_verify_key.clone())
    }

    #[classmethod]
    fn load(_cls: Bound<'_, PyType>, raw: &[u8]) -> PyResult<Self> {
        libparsec_types::AsyncEnrollmentAcceptPayload::load(raw)
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(Self)
    }

    fn dump<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump())
    }
}
