// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    pyclass, pymethods,
    types::{PyBytes, PyString, PyType},
    Bound, PyResult, Python,
};

use crate::ids::HumanHandle;

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
pub(crate) struct X509Certificate {
    #[pyo3(get)]
    der: Py<PyBytes>,
    #[pyo3(get)]
    issuer: Py<PyBytes>,
    #[pyo3(get)]
    subject: Py<PyBytes>,
    #[pyo3(get)]
    sha256_fingerprint: Py<PyBytes>,
}

impl X509Certificate {
    fn new<'py>(py: Python<'py>, der: Bound<'py, PyBytes>) -> PyResult<Self> {
        let cert_der = rustls_pki_types::CertificateDer::from_slice(der.as_bytes());
        let end = webpki::EndEntityCert::try_from(&cert_der)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let issuer = PyBytes::new(py, end.issuer());
        let subject = PyBytes::new(py, end.subject());
        let sha256_fingerprint = {
            use sha2::Digest;
            PyBytes::new(py, sha2::Sha256::digest(der.as_bytes()).as_ref())
        };
        Ok(Self {
            der: der.unbind(),
            issuer: issuer.unbind(),
            subject: subject.unbind(),
            sha256_fingerprint: sha256_fingerprint.unbind(),
        })
    }
}

#[pymethods]
impl X509Certificate {
    #[classmethod]
    fn load_pem<'py>(_cls: Bound<'py, PyType>, py: Python<'py>, raw_pem: &[u8]) -> PyResult<Self> {
        use rustls_pki_types::pem::PemObject;

        let raw_der = rustls_pki_types::CertificateDer::from_pem_slice(raw_pem)
            .map(|cert_der| PyBytes::new(py, cert_der.as_ref()))
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Self::new(py, raw_der)
    }

    #[classmethod]
    fn load_der<'py>(
        _cls: Bound<'py, PyType>,
        py: Python<'py>,
        raw_der: Bound<'py, PyBytes>,
    ) -> PyResult<Self> {
        Self::new(py, raw_der)
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
            .map_err(|e| PyValueError::new_err(e.to_string()))
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
pub(crate) struct X509TrustAnchor(pub rustls_pki_types::TrustAnchor<'static>);

impl TryFrom<&'_ rustls_pki_types::CertificateDer<'_>> for X509TrustAnchor {
    type Error = pyo3::PyErr;

    fn try_from(value: &'_ rustls_pki_types::CertificateDer<'_>) -> Result<Self, Self::Error> {
        let anchor = webpki::anchor_from_trusted_cert(value)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        let anchor = anchor.to_owned();

        Ok(Self(anchor))
    }
}

#[pymethods]
impl X509TrustAnchor {
    #[classmethod]
    fn load_pem(_cls: Bound<'_, PyType>, pem_data: &[u8]) -> PyResult<Self> {
        use rustls_pki_types::pem::PemObject;

        let cert_der = rustls_pki_types::CertificateDer::from_pem_slice(pem_data)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        Self::try_from(&cert_der)
    }

    #[classmethod]
    fn load_der(_cls: Bound<'_, PyType>, der_data: &[u8]) -> PyResult<Self> {
        let cert_der = rustls_pki_types::CertificateDer::from_slice(der_data);
        Self::try_from(&cert_der)
    }

    #[getter]
    fn subject<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.subject)
    }
}
