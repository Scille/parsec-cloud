// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use pyo3::{
    prelude::*,
    types::{PyBytes, PyDict, PyType},
};

use libparsec::low_level::types::{
    CertificateSignerOwned, CertificateSignerRef, UnsecureSkipValidationReason,
};

use crate::{
    api_crypto::{PublicKey, SequesterPublicKeyDer, SequesterVerifyKeyDer, SigningKey, VerifyKey},
    data::DataResult,
    enumerate::{RealmRole, UserProfile},
    ids::{DeviceID, DeviceLabel, HumanHandle, RealmID, SequesterServiceID, UserID},
    time::DateTime,
};

crate::binding_utils::gen_py_wrapper_class!(
    UserCertificate,
    Arc<libparsec::low_level::types::UserCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl UserCertificate {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [author: Option<DeviceID>, "author"],
            [timestamp: DateTime, "timestamp"],
            [user_id: UserID, "user_id"],
            [human_handle: Option<HumanHandle>, "human_handle"],
            [public_key: PublicKey, "public_key"],
            [profile: UserProfile, "profile"]
        );

        Ok(Self(Arc::new(
            libparsec::low_level::types::UserCertificate {
                author: match author {
                    Some(device_id) => CertificateSignerOwned::User(device_id.0),
                    None => CertificateSignerOwned::Root,
                },
                timestamp: timestamp.0,
                user_id: user_id.0,
                human_handle: human_handle.map(|human_handle| human_handle.0),
                public_key: public_key.0,
                profile: profile.0,
            },
        )))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: Option<DeviceID>, "author"],
            [timestamp: DateTime, "timestamp"],
            [user_id: UserID, "user_id"],
            [human_handle: Option<HumanHandle>, "human_handle"],
            [public_key: PublicKey, "public_key"],
            [profile: UserProfile, "profile"]
        );

        let mut m = self.0.clone();
        let r = Arc::make_mut(&mut m);

        if let Some(x) = author {
            r.author = match x {
                Some(x) => CertificateSignerOwned::User(x.0),
                None => CertificateSignerOwned::Root,
            }
        }
        if let Some(x) = timestamp {
            r.timestamp = x.0;
        }
        if let Some(x) = user_id {
            r.user_id = x.0;
        }
        if let Some(x) = human_handle {
            r.human_handle = x.map(|x| x.0);
        }
        if let Some(x) = public_key {
            r.public_key = x.0;
        }
        if let Some(x) = profile {
            r.profile = x.0;
        }

        Ok(Self(m))
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &PyType,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: Option<&DeviceID>,
        expected_user: Option<&UserID>,
        expected_human_handle: Option<&HumanHandle>,
    ) -> DataResult<Self> {
        Ok(
            libparsec::low_level::types::UserCertificate::verify_and_load(
                signed,
                &author_verify_key.0,
                match expected_author {
                    Some(device_id) => CertificateSignerRef::User(&device_id.0),
                    None => CertificateSignerRef::Root,
                },
                expected_user.map(|x| &x.0),
                expected_human_handle.map(|x| &x.0),
            )
            .map(|x| Self(Arc::new(x)))?,
        )
    }

    fn dump_and_sign<'py>(&self, author_signkey: &SigningKey, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &PyType, signed: &[u8]) -> DataResult<Self> {
        Ok(
            libparsec::low_level::types::UserCertificate::unsecure_load(signed.to_vec().into())
                .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
                .map(|x| Self(Arc::new(x)))?,
        )
    }

    #[getter]
    fn is_admin(&self) -> bool {
        self.0.profile == libparsec::low_level::types::UserProfile::Admin
    }

    #[getter]
    fn author(&self) -> Option<DeviceID> {
        match &self.0.author {
            CertificateSignerOwned::Root => None,
            CertificateSignerOwned::User(device_id) => Some(DeviceID(device_id.clone())),
        }
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        self.0.timestamp.into()
    }

    #[getter]
    fn user_id(&self) -> UserID {
        self.0.user_id.clone().into()
    }

    #[getter]
    fn human_handle(&self) -> Option<HumanHandle> {
        self.0.human_handle.clone().map(|x| x.into())
    }

    #[getter]
    fn public_key(&self) -> PublicKey {
        self.0.public_key.clone().into()
    }

    #[getter]
    fn profile(&self) -> &'static PyObject {
        UserProfile::convert(self.0.profile)
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    DeviceCertificate,
    Arc<libparsec::low_level::types::DeviceCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl DeviceCertificate {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [author: Option<DeviceID>, "author"],
            [timestamp: DateTime, "timestamp"],
            [device_id: DeviceID, "device_id"],
            [device_label: Option<DeviceLabel>, "device_label"],
            [verify_key: VerifyKey, "verify_key"],
        );

        Ok(Self(Arc::new(
            libparsec::low_level::types::DeviceCertificate {
                author: match author {
                    Some(device_id) => CertificateSignerOwned::User(device_id.0),
                    None => CertificateSignerOwned::Root,
                },
                timestamp: timestamp.0,
                device_id: device_id.0,
                device_label: device_label.map(|x| x.0),
                verify_key: verify_key.0,
            },
        )))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: Option<DeviceID>, "author"],
            [timestamp: DateTime, "timestamp"],
            [device_id: DeviceID, "device_id"],
            [device_label: Option<DeviceLabel>, "device_label"],
            [verify_key: VerifyKey, "verify_key"],
        );

        let mut m = self.0.clone();
        let r = Arc::make_mut(&mut m);

        if let Some(x) = author {
            r.author = match x {
                Some(x) => CertificateSignerOwned::User(x.0),
                None => CertificateSignerOwned::Root,
            }
        }
        if let Some(x) = timestamp {
            r.timestamp = x.0;
        }
        if let Some(x) = device_id {
            r.device_id = x.0;
        }
        if let Some(x) = device_label {
            r.device_label = x.map(|x| x.0);
        }
        if let Some(x) = verify_key {
            r.verify_key = x.0;
        }

        Ok(Self(m))
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &PyType,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: Option<&DeviceID>,
        expected_device: Option<&DeviceID>,
    ) -> DataResult<Self> {
        Ok(
            libparsec::low_level::types::DeviceCertificate::verify_and_load(
                signed,
                &author_verify_key.0,
                match &expected_author {
                    Some(device_id) => CertificateSignerRef::User(&device_id.0),
                    None => CertificateSignerRef::Root,
                },
                expected_device.map(|x| &x.0),
            )
            .map(|x| Self(Arc::new(x)))?,
        )
    }

    fn dump_and_sign<'py>(&self, author_signkey: &SigningKey, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &PyType, signed: &[u8]) -> DataResult<Self> {
        Ok(
            libparsec::low_level::types::DeviceCertificate::unsecure_load(signed.to_vec().into())
                .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
                .map(|x| Self(Arc::new(x)))?,
        )
    }

    #[getter]
    fn author(&self) -> Option<DeviceID> {
        match &self.0.author {
            CertificateSignerOwned::Root => None,
            CertificateSignerOwned::User(device_id) => Some(DeviceID(device_id.clone())),
        }
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        DateTime(self.0.timestamp)
    }

    #[getter]
    fn device_id(&self) -> DeviceID {
        DeviceID(self.0.device_id.clone())
    }

    #[getter]
    fn device_label(&self) -> Option<DeviceLabel> {
        self.0.device_label.clone().map(DeviceLabel)
    }

    #[getter]
    fn verify_key(&self) -> VerifyKey {
        VerifyKey(self.0.verify_key.clone())
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    RevokedUserCertificate,
    Arc<libparsec::low_level::types::RevokedUserCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl RevokedUserCertificate {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [user_id: UserID, "user_id"],
        );

        Ok(Self(Arc::new(
            libparsec::low_level::types::RevokedUserCertificate {
                author: author.0,
                timestamp: timestamp.0,
                user_id: user_id.0,
            },
        )))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [user_id: UserID, "user_id"],
        );

        let mut m = self.0.clone();
        let r = Arc::make_mut(&mut m);

        if let Some(x) = author {
            r.author = x.0
        }
        if let Some(x) = timestamp {
            r.timestamp = x.0;
        }
        if let Some(x) = user_id {
            r.user_id = x.0;
        }

        Ok(Self(m))
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &PyType,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_user: Option<&UserID>,
    ) -> DataResult<Self> {
        Ok(
            libparsec::low_level::types::RevokedUserCertificate::verify_and_load(
                signed,
                &author_verify_key.0,
                &expected_author.0,
                expected_user.map(|x| &x.0),
            )
            .map(|x| Self(Arc::new(x)))?,
        )
    }

    fn dump_and_sign<'py>(&self, author_signkey: &SigningKey, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &PyType, signed: &[u8]) -> DataResult<Self> {
        Ok(
            libparsec::low_level::types::RevokedUserCertificate::unsecure_load(
                signed.to_vec().into(),
            )
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))?,
        )
    }

    #[getter]
    fn author(&self) -> DeviceID {
        DeviceID(self.0.author.clone())
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        DateTime(self.0.timestamp)
    }

    #[getter]
    fn user_id(&self) -> UserID {
        UserID(self.0.user_id.clone())
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    UserUpdateCertificate,
    Arc<libparsec::low_level::types::UserUpdateCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl UserUpdateCertificate {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [user_id: UserID, "user_id"],
            [new_profile: UserProfile, "new_profile"]
        );

        Ok(Self(Arc::new(
            libparsec::low_level::types::UserUpdateCertificate {
                author: author.0,
                timestamp: timestamp.0,
                user_id: user_id.0,
                new_profile: new_profile.0,
            },
        )))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [user_id: UserID, "user_id"],
            [new_profile: UserProfile, "new_profile"]
        );

        let mut m = self.0.clone();
        let r = Arc::make_mut(&mut m);

        if let Some(x) = author {
            r.author = x.0;
        }
        if let Some(x) = timestamp {
            r.timestamp = x.0;
        }
        if let Some(x) = user_id {
            r.user_id = x.0;
        }
        if let Some(x) = new_profile {
            r.new_profile = x.0;
        }

        Ok(Self(m))
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &PyType,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_user: Option<&UserID>,
    ) -> DataResult<Self> {
        Ok(
            libparsec::low_level::types::UserUpdateCertificate::verify_and_load(
                signed,
                &author_verify_key.0,
                &expected_author.0,
                expected_user.map(|x| &x.0),
            )
            .map(|x| Self(Arc::new(x)))?,
        )
    }

    fn dump_and_sign<'py>(&self, author_signkey: &SigningKey, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &PyType, signed: &[u8]) -> DataResult<Self> {
        Ok(
            libparsec::low_level::types::UserUpdateCertificate::unsecure_load(
                signed.to_vec().into(),
            )
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))?,
        )
    }

    #[getter]
    fn author(&self) -> DeviceID {
        self.0.author.clone().into()
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        self.0.timestamp.into()
    }

    #[getter]
    fn user_id(&self) -> UserID {
        self.0.user_id.clone().into()
    }

    #[getter]
    fn new_profile(&self) -> &'static PyObject {
        UserProfile::convert(self.0.new_profile)
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    RealmRoleCertificate,
    Arc<libparsec::low_level::types::RealmRoleCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl RealmRoleCertificate {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [author: Option<DeviceID>, "author"],
            [timestamp: DateTime, "timestamp"],
            [realm_id: RealmID, "realm_id"],
            [user_id: UserID, "user_id"],
            [role: Option<RealmRole>, "role"],
        );

        Ok(Self(Arc::new(
            libparsec::low_level::types::RealmRoleCertificate {
                timestamp: timestamp.0,
                author: match author {
                    Some(device_id) => CertificateSignerOwned::User(device_id.0),
                    None => CertificateSignerOwned::Root,
                },
                realm_id: realm_id.0,
                user_id: user_id.0,
                role: role.map(|x| x.0),
            },
        )))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: Option<DeviceID>, "author"],
            [timestamp: DateTime, "timestamp"],
            [realm_id: RealmID, "realm_id"],
            [user_id: UserID, "user_id"],
            [role: Option<RealmRole>, "role"],
        );

        let mut m = self.0.clone();
        let r = Arc::make_mut(&mut m);

        if let Some(x) = author {
            r.author = match x {
                Some(x) => CertificateSignerOwned::User(x.0),
                None => CertificateSignerOwned::Root,
            }
        }
        if let Some(x) = timestamp {
            r.timestamp = x.0;
        }
        if let Some(x) = realm_id {
            r.realm_id = x.0;
        }
        if let Some(x) = user_id {
            r.user_id = x.0;
        }
        if let Some(x) = role {
            r.role = x.map(|y| y.0);
        }

        Ok(Self(m))
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &PyType,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: Option<&DeviceID>,
        expected_realm: Option<RealmID>,
        expected_user: Option<&UserID>,
    ) -> DataResult<Self> {
        Ok(
            libparsec::low_level::types::RealmRoleCertificate::verify_and_load(
                signed,
                &author_verify_key.0,
                match &expected_author {
                    Some(device_id) => CertificateSignerRef::User(&device_id.0),
                    None => CertificateSignerRef::Root,
                },
                expected_realm.map(|x| x.0),
                expected_user.map(|x| &x.0),
            )
            .map(|x| Self(Arc::new(x)))?,
        )
    }

    fn dump_and_sign<'py>(&self, author_signkey: &SigningKey, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &PyType, signed: &[u8]) -> DataResult<Self> {
        Ok(
            libparsec::low_level::types::RealmRoleCertificate::unsecure_load(
                signed.to_vec().into(),
            )
            .map(|u| u.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage))
            .map(|x| Self(Arc::new(x)))?,
        )
    }

    #[classmethod]
    fn build_realm_root_certif(
        _cls: &PyType,
        author: DeviceID,
        timestamp: DateTime,
        realm_id: RealmID,
    ) -> Self {
        Self(Arc::new(
            libparsec::low_level::types::RealmRoleCertificate {
                user_id: author.0.user_id().clone(),
                author: CertificateSignerOwned::User(author.0),
                timestamp: timestamp.0,
                realm_id: realm_id.0,
                role: Some(libparsec::low_level::types::RealmRole::Owner),
            },
        ))
    }

    #[getter]
    fn author(&self) -> Option<DeviceID> {
        match &self.0.author {
            CertificateSignerOwned::Root => None,
            CertificateSignerOwned::User(device_id) => Some(DeviceID(device_id.clone())),
        }
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        self.0.timestamp.into()
    }

    #[getter]
    fn realm_id(&self) -> RealmID {
        self.0.realm_id.into()
    }

    #[getter]
    fn user_id(&self) -> UserID {
        self.0.user_id.clone().into()
    }

    #[getter]
    fn role(&self) -> Option<&'static PyObject> {
        self.0.role.map(RealmRole::convert)
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    SequesterAuthorityCertificate,
    Arc<libparsec::low_level::types::SequesterAuthorityCertificate>,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl SequesterAuthorityCertificate {
    #[new]
    fn new(timestamp: DateTime, verify_key_der: SequesterVerifyKeyDer) -> Self {
        Self(Arc::new(
            libparsec::low_level::types::SequesterAuthorityCertificate {
                timestamp: timestamp.0,
                verify_key_der: verify_key_der.0,
            },
        ))
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &PyType,
        signed: &[u8],
        author_verify_key: &VerifyKey,
    ) -> DataResult<Self> {
        Ok(
            libparsec::low_level::types::SequesterAuthorityCertificate::verify_and_load(
                signed,
                &author_verify_key.0,
            )
            .map(|x| Self(Arc::new(x)))?,
        )
    }

    fn dump_and_sign<'py>(&self, author_signkey: &SigningKey, py: Python<'py>) -> &'py PyBytes {
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
    Arc<libparsec::low_level::types::SequesterServiceCertificate>,
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
        Self(Arc::new(
            libparsec::low_level::types::SequesterServiceCertificate {
                timestamp: timestamp.0,
                service_id: service_id.0,
                service_label,
                encryption_key_der: encryption_key_der.0,
            },
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, data: &[u8]) -> DataResult<Self> {
        Ok(
            libparsec::low_level::types::SequesterServiceCertificate::load(data)
                .map(|x| Self(Arc::new(x)))?,
        )
    }

    fn dump<'py>(&self, py: Python<'py>) -> &'py PyBytes {
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
