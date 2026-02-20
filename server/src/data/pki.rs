// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    create_exception,
    exceptions::{PyException, PyValueError},
    pyclass, pymethods,
    types::{PyBytes, PyString, PyType},
    Bound, PyResult, Python,
};
use rustls_pki_types::CertificateDer;

use crate::ids::HumanHandle;

create_exception!(_parsec, PkiInvalidSignature, PyException);
create_exception!(_parsec, PkiUntrusted, PyException);
create_exception!(_parsec, PkiInvalidCertificateDER, PyValueError);

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
