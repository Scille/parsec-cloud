// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{exceptions::PyNotImplementedError, import_exception, prelude::*, types::PyBytes};

use crate::ids::{BlockID, RealmID};
use crate::protocol::gen_rep;
use libparsec::protocol::authenticated_cmds::{block_create, block_read};

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(Clone)]
pub(crate) struct BlockCreateReq(pub block_create::Req);

crate::binding_utils::gen_proto!(BlockCreateReq, __repr__);
crate::binding_utils::gen_proto!(BlockCreateReq, __richcmp__, eq);

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

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn block_id(&self) -> PyResult<BlockID> {
        Ok(BlockID(self.0.block_id))
    }

    #[getter]
    fn realm_id(&self) -> PyResult<RealmID> {
        Ok(RealmID(self.0.realm_id))
    }

    #[getter]
    fn block(&self) -> PyResult<&[u8]> {
        Ok(&self.0.block)
    }
}

gen_rep!(
    block_create,
    BlockCreateRep,
    [AlreadyExists],
    [NotFound],
    [Timeout],
    [NotAllowed],
    [InMaintenance]
);

#[pyclass(extends=BlockCreateRep)]
pub(crate) struct BlockCreateRepOk;

#[pymethods]
impl BlockCreateRepOk {
    #[new]
    fn new() -> PyResult<(Self, BlockCreateRep)> {
        Ok((Self, BlockCreateRep(block_create::Rep::Ok)))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct BlockReadReq(pub block_read::Req);

crate::binding_utils::gen_proto!(BlockReadReq, __repr__);
crate::binding_utils::gen_proto!(BlockReadReq, __richcmp__, eq);

#[pymethods]
impl BlockReadReq {
    #[new]
    fn new(block_id: BlockID) -> PyResult<Self> {
        Ok(Self(block_read::Req {
            block_id: block_id.0,
        }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn block_id(&self) -> PyResult<BlockID> {
        Ok(BlockID(self.0.block_id))
    }
}

gen_rep!(
    block_read,
    BlockReadRep,
    { .. },
    [NotFound],
    [Timeout],
    [NotAllowed],
    [InMaintenance],
);

#[pyclass(extends=BlockReadRep)]
pub(crate) struct BlockReadRepOk;

#[pymethods]
impl BlockReadRepOk {
    #[new]
    fn new(block: Vec<u8>) -> PyResult<(Self, BlockReadRep)> {
        Ok((Self, BlockReadRep(block_read::Rep::Ok { block })))
    }

    #[getter]
    fn block<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(match &_self.as_ref().0 {
            block_read::Rep::Ok { block, .. } => PyBytes::new(py, block),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}
