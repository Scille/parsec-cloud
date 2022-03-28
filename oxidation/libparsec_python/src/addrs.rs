// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyString, PyType};
use std::str::FromStr;

use crate::binding_utils::{comp_op, hash_generic};
use crate::crypto::VerifyKey;
use crate::ids::{EntryID, OrganizationID};
use crate::invite::InvitationToken;

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct BackendAddr(parsec_api_types::BackendAddr);

#[pymethods]
impl BackendAddr {
    #[new]
    fn new(hostname: String, port: Option<u16>, use_ssl: bool) -> PyResult<Self> {
        Ok(Self(parsec_api_types::BackendAddr::new(
            hostname, port, use_ssl,
        )))
    }

    #[getter]
    fn hostname(&self) -> PyResult<&str> {
        Ok(self.0.hostname())
    }

    #[getter]
    fn port(&self) -> PyResult<u16> {
        Ok(self.0.port())
    }

    #[getter]
    fn use_ssl(&self) -> PyResult<bool> {
        Ok(self.0.use_ssl())
    }

    #[getter]
    fn netloc(&self) -> PyResult<String> {
        if self.0.is_default_port() {
            Ok(String::from(self.0.hostname()))
        } else {
            Ok(format!("{}:{}", self.0.hostname(), self.0.port()))
        }
    }

    fn __str__(&self) -> PyResult<String> {
        self.to_url()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("BackendAddr(url={})", self.to_url().unwrap()))
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.to_url().unwrap(), py)
    }

    fn __richcmp__(&self, py: Python, other: &BackendAddr, op: CompareOp) -> PyResult<bool> {
        let h1 = self.__hash__(py).unwrap();
        let h2 = other.__hash__(py).unwrap();
        comp_op(op, h1, h2)
    }

    fn to_url(&self) -> PyResult<String> {
        Ok(self.0.to_url().to_string())
    }

    #[args(path = "\"\"")]
    fn to_http_domain_url(&self, path: &str) -> PyResult<String> {
        Ok(self.0.to_http_domain_url(Some(path)).to_string())
    }

    fn to_http_redirection_url(&self) -> PyResult<String> {
        Ok(self.0.to_http_redirection_url().to_string())
    }

    #[classmethod]
    #[args(allow_http_redirection = "false")]
    fn from_url(_cls: &PyType, url: &str, allow_http_redirection: bool) -> PyResult<Self> {
        match allow_http_redirection {
            true => match parsec_api_types::BackendAddr::from_any(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
            false => match parsec_api_types::BackendAddr::from_str(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
        }
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct BackendOrganizationAddr(parsec_api_types::BackendOrganizationAddr);

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
            None => Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(parsec_api_types::BackendOrganizationAddr::new(
            addr.unwrap().0,
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
        .unwrap()
    }

    #[getter]
    fn hostname(&self) -> PyResult<&str> {
        Ok(self.0.hostname())
    }

    #[getter]
    fn port(&self) -> PyResult<u16> {
        Ok(self.0.port())
    }

    #[getter]
    fn use_ssl(&self) -> PyResult<bool> {
        Ok(self.0.use_ssl())
    }

    #[getter]
    fn netloc(&self) -> PyResult<String> {
        if self.0.is_default_port() {
            Ok(String::from(self.0.hostname()))
        } else {
            Ok(format!("{}:{}", self.0.hostname(), self.0.port()))
        }
    }

    #[getter]
    fn organization_id(&self) -> PyResult<OrganizationID> {
        Ok(OrganizationID::new(self.0.organization_id().as_ref()).unwrap())
    }

    #[getter]
    fn root_verify_key(&self) -> PyResult<VerifyKey> {
        let data = self.0.root_verify_key().as_ref();
        Ok(VerifyKey::new(data).unwrap())
    }

    fn __str__(&self) -> PyResult<String> {
        self.to_url()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "BackendOrganizationAddr(url={})",
            self.to_url().unwrap()
        ))
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.to_url().unwrap(), py)
    }

    fn __richcmp__(
        &self,
        py: Python,
        other: &BackendOrganizationAddr,
        op: CompareOp,
    ) -> PyResult<bool> {
        let h1 = self.__hash__(py).unwrap();
        let h2 = other.__hash__(py).unwrap();
        comp_op(op, h1, h2)
    }

    fn to_url(&self) -> PyResult<String> {
        Ok(self.0.to_url().to_string())
    }

    fn to_http_redirection_url(&self) -> PyResult<String> {
        Ok(self.0.to_http_redirection_url().to_string())
    }

    #[classmethod]
    #[args(allow_http_redirection = "false")]
    fn from_url(_cls: &PyType, url: &str, allow_http_redirection: bool) -> PyResult<Self> {
        match allow_http_redirection {
            true => match parsec_api_types::BackendOrganizationAddr::from_any(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
            false => match parsec_api_types::BackendOrganizationAddr::from_str(url) {
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
    ) -> PyResult<Self> {
        Ok(Self(parsec_api_types::BackendOrganizationAddr::new(
            backend_addr.0,
            organization_id.0,
            root_verify_key.0,
        )))
    }
}

#[pyclass]
#[derive(PartialEq, Eq)]
pub(crate) struct BackendActionAddr();

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
            true => match parsec_api_types::BackendActionAddr::from_any(url) {
                Ok(ba) => match ba {
                    parsec_api_types::BackendActionAddr::OrganizationBootstrap(v) => {
                        Ok(BackendOrganizationBootstrapAddr(v)
                            .into_py(py)
                            .to_object(py))
                    }
                    parsec_api_types::BackendActionAddr::OrganizationFileLink(v) => {
                        Ok(BackendOrganizationFileLinkAddr(v).into_py(py).to_object(py))
                    }
                    parsec_api_types::BackendActionAddr::Invitation(v) => {
                        Ok(BackendInvitationAddr(v).into_py(py).to_object(py))
                    }
                },
                Err(err) => Err(PyValueError::new_err(err)),
            },
            false => match url.parse::<parsec_api_types::BackendActionAddr>() {
                Ok(ba) => match ba {
                    parsec_api_types::BackendActionAddr::OrganizationBootstrap(v) => {
                        Ok(BackendOrganizationBootstrapAddr(v)
                            .into_py(py)
                            .to_object(py))
                    }
                    parsec_api_types::BackendActionAddr::OrganizationFileLink(v) => {
                        Ok(BackendOrganizationFileLinkAddr(v).into_py(py).to_object(py))
                    }
                    parsec_api_types::BackendActionAddr::Invitation(v) => {
                        Ok(BackendInvitationAddr(v).into_py(py).to_object(py))
                    }
                },
                Err(err) => Err(PyValueError::new_err(err)),
            },
        }
    }
}

#[pyclass]
#[derive(PartialEq, Eq)]
pub struct BackendOrganizationBootstrapAddr(parsec_api_types::BackendOrganizationBootstrapAddr);

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
            None => Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(
            parsec_api_types::BackendOrganizationBootstrapAddr::new(
                addr.unwrap().0,
                organization_id.0,
                token,
            ),
        ))
    }

    #[getter]
    fn hostname(&self) -> PyResult<&str> {
        Ok(self.0.hostname())
    }

    #[getter]
    fn port(&self) -> PyResult<u16> {
        Ok(self.0.port())
    }

    #[getter]
    fn use_ssl(&self) -> PyResult<bool> {
        Ok(self.0.use_ssl())
    }

    #[getter]
    fn netloc(&self) -> PyResult<String> {
        if self.0.is_default_port() {
            Ok(String::from(self.0.hostname()))
        } else {
            Ok(format!("{}:{}", self.0.hostname(), self.0.port()))
        }
    }

    #[getter]
    fn organization_id(&self) -> PyResult<OrganizationID> {
        Ok(OrganizationID::new(self.0.organization_id().as_ref()).unwrap())
    }

    #[getter]
    fn token(&self) -> PyResult<String> {
        match self.0.token() {
            Some(token) => Ok(token.to_string()),
            None => Ok(String::from("")),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        self.to_url()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "BackendOrganizationBootstrapAddr(url={})",
            self.to_url().unwrap()
        ))
    }

    fn __richcmp__(
        &self,
        py: Python,
        other: &BackendOrganizationBootstrapAddr,
        op: CompareOp,
    ) -> PyResult<bool> {
        let h1 = self.__hash__(py).unwrap();
        let h2 = other.__hash__(py).unwrap();
        comp_op(op, h1, h2)
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.to_url().unwrap(), py)
    }

    fn to_url(&self) -> PyResult<String> {
        Ok(self.0.to_url().to_string())
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
        .unwrap()
    }

    fn generate_organization_addr(
        &self,
        py: Python,
        root_verify_key: VerifyKey,
    ) -> PyResult<BackendOrganizationAddr> {
        match BackendOrganizationAddr::build(
            PyType::new::<BackendOrganizationAddr>(py),
            BackendAddr::new(
                String::from(self.0.hostname()),
                if !self.0.is_default_port() {
                    Some(self.0.port())
                } else {
                    None
                },
                self.0.use_ssl(),
            )
            .unwrap(),
            self.organization_id().unwrap(),
            root_verify_key,
        ) {
            Ok(org_addr) => Ok(org_addr),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    fn to_http_redirection_url(&self) -> PyResult<String> {
        Ok(self.0.to_http_redirection_url().to_string())
    }

    #[classmethod]
    #[args(allow_http_redirection = "false")]
    fn from_url(_cls: &PyType, url: &str, allow_http_redirection: bool) -> PyResult<Self> {
        match allow_http_redirection {
            true => match parsec_api_types::BackendOrganizationBootstrapAddr::from_any(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
            false => match parsec_api_types::BackendOrganizationBootstrapAddr::from_str(url) {
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
    ) -> PyResult<Self> {
        Ok(Self(
            parsec_api_types::BackendOrganizationBootstrapAddr::new(
                backend_addr.0,
                organization_id.0,
                token,
            ),
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub struct BackendOrganizationFileLinkAddr(parsec_api_types::BackendOrganizationFileLinkAddr);

#[pymethods]
impl BackendOrganizationFileLinkAddr {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(
        organization_id: OrganizationID,
        workspace_id: EntryID,
        encrypted_path: Vec<u8>,
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
            None => Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(
            parsec_api_types::BackendOrganizationFileLinkAddr::new(
                addr.unwrap().0,
                organization_id.0,
                workspace_id.0,
                encrypted_path,
            ),
        ))
    }

    #[getter]
    fn hostname(&self) -> PyResult<&str> {
        Ok(self.0.hostname())
    }

    #[getter]
    fn port(&self) -> PyResult<u16> {
        Ok(self.0.port())
    }

    #[getter]
    fn use_ssl(&self) -> PyResult<bool> {
        Ok(self.0.use_ssl())
    }

    #[getter]
    fn netloc(&self) -> PyResult<String> {
        if self.0.is_default_port() {
            Ok(String::from(self.0.hostname()))
        } else {
            Ok(format!("{}:{}", self.0.hostname(), self.0.port()))
        }
    }

    #[getter]
    fn organization_id(&self) -> PyResult<OrganizationID> {
        Ok(OrganizationID::new(self.0.organization_id().as_ref()).unwrap())
    }

    #[getter]
    fn workspace_id(&self) -> PyResult<EntryID> {
        Python::with_gil(|py| {
            let py_str = PyString::new(py, &self.0.workspace_id().to_string());
            EntryID::from_hex(PyType::new::<EntryID>(py), py_str)
        })
    }

    #[getter]
    fn encrypted_path<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.encrypted_path().as_slice()))
    }

    fn __str__(&self) -> PyResult<String> {
        self.to_url()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "BackendOrganizationFileLinkAddr(url={})",
            self.to_url().unwrap()
        ))
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.to_url().unwrap(), py)
    }

    fn __richcmp__(
        &self,
        py: Python,
        other: &BackendOrganizationFileLinkAddr,
        op: CompareOp,
    ) -> PyResult<bool> {
        let h1 = self.__hash__(py).unwrap();
        let h2 = other.__hash__(py).unwrap();
        comp_op(op, h1, h2)
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
        .unwrap()
    }

    fn to_url(&self) -> PyResult<String> {
        Ok(self.0.to_url().to_string())
    }

    fn to_http_redirection_url(&self) -> PyResult<String> {
        Ok(self.0.to_http_redirection_url().to_string())
    }

    #[classmethod]
    #[args(allow_http_redirection = "false")]
    fn from_url(_cls: &PyType, url: &str, allow_http_redirection: bool) -> PyResult<Self> {
        match allow_http_redirection {
            true => match parsec_api_types::BackendOrganizationFileLinkAddr::from_any(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
            false => match parsec_api_types::BackendOrganizationFileLinkAddr::from_str(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
        }
    }

    #[classmethod]
    fn build(
        _cls: &PyType,
        organization_addr: BackendOrganizationAddr,
        workspace_id: EntryID,
        encrypted_path: Vec<u8>,
    ) -> PyResult<Self> {
        Ok(Self(
            parsec_api_types::BackendOrganizationFileLinkAddr::new(
                organization_addr.get_backend_addr().0,
                organization_addr.organization_id().unwrap().0,
                workspace_id.0,
                encrypted_path,
            ),
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub struct BackendInvitationAddr(parsec_api_types::BackendInvitationAddr);

#[pymethods]
impl BackendInvitationAddr {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(
        organization_id: OrganizationID,
        invitation_type: &PyAny,
        token: InvitationToken,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Self> {
        let inv_type = match parsec_api_types::InvitationType::from_str(
            invitation_type.getattr("name")?.extract::<&str>()?,
        ) {
            Ok(iv) => iv,
            Err(err) => return Err(PyValueError::new_err(err)),
        };
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
            None => Err(PyValueError::new_err("Missing parameters")),
        };
        Ok(Self(parsec_api_types::BackendInvitationAddr::new(
            addr.unwrap().0,
            organization_id.0,
            inv_type,
            token.0,
        )))
    }

    #[getter]
    fn hostname(&self) -> PyResult<&str> {
        Ok(self.0.hostname())
    }

    #[getter]
    fn port(&self) -> PyResult<u16> {
        Ok(self.0.port())
    }

    #[getter]
    fn use_ssl(&self) -> PyResult<bool> {
        Ok(self.0.use_ssl())
    }

    #[getter]
    fn netloc(&self) -> PyResult<String> {
        if self.0.is_default_port() {
            Ok(String::from(self.0.hostname()))
        } else {
            Ok(format!("{}:{}", self.0.hostname(), self.0.port()))
        }
    }

    #[getter]
    fn organization_id(&self) -> PyResult<OrganizationID> {
        Ok(OrganizationID::new(self.0.organization_id().as_ref()).unwrap())
    }

    #[getter]
    fn invitation_type(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            let cls = py
                .import("parsec.api.protocol")?
                .getattr("InvitationType")?;
            let name = self.0.invitation_type().to_string();
            let obj = cls.getattr(name)?;
            Ok(obj.into_py(py))
        })
    }

    #[getter]
    fn token(&self) -> PyResult<InvitationToken> {
        Ok(InvitationToken(self.0.token().clone()))
    }

    fn __str__(&self) -> PyResult<String> {
        self.to_url()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "BackendInvitationAddr(url={})",
            self.to_url().unwrap()
        ))
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.to_url().unwrap(), py)
    }

    fn __richcmp__(
        &self,
        py: Python,
        other: &BackendInvitationAddr,
        op: CompareOp,
    ) -> PyResult<bool> {
        let h1 = self.__hash__(py).unwrap();
        let h2 = other.__hash__(py).unwrap();
        comp_op(op, h1, h2)
    }

    fn to_url(&self) -> PyResult<String> {
        Ok(self.0.to_url().to_string())
    }

    fn to_http_redirection_url(&self) -> PyResult<String> {
        Ok(self.0.to_http_redirection_url().to_string())
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
        .unwrap()
    }

    fn generate_organization_addr(
        &self,
        py: Python,
        root_verify_key: VerifyKey,
    ) -> PyResult<BackendOrganizationAddr> {
        match BackendOrganizationAddr::build(
            PyType::new::<BackendOrganizationAddr>(py),
            BackendAddr::new(
                String::from(self.0.hostname()),
                if !self.0.is_default_port() {
                    Some(self.0.port())
                } else {
                    None
                },
                self.0.use_ssl(),
            )
            .unwrap(),
            self.organization_id().unwrap(),
            root_verify_key,
        ) {
            Ok(org_addr) => Ok(org_addr),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    #[args(allow_http_redirection = "false")]
    fn from_url(_cls: &PyType, url: &str, allow_http_redirection: bool) -> PyResult<Self> {
        match allow_http_redirection {
            true => match parsec_api_types::BackendInvitationAddr::from_any(url) {
                Ok(backend_addr) => Ok(Self(backend_addr)),
                Err(err) => Err(PyValueError::new_err(err)),
            },
            false => match parsec_api_types::BackendInvitationAddr::from_str(url) {
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
        invitation_type: &PyAny,
        token: InvitationToken,
    ) -> PyResult<Self> {
        let inv_type = match parsec_api_types::InvitationType::from_str(
            invitation_type.getattr("name")?.extract::<&str>()?,
        ) {
            Ok(iv) => iv,
            Err(err) => return Err(PyValueError::new_err(err)),
        };

        Ok(Self(parsec_api_types::BackendInvitationAddr::new(
            backend_addr.0,
            organization_id.0,
            inv_type,
            token.0,
        )))
    }
}
