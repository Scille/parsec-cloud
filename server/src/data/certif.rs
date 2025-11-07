// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
    sync::Arc,
};

use pyo3::{
    exceptions::{PyAttributeError, PyValueError},
    prelude::*,
    types::{PyBytes, PyDict, PySet, PyType},
    Bound, IntoPyObjectExt,
};

use libparsec_types::{CertificateSigner, IndexInt, UnsecureSkipValidationReason};

use crate::{
    crypto::{PublicKey, SequesterPublicKeyDer, SequesterVerifyKeyDer, SigningKey, VerifyKey},
    enumerate::{DevicePurpose, RealmRole, UserProfile},
    ids::{DeviceID, DeviceLabel, HumanHandle, SequesterServiceID, UserID, VlobID},
    time::DateTime,
};

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    PrivateKeyAlgorithm,
    libparsec_types::PrivateKeyAlgorithm,
    [
        "X25519_XSALSA20_POLY1305",
        x25519_xsalsa20_poly1305,
        libparsec_types::PrivateKeyAlgorithm::X25519XSalsa20Poly1305
    ],
);

crate::binding_utils::gen_py_wrapper_class!(
    UserCertificate,
    Arc<libparsec_types::UserCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl UserCertificate {
    #[new]
    #[pyo3(signature = (author, timestamp, user_id, human_handle, public_key, algorithm, profile))]
    fn new(
        author: Option<DeviceID>,
        timestamp: DateTime,
        user_id: UserID,
        human_handle: Option<HumanHandle>,
        public_key: PublicKey,
        algorithm: PrivateKeyAlgorithm,
        profile: UserProfile,
    ) -> PyResult<Self> {
        let human_handle = match human_handle {
            Some(human_handle) => libparsec_types::MaybeRedacted::Real(human_handle.0),
            None => libparsec_types::MaybeRedacted::Redacted(
                libparsec_types::HumanHandle::new_redacted(user_id.0),
            ),
        };
        Ok(Self(Arc::new(libparsec_types::UserCertificate {
            author: match author {
                Some(device_id) => CertificateSigner::User(device_id.0),
                None => CertificateSigner::Root,
            },
            timestamp: timestamp.0,
            user_id: user_id.0,
            human_handle,
            public_key: public_key.0,
            algorithm: algorithm.0,
            profile: profile.0,
        })))
    }

    #[classmethod]
    #[pyo3(signature = (signed, author_verify_key, expected_author=None, expected_user=None, expected_human_handle=None))]
    fn verify_and_load(
        _cls: &Bound<'_, PyType>,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: Option<DeviceID>,
        expected_user: Option<UserID>,
        expected_human_handle: Option<&HumanHandle>,
    ) -> PyResult<Self> {
        libparsec_types::UserCertificate::verify_and_load(
            signed,
            &author_verify_key.0,
            match expected_author {
                Some(device_id) => CertificateSigner::User(device_id.0),
                None => CertificateSigner::Root,
            },
            expected_user.map(|x| x.0),
            expected_human_handle.map(|x| &x.0),
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(|x| Self(Arc::new(x)))
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &Bound<'_, PyType>, signed: &[u8]) -> PyResult<Self> {
        libparsec_types::UserCertificate::unsecure_load(signed.to_vec().into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))
    }

    fn redacted_compare(&self, redacted: UserCertificate) -> bool {
        let libparsec_types::UserCertificate {
            author,
            timestamp,
            user_id,
            public_key,
            profile,
            algorithm,
            human_handle: _,
        } = &*self.0;

        let libparsec_types::UserCertificate {
            author: redacted_author,
            timestamp: redacted_timestamp,
            user_id: redacted_user_id,
            public_key: redacted_public_key,
            algorithm: redacted_algorithm,
            profile: redacted_profile,
            human_handle: _,
        } = &*redacted.0;

        author == redacted_author
            && timestamp == redacted_timestamp
            && user_id == redacted_user_id
            && public_key == redacted_public_key
            && algorithm == redacted_algorithm
            && profile == redacted_profile
    }

    #[getter]
    fn author(&self) -> Option<DeviceID> {
        match &self.0.author {
            CertificateSigner::Root => None,
            CertificateSigner::User(device_id) => Some(DeviceID(*device_id)),
        }
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        self.0.timestamp.into()
    }

    #[getter]
    fn user_id(&self) -> UserID {
        self.0.user_id.into()
    }

    #[getter]
    fn human_handle(&self) -> HumanHandle {
        self.0.human_handle.as_ref().to_owned().into()
    }

    #[getter]
    fn is_redacted(&self) -> bool {
        matches!(
            self.0.human_handle,
            libparsec_types::MaybeRedacted::Redacted(_)
        )
    }

    #[getter]
    fn public_key(&self) -> PublicKey {
        self.0.public_key.clone().into()
    }

    #[getter]
    fn algorithm(&self) -> &'static PyObject {
        PrivateKeyAlgorithm::convert(self.0.algorithm)
    }

    #[getter]
    fn profile(&self) -> &'static PyObject {
        UserProfile::convert(self.0.profile)
    }
}

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    SigningKeyAlgorithm,
    libparsec_types::SigningKeyAlgorithm,
    [
        "ED25519",
        ed25519,
        libparsec_types::SigningKeyAlgorithm::Ed25519
    ],
);

crate::binding_utils::gen_py_wrapper_class!(
    DeviceCertificate,
    Arc<libparsec_types::DeviceCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl DeviceCertificate {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (author, timestamp, purpose, user_id, device_id, device_label, verify_key, algorithm))]
    fn new(
        author: Option<DeviceID>,
        timestamp: DateTime,
        purpose: DevicePurpose,
        user_id: UserID,
        device_id: DeviceID,
        device_label: Option<DeviceLabel>,
        verify_key: VerifyKey,
        algorithm: SigningKeyAlgorithm,
    ) -> PyResult<Self> {
        let device_label = match device_label {
            Some(device_label) => libparsec_types::MaybeRedacted::Real(device_label.0),
            None => libparsec_types::MaybeRedacted::Redacted(
                libparsec_types::DeviceLabel::new_redacted(device_id.0),
            ),
        };
        Ok(Self(Arc::new(libparsec_types::DeviceCertificate {
            author: match author {
                Some(device_id) => CertificateSigner::User(device_id.0),
                None => CertificateSigner::Root,
            },
            timestamp: timestamp.0,
            purpose: purpose.0,
            user_id: user_id.0,
            device_id: device_id.0,
            device_label,
            verify_key: verify_key.0,
            algorithm: algorithm.0,
        })))
    }

    #[classmethod]
    #[pyo3(signature = (signed, author_verify_key, expected_author=None, expected_device=None))]
    fn verify_and_load(
        _cls: &Bound<'_, PyType>,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: Option<DeviceID>,
        expected_device: Option<DeviceID>,
    ) -> PyResult<Self> {
        libparsec_types::DeviceCertificate::verify_and_load(
            signed,
            &author_verify_key.0,
            match &expected_author {
                Some(device_id) => CertificateSigner::User(device_id.0),
                None => CertificateSigner::Root,
            },
            expected_device.map(|x| x.0),
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(|x| Self(Arc::new(x)))
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &Bound<'_, PyType>, signed: &[u8]) -> PyResult<Self> {
        libparsec_types::DeviceCertificate::unsecure_load(signed.to_vec().into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))
    }

    fn redacted_compare(&self, redacted: DeviceCertificate) -> bool {
        let libparsec_types::DeviceCertificate {
            author,
            timestamp,
            purpose,
            user_id,
            device_id,
            verify_key,
            algorithm,
            device_label: _,
        } = &*self.0;

        let libparsec_types::DeviceCertificate {
            author: redacted_author,
            timestamp: redacted_timestamp,
            purpose: redacted_purpose,
            user_id: redacted_user_id,
            device_id: redacted_device_id,
            verify_key: redacted_verify_key,
            algorithm: redacted_algorithm,
            device_label: _,
        } = &*redacted.0;

        author == redacted_author
            && timestamp == redacted_timestamp
            && purpose == redacted_purpose
            && device_id == redacted_device_id
            && user_id == redacted_user_id
            && verify_key == redacted_verify_key
            && algorithm == redacted_algorithm
    }

    #[getter]
    fn author(&self) -> Option<DeviceID> {
        match &self.0.author {
            CertificateSigner::Root => None,
            CertificateSigner::User(device_id) => Some(DeviceID(*device_id)),
        }
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        DateTime(self.0.timestamp)
    }

    #[getter]
    fn purpose(&self) -> DevicePurpose {
        DevicePurpose(self.0.purpose)
    }

    #[getter]
    fn user_id(&self) -> UserID {
        UserID(self.0.user_id)
    }

    #[getter]
    fn device_id(&self) -> DeviceID {
        DeviceID(self.0.device_id)
    }

    #[getter]
    fn device_label(&self) -> DeviceLabel {
        self.0.device_label.as_ref().to_owned().into()
    }

    #[getter]
    fn is_redacted(&self) -> bool {
        matches!(
            self.0.device_label,
            libparsec_types::MaybeRedacted::Redacted(_)
        )
    }

    #[getter]
    fn verify_key(&self) -> VerifyKey {
        VerifyKey(self.0.verify_key.clone())
    }

    #[getter]
    fn algorithm(&self) -> &'static PyObject {
        SigningKeyAlgorithm::convert(self.0.algorithm)
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    RevokedUserCertificate,
    Arc<libparsec_types::RevokedUserCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl RevokedUserCertificate {
    #[new]
    #[pyo3(signature = (author, timestamp, user_id))]
    fn new(author: DeviceID, timestamp: DateTime, user_id: UserID) -> PyResult<Self> {
        Ok(Self(Arc::new(libparsec_types::RevokedUserCertificate {
            author: author.0,
            timestamp: timestamp.0,
            user_id: user_id.0,
        })))
    }

    #[classmethod]
    #[pyo3(signature = (signed, author_verify_key, expected_author, expected_user=None))]
    fn verify_and_load(
        _cls: &Bound<'_, PyType>,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_user: Option<UserID>,
    ) -> PyResult<Self> {
        libparsec_types::RevokedUserCertificate::verify_and_load(
            signed,
            &author_verify_key.0,
            expected_author.0,
            expected_user.map(|x| x.0),
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(|x| Self(Arc::new(x)))
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &Bound<'_, PyType>, signed: &[u8]) -> PyResult<Self> {
        libparsec_types::RevokedUserCertificate::unsecure_load(signed.to_vec().into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))
    }

    #[getter]
    fn author(&self) -> DeviceID {
        DeviceID(self.0.author)
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        DateTime(self.0.timestamp)
    }

    #[getter]
    fn user_id(&self) -> UserID {
        UserID(self.0.user_id)
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    UserUpdateCertificate,
    Arc<libparsec_types::UserUpdateCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl UserUpdateCertificate {
    #[new]
    #[pyo3(signature = (author, timestamp, user_id, new_profile))]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        user_id: UserID,
        new_profile: UserProfile,
    ) -> PyResult<Self> {
        Ok(Self(Arc::new(libparsec_types::UserUpdateCertificate {
            author: author.0,
            timestamp: timestamp.0,
            user_id: user_id.0,
            new_profile: new_profile.0,
        })))
    }

    #[classmethod]
    #[pyo3(signature = (signed, author_verify_key, expected_author, expected_user=None))]
    fn verify_and_load(
        _cls: &Bound<'_, PyType>,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_user: Option<UserID>,
    ) -> PyResult<Self> {
        libparsec_types::UserUpdateCertificate::verify_and_load(
            signed,
            &author_verify_key.0,
            expected_author.0,
            expected_user.map(|x| x.0),
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(|x| Self(Arc::new(x)))
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &Bound<'_, PyType>, signed: &[u8]) -> PyResult<Self> {
        libparsec_types::UserUpdateCertificate::unsecure_load(signed.to_vec().into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))
    }

    #[getter]
    fn author(&self) -> DeviceID {
        self.0.author.into()
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        self.0.timestamp.into()
    }

    #[getter]
    fn user_id(&self) -> UserID {
        self.0.user_id.into()
    }

    #[getter]
    fn new_profile(&self) -> &'static PyObject {
        UserProfile::convert(self.0.new_profile)
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    RealmRoleCertificate,
    Arc<libparsec_types::RealmRoleCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl RealmRoleCertificate {
    #[new]
    #[pyo3(signature = (author, timestamp, realm_id, user_id, role))]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        realm_id: VlobID,
        user_id: UserID,
        role: Option<RealmRole>,
    ) -> PyResult<Self> {
        Ok(Self(Arc::new(libparsec_types::RealmRoleCertificate {
            timestamp: timestamp.0,
            author: author.0,
            realm_id: realm_id.0,
            user_id: user_id.0,
            role: role.map(|x| x.0),
        })))
    }

    #[classmethod]
    #[pyo3(signature = (signed, author_verify_key, expected_author, expected_realm=None, expected_user=None))]
    fn verify_and_load(
        _cls: &Bound<'_, PyType>,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_realm: Option<VlobID>,
        expected_user: Option<UserID>,
    ) -> PyResult<Self> {
        libparsec_types::RealmRoleCertificate::verify_and_load(
            signed,
            &author_verify_key.0,
            expected_author.0,
            expected_realm.map(|x| x.0),
            expected_user.map(|x| x.0),
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(|x| Self(Arc::new(x)))
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &Bound<'_, PyType>, signed: &[u8]) -> PyResult<Self> {
        libparsec_types::RealmRoleCertificate::unsecure_load(signed.to_vec().into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))
    }

    #[getter]
    fn author(&self) -> DeviceID {
        self.0.author.into()
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        self.0.timestamp.into()
    }

    #[getter]
    fn realm_id(&self) -> VlobID {
        self.0.realm_id.into()
    }

    #[getter]
    fn user_id(&self) -> UserID {
        self.0.user_id.into()
    }

    #[getter]
    fn role(&self) -> Option<&'static PyObject> {
        self.0.role.map(RealmRole::convert)
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    RealmNameCertificate,
    Arc<libparsec_types::RealmNameCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl RealmNameCertificate {
    #[new]
    #[pyo3(signature = (author, timestamp, realm_id, key_index, encrypted_name))]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        realm_id: VlobID,
        key_index: IndexInt,
        encrypted_name: Vec<u8>,
    ) -> PyResult<Self> {
        Ok(Self(Arc::new(libparsec_types::RealmNameCertificate {
            timestamp: timestamp.0,
            author: author.0,
            realm_id: realm_id.0,
            key_index,
            encrypted_name,
        })))
    }

    #[classmethod]
    #[pyo3(signature = (signed, author_verify_key, expected_author, expected_realm=None))]
    fn verify_and_load(
        _cls: &Bound<'_, PyType>,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_realm: Option<VlobID>,
    ) -> PyResult<Self> {
        libparsec_types::RealmNameCertificate::verify_and_load(
            signed,
            &author_verify_key.0,
            expected_author.0,
            expected_realm.map(|x| x.0),
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(|x| Self(Arc::new(x)))
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &Bound<'_, PyType>, signed: &[u8]) -> PyResult<Self> {
        libparsec_types::RealmNameCertificate::unsecure_load(signed.to_vec().into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))
    }

    #[getter]
    fn author(&self) -> DeviceID {
        DeviceID(self.0.author)
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        self.0.timestamp.into()
    }

    #[getter]
    fn realm_id(&self) -> VlobID {
        self.0.realm_id.into()
    }

    #[getter]
    fn key_index(&self) -> IndexInt {
        self.0.key_index
    }

    #[getter]
    fn encrypted_name<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.encrypted_name)
    }
}

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    SecretKeyAlgorithm,
    libparsec_types::SecretKeyAlgorithm,
    [
        "BLAKE2B_XSALSA20_POLY1305",
        blake2b_xsalsa20_poly1305,
        libparsec_types::SecretKeyAlgorithm::Blake2bXsalsa20Poly1305
    ],
);

crate::binding_utils::gen_py_wrapper_class_for_enum!(
    HashAlgorithm,
    libparsec_types::HashAlgorithm,
    ["SHA256", sha256, libparsec_types::HashAlgorithm::Sha256],
);

crate::binding_utils::gen_py_wrapper_class!(
    RealmKeyRotationCertificate,
    Arc<libparsec_types::RealmKeyRotationCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl RealmKeyRotationCertificate {
    #[new]
    #[pyo3(signature = (author, timestamp, realm_id, key_index, encryption_algorithm, hash_algorithm, key_canary))]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        realm_id: VlobID,
        key_index: IndexInt,
        encryption_algorithm: SecretKeyAlgorithm,
        hash_algorithm: HashAlgorithm,
        key_canary: Vec<u8>,
    ) -> PyResult<Self> {
        Ok(Self(Arc::new(
            libparsec_types::RealmKeyRotationCertificate {
                timestamp: timestamp.0,
                author: author.0,
                realm_id: realm_id.0,
                key_index,
                encryption_algorithm: encryption_algorithm.0,
                hash_algorithm: hash_algorithm.0,
                key_canary,
            },
        )))
    }

    #[classmethod]
    #[pyo3(signature = (signed, author_verify_key, expected_author, expected_realm=None))]
    fn verify_and_load(
        _cls: &Bound<'_, PyType>,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_realm: Option<VlobID>,
    ) -> PyResult<Self> {
        libparsec_types::RealmKeyRotationCertificate::verify_and_load(
            signed,
            &author_verify_key.0,
            expected_author.0,
            expected_realm.map(|x| x.0),
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(|x| Self(Arc::new(x)))
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &Bound<'_, PyType>, signed: &[u8]) -> PyResult<Self> {
        libparsec_types::RealmKeyRotationCertificate::unsecure_load(signed.to_vec().into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))
    }

    #[getter]
    fn author(&self) -> DeviceID {
        DeviceID(self.0.author)
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        self.0.timestamp.into()
    }

    #[getter]
    fn realm_id(&self) -> VlobID {
        self.0.realm_id.into()
    }

    #[getter]
    fn key_index(&self) -> IndexInt {
        self.0.key_index
    }

    #[getter]
    fn encryption_algorithm(&self) -> &'static PyObject {
        SecretKeyAlgorithm::convert(self.0.encryption_algorithm)
    }

    #[getter]
    fn hash_algorithm(&self) -> &'static PyObject {
        HashAlgorithm::convert(self.0.hash_algorithm)
    }

    #[getter]
    fn key_canary<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.key_canary)
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    RealmArchivingConfiguration,
    libparsec_types::RealmArchivingConfiguration,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl RealmArchivingConfiguration {
    #[classmethod]
    pub(crate) fn deletion_planned(_cls: &Bound<'_, PyType>, deletion_date: DateTime) -> Self {
        Self(
            libparsec_types::RealmArchivingConfiguration::DeletionPlanned {
                deletion_date: deletion_date.0,
            },
        )
    }

    #[classattr]
    #[pyo3(name = "AVAILABLE")]
    pub(crate) fn available() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    RealmArchivingConfiguration(libparsec_types::RealmArchivingConfiguration::Available)
                        .into_py_any(py)
                        .expect("Cannot create static value for RealmArchivingConfiguration::available")
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "ARCHIVED")]
    pub(crate) fn archived() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    RealmArchivingConfiguration(libparsec_types::RealmArchivingConfiguration::Archived)
                        .into_py_any(py)
                        .expect("Cannot create static value for RealmArchivingConfiguration::archived")
                })
            };
        };

        &VALUE
    }

    #[getter]
    fn deletion_date(&self) -> PyResult<DateTime> {
        match self.0 {
            libparsec_types::RealmArchivingConfiguration::DeletionPlanned { deletion_date } => {
                Ok(deletion_date.into())
            }
            _ => Err(PyAttributeError::new_err(
                "`deletion_data` only available for DELETION_PLANNED",
            )),
        }
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    RealmArchivingCertificate,
    Arc<libparsec_types::RealmArchivingCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl RealmArchivingCertificate {
    #[new]
    #[pyo3(signature = (author, timestamp, realm_id, configuration))]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        realm_id: VlobID,
        configuration: RealmArchivingConfiguration,
    ) -> PyResult<Self> {
        Ok(Self(Arc::new(libparsec_types::RealmArchivingCertificate {
            timestamp: timestamp.0,
            author: author.0,
            realm_id: realm_id.0,
            configuration: configuration.0,
        })))
    }

    #[classmethod]
    #[pyo3(signature = (signed, author_verify_key, expected_author, expected_realm=None))]
    fn verify_and_load(
        _cls: &Bound<'_, PyType>,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_realm: Option<VlobID>,
    ) -> PyResult<Self> {
        libparsec_types::RealmArchivingCertificate::verify_and_load(
            signed,
            &author_verify_key.0,
            expected_author.0,
            expected_realm.map(|x| x.0),
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(|x| Self(Arc::new(x)))
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &Bound<'_, PyType>, signed: &[u8]) -> PyResult<Self> {
        libparsec_types::RealmArchivingCertificate::unsecure_load(signed.to_vec().into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))
    }

    #[getter]
    fn author(&self) -> DeviceID {
        DeviceID(self.0.author)
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        self.0.timestamp.into()
    }

    #[getter]
    fn realm_id(&self) -> VlobID {
        self.0.realm_id.into()
    }

    #[getter]
    fn configuration(&self) -> RealmArchivingConfiguration {
        self.0.configuration.clone().into()
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    ShamirRecoveryBriefCertificate,
    Arc<libparsec_types::ShamirRecoveryBriefCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl ShamirRecoveryBriefCertificate {
    #[new]
    #[pyo3(signature = (author, timestamp, user_id, threshold, per_recipient_shares))]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        user_id: UserID,
        threshold: NonZeroU8,
        per_recipient_shares: Bound<'_, PyDict>,
    ) -> PyResult<Self> {
        Ok(Self(Arc::new(
            libparsec_types::ShamirRecoveryBriefCertificate {
                timestamp: timestamp.0,
                author: author.0,
                user_id: user_id.0,
                threshold,
                per_recipient_shares: {
                    let mut hash_map = HashMap::with_capacity(per_recipient_shares.len());
                    for (py_key_any, py_value_any) in &per_recipient_shares {
                        let py_key = py_key_any.downcast::<UserID>()?;
                        let py_value: NonZeroU8 = py_value_any.extract()?;
                        hash_map.insert(py_key.borrow().0, py_value);
                    }
                    hash_map
                },
            },
        )))
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &Bound<'_, PyType>,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
    ) -> PyResult<Self> {
        libparsec_types::ShamirRecoveryBriefCertificate::verify_and_load(
            signed,
            &author_verify_key.0,
            expected_author.0,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(|x| Self(Arc::new(x)))
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &Bound<'_, PyType>, signed: &[u8]) -> PyResult<Self> {
        libparsec_types::ShamirRecoveryBriefCertificate::unsecure_load(signed.to_vec().into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))
    }

    #[getter]
    fn author(&self) -> DeviceID {
        DeviceID(self.0.author)
    }

    #[getter]
    fn user_id(&self) -> UserID {
        UserID(self.0.user_id)
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        self.0.timestamp.into()
    }

    #[getter]
    fn threshold(&self) -> NonZeroU8 {
        self.0.threshold
    }

    #[getter]
    fn per_recipient_shares<'py>(&self, py: Python<'py>) -> Bound<'py, PyDict> {
        let d = PyDict::new(py);

        for (k, v) in &self.0.per_recipient_shares {
            let py_k = UserID(*k);
            let py_v = *v;
            let _ = d.set_item(py_k, py_v);
        }

        d
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    ShamirRecoveryShareCertificate,
    Arc<libparsec_types::ShamirRecoveryShareCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl ShamirRecoveryShareCertificate {
    #[new]
    #[pyo3(signature = (author, timestamp, user_id, recipient, ciphered_share))]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        user_id: UserID,
        recipient: UserID,
        ciphered_share: Vec<u8>,
    ) -> PyResult<Self> {
        Ok(Self(Arc::new(
            libparsec_types::ShamirRecoveryShareCertificate {
                timestamp: timestamp.0,
                author: author.0,
                user_id: user_id.0,
                recipient: recipient.0,
                ciphered_share,
            },
        )))
    }

    #[classmethod]
    #[pyo3(signature = (signed, author_verify_key, expected_author, expected_recipient=None))]
    fn verify_and_load(
        _cls: &Bound<'_, PyType>,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_recipient: Option<UserID>,
    ) -> PyResult<Self> {
        libparsec_types::ShamirRecoveryShareCertificate::verify_and_load(
            signed,
            &author_verify_key.0,
            expected_author.0,
            expected_recipient.map(|x| x.0),
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(|x| Self(Arc::new(x)))
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &Bound<'_, PyType>, signed: &[u8]) -> PyResult<Self> {
        libparsec_types::ShamirRecoveryShareCertificate::unsecure_load(signed.to_vec().into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))
    }

    #[getter]
    fn author(&self) -> DeviceID {
        DeviceID(self.0.author)
    }

    #[getter]
    fn user_id(&self) -> UserID {
        UserID(self.0.user_id)
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        self.0.timestamp.into()
    }

    #[getter]
    fn recipient(&self) -> UserID {
        UserID(self.0.recipient)
    }

    #[getter]
    fn ciphered_share<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.ciphered_share)
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    ShamirRecoveryDeletionCertificate,
    Arc<libparsec_types::ShamirRecoveryDeletionCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl ShamirRecoveryDeletionCertificate {
    #[new]
    #[pyo3(signature = (author, timestamp, setup_to_delete_timestamp, setup_to_delete_user_id, share_recipients))]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        setup_to_delete_timestamp: DateTime,
        setup_to_delete_user_id: UserID,
        share_recipients: Bound<'_, PySet>,
    ) -> PyResult<Self> {
        Ok(Self(Arc::new(
            libparsec_types::ShamirRecoveryDeletionCertificate {
                timestamp: timestamp.0,
                author: author.0,
                setup_to_delete_timestamp: setup_to_delete_timestamp.0,
                setup_to_delete_user_id: setup_to_delete_user_id.0,
                share_recipients: {
                    let mut hash_set = HashSet::with_capacity(share_recipients.len());
                    for py_item_any in &share_recipients {
                        let py_item = py_item_any.downcast::<UserID>()?;
                        hash_set.insert(py_item.borrow().0);
                    }
                    hash_set
                },
            },
        )))
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &Bound<'_, PyType>,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
    ) -> PyResult<Self> {
        libparsec_types::ShamirRecoveryDeletionCertificate::verify_and_load(
            signed,
            &author_verify_key.0,
            expected_author.0,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(|x| Self(Arc::new(x)))
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &Bound<'_, PyType>, signed: &[u8]) -> PyResult<Self> {
        libparsec_types::ShamirRecoveryDeletionCertificate::unsecure_load(signed.to_vec().into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))
    }

    #[getter]
    fn author(&self) -> DeviceID {
        DeviceID(self.0.author)
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        self.0.timestamp.into()
    }

    #[getter]
    fn setup_to_delete_timestamp(&self) -> DateTime {
        self.0.setup_to_delete_timestamp.into()
    }

    #[getter]
    fn setup_to_delete_user_id(&self) -> UserID {
        self.0.setup_to_delete_user_id.into()
    }

    #[getter]
    fn share_recipients<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PySet>> {
        let py_recipients = PySet::empty(py)?;
        for recipient in &self.0.share_recipients {
            py_recipients.add(UserID(*recipient))?;
        }
        Ok(py_recipients)
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SequesterAuthorityCertificate,
    Arc<libparsec_types::SequesterAuthorityCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl SequesterAuthorityCertificate {
    #[new]
    fn new(timestamp: DateTime, verify_key_der: SequesterVerifyKeyDer) -> Self {
        Self(Arc::new(libparsec_types::SequesterAuthorityCertificate {
            timestamp: timestamp.0,
            verify_key_der: verify_key_der.0,
        }))
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &Bound<'_, PyType>,
        signed: &[u8],
        author_verify_key: &VerifyKey,
    ) -> PyResult<Self> {
        libparsec_types::SequesterAuthorityCertificate::verify_and_load(
            signed,
            &author_verify_key.0,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(|x| Self(Arc::new(x)))
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        DateTime(self.0.timestamp)
    }

    #[getter]
    fn verify_key_der(&self) -> SequesterVerifyKeyDer {
        SequesterVerifyKeyDer(self.0.verify_key_der.clone())
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SequesterServiceCertificate,
    Arc<libparsec_types::SequesterServiceCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl SequesterServiceCertificate {
    #[new]
    fn new(
        timestamp: DateTime,
        service_id: SequesterServiceID,
        service_label: String,
        encryption_key_der: SequesterPublicKeyDer,
    ) -> Self {
        Self(Arc::new(libparsec_types::SequesterServiceCertificate {
            timestamp: timestamp.0,
            service_id: service_id.0,
            service_label,
            encryption_key_der: encryption_key_der.0,
        }))
    }

    #[classmethod]
    fn load(_cls: &Bound<'_, PyType>, data: &[u8]) -> PyResult<Self> {
        libparsec_types::SequesterServiceCertificate::load(data)
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|x| Self(Arc::new(x)))
    }

    fn dump<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump())
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        DateTime(self.0.timestamp)
    }

    #[getter]
    fn service_id(&self) -> SequesterServiceID {
        SequesterServiceID(self.0.service_id)
    }

    #[getter]
    fn service_label(&self) -> &str {
        &self.0.service_label
    }

    #[getter]
    fn encryption_key_der(&self) -> SequesterPublicKeyDer {
        SequesterPublicKeyDer(self.0.encryption_key_der.clone())
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SequesterRevokedServiceCertificate,
    Arc<libparsec_types::SequesterRevokedServiceCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl SequesterRevokedServiceCertificate {
    #[new]
    fn new(timestamp: DateTime, service_id: SequesterServiceID) -> Self {
        Self(Arc::new(
            libparsec_types::SequesterRevokedServiceCertificate {
                timestamp: timestamp.0,
                service_id: service_id.0,
            },
        ))
    }

    #[classmethod]
    fn load(_cls: &Bound<'_, PyType>, data: &[u8]) -> PyResult<Self> {
        libparsec_types::SequesterRevokedServiceCertificate::load(data)
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|x| Self(Arc::new(x)))
    }

    fn dump<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.dump())
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        DateTime(self.0.timestamp)
    }

    #[getter]
    fn service_id(&self) -> SequesterServiceID {
        SequesterServiceID(self.0.service_id)
    }
}
