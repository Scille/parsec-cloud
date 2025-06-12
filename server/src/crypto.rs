// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// FIXME: Remove me once we migrate to pyo3@0.24
// Pyo3 generate useless conversion in the generated code, it was fixed in
// https://github.com/PyO3/pyo3/pull/4838 that was release in 0.24
#![allow(clippy::useless_conversion)]

use pyo3::{
    create_exception,
    exceptions::{PyException, PyValueError},
    prelude::*,
    types::{PyByteArray, PyBytes, PyType},
    Bound,
};

use crate::BytesWrapper;

create_exception!(_parsec, CryptoError, PyException);

pub(crate) fn add_mod(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
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
    m.add_class::<PasswordAlgorithm>()?;
    m.add_class::<PasswordAlgorithmArgon2id>()?;
    m.add_function(wrap_pyfunction!(generate_nonce, m)?)?;

    m.add("CryptoError", py.get_type_bound::<CryptoError>())?;

    Ok(())
}

// Note the `_for_id` flavor of this macro, this is to allow `HashDigest` to
// be used as key in a dictionary
crate::binding_utils::gen_py_wrapper_class_for_id!(
    HashDigest,
    libparsec_types::HashDigest,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
    __hash__,
);

#[pymethods]
impl HashDigest {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec_crypto::HashDigest::try_from(data)
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
        Ok(Self(libparsec_crypto::HashDigest::from_data(bytes)))
    }

    #[getter]
    fn digest<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, self.0.as_ref())
    }

    fn hexdigest(&self) -> String {
        self.0.hexdigest()
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SigningKey,
    libparsec_types::SigningKey,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl SigningKey {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec_crypto::SigningKey::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    #[getter]
    fn verify_key(&self) -> VerifyKey {
        VerifyKey(self.0.verify_key())
    }

    #[classmethod]
    fn generate(_cls: Bound<'_, PyType>) -> Self {
        Self(libparsec_crypto::SigningKey::generate())
    }

    /// Return the signature + the signed data
    fn sign<'py>(&self, py: Python<'py>, data: &[u8]) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, self.0.sign(data).as_slice())
    }

    /// Return only the signature of the data
    fn sign_only_signature<'py>(&self, py: Python<'py>, data: &[u8]) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, self.0.sign_only_signature(data).as_slice())
    }

    fn encode<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.to_bytes())
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    VerifyKey,
    libparsec_types::VerifyKey,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl VerifyKey {
    #[new]
    pub fn new(data: &[u8]) -> PyResult<Self> {
        libparsec_crypto::VerifyKey::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    /// Verify a message using the given `VerifyKey` and `signed` data
    /// `signed` data is the concatenation of the `signature` + `data`
    fn verify<'py>(&self, py: Python<'py>, signed: &[u8]) -> PyResult<Bound<'py, PyBytes>> {
        match self.0.verify(signed) {
            Ok(v) => Ok(PyBytes::new_bound(py, v)),
            Err(_) => Err(CryptoError::new_err("Signature was forged or corrupt")),
        }
    }

    /// Verify a message using the given `VerifyKey`, `Signature` and `message`
    fn verify_with_signature(&self, signature: &[u8], message: &[u8]) -> PyResult<()> {
        self.0
            .verify_with_signature(
                <&[u8; libparsec_crypto::SigningKey::SIGNATURE_SIZE]>::try_from(signature)
                    .map_err(|_| CryptoError::new_err("Invalid signature size"))?,
                message,
            )
            .map_err(|_| CryptoError::new_err("Signature was forged or corrupt"))
    }

    #[classmethod]
    fn unsecure_unwrap<'py>(
        _cls: Bound<'_, PyType>,
        py: Python<'py>,
        signed: &[u8],
    ) -> PyResult<Bound<'py, PyBytes>> {
        let (_, message) = libparsec_crypto::VerifyKey::unsecure_unwrap(signed)
            .map_err(|_| CryptoError::new_err("Signature was forged or corrupt"))?;
        Ok(PyBytes::new_bound(py, message))
    }

    fn encode<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, self.0.as_ref())
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SecretKey,
    libparsec_types::SecretKey,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl SecretKey {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec_crypto::SecretKey::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    #[classmethod]
    fn generate(_cls: Bound<'_, PyType>) -> Self {
        Self(libparsec_crypto::SecretKey::generate())
    }

    #[getter]
    fn secret<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, self.0.as_ref())
    }

    fn encrypt<'py>(&self, py: Python<'py>, data: PyObject) -> PyResult<Bound<'py, PyBytes>> {
        let bytes = match data.extract::<&PyByteArray>(py) {
            // SAFETY: Using PyByteArray::as_bytes is safe as long as the corresponding memory is not modified.
            // Here, the GIL is held during the entire access to `bytes` so there is no risk of another
            // python thread modifying the bytearray behind our back.
            Ok(x) => unsafe { x.as_bytes() },
            Err(_) => data.extract::<&PyBytes>(py)?.as_bytes(),
        };
        Ok(PyBytes::new_bound(py, &self.0.encrypt(bytes)))
    }

    fn decrypt<'py>(&self, py: Python<'py>, ciphered: &[u8]) -> PyResult<Bound<'py, PyBytes>> {
        match self.0.decrypt(ciphered) {
            Ok(v) => Ok(PyBytes::new_bound(py, &v)),
            Err(err) => Err(CryptoError::new_err(err.to_string())),
        }
    }

    fn mac_512<'py>(&self, py: Python<'py>, data: &[u8]) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.mac_512(data))
    }

    fn sas_code<'py>(&self, py: Python<'py>, data: &[u8]) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.sas_code(data))
    }

    #[classmethod]
    fn generate_salt<'py>(_cls: Bound<'_, PyType>, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &libparsec_crypto::SecretKey::generate_salt())
    }

    #[classmethod]
    fn generate_recovery_passphrase(_cls: Bound<'_, PyType>) -> (String, Self) {
        let (passphrase, key) = libparsec_crypto::SecretKey::generate_recovery_passphrase();
        (passphrase.to_string(), Self(key))
    }

    #[classmethod]
    fn from_recovery_passphrase(_cls: Bound<'_, PyType>, passphrase: &str) -> PyResult<Self> {
        libparsec_crypto::SecretKey::from_recovery_passphrase(passphrase.to_owned().into())
            .map(Self)
            .map_err(|err| CryptoError::new_err(err.to_string()))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    PrivateKey,
    libparsec_types::PrivateKey,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl PrivateKey {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec_crypto::PrivateKey::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    #[classmethod]
    fn generate(_cls: Bound<'_, PyType>) -> Self {
        PrivateKey(libparsec_crypto::PrivateKey::generate())
    }

    #[getter]
    fn public_key(&self) -> PublicKey {
        PublicKey(libparsec_crypto::PrivateKey::public_key(&self.0))
    }

    fn decrypt_from_self<'py>(
        &self,
        py: Python<'py>,
        ciphered: &[u8],
    ) -> PyResult<Bound<'py, PyBytes>> {
        match self.0.decrypt_from_self(ciphered) {
            Ok(v) => Ok(PyBytes::new_bound(py, &v)),
            Err(err) => Err(CryptoError::new_err(err.to_string())),
        }
    }

    fn encode<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.to_bytes())
    }

    fn generate_shared_secret_key(&self, peer_public_key: &PublicKey) -> SecretKey {
        SecretKey(self.0.generate_shared_secret_key(&peer_public_key.0))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    PublicKey,
    libparsec_types::PublicKey,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl PublicKey {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec_crypto::PublicKey::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn encrypt_for_self<'py>(
        &self,
        py: Python<'py>,
        data: PyObject,
    ) -> PyResult<Bound<'py, PyBytes>> {
        let bytes = match data.extract::<&PyByteArray>(py) {
            // SAFETY: Using PyByteArray::as_bytes is safe as long as the corresponding memory is not modified.
            // Here, the GIL is held during the entire access to `bytes` so there is no risk of another
            // python thread modifying the bytearray behind our back.
            Ok(x) => unsafe { x.as_bytes() },
            Err(_) => data.extract::<&PyBytes>(py)?.as_bytes(),
        };
        Ok(PyBytes::new_bound(py, &self.0.encrypt_for_self(bytes)))
    }

    fn encode<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, self.0.as_ref())
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SequesterPrivateKeyDer,
    libparsec_types::SequesterPrivateKeyDer,
    __repr__,
    __copy__,
    __deepcopy__,
);

#[pymethods]
impl SequesterPrivateKeyDer {
    #[new]
    pub fn new(data: &[u8]) -> PyResult<Self> {
        libparsec_crypto::SequesterPrivateKeyDer::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    // TODO: Remove once it's no longer used in python test
    #[classmethod]
    fn generate_pair(
        _cls: Bound<'_, PyType>,
        size_in_bits: usize,
    ) -> PyResult<(Self, SequesterPublicKeyDer)> {
        let (priv_key, pub_key) =
            libparsec_crypto::SequesterPrivateKeyDer::generate_pair(match size_in_bits {
                1024 => libparsec_crypto::SequesterKeySize::_1024Bits,
                2048 => libparsec_crypto::SequesterKeySize::_2048Bits,
                3072 => libparsec_crypto::SequesterKeySize::_3072Bits,
                4096 => libparsec_crypto::SequesterKeySize::_4096Bits,
                _ => {
                    return Err(PyValueError::new_err(
                        "Invalid argument: size_in_bits must be equal to 1024 | 2048 | 3072 | 4096",
                    ))
                }
            });
        Ok((Self(priv_key), SequesterPublicKeyDer(pub_key)))
    }

    fn dump<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.dump())
    }

    fn dump_pem(&self) -> String {
        self.0.dump_pem().to_string()
    }

    #[classmethod]
    fn load_pem(_cls: Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        libparsec_crypto::SequesterPrivateKeyDer::load_pem(s)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn decrypt<'py>(&self, py: Python<'py>, data: &[u8]) -> PyResult<Bound<'py, PyBytes>> {
        self.0
            .decrypt(data)
            .map(|x| PyBytes::new_bound(py, &x))
            .map_err(|err| CryptoError::new_err(err.to_string()))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SequesterPublicKeyDer,
    libparsec_types::SequesterPublicKeyDer,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl SequesterPublicKeyDer {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec_crypto::SequesterPublicKeyDer::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn dump<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.dump())
    }

    fn dump_pem(&self) -> String {
        self.0.dump_pem()
    }

    #[classmethod]
    fn load_pem(_cls: Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        libparsec_crypto::SequesterPublicKeyDer::load_pem(s)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn encrypt<'py>(&self, py: Python<'py>, data: &[u8]) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.encrypt(data))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SequesterSigningKeyDer,
    libparsec_types::SequesterSigningKeyDer,
    __repr__,
    __copy__,
    __deepcopy__,
);

#[pymethods]
impl SequesterSigningKeyDer {
    #[new]
    pub fn new(data: &[u8]) -> PyResult<Self> {
        libparsec_crypto::SequesterSigningKeyDer::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    // TODO: Remove once it's no longer used in python test
    #[classmethod]
    fn generate_pair(
        _cls: Bound<'_, PyType>,
        size_in_bits: usize,
    ) -> PyResult<(Self, SequesterVerifyKeyDer)> {
        let (priv_key, pub_key) =
            libparsec_crypto::SequesterSigningKeyDer::generate_pair(match size_in_bits {
                1024 => libparsec_crypto::SequesterKeySize::_1024Bits,
                2048 => libparsec_crypto::SequesterKeySize::_2048Bits,
                3072 => libparsec_crypto::SequesterKeySize::_3072Bits,
                4096 => libparsec_crypto::SequesterKeySize::_4096Bits,
                _ => {
                    return Err(PyValueError::new_err(
                        "Invalid argument: size_in_bits must be equal to 1024 | 2048 | 3072 | 4096",
                    ))
                }
            });
        Ok((Self(priv_key), SequesterVerifyKeyDer(pub_key)))
    }

    fn dump<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.dump())
    }

    fn dump_pem(&self) -> String {
        self.0.dump_pem().to_string()
    }

    #[classmethod]
    fn load_pem(_cls: Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        libparsec_crypto::SequesterSigningKeyDer::load_pem(s)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn sign<'py>(&self, py: Python<'py>, data: &[u8]) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.sign(data))
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SequesterVerifyKeyDer,
    libparsec_types::SequesterVerifyKeyDer,
    __repr__,
    __copy__,
    __deepcopy__,
);

#[pymethods]
impl SequesterVerifyKeyDer {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        libparsec_crypto::SequesterVerifyKeyDer::try_from(data)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn dump<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.0.dump())
    }

    fn dump_pem(&self) -> String {
        self.0.dump_pem()
    }

    #[classmethod]
    fn load_pem(_cls: Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        libparsec_crypto::SequesterVerifyKeyDer::load_pem(s)
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn verify<'py>(&self, py: Python<'py>, data: &[u8]) -> PyResult<Bound<'py, PyBytes>> {
        self.0
            .verify(data)
            .map(|x| PyBytes::new_bound(py, &x))
            .map_err(|err| CryptoError::new_err(err.to_string()))
    }
}

#[pyfunction]
pub(crate) fn generate_nonce(py: Python) -> Bound<'_, PyBytes> {
    PyBytes::new_bound(py, &libparsec_crypto::generate_nonce())
}

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

#[pymethods]
impl PasswordAlgorithm {
    #[classmethod]
    fn generate_argon2id<'py>(
        _cls: Bound<'py, PyType>,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        Self::convert(py, libparsec_types::PasswordAlgorithm::generate_argon2id())
    }

    #[classmethod]
    fn generate_fake_from_seed<'py>(
        _cls: Bound<'py, PyType>,
        py: Python<'py>,
        email: &str,
        seed: SecretKey,
    ) -> PyResult<Bound<'py, PyAny>> {
        Self::convert(
            py,
            libparsec_types::PasswordAlgorithm::generate_fake_from_seed(email, &seed.0),
        )
    }

    fn compute_secret_key(&self, password: String) -> PyResult<SecretKey> {
        self.0
            .compute_secret_key(&password.into())
            .map(SecretKey)
            .map_err(|err| CryptoError::new_err(err.to_string()))
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
        (
            Self {},
            PasswordAlgorithm(libparsec_types::PasswordAlgorithm::Argon2id {
                salt: salt.into(),
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
