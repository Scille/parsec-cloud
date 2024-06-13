// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{IntoPyDict, PyAnyMethods, PyBytes, PyDict, PyType},
    Bound, PyAny, PyObject, PyResult, Python,
};
use std::{collections::HashMap, path::Path};

use crate::{
    addrs::ParsecPkiEnrollmentAddr,
    binding_utils::BytesWrapper,
    crypto::{PublicKey, VerifyKey},
    enumerate::UserProfile,
    ids::{DeviceID, DeviceLabel, EnrollmentID, HumanHandle, UserID},
    time::DateTime,
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct PkiEnrollmentAnswerPayload(pub libparsec_types::PkiEnrollmentAnswerPayload);

crate::binding_utils::gen_proto!(PkiEnrollmentAnswerPayload, __repr__);
crate::binding_utils::gen_proto!(PkiEnrollmentAnswerPayload, __copy__);
crate::binding_utils::gen_proto!(PkiEnrollmentAnswerPayload, __deepcopy__);
crate::binding_utils::gen_proto!(PkiEnrollmentAnswerPayload, __richcmp__, eq);

#[pymethods]
impl PkiEnrollmentAnswerPayload {
    #[new]
    fn new(
        user_id: UserID,
        device_id: DeviceID,
        device_label: DeviceLabel,
        human_handle: HumanHandle,
        profile: UserProfile,
        root_verify_key: VerifyKey,
    ) -> Self {
        Self(libparsec_types::PkiEnrollmentAnswerPayload {
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
        libparsec_types::PkiEnrollmentAnswerPayload::load(raw)
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(Self)
    }

    fn dump<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.dump())
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct PkiEnrollmentSubmitPayload(pub libparsec_types::PkiEnrollmentSubmitPayload);

crate::binding_utils::gen_proto!(PkiEnrollmentSubmitPayload, __repr__);
crate::binding_utils::gen_proto!(PkiEnrollmentSubmitPayload, __copy__);
crate::binding_utils::gen_proto!(PkiEnrollmentSubmitPayload, __deepcopy__);
crate::binding_utils::gen_proto!(PkiEnrollmentSubmitPayload, __richcmp__, eq);

#[pymethods]
impl PkiEnrollmentSubmitPayload {
    #[new]
    fn new(
        verify_key: VerifyKey,
        public_key: PublicKey,
        requested_device_label: DeviceLabel,
    ) -> Self {
        Self(libparsec_types::PkiEnrollmentSubmitPayload {
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
    fn load(_cls: Bound<'_, PyType>, raw: &[u8]) -> PyResult<Self> {
        libparsec_types::PkiEnrollmentSubmitPayload::load(raw)
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(Self)
    }

    fn dump<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.dump())
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct X509Certificate(pub libparsec_types::X509Certificate);

crate::binding_utils::gen_proto!(X509Certificate, __repr__);
crate::binding_utils::gen_proto!(X509Certificate, __copy__);
crate::binding_utils::gen_proto!(X509Certificate, __deepcopy__);
crate::binding_utils::gen_proto!(X509Certificate, __richcmp__, eq);

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
        Self(libparsec_types::X509Certificate {
            issuer,
            subject,
            der_x509_certificate,
            certificate_sha1,
            certificate_id,
        })
    }

    #[pyo3(signature = (**py_kwargs))]
    fn evolve(&self, py_kwargs: Option<Bound<'_, PyDict>>) -> PyResult<Self> {
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
    fn issuer<'py>(&self, py: Python<'py>) -> Bound<'py, PyDict> {
        self.0.issuer.clone().into_py_dict_bound(py)
    }

    #[getter]
    fn subject<'py>(&self, py: Python<'py>) -> Bound<'py, PyDict> {
        self.0.subject.clone().into_py_dict_bound(py)
    }

    #[getter]
    fn der_x509_certificate<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.der_x509_certificate)
    }

    #[getter]
    fn certificate_sha1<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.certificate_sha1)
    }

    #[getter]
    fn certificate_id(&self) -> Option<&String> {
        self.0.certificate_id.as_ref()
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct LocalPendingEnrollment(pub libparsec_types::LocalPendingEnrollment);

crate::binding_utils::gen_proto!(LocalPendingEnrollment, __repr__);
crate::binding_utils::gen_proto!(LocalPendingEnrollment, __copy__);
crate::binding_utils::gen_proto!(LocalPendingEnrollment, __deepcopy__);
crate::binding_utils::gen_proto!(LocalPendingEnrollment, __richcmp__, eq);

#[pymethods]
impl LocalPendingEnrollment {
    #[new]
    fn new(
        x509_certificate: X509Certificate,
        addr: ParsecPkiEnrollmentAddr,
        submitted_on: DateTime,
        enrollment_id: EnrollmentID,
        submit_payload: PkiEnrollmentSubmitPayload,
        encrypted_key: BytesWrapper,
        ciphertext: BytesWrapper,
    ) -> Self {
        crate::binding_utils::unwrap_bytes!(encrypted_key, ciphertext);
        Self(libparsec_types::LocalPendingEnrollment {
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
    fn load(_cls: Bound<'_, PyType>, raw: &[u8]) -> PyResult<Self> {
        libparsec_types::LocalPendingEnrollment::load(raw)
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(Self)
    }

    fn dump<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.dump())
    }

    fn save(&self, config_dir: Bound<'_, PyAny>) -> PyResult<String> {
        let config_dir_pyobj = config_dir
            .call_method0("__str__")
            .expect("config_dir should be a Path");
        let config_dir = config_dir_pyobj
            .extract::<&str>()
            .map(Path::new)
            .expect("Unreachable");
        let path = self
            .0
            .save(config_dir)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let path_str = path
            .to_str()
            .ok_or(PyValueError::new_err("Non UTF8 path"))?;
        Ok(path_str.to_string())
    }

    #[classmethod]
    fn load_from_path(_cls: Bound<'_, PyType>, path: Bound<'_, PyAny>) -> PyResult<Self> {
        let path_pyobj = path.call_method0("__str__").expect("path should be a Path");
        let path = path_pyobj
            .extract::<&str>()
            .map(Path::new)
            .expect("Unreachable");
        libparsec_types::LocalPendingEnrollment::load_from_path(path)
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(Self)
    }

    #[classmethod]
    fn load_from_enrollment_id(
        _cls: Bound<'_, PyType>,
        config_dir: Bound<'_, PyAny>,
        enrollment_id: EnrollmentID,
    ) -> PyResult<Self> {
        let config_dir_pyobj = config_dir
            .call_method0("__str__")
            .expect("config_dir should be a Path");
        let config_dir = config_dir_pyobj
            .extract::<&str>()
            .map(Path::new)
            .expect("Unreachable");
        libparsec_types::LocalPendingEnrollment::load_from_enrollment_id(
            config_dir,
            enrollment_id.0,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(Self)
    }

    #[classmethod]
    fn remove_from_enrollment_id(
        _cls: Bound<'_, PyType>,
        config_dir: Bound<'_, PyAny>,
        enrollment_id: EnrollmentID,
    ) -> PyResult<()> {
        let config_dir_pyobj = config_dir
            .call_method0("__str__")
            .expect("config_dir should be a Path");
        let config_dir = config_dir_pyobj
            .extract::<&str>()
            .map(Path::new)
            .expect("Unreachable");
        libparsec_types::LocalPendingEnrollment::remove_from_enrollment_id(
            config_dir,
            enrollment_id.0,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[classmethod]
    fn list(_cls: Bound<'_, PyType>, config_dir: Bound<'_, PyAny>) -> Vec<Self> {
        let config_dir_pyobj = config_dir
            .call_method0("__str__")
            .expect("config_dir should be a Path");
        let config_dir = config_dir_pyobj
            .extract::<&str>()
            .map(Path::new)
            .expect("Unreachable");
        libparsec_types::LocalPendingEnrollment::list(config_dir)
            .into_iter()
            .map(Self)
            .collect()
    }

    #[getter]
    fn x509_certificate(&self) -> X509Certificate {
        X509Certificate(self.0.x509_certificate.clone())
    }

    #[getter]
    fn addr(&self) -> ParsecPkiEnrollmentAddr {
        ParsecPkiEnrollmentAddr(self.0.addr.clone())
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
    fn encrypted_key<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.encrypted_key)
    }

    #[getter]
    fn ciphertext<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.ciphertext)
    }
}
