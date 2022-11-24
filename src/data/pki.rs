use pyo3::{
    pyclass, pymethods,
    types::{PyBytes, PyType},
    Python,
};

use crate::{
    api_crypto::{PublicKey, VerifyKey},
    data::DataResult,
    enumerate::UserProfile,
    ids::{DeviceID, DeviceLabel, HumanHandle},
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct PkiEnrollmentAnswerPayload(pub libparsec::types::PkiEnrollmentAnswerPayload);

crate::binding_utils::gen_proto!(PkiEnrollmentAnswerPayload, __repr__);
crate::binding_utils::gen_proto!(PkiEnrollmentAnswerPayload, __richcmp__, eq);

#[pymethods]
impl PkiEnrollmentAnswerPayload {
    #[new]
    fn new(
        device_id: DeviceID,
        device_label: Option<DeviceLabel>,
        human_handle: Option<HumanHandle>,
        profile: UserProfile,
        root_verify_key: VerifyKey,
    ) -> Self {
        Self(libparsec::types::PkiEnrollmentAnswerPayload {
            device_id: device_id.0,
            device_label: device_label.map(|x| x.0),
            human_handle: human_handle.map(|x| x.0),
            profile: profile.0,
            root_verify_key: root_verify_key.0,
        })
    }

    #[getter]
    fn device_id(&self) -> DeviceID {
        DeviceID(self.0.device_id.clone())
    }

    #[getter]
    fn device_label(&self) -> Option<DeviceLabel> {
        self.0.device_label.clone().map(DeviceLabel)
    }

    #[getter]
    fn human_handle(&self) -> Option<HumanHandle> {
        self.0.human_handle.clone().map(HumanHandle)
    }

    #[getter]
    fn profile(&self) -> UserProfile {
        UserProfile(self.0.profile)
    }

    #[getter]
    fn root_verify_key(&self) -> VerifyKey {
        VerifyKey(self.0.root_verify_key.clone())
    }

    #[classmethod]
    fn load(_cls: &PyType, raw: &[u8]) -> DataResult<Self> {
        Ok(libparsec::types::PkiEnrollmentAnswerPayload::load(raw).map(Self)?)
    }

    fn dump<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump())
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct PkiEnrollmentSubmitPayload(pub libparsec::types::PkiEnrollmentSubmitPayload);

crate::binding_utils::gen_proto!(PkiEnrollmentSubmitPayload, __repr__);
crate::binding_utils::gen_proto!(PkiEnrollmentSubmitPayload, __richcmp__, eq);

#[pymethods]
impl PkiEnrollmentSubmitPayload {
    #[new]
    fn new(
        verify_key: VerifyKey,
        public_key: PublicKey,
        requested_device_label: DeviceLabel,
    ) -> Self {
        Self(libparsec::types::PkiEnrollmentSubmitPayload {
            verify_key: verify_key.0,
            public_key: public_key.0,
            requested_device_label: requested_device_label.0,
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

    #[classmethod]
    fn load(_cls: &PyType, raw: &[u8]) -> DataResult<Self> {
        Ok(libparsec::types::PkiEnrollmentSubmitPayload::load(raw).map(Self)?)
    }

    fn dump<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump())
    }
}
