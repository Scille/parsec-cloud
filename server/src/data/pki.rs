// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    pyclass, pymethods,
    types::{IntoPyDict, PyBytes, PyDict, PyType},
    PyAny, PyObject, PyResult, Python,
};
use std::{collections::HashMap, path::Path};

use crate::{
    BackendPkiEnrollmentAddr, BytesWrapper, DataResult, DateTime, DeviceID, DeviceLabel,
    EnrollmentID, HumanHandle, PkiEnrollmentLocalPendingResult, PublicKey, UserProfile, VerifyKey,
};

crate::binding_utils::gen_py_wrapper_class!(
    PkiEnrollmentAnswerPayload,
    libparsec::low_level::types::PkiEnrollmentAnswerPayload,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl PkiEnrollmentAnswerPayload {
    #[new]
    #[pyo3(signature = (device_id, device_label, human_handle, profile, root_verify_key))]
    fn new(
        device_id: DeviceID,
        device_label: Option<DeviceLabel>,
        human_handle: Option<HumanHandle>,
        profile: UserProfile,
        root_verify_key: VerifyKey,
    ) -> Self {
        Self(libparsec::low_level::types::PkiEnrollmentAnswerPayload {
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
    fn profile(&self) -> &'static PyObject {
        UserProfile::convert(self.0.profile)
    }

    #[getter]
    fn root_verify_key(&self) -> VerifyKey {
        VerifyKey(self.0.root_verify_key.clone())
    }

    #[classmethod]
    fn load(_cls: &PyType, raw: &[u8]) -> DataResult<Self> {
        Ok(libparsec::low_level::types::PkiEnrollmentAnswerPayload::load(raw).map(Self)?)
    }

    fn dump<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump())
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    PkiEnrollmentSubmitPayload,
    libparsec::low_level::types::PkiEnrollmentSubmitPayload,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl PkiEnrollmentSubmitPayload {
    #[new]
    fn new(
        verify_key: VerifyKey,
        public_key: PublicKey,
        requested_device_label: DeviceLabel,
    ) -> Self {
        Self(libparsec::low_level::types::PkiEnrollmentSubmitPayload {
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
        Ok(libparsec::low_level::types::PkiEnrollmentSubmitPayload::load(raw).map(Self)?)
    }

    fn dump<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump())
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    X509Certificate,
    libparsec::low_level::types::X509Certificate,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl X509Certificate {
    #[new]
    fn new(
        issuer: HashMap<String, String>,
        subject: HashMap<String, String>,
        der_x509_certificate: BytesWrapper,
        certificate_sha1: BytesWrapper,
        certificate_id: Option<String>,
    ) -> Self {
        crate::binding_utils::unwrap_bytes!(der_x509_certificate, certificate_sha1);
        Self(libparsec::low_level::types::X509Certificate {
            issuer,
            subject,
            der_x509_certificate,
            certificate_sha1,
            certificate_id,
        })
    }

    #[pyo3(signature = (**py_kwargs))]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [issuer: HashMap<String, String>, "issuer"],
            [subject: HashMap<String, String>, "subject"],
            [der_x509_certificate: BytesWrapper, "der_x509_certificate"],
            [certificate_sha1: BytesWrapper, "certificate_sha1"],
            [certificate_id: Option<String>, "certificate_id"],
        );
        crate::binding_utils::unwrap_bytes!(der_x509_certificate, certificate_sha1);

        let mut r = self.0.clone();

        if let Some(v) = issuer {
            r.issuer = v;
        }
        if let Some(v) = subject {
            r.subject = v;
        }
        if let Some(v) = der_x509_certificate {
            r.der_x509_certificate = v;
        }
        if let Some(v) = certificate_sha1 {
            r.certificate_sha1 = v;
        }
        if let Some(v) = certificate_id {
            r.certificate_id = v
        }

        Ok(Self(r))
    }

    fn is_available_locally(&self) -> bool {
        self.0.is_available_locally()
    }

    #[getter]
    fn subject_common_name(&self) -> Option<&String> {
        self.0.subject_common_name()
    }

    #[getter]
    fn subject_email_address(&self) -> Option<&String> {
        self.0.subject_email_address()
    }

    #[getter]
    fn issuer_common_name(&self) -> Option<&String> {
        self.0.issuer_common_name()
    }

    #[getter]
    fn issuer<'py>(&self, py: Python<'py>) -> &'py PyDict {
        self.0.issuer.clone().into_py_dict(py)
    }

    #[getter]
    fn subject<'py>(&self, py: Python<'py>) -> &'py PyDict {
        self.0.subject.clone().into_py_dict(py)
    }

    #[getter]
    fn der_x509_certificate<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.der_x509_certificate)
    }

    #[getter]
    fn certificate_sha1<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.certificate_sha1)
    }

    #[getter]
    fn certificate_id(&self) -> Option<&String> {
        self.0.certificate_id.as_ref()
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    LocalPendingEnrollment,
    libparsec::low_level::types::LocalPendingEnrollment,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl LocalPendingEnrollment {
    #[new]
    fn new(
        x509_certificate: X509Certificate,
        addr: BackendPkiEnrollmentAddr,
        submitted_on: DateTime,
        enrollment_id: EnrollmentID,
        submit_payload: PkiEnrollmentSubmitPayload,
        encrypted_key: BytesWrapper,
        ciphertext: BytesWrapper,
    ) -> Self {
        crate::binding_utils::unwrap_bytes!(encrypted_key, ciphertext);
        Self(libparsec::low_level::types::LocalPendingEnrollment {
            x509_certificate: x509_certificate.0,
            addr: addr.0,
            submitted_on: submitted_on.0,
            enrollment_id: enrollment_id.0,
            submit_payload: submit_payload.0,
            encrypted_key,
            ciphertext,
        })
    }

    #[classmethod]
    fn load(_cls: &PyType, raw: &[u8]) -> DataResult<Self> {
        Ok(libparsec::low_level::types::LocalPendingEnrollment::load(raw).map(Self)?)
    }

    fn dump<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump())
    }

    fn save(&self, config_dir: &PyAny) -> PkiEnrollmentLocalPendingResult<String> {
        let config_dir = config_dir
            .call_method0("__str__")
            .expect("config_dir should be a Path")
            .extract::<&str>()
            .map(Path::new)
            .expect("Unreachable");
        Ok(self
            .0
            .save(config_dir)
            .map(|x| x.to_str().expect("Unreachable").to_string())?)
    }

    #[classmethod]
    fn load_from_path(_cls: &PyType, path: &PyAny) -> PkiEnrollmentLocalPendingResult<Self> {
        let path = path
            .call_method0("__str__")
            .expect("path should be a Path")
            .extract::<&str>()
            .map(Path::new)
            .expect("Unreachable");
        Ok(libparsec::low_level::types::LocalPendingEnrollment::load_from_path(path).map(Self)?)
    }

    #[classmethod]
    fn load_from_enrollment_id(
        _cls: &PyType,
        config_dir: &PyAny,
        enrollment_id: EnrollmentID,
    ) -> PkiEnrollmentLocalPendingResult<Self> {
        let config_dir = config_dir
            .call_method0("__str__")
            .expect("config_dir should be a Path")
            .extract::<&str>()
            .map(Path::new)
            .expect("Unreachable");
        Ok(
            libparsec::low_level::types::LocalPendingEnrollment::load_from_enrollment_id(
                config_dir,
                enrollment_id.0,
            )
            .map(Self)?,
        )
    }

    #[classmethod]
    fn remove_from_enrollment_id(
        _cls: &PyType,
        config_dir: &PyAny,
        enrollment_id: EnrollmentID,
    ) -> PkiEnrollmentLocalPendingResult<()> {
        let config_dir = config_dir
            .call_method0("__str__")
            .expect("config_dir should be a Path")
            .extract::<&str>()
            .map(Path::new)
            .expect("Unreachable");
        Ok(
            libparsec::low_level::types::LocalPendingEnrollment::remove_from_enrollment_id(
                config_dir,
                enrollment_id.0,
            )?,
        )
    }

    #[classmethod]
    fn list(_cls: &PyType, config_dir: &PyAny) -> Vec<Self> {
        let config_dir = config_dir
            .call_method0("__str__")
            .expect("config_dir should be a Path")
            .extract::<&str>()
            .map(Path::new)
            .expect("Unreachable");
        libparsec::low_level::types::LocalPendingEnrollment::list(config_dir)
            .into_iter()
            .map(Self)
            .collect()
    }

    #[getter]
    fn x509_certificate(&self) -> X509Certificate {
        X509Certificate(self.0.x509_certificate.clone())
    }

    #[getter]
    fn addr(&self) -> BackendPkiEnrollmentAddr {
        BackendPkiEnrollmentAddr(self.0.addr.clone())
    }

    #[getter]
    fn submitted_on(&self) -> DateTime {
        DateTime(self.0.submitted_on)
    }

    #[getter]
    fn enrollment_id(&self) -> EnrollmentID {
        EnrollmentID(self.0.enrollment_id)
    }

    #[getter]
    fn submit_payload(&self) -> PkiEnrollmentSubmitPayload {
        PkiEnrollmentSubmitPayload(self.0.submit_payload.clone())
    }

    #[getter]
    fn encrypted_key<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.encrypted_key)
    }

    #[getter]
    fn ciphertext<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.ciphertext)
    }
}
