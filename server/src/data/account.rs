// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    prelude::{pyclass, pymethods, PyAny, PyClassInitializer, PyRef, PyResult, Python},
    types::PyBytes,
    Bound,
};

use crate::BytesWrapper;

#[pyclass(subclass)]
#[derive(Clone)]
pub struct PasswordAlgorithm(pub libparsec_types::PasswordAlgorithm);

crate::binding_utils::gen_proto!(PasswordAlgorithm, __repr__);
crate::binding_utils::gen_proto!(PasswordAlgorithm, __copy__);
crate::binding_utils::gen_proto!(PasswordAlgorithm, __deepcopy__);
crate::binding_utils::gen_proto!(PasswordAlgorithm, __richcmp__, eq);

impl PasswordAlgorithm {
    pub fn convert(
        py: Python,
        item: libparsec_types::PasswordAlgorithm,
    ) -> PyResult<Bound<'_, PyAny>> {
        let init = PyClassInitializer::from(PasswordAlgorithm(item));
        match item {
            libparsec_types::PasswordAlgorithm::Argon2id { .. } => {
                Ok(Bound::new(py, init.add_subclass(PasswordAlgorithmArgon2id {}))?.into_any())
            }
        }
    }
}

#[pyclass(extends=PasswordAlgorithm, subclass)]
#[derive(Clone)]
pub struct PasswordAlgorithmArgon2id {}

#[pymethods]
impl PasswordAlgorithmArgon2id {
    #[new]
    #[pyo3(signature = (salt, opslimit, memlimit_kb, parallelism))]
    fn new(
        salt: BytesWrapper,
        opslimit: u32,
        memlimit_kb: u32,
        parallelism: u32,
    ) -> (Self, PasswordAlgorithm) {
        crate::binding_utils::unwrap_bytes!(salt);
        (
            Self {},
            PasswordAlgorithm(libparsec_types::PasswordAlgorithm::Argon2id {
                salt,
                opslimit,
                memlimit_kb,
                parallelism,
            }),
        )
    }

    #[getter]
    fn salt<'py>(self_: PyRef<Self>, py: Python<'py>) -> Bound<'py, PyBytes> {
        let super_: &PasswordAlgorithm = self_.as_ref();
        match &super_.0 {
            libparsec_types::PasswordAlgorithm::Argon2id { salt, .. } => {
                PyBytes::new_bound(py, salt)
            }
        }
    }

    #[getter]
    fn opslimit(self_: PyRef<Self>) -> u32 {
        let super_: &PasswordAlgorithm = self_.as_ref();
        match &super_.0 {
            libparsec_types::PasswordAlgorithm::Argon2id { opslimit, .. } => *opslimit,
        }
    }

    #[getter]
    fn memlimit_kb(self_: PyRef<Self>) -> u32 {
        let super_: &PasswordAlgorithm = self_.as_ref();
        match &super_.0 {
            libparsec_types::PasswordAlgorithm::Argon2id { memlimit_kb, .. } => *memlimit_kb,
        }
    }

    #[getter]
    fn parallelism(self_: PyRef<Self>) -> u32 {
        let super_: &PasswordAlgorithm = self_.as_ref();
        match &super_.0 {
            libparsec_types::PasswordAlgorithm::Argon2id { parallelism, .. } => *parallelism,
        }
    }
}
