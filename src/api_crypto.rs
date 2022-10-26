// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::exceptions::PyValueError;
use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes, PyType};

import_exception!(nacl.exceptions, CryptoError);

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct HashDigest(pub libparsec::crypto::HashDigest);

crate::binding_utils::gen_proto!(HashDigest, __repr__);
crate::binding_utils::gen_proto!(HashDigest, __richcmp__, eq);

#[pymethods]
impl HashDigest {
    #[new]
    fn new(hash: &[u8]) -> PyResult<Self> {
        match libparsec::crypto::HashDigest::try_from(hash) {
            Ok(h) => Ok(Self(h)),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    #[staticmethod]
    fn from_data(py: Python, data: PyObject) -> PyResult<HashDigest> {
        let bytes = match data.extract::<&PyByteArray>(py) {
            // SAFETY: Using PyByteArray::as_bytes is safe as long as the corresponding memory is not modified.
            // Here, the GIL is held during the entire access to `bytes` so there is no risk of another
            // python thread modifying the bytearray behind our back.
            Ok(x) => unsafe { x.as_bytes() },
            Err(_) => data.extract::<&PyBytes>(py)?.as_bytes(),
        };
        Ok(Self(libparsec::crypto::HashDigest::from_data(bytes)))
    }

    #[getter]
    fn digest<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_ref()))
    }

    fn hexdigest(&self) -> PyResult<String> {
        Ok(self.0.hexdigest())
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct SigningKey(pub libparsec::crypto::SigningKey);

crate::binding_utils::gen_proto!(SigningKey, __repr__);
crate::binding_utils::gen_proto!(SigningKey, __richcmp__, eq);

#[pymethods]
impl SigningKey {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        match libparsec::crypto::SigningKey::try_from(data) {
            Ok(h) => Ok(Self(h)),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    #[getter]
    fn verify_key(&self) -> PyResult<VerifyKey> {
        Ok(VerifyKey(self.0.verify_key()))
    }

    #[classmethod]
    fn generate(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(libparsec::crypto::SigningKey::generate()))
    }

    /// Return the signature + the signed data
    fn sign<'p>(&self, py: Python<'p>, data: &[u8]) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.sign(data).as_slice()))
    }

    /// Return only the signature of the data
    fn sign_only_signature<'p>(&self, py: Python<'p>, data: &[u8]) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(
            py,
            self.0.sign_only_signature(data).as_slice(),
        ))
    }

    fn encode<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_ref()))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct VerifyKey(pub libparsec::crypto::VerifyKey);

crate::binding_utils::gen_proto!(VerifyKey, __repr__);
crate::binding_utils::gen_proto!(VerifyKey, __richcmp__, eq);

#[pymethods]
impl VerifyKey {
    #[new]
    pub fn new(data: &[u8]) -> PyResult<Self> {
        match libparsec::crypto::VerifyKey::try_from(data) {
            Ok(h) => Ok(Self(h)),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    /// Verify a message using the given `VerifyKey` and `signed` data
    /// `signed` data is the concatenation of the `signature` + `data`
    fn verify<'p>(&self, py: Python<'p>, signed: &[u8]) -> PyResult<&'p PyBytes> {
        match self.0.verify(signed) {
            Ok(v) => Ok(PyBytes::new(py, &v)),
            Err(_) => Err(CryptoError::new_err("Signature was forged or corrupt")),
        }
    }

    /// Verify a message using the given `VerifyKey`, `Signature` and `message`
    fn verify_with_signature<'p>(
        &self,
        py: Python<'p>,
        signature: &[u8],
        message: &[u8],
    ) -> PyResult<&'p PyBytes> {
        match self.0.verify_with_signature(signature, message) {
            Ok(v) => Ok(PyBytes::new(py, &v)),
            Err(_) => Err(CryptoError::new_err("Signature was forged or corrupt")),
        }
    }

    #[classmethod]
    fn unsecure_unwrap<'p>(
        _cls: &PyType,
        py: Python<'p>,
        signed: &[u8],
    ) -> PyResult<Option<&'p PyBytes>> {
        match libparsec::crypto::VerifyKey::unsecure_unwrap(signed) {
            Some(v) => Ok(Some(PyBytes::new(py, v))),
            None => Ok(Some(PyBytes::new(py, &[]))),
        }
    }

    fn encode<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_ref()))
    }

    fn __bytes__<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_ref()))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub struct SecretKey(pub libparsec::crypto::SecretKey);

crate::binding_utils::gen_proto!(SecretKey, __repr__);
crate::binding_utils::gen_proto!(SecretKey, __richcmp__, eq);

#[pymethods]
impl SecretKey {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        match libparsec::crypto::SecretKey::try_from(data) {
            Ok(h) => Ok(Self(h)),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    #[classmethod]
    fn generate(_cls: &PyType) -> PyResult<SecretKey> {
        Ok(SecretKey(libparsec::crypto::SecretKey::generate()))
    }

    #[getter]
    fn secret<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_ref()))
    }

    pub fn encrypt<'p>(&self, py: Python<'p>, data: PyObject) -> PyResult<&'p PyBytes> {
        let bytes = match data.extract::<&PyByteArray>(py) {
            // SAFETY: Using PyByteArray::as_bytes is safe as long as the corresponding memory is not modified.
            // Here, the GIL is held during the entire access to `bytes` so there is no risk of another
            // python thread modifying the bytearray behind our back.
            Ok(x) => unsafe { x.as_bytes() },
            Err(_) => data.extract::<&PyBytes>(py)?.as_bytes(),
        };
        Ok(PyBytes::new(py, &self.0.encrypt(bytes)))
    }

    pub fn decrypt<'p>(&self, py: Python<'p>, ciphered: &[u8]) -> PyResult<&'p PyBytes> {
        match self.0.decrypt(ciphered) {
            Ok(v) => Ok(PyBytes::new(py, &v)),
            Err(err) => Err(CryptoError::new_err(err.to_string())),
        }
    }

    pub fn hmac<'p>(
        &self,
        py: Python<'p>,
        data: &[u8],
        digest_size: usize,
    ) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.hmac(data, digest_size)))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct PrivateKey(pub libparsec::crypto::PrivateKey);

crate::binding_utils::gen_proto!(PrivateKey, __repr__);
crate::binding_utils::gen_proto!(PrivateKey, __richcmp__, eq);

#[pymethods]
impl PrivateKey {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        match libparsec::crypto::PrivateKey::try_from(data) {
            Ok(h) => Ok(Self(h)),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    #[classmethod]
    fn generate(_cls: &PyType) -> PyResult<PrivateKey> {
        Ok(PrivateKey(libparsec::crypto::PrivateKey::generate()))
    }

    #[getter]
    fn public_key(&self) -> PublicKey {
        PublicKey(libparsec::crypto::PrivateKey::public_key(&self.0))
    }

    fn decrypt_from_self<'p>(&self, py: Python<'p>, ciphered: &[u8]) -> PyResult<&'p PyBytes> {
        match self.0.decrypt_from_self(ciphered) {
            Ok(v) => Ok(PyBytes::new(py, &v)),
            Err(err) => Err(CryptoError::new_err(err.to_string())),
        }
    }

    fn encode(&self) -> &[u8] {
        self.0.as_ref()
    }

    fn generate_shared_secret_key(&self, peer_public_key: &PublicKey) -> SecretKey {
        SecretKey(self.0.generate_shared_secret_key(&peer_public_key.0))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct PublicKey(pub libparsec::crypto::PublicKey);

crate::binding_utils::gen_proto!(PublicKey, __repr__);
crate::binding_utils::gen_proto!(PublicKey, __richcmp__, eq);

#[pymethods]
impl PublicKey {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        match libparsec::crypto::PublicKey::try_from(data) {
            Ok(h) => Ok(Self(h)),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    fn encrypt_for_self<'p>(&self, py: Python<'p>, data: PyObject) -> PyResult<&'p PyBytes> {
        let bytes = match data.extract::<&PyByteArray>(py) {
            // SAFETY: Using PyByteArray::as_bytes is safe as long as the corresponding memory is not modified.
            // Here, the GIL is held during the entire access to `bytes` so there is no risk of another
            // python thread modifying the bytearray behind our back.
            Ok(x) => unsafe { x.as_bytes() },
            Err(_) => data.extract::<&PyBytes>(py)?.as_bytes(),
        };
        Ok(PyBytes::new(py, &self.0.encrypt_for_self(bytes)))
    }

    fn encode(&self) -> &[u8] {
        self.0.as_ref()
    }
}
