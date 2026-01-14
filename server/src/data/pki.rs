// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::Bytes;
use pyo3::{
    create_exception,
    exceptions::{PyException, PyValueError},
    pyclass, pyfunction, pymethods,
    types::{PyBytes, PyBytesMethods, PyString, PyType},
    Bound, PyObject, PyResult, Python,
};
use rustls_pki_types::CertificateDer;

use crate::{
    crypto::{PublicKey, VerifyKey},
    enumerate::UserProfile,
    ids::{DeviceID, DeviceLabel, HumanHandle, UserID},
    DateTime,
};

create_exception!(_parsec, PkiInvalidSignature, PyException);
create_exception!(_parsec, PkiUntrusted, PyException);
create_exception!(_parsec, PkiInvalidCertificateDER, PyValueError);

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
        profile: UserProfile,
        root_verify_key: VerifyKey,
    ) -> Self {
        Self(libparsec_types::PkiEnrollmentAnswerPayload {
            user_id: user_id.0,
            device_id: device_id.0,
            device_label: device_label.0,
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
        PyBytes::new(py, &self.0.dump())
    }
}

impl From<libparsec_types::PkiEnrollmentAnswerPayload> for PkiEnrollmentAnswerPayload {
    fn from(value: libparsec_types::PkiEnrollmentAnswerPayload) -> Self {
        Self(value)
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
    fn new(verify_key: VerifyKey, public_key: PublicKey, device_label: DeviceLabel) -> Self {
        Self(libparsec_types::PkiEnrollmentSubmitPayload {
            verify_key: verify_key.0,
            public_key: public_key.0,
            device_label: device_label.0,
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
    fn device_label(&self) -> DeviceLabel {
        DeviceLabel(self.0.device_label.clone())
    }

    #[classmethod]
    fn load(_cls: Bound<'_, PyType>, raw: &[u8]) -> PyResult<Self> {
        libparsec_types::PkiEnrollmentSubmitPayload::load(raw)
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(Self)
    }

    fn dump<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump())
    }
}

impl From<libparsec_types::PkiEnrollmentSubmitPayload> for PkiEnrollmentSubmitPayload {
    fn from(value: libparsec_types::PkiEnrollmentSubmitPayload) -> Self {
        Self(value)
    }
}

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    PkiSignatureAlgorithm,
    libparsec_types::PkiSignatureAlgorithm,
    [
        "RSASSA_PSS_SHA256",
        rsassa_pss_sha256,
        libparsec_types::PkiSignatureAlgorithm::RsassaPssSha256
    ],
);

#[pyclass]
#[derive(Clone)]
pub(crate) struct X509Certificate(pub libparsec_platform_pki::Certificate<'static>);

impl X509Certificate {
    fn to_end_certificate(&self) -> PyResult<libparsec_platform_pki::X509EndCertificate<'_>> {
        self.0
            .to_end_certificate()
            .map_err(|e| PkiInvalidCertificateDER::new_err(e.to_string()))
    }
}

#[pymethods]
impl X509Certificate {
    #[classmethod]
    fn try_from_pem(_cls: Bound<'_, PyType>, raw_pem: &[u8]) -> PyResult<Self> {
        libparsec_platform_pki::Certificate::try_from_pem(raw_pem)
            .map(|v| v.into_owned())
            .map_err(|e| PkiInvalidCertificateDER::new_err(e.to_string()))
            .map(Self)
    }

    #[classmethod]
    fn from_der(_cls: Bound<'_, PyType>, raw_der: &[u8]) -> Self {
        Self(libparsec_platform_pki::Certificate::from_der(raw_der).into_owned())
    }

    fn issuer<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        self.to_end_certificate()
            .map(|v| PyBytes::new(py, v.issuer()))
    }

    fn subject<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        self.to_end_certificate()
            .map(|v| PyBytes::new(py, v.subject()))
    }

    #[getter]
    fn der<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, self.0.as_ref())
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct X509CertificateInformation(
    pub libparsec_platform_pki::x509::X509CertificateInformation,
);

#[pymethods]
impl X509CertificateInformation {
    #[classmethod]
    fn load_der(_cls: Bound<'_, PyType>, raw_der: &[u8]) -> PyResult<Self> {
        libparsec_platform_pki::x509::X509CertificateInformation::load_der(raw_der)
            .map(Self)
            .map_err(|e| PkiInvalidCertificateDER::new_err(e.to_string()))
    }

    fn common_name<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyString>> {
        self.0.common_name().map(|s| PyString::new(py, s))
    }

    fn emails<'py>(&self, py: Python<'py>) -> Vec<Bound<'py, PyString>> {
        self.0.emails().map(|s| PyString::new(py, s)).collect()
    }

    fn email<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyString>> {
        self.0.email().map(|s| PyString::new(py, s))
    }

    fn human_handle(&self) -> PyResult<HumanHandle> {
        self.0
            .human_handle()
            .map(HumanHandle)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct TrustAnchor(pub rustls_pki_types::TrustAnchor<'static>);

impl TryFrom<&'_ rustls_pki_types::CertificateDer<'_>> for TrustAnchor {
    type Error = pyo3::PyErr;

    fn try_from(value: &'_ rustls_pki_types::CertificateDer<'_>) -> Result<Self, Self::Error> {
        let anchor = webpki::anchor_from_trusted_cert(value)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        let anchor = anchor.to_owned();

        Ok(Self(anchor))
    }
}

#[pymethods]
impl TrustAnchor {
    #[classmethod]
    fn try_from_pem(_cls: Bound<'_, PyType>, pem_data: &[u8]) -> PyResult<Self> {
        use rustls_pki_types::{pem::PemObject, CertificateDer};

        let cert_der = CertificateDer::from_pem_slice(pem_data)
            .map_err(|e| PkiInvalidCertificateDER::new_err(e.to_string()))?;

        Self::try_from(&cert_der)
    }

    #[classmethod]
    fn from_der(_cls: Bound<'_, PyType>, der_data: &[u8]) -> PyResult<Self> {
        let cert_der = CertificateDer::from_slice(der_data);
        Self::try_from(&cert_der)
    }

    #[getter]
    fn subject<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.subject)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct SignedMessage(pub libparsec_platform_pki::SignedMessage);

#[pymethods]
impl SignedMessage {
    #[new]
    fn new(algo: PkiSignatureAlgorithm, signature: &[u8], message: &[u8]) -> Self {
        Self(libparsec_platform_pki::SignedMessage {
            algo: algo.0,
            signature: Bytes::copy_from_slice(signature),
            message: Bytes::copy_from_slice(message),
        })
    }
}

#[pyfunction]
pub(crate) fn load_submit_payload<'py>(
    signed_message: SignedMessage,
    der_certificate: &[u8],
    intermediate_certs: Vec<Bound<'py, PyBytes>>,
    trusted_roots: Vec<TrustAnchor>,
    now: DateTime,
) -> PyResult<PkiEnrollmentSubmitPayload> {
    let trusted_roots = trusted_roots.into_iter().map(|v| v.0).collect::<Vec<_>>();
    libparsec_platform_pki::load_submit_payload(
        &signed_message.0,
        der_certificate,
        intermediate_certs.iter().map(|v| v.as_bytes()),
        &trusted_roots,
        now.0,
    )
    .map_err(|e| {
        use libparsec_platform_pki::LoadSubmitPayloadError::*;
        match e {
            InvalidCertificateDer(error) => PkiInvalidCertificateDER::new_err(error.to_string()),
            Untrusted(..) => PkiUntrusted::new_err(e.to_string()),
            InvalidSignature(..) => PkiInvalidSignature::new_err(e.to_string()),
            DataError(..) => PyValueError::new_err(e.to_string()),
        }
    })
    .map(Into::into)
}

#[pyfunction]
pub(crate) fn load_accept_payload<'py>(
    signed_message: SignedMessage,
    der_certificate: &[u8],
    intermediate_certs: Vec<Bound<'py, PyBytes>>,
    trusted_roots: Vec<TrustAnchor>,
    now: DateTime,
) -> PyResult<PkiEnrollmentAnswerPayload> {
    let trusted_roots = trusted_roots.into_iter().map(|v| v.0).collect::<Vec<_>>();
    libparsec_platform_pki::load_answer_payload(
        &signed_message.0,
        der_certificate,
        intermediate_certs.iter().map(|v| v.as_bytes()),
        &trusted_roots,
        now.0,
    )
    .map_err(|e| {
        use libparsec_platform_pki::LoadAnswerPayloadError::*;
        match e {
            InvalidCertificateDer(error) => PkiInvalidCertificateDER::new_err(error.to_string()),
            Untrusted(..) => PkiUntrusted::new_err(e.to_string()),
            InvalidSignature(..) => PkiInvalidSignature::new_err(e.to_string()),
            DataError(..) => PyValueError::new_err(e.to_string()),
        }
    })
    .map(Into::into)
}
