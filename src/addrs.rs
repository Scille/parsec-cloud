// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    prelude::{pyclass, pymethods, IntoPy, PyObject, PyResult, Python, ToPyObject},
    pyfunction,
    types::{PyBytes, PyDict, PyType},
};
use std::str::FromStr;

use crate::{
    api_crypto::VerifyKey,
    binding_utils::BytesWrapper,
    enumerate::InvitationType,
    ids::{EntryID, InvitationToken, OrganizationID},
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct BackendAddr(pub libparsec::types::BackendAddr);

crate::binding_utils::gen_proto!(BackendAddr, __repr__);
crate::binding_utils::gen_proto!(BackendAddr, __copy__);
crate::binding_utils::gen_proto!(BackendAddr, __deepcopy__);
crate::binding_utils::gen_proto!(BackendAddr, __richcmp__, eq);
crate::binding_utils::gen_proto!(BackendAddr, __hash__);

#[pymethods]
impl BackendAddr {
    #[new]
    fn new(hostname: String, port: Option<u16>, use_ssl: bool) -> Self {
        Self(libparsec::types::BackendAddr::new(hostname, port, use_ssl))
    }

    #[getter]
    fn hostname(&self) -> &str {
        self.0.hostname()
    }

    #[getter]
    fn port(&self) -> u16 {
        self.0.port()
    }

    #[getter]
    fn use_ssl(&self) -> bool {
        self.0.use_ssl()
    }

    #[getter]
    fn netloc(&self) -> String {
        if self.0.is_default_port() {
            String::from(self.0.hostname())
        } else {
            format!("{}:{}", self.0.hostname(), self.0.port())
        }
    }

    fn to_url(&self) -> String {
        self.0.to_url().to_string()
    }

    #[args(path = "\"\"")]
    fn to_http_domain_url(&self, path: &str) -> String {
        self.0.to_http_url_with_path(Some(path)).to_string()
    }

    fn to_http_redirection_url(&self) -> String {
        self.0.to_http_redirection_url().to_string()
    }

    #[classmethod]
    #[args(allow_http_redirection = "false")]
    fn from_url(_cls: &PyType, url: &str, allow_http_redirection: bool) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec::types::BackendAddr::from_any(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
            false => match libparsec::types::BackendAddr::from_str(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct BackendOrganizationAddr(pub libparsec::types::BackendOrganizationAddr);

crate::binding_utils::gen_proto!(BackendOrganizationAddr, __repr__);
crate::binding_utils::gen_proto!(BackendOrganizationAddr, __copy__);
crate::binding_utils::gen_proto!(BackendOrganizationAddr, __deepcopy__);
crate::binding_utils::gen_proto!(BackendOrganizationAddr, __richcmp__, eq);

#[pymethods]
impl BackendOrganizationAddr {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(
        organization_id: OrganizationID,
        root_verify_key: VerifyKey,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Self> {
        let addr = match py_kwargs {
            Some(dict) => BackendAddr::new(
                match dict.get_item("hostname") {
                    Some(hostname) => hostname.to_string(),
                    None => String::from(""),
                },
                match dict.get_item("port") {
                    Some(port) => port.extract::<u16>().ok(),
                    None => None,
                },
                match dict.get_item("use_ssl") {
                    Some(use_ssl) => use_ssl.extract::<bool>().unwrap(),
                    None => true,
                },
            ),
            None => return Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(libparsec::types::BackendOrganizationAddr::new(
            addr.0,
            organization_id.0,
            root_verify_key.0,
        )))
    }

    fn get_backend_addr(&self) -> BackendAddr {
        BackendAddr::new(
            String::from(self.0.hostname()),
            if !self.0.is_default_port() {
                Some(self.0.port())
            } else {
                None
            },
            self.0.use_ssl(),
        )
    }

    #[getter]
    fn hostname(&self) -> &str {
        self.0.hostname()
    }

    #[getter]
    fn port(&self) -> u16 {
        self.0.port()
    }

    #[getter]
    fn use_ssl(&self) -> bool {
        self.0.use_ssl()
    }

    #[getter]
    fn netloc(&self) -> String {
        if self.0.is_default_port() {
            String::from(self.0.hostname())
        } else {
            format!("{}:{}", self.0.hostname(), self.0.port())
        }
    }

    #[getter]
    fn organization_id(&self) -> OrganizationID {
        OrganizationID(self.0.organization_id().clone())
    }

    #[getter]
    fn root_verify_key(&self) -> VerifyKey {
        VerifyKey(self.0.root_verify_key().clone())
    }

    fn to_url(&self) -> String {
        self.0.to_url().to_string()
    }

    fn to_http_redirection_url(&self) -> String {
        self.0.to_http_redirection_url().to_string()
    }

    #[classmethod]
    #[args(allow_http_redirection = "false")]
    fn from_url(_cls: &PyType, url: &str, allow_http_redirection: bool) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec::types::BackendOrganizationAddr::from_any(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
            false => match libparsec::types::BackendOrganizationAddr::from_str(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
        }
    }

    #[classmethod]
    fn build(
        _cls: &PyType,
        backend_addr: BackendAddr,
        organization_id: OrganizationID,
        root_verify_key: VerifyKey,
    ) -> Self {
        Self(libparsec::types::BackendOrganizationAddr::new(
            backend_addr.0,
            organization_id.0,
            root_verify_key.0,
        ))
    }
}

#[pyclass]
pub(crate) struct BackendActionAddr;

#[pymethods]
impl BackendActionAddr {
    #[classmethod]
    #[args(allow_http_redirection = "false")]
    fn from_url(
        _cls: &PyType,
        py: Python,
        url: &str,
        allow_http_redirection: bool,
    ) -> PyResult<PyObject> {
        match allow_http_redirection {
            true => match libparsec::types::BackendActionAddr::from_any(url) {
                Ok(ba) => match ba {
                    libparsec::types::BackendActionAddr::OrganizationBootstrap(v) => {
                        Ok(BackendOrganizationBootstrapAddr(v)
                            .into_py(py)
                            .to_object(py))
                    }
                    libparsec::types::BackendActionAddr::OrganizationFileLink(v) => {
                        Ok(BackendOrganizationFileLinkAddr(v).into_py(py).to_object(py))
                    }
                    libparsec::types::BackendActionAddr::Invitation(v) => {
                        Ok(BackendInvitationAddr(v).into_py(py).to_object(py))
                    }
                    libparsec::types::BackendActionAddr::PkiEnrollment(v) => {
                        Ok(BackendPkiEnrollmentAddr(v).into_py(py).to_object(py))
                    }
                },
                Err(err) => Err(PyValueError::new_err(err)),
            },
            false => match url.parse::<libparsec::types::BackendActionAddr>() {
                Ok(ba) => match ba {
                    libparsec::types::BackendActionAddr::OrganizationBootstrap(v) => {
                        Ok(BackendOrganizationBootstrapAddr(v)
                            .into_py(py)
                            .to_object(py))
                    }
                    libparsec::types::BackendActionAddr::OrganizationFileLink(v) => {
                        Ok(BackendOrganizationFileLinkAddr(v).into_py(py).to_object(py))
                    }
                    libparsec::types::BackendActionAddr::Invitation(v) => {
                        Ok(BackendInvitationAddr(v).into_py(py).to_object(py))
                    }
                    libparsec::types::BackendActionAddr::PkiEnrollment(v) => {
                        Ok(BackendPkiEnrollmentAddr(v).into_py(py).to_object(py))
                    }
                },
                Err(err) => Err(PyValueError::new_err(err)),
            },
        }
    }
}

#[pyclass]
pub(crate) struct BackendOrganizationBootstrapAddr(
    libparsec::types::BackendOrganizationBootstrapAddr,
);

crate::binding_utils::gen_proto!(BackendOrganizationBootstrapAddr, __repr__);
crate::binding_utils::gen_proto!(BackendOrganizationBootstrapAddr, __copy__);
crate::binding_utils::gen_proto!(BackendOrganizationBootstrapAddr, __deepcopy__);
crate::binding_utils::gen_proto!(BackendOrganizationBootstrapAddr, __richcmp__, eq);
crate::binding_utils::gen_proto!(BackendOrganizationBootstrapAddr, __hash__);

#[pymethods]
impl BackendOrganizationBootstrapAddr {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(
        organization_id: OrganizationID,
        token: Option<String>,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Self> {
        let addr = match py_kwargs {
            Some(dict) => BackendAddr::new(
                match dict.get_item("hostname") {
                    Some(hostname) => hostname.to_string(),
                    None => String::from(""),
                },
                match dict.get_item("port") {
                    Some(port) => port.extract::<u16>().ok(),
                    None => None,
                },
                match dict.get_item("use_ssl") {
                    Some(use_ssl) => use_ssl.extract::<bool>().unwrap(),
                    None => true,
                },
            ),
            None => return Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(
            libparsec::types::BackendOrganizationBootstrapAddr::new(
                addr.0,
                organization_id.0,
                token,
            ),
        ))
    }

    #[getter]
    fn hostname(&self) -> &str {
        self.0.hostname()
    }

    #[getter]
    fn port(&self) -> u16 {
        self.0.port()
    }

    #[getter]
    fn use_ssl(&self) -> bool {
        self.0.use_ssl()
    }

    #[getter]
    fn netloc(&self) -> String {
        if self.0.is_default_port() {
            String::from(self.0.hostname())
        } else {
            format!("{}:{}", self.0.hostname(), self.0.port())
        }
    }

    #[getter]
    fn organization_id(&self) -> OrganizationID {
        OrganizationID(self.0.organization_id().clone())
    }

    #[getter]
    fn token(&self) -> &str {
        match self.0.token() {
            Some(token) => token,
            None => "",
        }
    }

    fn to_url(&self) -> String {
        self.0.to_url().to_string()
    }

    fn get_backend_addr(&self) -> BackendAddr {
        BackendAddr::new(
            String::from(self.0.hostname()),
            if !self.0.is_default_port() {
                Some(self.0.port())
            } else {
                None
            },
            self.0.use_ssl(),
        )
    }

    fn generate_organization_addr(&self, root_verify_key: VerifyKey) -> BackendOrganizationAddr {
        BackendOrganizationAddr(self.0.generate_organization_addr(root_verify_key.0))
    }

    fn to_http_redirection_url(&self) -> String {
        self.0.to_http_redirection_url().to_string()
    }

    #[args(path = "\"\"")]
    fn to_http_domain_url(&self, path: &str) -> String {
        self.0.to_http_url_with_path(Some(path)).to_string()
    }

    #[classmethod]
    #[args(allow_http_redirection = "false")]
    fn from_url(_cls: &PyType, url: &str, allow_http_redirection: bool) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec::types::BackendOrganizationBootstrapAddr::from_any(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
            false => match libparsec::types::BackendOrganizationBootstrapAddr::from_str(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
        }
    }

    #[classmethod]
    fn build(
        _cls: &PyType,
        backend_addr: BackendAddr,
        organization_id: OrganizationID,
        token: Option<String>,
    ) -> Self {
        Self(libparsec::types::BackendOrganizationBootstrapAddr::new(
            backend_addr.0,
            organization_id.0,
            token,
        ))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct BackendOrganizationFileLinkAddr(
    libparsec::types::BackendOrganizationFileLinkAddr,
);

crate::binding_utils::gen_proto!(BackendOrganizationFileLinkAddr, __repr__);
crate::binding_utils::gen_proto!(BackendOrganizationFileLinkAddr, __copy__);
crate::binding_utils::gen_proto!(BackendOrganizationFileLinkAddr, __deepcopy__);
crate::binding_utils::gen_proto!(BackendOrganizationFileLinkAddr, __richcmp__, eq);
crate::binding_utils::gen_proto!(BackendOrganizationFileLinkAddr, __hash__);

#[pymethods]
impl BackendOrganizationFileLinkAddr {
    #[new]
    #[args(encrypted_timestamp = "None", py_kwargs = "**")]
    fn new(
        organization_id: OrganizationID,
        workspace_id: EntryID,
        encrypted_path: BytesWrapper,
        encrypted_timestamp: Option<BytesWrapper>,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Self> {
        crate::binding_utils::unwrap_bytes!(encrypted_path, encrypted_timestamp);
        let addr = match py_kwargs {
            Some(dict) => BackendAddr::new(
                match dict.get_item("hostname") {
                    Some(hostname) => hostname.to_string(),
                    None => String::from(""),
                },
                match dict.get_item("port") {
                    Some(port) => port.extract::<u16>().ok(),
                    None => None,
                },
                match dict.get_item("use_ssl") {
                    Some(use_ssl) => use_ssl.extract::<bool>().unwrap(),
                    None => true,
                },
            ),
            None => return Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(
            libparsec::types::BackendOrganizationFileLinkAddr::new(
                addr.0,
                organization_id.0,
                workspace_id.0,
                encrypted_path,
                encrypted_timestamp,
            ),
        ))
    }

    #[getter]
    fn encrypted_timestamp<'py>(&self, python: Python<'py>) -> Option<&'py PyBytes> {
        self.0
            .encrypted_timestamp()
            .as_ref()
            .map(|v| PyBytes::new(python, v))
    }

    #[getter]
    fn hostname(&self) -> &str {
        self.0.hostname()
    }

    #[getter]
    fn port(&self) -> u16 {
        self.0.port()
    }

    #[getter]
    fn use_ssl(&self) -> bool {
        self.0.use_ssl()
    }

    #[getter]
    fn netloc(&self) -> String {
        if self.0.is_default_port() {
            String::from(self.0.hostname())
        } else {
            format!("{}:{}", self.0.hostname(), self.0.port())
        }
    }

    #[getter]
    fn organization_id(&self) -> OrganizationID {
        OrganizationID(self.0.organization_id().clone())
    }

    #[getter]
    fn workspace_id(&self) -> EntryID {
        EntryID(self.0.workspace_id())
    }

    #[getter]
    fn encrypted_path<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, self.0.encrypted_path().as_slice())
    }

    fn get_backend_addr(&self) -> BackendAddr {
        BackendAddr::new(
            String::from(self.0.hostname()),
            if !self.0.is_default_port() {
                Some(self.0.port())
            } else {
                None
            },
            self.0.use_ssl(),
        )
    }

    fn to_url(&self) -> String {
        self.0.to_url().to_string()
    }

    fn to_http_redirection_url(&self) -> String {
        self.0.to_http_redirection_url().to_string()
    }

    #[classmethod]
    #[args(allow_http_redirection = "false")]
    fn from_url(_cls: &PyType, url: &str, allow_http_redirection: bool) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec::types::BackendOrganizationFileLinkAddr::from_any(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
            false => match libparsec::types::BackendOrganizationFileLinkAddr::from_str(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
        }
    }

    #[classmethod]
    #[args(encrypted_timestamp = "None")]
    fn build(
        _cls: &PyType,
        organization_addr: BackendOrganizationAddr,
        workspace_id: EntryID,
        encrypted_path: BytesWrapper,
        encrypted_timestamp: Option<BytesWrapper>,
    ) -> Self {
        crate::binding_utils::unwrap_bytes!(encrypted_path, encrypted_timestamp);
        Self(libparsec::types::BackendOrganizationFileLinkAddr::new(
            organization_addr.get_backend_addr().0,
            organization_addr.organization_id().0,
            workspace_id.0,
            encrypted_path,
            encrypted_timestamp,
        ))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct BackendInvitationAddr(libparsec::types::BackendInvitationAddr);

crate::binding_utils::gen_proto!(BackendInvitationAddr, __repr__);
crate::binding_utils::gen_proto!(BackendInvitationAddr, __copy__);
crate::binding_utils::gen_proto!(BackendInvitationAddr, __deepcopy__);
crate::binding_utils::gen_proto!(BackendInvitationAddr, __richcmp__, eq);
crate::binding_utils::gen_proto!(BackendInvitationAddr, __hash__);

#[pymethods]
impl BackendInvitationAddr {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(
        organization_id: OrganizationID,
        invitation_type: &InvitationType,
        token: InvitationToken,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Self> {
        let addr = match py_kwargs {
            Some(dict) => BackendAddr::new(
                match dict.get_item("hostname") {
                    Some(hostname) => hostname.to_string(),
                    None => String::from(""),
                },
                match dict.get_item("port") {
                    Some(port) => port.extract::<u16>().ok(),
                    None => None,
                },
                match dict.get_item("use_ssl") {
                    Some(use_ssl) => use_ssl.extract::<bool>().unwrap(),
                    None => true,
                },
            ),
            None => return Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(libparsec::types::BackendInvitationAddr::new(
            addr.0,
            organization_id.0,
            invitation_type.0,
            token.0,
        )))
    }

    #[getter]
    fn hostname(&self) -> &str {
        self.0.hostname()
    }

    #[getter]
    fn port(&self) -> u16 {
        self.0.port()
    }

    #[getter]
    fn use_ssl(&self) -> bool {
        self.0.use_ssl()
    }

    #[getter]
    fn netloc(&self) -> String {
        if self.0.is_default_port() {
            String::from(self.0.hostname())
        } else {
            format!("{}:{}", self.0.hostname(), self.0.port())
        }
    }

    #[getter]
    fn organization_id(&self) -> OrganizationID {
        OrganizationID(self.0.organization_id().clone())
    }

    #[getter]
    fn invitation_type(&self) -> InvitationType {
        InvitationType(self.0.invitation_type())
    }

    #[getter]
    fn token(&self) -> InvitationToken {
        InvitationToken(self.0.token())
    }

    fn to_url(&self) -> String {
        self.0.to_url().to_string()
    }

    fn to_http_redirection_url(&self) -> String {
        self.0.to_http_redirection_url().to_string()
    }

    fn get_backend_addr(&self) -> BackendAddr {
        BackendAddr::new(
            String::from(self.0.hostname()),
            if !self.0.is_default_port() {
                Some(self.0.port())
            } else {
                None
            },
            self.0.use_ssl(),
        )
    }

    fn generate_organization_addr(&self, root_verify_key: VerifyKey) -> BackendOrganizationAddr {
        BackendOrganizationAddr(self.0.generate_organization_addr(root_verify_key.0))
    }

    #[classmethod]
    #[args(allow_http_redirection = "false")]
    fn from_url(_cls: &PyType, url: &str, allow_http_redirection: bool) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec::types::BackendInvitationAddr::from_any(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
            false => match libparsec::types::BackendInvitationAddr::from_str(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
        }
    }

    #[classmethod]
    fn build(
        _cls: &PyType,
        backend_addr: BackendAddr,
        organization_id: OrganizationID,
        invitation_type: InvitationType,
        token: InvitationToken,
    ) -> Self {
        Self(libparsec::types::BackendInvitationAddr::new(
            backend_addr.0,
            organization_id.0,
            invitation_type.0,
            token.0,
        ))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct BackendPkiEnrollmentAddr(pub libparsec::types::BackendPkiEnrollmentAddr);

crate::binding_utils::gen_proto!(BackendPkiEnrollmentAddr, __repr__);
crate::binding_utils::gen_proto!(BackendPkiEnrollmentAddr, __copy__);
crate::binding_utils::gen_proto!(BackendPkiEnrollmentAddr, __deepcopy__);
crate::binding_utils::gen_proto!(BackendPkiEnrollmentAddr, __richcmp__, eq);
crate::binding_utils::gen_proto!(BackendPkiEnrollmentAddr, __hash__);

#[pymethods]
impl BackendPkiEnrollmentAddr {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(organization_id: OrganizationID, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let addr = match py_kwargs {
            Some(dict) => BackendAddr::new(
                match dict.get_item("hostname") {
                    Some(hostname) => hostname.to_string(),
                    None => String::from(""),
                },
                match dict.get_item("port") {
                    Some(port) => port.extract::<u16>().ok(),
                    None => None,
                },
                match dict.get_item("use_ssl") {
                    Some(use_ssl) => use_ssl.extract::<bool>().unwrap(),
                    None => true,
                },
            ),
            None => return Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(libparsec::types::BackendPkiEnrollmentAddr::new(
            addr.0,
            organization_id.0,
        )))
    }

    #[getter]
    fn hostname(&self) -> &str {
        self.0.hostname()
    }

    #[getter]
    fn port(&self) -> u16 {
        self.0.port()
    }

    #[getter]
    fn use_ssl(&self) -> bool {
        self.0.use_ssl()
    }

    #[getter]
    fn netloc(&self) -> String {
        if self.0.is_default_port() {
            String::from(self.0.hostname())
        } else {
            format!("{}:{}", self.0.hostname(), self.0.port())
        }
    }

    #[getter]
    fn organization_id(&self) -> OrganizationID {
        OrganizationID(self.0.organization_id().clone())
    }

    fn to_url(&self) -> String {
        self.0.to_url().to_string()
    }

    fn get_backend_addr(&self) -> BackendAddr {
        BackendAddr::new(
            String::from(self.0.hostname()),
            if !self.0.is_default_port() {
                Some(self.0.port())
            } else {
                None
            },
            self.0.use_ssl(),
        )
    }

    fn generate_organization_addr(&self, root_verify_key: VerifyKey) -> BackendOrganizationAddr {
        BackendOrganizationAddr(self.0.generate_organization_addr(root_verify_key.0))
    }

    fn to_http_redirection_url(&self) -> String {
        self.0.to_http_redirection_url().to_string()
    }

    #[args(path = "\"\"")]
    fn to_http_domain_url(&self, path: &str) -> String {
        self.0.to_http_url_with_path(Some(path)).to_string()
    }

    #[classmethod]
    #[args(allow_http_redirection = "false")]
    fn from_url(_cls: &PyType, url: &str, allow_http_redirection: bool) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec::types::BackendPkiEnrollmentAddr::from_any(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
            false => match libparsec::types::BackendPkiEnrollmentAddr::from_str(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
        }
    }

    #[classmethod]
    fn build(_cls: &PyType, backend_addr: BackendAddr, organization_id: OrganizationID) -> Self {
        Self(libparsec::types::BackendPkiEnrollmentAddr::new(
            backend_addr.0,
            organization_id.0,
        ))
    }
}

#[pyfunction]
pub(crate) fn export_root_verify_key(key: &VerifyKey) -> String {
    libparsec::types::export_root_verify_key(&key.0)
}
