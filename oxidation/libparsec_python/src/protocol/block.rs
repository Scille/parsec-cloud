// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};

use crate::ids::{BlockID, RealmID};
use parsec_api_protocol::authenticated_cmds::{block_create, block_read};

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct BlockCreateReq(pub block_create::Req);

#[pymethods]
impl BlockCreateReq {
    #[new]
    fn new(block_id: BlockID, realm_id: RealmID, block: Vec<u8>) -> PyResult<Self> {
        Ok(Self(block_create::Req {
            block_id: block_id.0,
            realm_id: realm_id.0,
            block,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}>", self.0))
    }

    fn dumps<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dumps().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct BlockCreateRep(pub block_create::Rep);

#[pymethods]
impl BlockCreateRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(block_create::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyExists")]
    fn already_exists(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(block_create::Rep::AlreadyExists))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(block_create::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "Timeout")]
    fn timeout(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(block_create::Rep::Timeout))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(block_create::Rep::NotAllowed))
    }

    #[classmethod]
    #[pyo3(name = "InMaintenance")]
    fn in_maintenance(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(block_create::Rep::InMaintenance))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}>", self.0))
    }

    fn dumps<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dumps().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn loads(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        Ok(Self(block_create::Rep::loads(&buf)))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct BlockReadReq(pub block_read::Req);

#[pymethods]
impl BlockReadReq {
    #[new]
    fn new(block_id: BlockID) -> PyResult<Self> {
        Ok(Self(block_read::Req {
            block_id: block_id.0,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}>", self.0))
    }

    fn dumps<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dumps().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct BlockReadRep(pub block_read::Rep);

#[pymethods]
impl BlockReadRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, block: Vec<u8>) -> PyResult<Self> {
        Ok(Self(block_read::Rep::Ok { block }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(block_read::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "Timeout")]
    fn timeout(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(block_read::Rep::Timeout))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(block_read::Rep::NotAllowed))
    }

    #[classmethod]
    #[pyo3(name = "InMaintenance")]
    fn in_maintenance(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(block_read::Rep::InMaintenance))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}>", self.0))
    }

    fn dumps<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dumps().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn loads(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        Ok(Self(block_read::Rep::loads(&buf)))
    }
}
