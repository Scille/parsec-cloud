// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    create_exception,
    exceptions::{PyException, PyValueError},
    prelude::*,
    types::{PyByteArray, PyBytes, PyType},
};

create_exception!(_parsec, CryptoError, PyException);

pub(crate) fn add_mod(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HashDigest>()?;
    m.add_class::<SigningKey>()?;
    m.add_class::<VerifyKey>()?;
    m.add_class::<SecretKey>()?;
    m.add_class::<PrivateKey>()?;
    m.add_class::<PublicKey>()?;
    m.add_class::<SequesterPrivateKeyDer>()?;
    m.add_class::<SequesterPublicKeyDer>()?;
    m.add_class::<SequesterSigningKeyDer>()?;
    m.add_class::<SequesterVerifyKeyDer>()?;
    m.add_function(wrap_pyfunction!(generate_nonce, m)?)?;

    m.add("CryptoError", py.get_type::<CryptoError>())?;

    Ok(())
}

crate::binding_utils::gen_py_wrapper_class!(
    HashDigest,
    libparsec::low_level::types::HashDigest,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl HashDigest {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec::low_level::crypto::HashDigest::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    #[staticmethod]
    fn from_data(py: Python, data: PyObject) -> PyResult<Self> {
        let bytes = match data.extract::<&PyByteArray>(py) {
            // SAFETY: Using PyByteArray::as_bytes is safe as long as the corresponding memory is not modified.
            // Here, the GIL is held during the entire access to `bytes` so there is no risk of another
            // python thread modifying the bytearray behind our back.
            Ok(x) => unsafe { x.as_bytes() },
            Err(_) => data.extract::<&PyBytes>(py)?.as_bytes(),
        };
        Ok(Self(libparsec::low_level::crypto::HashDigest::from_data(
            bytes,
        )))
    }

    #[getter]
    fn digest<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, self.0.as_ref())
    }

    fn hexdigest(&self) -> String {
        self.0.hexdigest()
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SigningKey,
    libparsec::low_level::types::SigningKey,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl SigningKey {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec::low_level::crypto::SigningKey::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    #[getter]
    fn verify_key(&self) -> VerifyKey {
        VerifyKey(self.0.verify_key())
    }

    #[classmethod]
    fn generate(_cls: &PyType) -> Self {
        Self(libparsec::low_level::crypto::SigningKey::generate())
    }

    /// Return the signature + the signed data
    fn sign<'py>(&self, py: Python<'py>, data: &[u8]) -> &'py PyBytes {
        PyBytes::new(py, self.0.sign(data).as_slice())
    }

    /// Return only the signature of the data
    fn sign_only_signature<'py>(&self, py: Python<'py>, data: &[u8]) -> &'py PyBytes {
        PyBytes::new(py, self.0.sign_only_signature(data).as_slice())
    }

    fn encode<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, self.0.as_ref())
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    VerifyKey,
    libparsec::low_level::types::VerifyKey,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl VerifyKey {
    #[new]
    pub fn new(data: &[u8]) -> PyResult<Self> {
        libparsec::low_level::crypto::VerifyKey::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    /// Verify a message using the given `VerifyKey` and `signed` data
    /// `signed` data is the concatenation of the `signature` + `data`
    fn verify<'py>(&self, py: Python<'py>, signed: &[u8]) -> PyResult<&'py PyBytes> {
        match self.0.verify(signed) {
            Ok(v) => Ok(PyBytes::new(py, v)),
            Err(_) => Err(CryptoError::new_err("Signature was forged or corrupt")),
        }
    }

    /// Verify a message using the given `VerifyKey`, `Signature` and `message`
    fn verify_with_signature(&self, signature: &[u8], message: &[u8]) -> PyResult<()> {
        self.0
            .verify_with_signature(
                <&[u8; libparsec::low_level::crypto::SigningKey::SIGNATURE_SIZE]>::try_from(
                    signature,
                )
                .map_err(|_| CryptoError::new_err("Invalid signature size"))?,
                message,
            )
            .map_err(|_| CryptoError::new_err("Signature was forged or corrupt"))
    }

    #[classmethod]
    fn unsecure_unwrap<'py>(
        _cls: &PyType,
        py: Python<'py>,
        signed: &[u8],
    ) -> PyResult<&'py PyBytes> {
        let (_, message) = libparsec::low_level::crypto::VerifyKey::unsecure_unwrap(signed)
            .map_err(|_| CryptoError::new_err("Signature was forged or corrupt"))?;
        Ok(PyBytes::new(py, message))
    }

    fn encode<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, self.0.as_ref())
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, self.0.as_ref())
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SecretKey,
    libparsec::low_level::types::SecretKey,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl SecretKey {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec::low_level::crypto::SecretKey::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    #[classmethod]
    fn generate(_cls: &PyType) -> Self {
        Self(libparsec::low_level::crypto::SecretKey::generate())
    }

    #[getter]
    fn secret<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, self.0.as_ref())
    }

    fn encrypt<'py>(&self, py: Python<'py>, data: PyObject) -> PyResult<&'py PyBytes> {
        let bytes = match data.extract::<&PyByteArray>(py) {
            // SAFETY: Using PyByteArray::as_bytes is safe as long as the corresponding memory is not modified.
            // Here, the GIL is held during the entire access to `bytes` so there is no risk of another
            // python thread modifying the bytearray behind our back.
            Ok(x) => unsafe { x.as_bytes() },
            Err(_) => data.extract::<&PyBytes>(py)?.as_bytes(),
        };
        Ok(PyBytes::new(py, &self.0.encrypt(bytes)))
    }

    fn decrypt<'py>(&self, py: Python<'py>, ciphered: &[u8]) -> PyResult<&'py PyBytes> {
        match self.0.decrypt(ciphered) {
            Ok(v) => Ok(PyBytes::new(py, &v)),
            Err(err) => Err(CryptoError::new_err(err.to_string())),
        }
    }

    fn hmac<'py>(&self, py: Python<'py>, data: &[u8], digest_size: usize) -> &'py PyBytes {
        PyBytes::new(py, &self.0.hmac(data, digest_size))
    }

    #[classmethod]
    fn generate_salt<'py>(_cls: &PyType, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(
            py,
            &libparsec::low_level::crypto::SecretKey::generate_salt(),
        )
    }

    #[classmethod]
    fn from_password(_cls: &PyType, password: &str, salt: &[u8]) -> PyResult<Self> {
        libparsec::low_level::crypto::SecretKey::from_password(password, salt)
            .map(Self)
            .map_err(|err| CryptoError::new_err(err.to_string()))
    }

    #[classmethod]
    fn generate_recovery_passphrase(_cls: &PyType) -> (String, Self) {
        let (passphrase, key) =
            libparsec::low_level::crypto::SecretKey::generate_recovery_passphrase();
        (passphrase, Self(key))
    }

    #[classmethod]
    fn from_recovery_passphrase(_cls: &PyType, passphrase: &str) -> PyResult<Self> {
        libparsec::low_level::crypto::SecretKey::from_recovery_passphrase(passphrase)
            .map(Self)
            .map_err(|err| CryptoError::new_err(err.to_string()))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    PrivateKey,
    libparsec::low_level::types::PrivateKey,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl PrivateKey {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec::low_level::crypto::PrivateKey::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    #[classmethod]
    fn generate(_cls: &PyType) -> Self {
        PrivateKey(libparsec::low_level::crypto::PrivateKey::generate())
    }

    #[getter]
    fn public_key(&self) -> PublicKey {
        PublicKey(libparsec::low_level::crypto::PrivateKey::public_key(
            &self.0,
        ))
    }

    fn decrypt_from_self<'py>(&self, py: Python<'py>, ciphered: &[u8]) -> PyResult<&'py PyBytes> {
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

crate::binding_utils::gen_py_wrapper_class!(
    PublicKey,
    libparsec::low_level::types::PublicKey,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl PublicKey {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec::low_level::crypto::PublicKey::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn encrypt_for_self<'py>(&self, py: Python<'py>, data: PyObject) -> PyResult<&'py PyBytes> {
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

crate::binding_utils::gen_py_wrapper_class!(
    SequesterPrivateKeyDer,
    libparsec::low_level::types::SequesterPrivateKeyDer,
    __repr__,
    __copy__,
    __deepcopy__,
);

#[pymethods]
impl SequesterPrivateKeyDer {
    #[new]
    pub fn new(data: &[u8]) -> PyResult<Self> {
        libparsec::low_level::crypto::SequesterPrivateKeyDer::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    // TODO: Remove once it's no longer used in python test
    #[classmethod]
    fn generate_pair(
        _cls: &PyType,
        size_in_bits: usize,
    ) -> PyResult<(Self, SequesterPublicKeyDer)> {
        let (priv_key, pub_key) =
            libparsec::low_level::crypto::SequesterPrivateKeyDer::generate_pair(
                match size_in_bits {
                    1024 => libparsec::low_level::crypto::SequesterKeySize::_1024Bits,
                    2048 => libparsec::low_level::crypto::SequesterKeySize::_2048Bits,
                    3072 => libparsec::low_level::crypto::SequesterKeySize::_3072Bits,
                    4096 => libparsec::low_level::crypto::SequesterKeySize::_4096Bits,
                    _ => return Err(PyValueError::new_err(
                        "Invalid argument: size_in_bits must be equal to 1024 | 2048 | 3072 | 4096",
                    )),
                },
            );
        Ok((Self(priv_key), SequesterPublicKeyDer(pub_key)))
    }

    fn dump<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump())
    }

    fn dump_pem(&self) -> String {
        self.0.dump_pem().to_string()
    }

    #[classmethod]
    fn load_pem(_cls: &PyType, s: &str) -> PyResult<Self> {
        libparsec::low_level::crypto::SequesterPrivateKeyDer::load_pem(s)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn decrypt<'py>(&self, py: Python<'py>, data: &[u8]) -> PyResult<&'py PyBytes> {
        self.0
            .decrypt(data)
            .map(|x| PyBytes::new(py, &x))
            .map_err(|err| CryptoError::new_err(err.to_string()))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SequesterPublicKeyDer,
    libparsec::low_level::types::SequesterPublicKeyDer,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl SequesterPublicKeyDer {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec::low_level::crypto::SequesterPublicKeyDer::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn dump<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump())
    }

    fn dump_pem(&self) -> String {
        self.0.dump_pem()
    }

    #[classmethod]
    fn load_pem(_cls: &PyType, s: &str) -> PyResult<Self> {
        libparsec::low_level::crypto::SequesterPublicKeyDer::load_pem(s)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn encrypt<'py>(&self, py: Python<'py>, data: &[u8]) -> &'py PyBytes {
        PyBytes::new(py, &self.0.encrypt(data))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SequesterSigningKeyDer,
    libparsec::low_level::types::SequesterSigningKeyDer,
    __repr__,
    __copy__,
    __deepcopy__,
);

#[pymethods]
impl SequesterSigningKeyDer {
    #[new]
    pub fn new(data: &[u8]) -> PyResult<Self> {
        libparsec::low_level::crypto::SequesterSigningKeyDer::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    // TODO: Remove once it's no longer used in python test
    #[classmethod]
    fn generate_pair(
        _cls: &PyType,
        size_in_bits: usize,
    ) -> PyResult<(Self, SequesterVerifyKeyDer)> {
        let (priv_key, pub_key) =
            libparsec::low_level::crypto::SequesterSigningKeyDer::generate_pair(
                match size_in_bits {
                    1024 => libparsec::low_level::crypto::SequesterKeySize::_1024Bits,
                    2048 => libparsec::low_level::crypto::SequesterKeySize::_2048Bits,
                    3072 => libparsec::low_level::crypto::SequesterKeySize::_3072Bits,
                    4096 => libparsec::low_level::crypto::SequesterKeySize::_4096Bits,
                    _ => return Err(PyValueError::new_err(
                        "Invalid argument: size_in_bits must be equal to 1024 | 2048 | 3072 | 4096",
                    )),
                },
            );
        Ok((Self(priv_key), SequesterVerifyKeyDer(pub_key)))
    }

    fn dump<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump())
    }

    fn dump_pem(&self) -> String {
        self.0.dump_pem().to_string()
    }

    #[classmethod]
    fn load_pem(_cls: &PyType, s: &str) -> PyResult<Self> {
        libparsec::low_level::crypto::SequesterSigningKeyDer::load_pem(s)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn sign<'py>(&self, py: Python<'py>, data: &[u8]) -> &'py PyBytes {
        PyBytes::new(py, &self.0.sign(data))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SequesterVerifyKeyDer,
    libparsec::low_level::types::SequesterVerifyKeyDer,
    __repr__,
    __copy__,
    __deepcopy__,
);

#[pymethods]
impl SequesterVerifyKeyDer {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec::low_level::crypto::SequesterVerifyKeyDer::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn dump<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump())
    }

    fn dump_pem(&self) -> String {
        self.0.dump_pem()
    }

    #[classmethod]
    fn load_pem(_cls: &PyType, s: &str) -> PyResult<Self> {
        libparsec::low_level::crypto::SequesterVerifyKeyDer::load_pem(s)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn verify<'py>(&self, py: Python<'py>, data: &[u8]) -> PyResult<&'py PyBytes> {
        self.0
            .verify(data)
            .map(|x| PyBytes::new(py, &x))
            .map_err(|err| CryptoError::new_err(err.to_string()))
    }
}

#[pyfunction]
pub(crate) fn generate_nonce(py: Python<'_>) -> &PyBytes {
    PyBytes::new(py, &libparsec::low_level::crypto::generate_nonce())
}
