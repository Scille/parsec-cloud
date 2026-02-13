// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    prelude::{pyclass, pymethods, IntoPyObject, PyObject, PyResult, Python},
    types::{PyAnyMethods, PyBytes, PyDict, PyDictMethods, PyType},
    Bound, BoundObject, PyAny,
};
use std::str::FromStr;

use crate::{AccessToken, BytesWrapper, InvitationType, OrganizationID, UserID, VerifyKey, VlobID};

crate::binding_utils::gen_py_wrapper_class!(
    ParsecAddr,
    libparsec_types::ParsecAddr,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
    __hash__,
);

#[pymethods]
impl ParsecAddr {
    #[new]
    #[pyo3(signature = (hostname, port, use_ssl))]
    fn new(hostname: String, port: Option<u16>, use_ssl: bool) -> Self {
        Self(libparsec_types::ParsecAddr::new(hostname, port, use_ssl))
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

    #[pyo3(signature = (path = ""))]
    fn to_http_url(&self, path: &str) -> String {
        self.0.to_http_url(Some(path)).to_string()
    }

    fn to_http_redirection_url(&self) -> String {
        self.0.to_http_redirection_url().to_string()
    }

    #[classmethod]
    #[pyo3(signature = (url, allow_http_redirection = false))]
    fn from_url(
        _cls: Bound<'_, PyType>,
        url: &str,
        allow_http_redirection: bool,
    ) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec_types::ParsecAddr::from_any(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
            false => match libparsec_types::ParsecAddr::from_str(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
        }
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    ParsecOrganizationAddr,
    libparsec_types::ParsecOrganizationAddr,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
    __hash__,
);

#[pymethods]
impl ParsecOrganizationAddr {
    #[new]
    #[pyo3(signature = (organization_id, root_verify_key, **py_kwargs))]
    fn new(
        organization_id: OrganizationID,
        root_verify_key: VerifyKey,
        py_kwargs: Option<Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let addr = match py_kwargs {
            Some(dict) => ParsecAddr::new(
                match dict.get_item("hostname")? {
                    Some(hostname) => hostname.to_string(),
                    None => String::from(""),
                },
                match dict.get_item("port")? {
                    Some(port) => port.extract::<u16>().ok(),
                    None => None,
                },
                match dict.get_item("use_ssl")? {
                    Some(use_ssl) => use_ssl
                        .extract::<bool>()
                        .expect("`use_ssl` should be a boolean value"),
                    None => true,
                },
            ),
            None => return Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(libparsec_types::ParsecOrganizationAddr::new(
            addr.0,
            organization_id.0,
            root_verify_key.0,
        )))
    }

    fn get_server_addr(&self) -> ParsecAddr {
        ParsecAddr::new(
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
    #[pyo3(signature = (url, allow_http_redirection = false))]
    fn from_url(
        _cls: Bound<'_, PyType>,
        url: &str,
        allow_http_redirection: bool,
    ) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec_types::ParsecOrganizationAddr::from_any(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
            false => match libparsec_types::ParsecOrganizationAddr::from_str(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
        }
    }

    #[classmethod]
    fn build(
        _cls: Bound<'_, PyType>,
        server_addr: ParsecAddr,
        organization_id: OrganizationID,
        root_verify_key: VerifyKey,
    ) -> Self {
        Self(libparsec_types::ParsecOrganizationAddr::new(
            server_addr.0,
            organization_id.0,
            root_verify_key.0,
        ))
    }
}

#[pyclass]
pub(crate) struct ParsecActionAddr;

#[pymethods]
impl ParsecActionAddr {
    #[classmethod]
    #[pyo3(signature = (url, allow_http_redirection = false))]
    fn from_url<'py>(
        _cls: Bound<'_, PyType>,
        py: Python<'py>,
        url: &str,
        allow_http_redirection: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        match allow_http_redirection {
            true => match libparsec_types::ParsecActionAddr::from_any(url) {
                Ok(ba) => match ba {
                    libparsec_types::ParsecActionAddr::OrganizationBootstrap(v) => {
                        ParsecOrganizationBootstrapAddr(v)
                            .into_pyobject(py)
                            .map(BoundObject::into_any)
                    }
                    libparsec_types::ParsecActionAddr::WorkspacePath(v) => {
                        ParsecWorkspacePathAddr(v)
                            .into_pyobject(py)
                            .map(BoundObject::into_any)
                    }
                    libparsec_types::ParsecActionAddr::Invitation(v) => ParsecInvitationAddr(v)
                        .into_pyobject(py)
                        .map(BoundObject::into_any),
                    libparsec_types::ParsecActionAddr::PkiEnrollment(v) => {
                        ParsecPkiEnrollmentAddr(v)
                            .into_pyobject(py)
                            .map(BoundObject::into_any)
                    }
                    libparsec_types::ParsecActionAddr::AsyncEnrollment(v) => {
                        ParsecAsyncEnrollmentAddr(v)
                            .into_pyobject(py)
                            .map(BoundObject::into_any)
                    }
                    libparsec_types::ParsecActionAddr::TOTPReset(v) => ParsecTOTPResetAddr(v)
                        .into_pyobject(py)
                        .map(BoundObject::into_any),
                },
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
            false => match url.parse::<libparsec_types::ParsecActionAddr>() {
                Ok(ba) => match ba {
                    libparsec_types::ParsecActionAddr::OrganizationBootstrap(v) => {
                        ParsecOrganizationBootstrapAddr(v)
                            .into_pyobject(py)
                            .map(BoundObject::into_any)
                    }
                    libparsec_types::ParsecActionAddr::WorkspacePath(v) => {
                        ParsecWorkspacePathAddr(v)
                            .into_pyobject(py)
                            .map(BoundObject::into_any)
                    }
                    libparsec_types::ParsecActionAddr::Invitation(v) => ParsecInvitationAddr(v)
                        .into_pyobject(py)
                        .map(BoundObject::into_any),
                    libparsec_types::ParsecActionAddr::PkiEnrollment(v) => {
                        ParsecPkiEnrollmentAddr(v)
                            .into_pyobject(py)
                            .map(BoundObject::into_any)
                    }
                    libparsec_types::ParsecActionAddr::AsyncEnrollment(v) => {
                        ParsecAsyncEnrollmentAddr(v)
                            .into_pyobject(py)
                            .map(BoundObject::into_any)
                    }
                    libparsec_types::ParsecActionAddr::TOTPReset(v) => ParsecTOTPResetAddr(v)
                        .into_pyobject(py)
                        .map(BoundObject::into_any),
                },
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
        }
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    ParsecOrganizationBootstrapAddr,
    libparsec_types::ParsecOrganizationBootstrapAddr,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
    __hash__,
);

#[pymethods]
impl ParsecOrganizationBootstrapAddr {
    #[new]
    #[pyo3(signature = (organization_id, token, **py_kwargs))]
    fn new(
        organization_id: OrganizationID,
        token: Option<AccessToken>,
        py_kwargs: Option<Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let addr = match py_kwargs {
            Some(dict) => ParsecAddr::new(
                match dict.get_item("hostname")? {
                    Some(hostname) => hostname.to_string(),
                    None => String::from(""),
                },
                match dict.get_item("port")? {
                    Some(port) => port.extract::<u16>().ok(),
                    None => None,
                },
                match dict.get_item("use_ssl")? {
                    Some(use_ssl) => use_ssl
                        .extract::<bool>()
                        .expect("`use_ssl` should be a boolean value"),
                    None => true,
                },
            ),
            None => return Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(libparsec_types::ParsecOrganizationBootstrapAddr::new(
            addr.0,
            organization_id.0,
            token.map(|t| t.0),
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
    fn token(&self) -> Option<AccessToken> {
        self.0.token().map(|token| (*token).into())
    }

    fn to_url(&self) -> String {
        self.0.to_url().to_string()
    }

    fn get_server_addr(&self) -> ParsecAddr {
        ParsecAddr::new(
            String::from(self.0.hostname()),
            if !self.0.is_default_port() {
                Some(self.0.port())
            } else {
                None
            },
            self.0.use_ssl(),
        )
    }

    fn generate_organization_addr(&self, root_verify_key: VerifyKey) -> ParsecOrganizationAddr {
        ParsecOrganizationAddr(self.0.generate_organization_addr(root_verify_key.0))
    }

    fn to_http_redirection_url(&self) -> String {
        self.0.to_http_redirection_url().to_string()
    }

    #[classmethod]
    #[pyo3(signature = (url, allow_http_redirection = false))]
    fn from_url(
        _cls: Bound<'_, PyType>,
        url: &str,
        allow_http_redirection: bool,
    ) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec_types::ParsecOrganizationBootstrapAddr::from_any(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
            false => match libparsec_types::ParsecOrganizationBootstrapAddr::from_str(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
        }
    }

    #[classmethod]
    #[pyo3(signature = (server_addr, organization_id, token=None))]
    fn build(
        _cls: Bound<'_, PyType>,
        server_addr: ParsecAddr,
        organization_id: OrganizationID,
        token: Option<AccessToken>,
    ) -> Self {
        Self(libparsec_types::ParsecOrganizationBootstrapAddr::new(
            server_addr.0,
            organization_id.0,
            token.map(|t| t.0),
        ))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    ParsecWorkspacePathAddr,
    libparsec_types::ParsecWorkspacePathAddr,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
    __hash__,
);

#[pymethods]
impl ParsecWorkspacePathAddr {
    #[new]
    #[pyo3(signature = (organization_id, workspace_id, key_index, encrypted_path, **py_kwargs))]
    fn new(
        organization_id: OrganizationID,
        workspace_id: VlobID,
        key_index: libparsec_types::IndexInt,
        encrypted_path: BytesWrapper,
        py_kwargs: Option<Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        crate::binding_utils::unwrap_bytes!(encrypted_path);
        let addr = match py_kwargs {
            Some(dict) => ParsecAddr::new(
                match dict.get_item("hostname")? {
                    Some(hostname) => hostname.to_string(),
                    None => String::from(""),
                },
                match dict.get_item("port")? {
                    Some(port) => port.extract::<u16>().ok(),
                    None => None,
                },
                match dict.get_item("use_ssl")? {
                    Some(use_ssl) => use_ssl
                        .extract::<bool>()
                        .expect("`use_ssl` should be a boolean value"),
                    None => true,
                },
            ),
            None => return Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(libparsec_types::ParsecWorkspacePathAddr::new(
            addr.0,
            organization_id.0,
            workspace_id.0,
            key_index,
            encrypted_path.into(),
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
    fn workspace_id(&self) -> VlobID {
        VlobID(self.0.workspace_id())
    }

    #[getter]
    fn key_index(&self) -> libparsec_types::IndexInt {
        self.0.key_index()
    }

    #[getter]
    fn encrypted_path<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, self.0.encrypted_path().as_slice())
    }

    fn get_server_addr(&self) -> ParsecAddr {
        ParsecAddr::new(
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
    #[pyo3(signature = (url, allow_http_redirection = false))]
    fn from_url(
        _cls: Bound<'_, PyType>,
        url: &str,
        allow_http_redirection: bool,
    ) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec_types::ParsecWorkspacePathAddr::from_any(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
            false => match libparsec_types::ParsecWorkspacePathAddr::from_str(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
        }
    }

    #[classmethod]
    #[pyo3(signature = (organization_addr, workspace_id, key_index, encrypted_path))]
    fn build(
        _cls: Bound<'_, PyType>,
        organization_addr: ParsecOrganizationAddr,
        workspace_id: VlobID,
        key_index: libparsec_types::IndexInt,
        encrypted_path: BytesWrapper,
    ) -> Self {
        crate::binding_utils::unwrap_bytes!(encrypted_path);
        Self(libparsec_types::ParsecWorkspacePathAddr::new(
            organization_addr.get_server_addr().0,
            organization_addr.organization_id().0,
            workspace_id.0,
            key_index,
            encrypted_path.to_vec(),
        ))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    ParsecInvitationAddr,
    libparsec_types::ParsecInvitationAddr,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
    __hash__,
);

#[pymethods]
impl ParsecInvitationAddr {
    #[new]
    #[pyo3(signature = (organization_id, invitation_type, token, **py_kwargs))]
    fn new(
        organization_id: OrganizationID,
        invitation_type: &InvitationType,
        token: AccessToken,
        py_kwargs: Option<Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let addr = match py_kwargs {
            Some(dict) => ParsecAddr::new(
                match dict.get_item("hostname")? {
                    Some(hostname) => hostname.to_string(),
                    None => String::from(""),
                },
                match dict.get_item("port")? {
                    Some(port) => port.extract::<u16>().ok(),
                    None => None,
                },
                match dict.get_item("use_ssl")? {
                    Some(use_ssl) => use_ssl
                        .extract::<bool>()
                        .expect("`use_ssl` should be a boolean value"),
                    None => true,
                },
            ),
            None => return Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(libparsec_types::ParsecInvitationAddr::new(
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
    fn invitation_type(&self) -> &'static PyObject {
        InvitationType::convert(self.0.invitation_type())
    }

    #[getter]
    fn token(&self) -> AccessToken {
        AccessToken(self.0.token())
    }

    fn to_url(&self) -> String {
        self.0.to_url().to_string()
    }

    fn to_http_redirection_url(&self) -> String {
        self.0.to_http_redirection_url().to_string()
    }

    fn get_server_addr(&self) -> ParsecAddr {
        ParsecAddr::new(
            String::from(self.0.hostname()),
            if !self.0.is_default_port() {
                Some(self.0.port())
            } else {
                None
            },
            self.0.use_ssl(),
        )
    }

    fn generate_organization_addr(&self, root_verify_key: VerifyKey) -> ParsecOrganizationAddr {
        ParsecOrganizationAddr(self.0.generate_organization_addr(root_verify_key.0))
    }

    #[classmethod]
    #[pyo3(signature = (url, allow_http_redirection = false))]
    fn from_url(
        _cls: Bound<'_, PyType>,
        url: &str,
        allow_http_redirection: bool,
    ) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec_types::ParsecInvitationAddr::from_any(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
            false => match libparsec_types::ParsecInvitationAddr::from_str(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
        }
    }

    #[classmethod]
    fn build(
        _cls: Bound<'_, PyType>,
        server_addr: ParsecAddr,
        organization_id: OrganizationID,
        invitation_type: InvitationType,
        token: AccessToken,
    ) -> Self {
        Self(libparsec_types::ParsecInvitationAddr::new(
            server_addr.0,
            organization_id.0,
            invitation_type.0,
            token.0,
        ))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    ParsecPkiEnrollmentAddr,
    libparsec_types::ParsecPkiEnrollmentAddr,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
    __hash__,
);

#[pymethods]
impl ParsecPkiEnrollmentAddr {
    #[new]
    #[pyo3(signature = (organization_id, **py_kwargs))]
    fn new(
        organization_id: OrganizationID,
        py_kwargs: Option<Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let addr = match py_kwargs {
            Some(dict) => ParsecAddr::new(
                match dict.get_item("hostname")? {
                    Some(hostname) => hostname.to_string(),
                    None => String::from(""),
                },
                match dict.get_item("port")? {
                    Some(port) => port.extract::<u16>().ok(),
                    None => None,
                },
                match dict.get_item("use_ssl")? {
                    Some(use_ssl) => use_ssl
                        .extract::<bool>()
                        .expect("`use_ssl` should be a boolean value"),
                    None => true,
                },
            ),
            None => return Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(libparsec_types::ParsecPkiEnrollmentAddr::new(
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

    fn get_server_addr(&self) -> ParsecAddr {
        ParsecAddr::new(
            String::from(self.0.hostname()),
            if !self.0.is_default_port() {
                Some(self.0.port())
            } else {
                None
            },
            self.0.use_ssl(),
        )
    }

    fn generate_organization_addr(&self, root_verify_key: VerifyKey) -> ParsecOrganizationAddr {
        ParsecOrganizationAddr(self.0.generate_organization_addr(root_verify_key.0))
    }

    fn to_http_redirection_url(&self) -> String {
        self.0.to_http_redirection_url().to_string()
    }

    #[classmethod]
    #[pyo3(signature = (url, allow_http_redirection = false))]
    fn from_url(
        _cls: Bound<'_, PyType>,
        url: &str,
        allow_http_redirection: bool,
    ) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec_types::ParsecPkiEnrollmentAddr::from_any(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
            false => match libparsec_types::ParsecPkiEnrollmentAddr::from_str(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
        }
    }

    #[classmethod]
    fn build(
        _cls: Bound<'_, PyType>,
        server_addr: ParsecAddr,
        organization_id: OrganizationID,
    ) -> Self {
        Self(libparsec_types::ParsecPkiEnrollmentAddr::new(
            server_addr.0,
            organization_id.0,
        ))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    ParsecAsyncEnrollmentAddr,
    libparsec_types::ParsecAsyncEnrollmentAddr,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
    __hash__,
);

#[pymethods]
impl ParsecAsyncEnrollmentAddr {
    #[new]
    #[pyo3(signature = (organization_id, **py_kwargs))]
    fn new(
        organization_id: OrganizationID,
        py_kwargs: Option<Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let addr = match py_kwargs {
            Some(dict) => ParsecAddr::new(
                match dict.get_item("hostname")? {
                    Some(hostname) => hostname.to_string(),
                    None => String::from(""),
                },
                match dict.get_item("port")? {
                    Some(port) => port.extract::<u16>().ok(),
                    None => None,
                },
                match dict.get_item("use_ssl")? {
                    Some(use_ssl) => use_ssl
                        .extract::<bool>()
                        .expect("`use_ssl` should be a boolean value"),
                    None => true,
                },
            ),
            None => return Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(libparsec_types::ParsecAsyncEnrollmentAddr::new(
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

    fn get_server_addr(&self) -> ParsecAddr {
        ParsecAddr::new(
            String::from(self.0.hostname()),
            if !self.0.is_default_port() {
                Some(self.0.port())
            } else {
                None
            },
            self.0.use_ssl(),
        )
    }

    fn generate_organization_addr(&self, root_verify_key: VerifyKey) -> ParsecOrganizationAddr {
        ParsecOrganizationAddr(self.0.generate_organization_addr(root_verify_key.0))
    }

    fn to_http_redirection_url(&self) -> String {
        self.0.to_http_redirection_url().to_string()
    }

    #[classmethod]
    #[pyo3(signature = (url, allow_http_redirection = false))]
    fn from_url(
        _cls: Bound<'_, PyType>,
        url: &str,
        allow_http_redirection: bool,
    ) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec_types::ParsecAsyncEnrollmentAddr::from_any(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
            false => match libparsec_types::ParsecAsyncEnrollmentAddr::from_str(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
        }
    }

    #[classmethod]
    fn build(
        _cls: Bound<'_, PyType>,
        server_addr: ParsecAddr,
        organization_id: OrganizationID,
    ) -> Self {
        Self(libparsec_types::ParsecAsyncEnrollmentAddr::new(
            server_addr.0,
            organization_id.0,
        ))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    ParsecTOTPResetAddr,
    libparsec_types::ParsecTOTPResetAddr,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
    __hash__,
);

#[pymethods]
impl ParsecTOTPResetAddr {
    #[new]
    #[pyo3(signature = (organization_id, user_id, token, **py_kwargs))]
    fn new(
        organization_id: OrganizationID,
        user_id: UserID,
        token: AccessToken,
        py_kwargs: Option<Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let addr = match py_kwargs {
            Some(dict) => ParsecAddr::new(
                match dict.get_item("hostname")? {
                    Some(hostname) => hostname.to_string(),
                    None => String::from(""),
                },
                match dict.get_item("port")? {
                    Some(port) => port.extract::<u16>().ok(),
                    None => None,
                },
                match dict.get_item("use_ssl")? {
                    Some(use_ssl) => use_ssl
                        .extract::<bool>()
                        .expect("`use_ssl` should be a boolean value"),
                    None => true,
                },
            ),
            None => return Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(libparsec_types::ParsecTOTPResetAddr::new(
            addr.0,
            organization_id.0,
            user_id.0,
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
    fn user_id(&self) -> UserID {
        UserID(self.0.user_id())
    }

    #[getter]
    fn token(&self) -> AccessToken {
        AccessToken(self.0.token())
    }

    fn to_url(&self) -> String {
        self.0.to_url().to_string()
    }

    fn to_http_redirection_url(&self) -> String {
        self.0.to_http_redirection_url().to_string()
    }

    fn get_server_addr(&self) -> ParsecAddr {
        ParsecAddr::new(
            String::from(self.0.hostname()),
            if !self.0.is_default_port() {
                Some(self.0.port())
            } else {
                None
            },
            self.0.use_ssl(),
        )
    }

    #[classmethod]
    #[pyo3(signature = (url, allow_http_redirection = false))]
    fn from_url(
        _cls: Bound<'_, PyType>,
        url: &str,
        allow_http_redirection: bool,
    ) -> PyResult<Self> {
        match allow_http_redirection {
            true => match libparsec_types::ParsecTOTPResetAddr::from_any(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
            false => match libparsec_types::ParsecTOTPResetAddr::from_str(url) {
                Ok(addr) => Ok(Self(addr)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            },
        }
    }

    #[classmethod]
    fn build(
        _cls: Bound<'_, PyType>,
        server_addr: ParsecAddr,
        organization_id: OrganizationID,
        user_id: UserID,
        token: AccessToken,
    ) -> Self {
        Self(libparsec_types::ParsecTOTPResetAddr::new(
            server_addr.0,
            organization_id.0,
            user_id.0,
            token.0,
        ))
    }
}
