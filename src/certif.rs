// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec::types::{CertificateSignerOwned, CertificateSignerRef, UserProfile};
use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyBytes, PyDict, PyType};

use crate::api_crypto::{PublicKey, SigningKey, VerifyKey};
use crate::binding_utils::{
    py_to_rs_realm_role, py_to_rs_user_profile, rs_to_py_realm_role, rs_to_py_user_profile,
};
use crate::ids::{DeviceID, DeviceLabel, HumanHandle, RealmID, UserID};
use crate::time::DateTime;

import_exception!(parsec.api.data, DataError);

#[pyclass]
pub(crate) struct UserCertificate(pub libparsec::types::UserCertificate);

#[pymethods]
impl UserCertificate {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [author: Option<DeviceID>, "author"],
            [timestamp: DateTime, "timestamp"],
            [user_id: UserID, "user_id"],
            [human_handle: Option<HumanHandle>, "human_handle"],
            [public_key: PublicKey, "public_key"],
            [profile, "profile", py_to_rs_user_profile]
        );

        Ok(Self(libparsec::types::UserCertificate {
            author: match author {
                Some(device_id) => CertificateSignerOwned::User(device_id.0),
                None => CertificateSignerOwned::Root,
            },
            timestamp: timestamp.0,
            user_id: user_id.0,
            human_handle: human_handle.map(|human_handle| human_handle.0),
            public_key: public_key.0,
            profile,
        }))
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
            [profile, "profile", py_to_rs_user_profile]
        );

        let mut r = self.0.clone();

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
            r.profile = x;
        }

        Ok(Self(r))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            _ => unimplemented!(),
        }
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &PyType,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: Option<&DeviceID>,
        expected_user: Option<&UserID>,
        expected_human_handle: Option<&HumanHandle>,
    ) -> PyResult<Self> {
        let r = Self(
            libparsec::types::UserCertificate::verify_and_load(
                signed,
                &author_verify_key.0,
                match expected_author {
                    Some(device_id) => CertificateSignerRef::User(&device_id.0),
                    None => CertificateSignerRef::Root,
                },
            )
            .map_err(|err| DataError::new_err(err.to_string()))?,
        );

        if let Some(expected_user_id) = expected_user {
            if r.0.user_id != expected_user_id.0 {
                return Err(DataError::new_err(format!(
                    "Invalid user ID: expected `{}`, got `{}`",
                    expected_user_id.0, r.0.user_id
                )));
            }
        }
        if let Some(expected_human_handle) = expected_human_handle {
            if r.0.human_handle.as_ref() != Some(&expected_human_handle.0) {
                return Err(DataError::new_err("Invalid HumanHandle"));
            }
        }

        Ok(r)
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0)))
    }

    #[classmethod]
    fn unsecure_load(_cls: &PyType, signed: &[u8]) -> PyResult<Self> {
        Ok(Self(
            libparsec::types::UserCertificate::unsecure_load(signed).map_err(DataError::new_err)?,
        ))
    }

    #[getter]
    fn is_admin(&self) -> PyResult<bool> {
        Ok(self.0.profile == UserProfile::Admin)
    }

    #[getter]
    fn author(&self) -> PyResult<Option<DeviceID>> {
        Ok(match &self.0.author {
            CertificateSignerOwned::Root => None,
            CertificateSignerOwned::User(device_id) => Some(DeviceID(device_id.clone())),
        })
    }

    #[getter]
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.timestamp))
    }

    #[getter]
    fn user_id(&self) -> PyResult<UserID> {
        Ok(UserID(self.0.user_id.clone()))
    }

    #[getter]
    fn human_handle(&self) -> PyResult<Option<HumanHandle>> {
        Ok(self.0.human_handle.clone().map(HumanHandle))
    }

    #[getter]
    fn public_key(&self) -> PyResult<PublicKey> {
        Ok(PublicKey(self.0.public_key.clone()))
    }

    #[getter]
    fn profile(&self) -> PyResult<PyObject> {
        rs_to_py_user_profile(&self.0.profile)
    }
}

#[pyclass]
pub(crate) struct DeviceCertificate(pub libparsec::types::DeviceCertificate);

#[pymethods]
impl DeviceCertificate {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [author: Option<DeviceID>, "author"],
            [timestamp: DateTime, "timestamp"],
            [device_id: DeviceID, "device_id"],
            [device_label: Option<DeviceLabel>, "device_label"],
            [verify_key: VerifyKey, "verify_key"],
        );

        Ok(Self(libparsec::types::DeviceCertificate {
            author: match author {
                Some(device_id) => CertificateSignerOwned::User(device_id.0),
                None => CertificateSignerOwned::Root,
            },
            timestamp: timestamp.0,
            device_id: device_id.0,
            device_label: device_label.map(|x| x.0),
            verify_key: verify_key.0,
        }))
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

        let mut r = self.0.clone();

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

        Ok(Self(r))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            _ => unimplemented!(),
        }
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &PyType,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: Option<&DeviceID>,
        expected_device: Option<&DeviceID>,
    ) -> PyResult<Self> {
        let r = Self(
            libparsec::types::DeviceCertificate::verify_and_load(
                signed,
                &author_verify_key.0,
                match &expected_author {
                    Some(device_id) => CertificateSignerRef::User(&device_id.0),
                    None => CertificateSignerRef::Root,
                },
            )
            .map_err(|err| DataError::new_err(err.to_string()))?,
        );

        if let Some(expected_device_id) = expected_device {
            if r.0.device_id != expected_device_id.0 {
                return Err(DataError::new_err(format!(
                    "Invalid device ID: expected `{}`, got `{}`",
                    expected_device_id.0, r.0.device_id
                )));
            }
        }

        Ok(r)
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0)))
    }

    #[classmethod]
    fn unsecure_load(_cls: &PyType, signed: &[u8]) -> PyResult<Self> {
        Ok(Self(
            libparsec::types::DeviceCertificate::unsecure_load(signed)
                .map_err(DataError::new_err)?,
        ))
    }

    #[getter]
    fn author(&self) -> PyResult<Option<DeviceID>> {
        Ok(match &self.0.author {
            CertificateSignerOwned::Root => None,
            CertificateSignerOwned::User(device_id) => Some(DeviceID(device_id.clone())),
        })
    }

    #[getter]
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.timestamp))
    }

    #[getter]
    fn device_id(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.device_id.clone()))
    }

    #[getter]
    fn device_label(&self) -> PyResult<Option<DeviceLabel>> {
        Ok(self.0.device_label.clone().map(DeviceLabel))
    }

    #[getter]
    fn verify_key(&self) -> PyResult<VerifyKey> {
        Ok(VerifyKey(self.0.verify_key.clone()))
    }
}

#[pyclass]
pub(crate) struct RevokedUserCertificate(pub libparsec::types::RevokedUserCertificate);

#[pymethods]
impl RevokedUserCertificate {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [user_id: UserID, "user_id"],
        );

        Ok(Self(libparsec::types::RevokedUserCertificate {
            author: author.0,
            timestamp: timestamp.0,
            user_id: user_id.0,
        }))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [user_id: UserID, "user_id"],
        );

        let mut r = self.0.clone();

        if let Some(x) = author {
            r.author = x.0
        }
        if let Some(x) = timestamp {
            r.timestamp = x.0;
        }
        if let Some(x) = user_id {
            r.user_id = x.0;
        }

        Ok(Self(r))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            _ => unimplemented!(),
        }
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &PyType,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_user: Option<&UserID>,
    ) -> PyResult<Self> {
        let r = Self(
            libparsec::types::RevokedUserCertificate::verify_and_load(
                signed,
                &author_verify_key.0,
                &expected_author.0,
            )
            .map_err(|err| DataError::new_err(err.to_string()))?,
        );

        if let Some(expected_user_id) = expected_user {
            if r.0.user_id != expected_user_id.0 {
                return Err(DataError::new_err(format!(
                    "Invalid user ID: expected `{}`, got `{}`",
                    expected_user_id.0, r.0.user_id
                )));
            }
        }

        Ok(r)
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0)))
    }

    #[classmethod]
    fn unsecure_load(_cls: &PyType, signed: &[u8]) -> PyResult<Self> {
        Ok(Self(
            libparsec::types::RevokedUserCertificate::unsecure_load(signed)
                .map_err(DataError::new_err)?,
        ))
    }

    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.author.clone()))
    }

    #[getter]
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.timestamp))
    }

    #[getter]
    fn user_id(&self) -> PyResult<UserID> {
        Ok(UserID(self.0.user_id.clone()))
    }
}

#[pyclass]
pub(crate) struct RealmRoleCertificate(pub libparsec::types::RealmRoleCertificate);

#[pymethods]
impl RealmRoleCertificate {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [author: Option<DeviceID>, "author"],
            [timestamp: DateTime, "timestamp"],
            [realm_id: RealmID, "realm_id"],
            [user_id: UserID, "user_id"],
            [role, "role", py_to_rs_realm_role],
        );

        Ok(Self(libparsec::types::RealmRoleCertificate {
            timestamp: timestamp.0,
            author: match author {
                Some(device_id) => CertificateSignerOwned::User(device_id.0),
                None => CertificateSignerOwned::Root,
            },
            realm_id: realm_id.0,
            user_id: user_id.0,
            role,
        }))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: Option<DeviceID>, "author"],
            [timestamp: DateTime, "timestamp"],
            [realm_id: RealmID, "realm_id"],
            [user_id: UserID, "user_id"],
            [role, "role", py_to_rs_realm_role],
        );

        let mut r = self.0.clone();

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
            r.role = x;
        }

        Ok(Self(r))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            _ => unimplemented!(),
        }
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &PyType,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: Option<DeviceID>,
        expected_realm: Option<RealmID>,
        expected_user: Option<UserID>,
        expected_role: Option<&PyAny>,
    ) -> PyResult<Self> {
        let r = Self(
            libparsec::types::RealmRoleCertificate::verify_and_load(
                signed,
                &author_verify_key.0,
                match &expected_author {
                    Some(device_id) => CertificateSignerRef::User(&device_id.0),
                    None => CertificateSignerRef::Root,
                },
            )
            .map_err(|err| DataError::new_err(err.to_string()))?,
        );

        if let Some(expected_realm_id) = expected_realm {
            if r.0.realm_id != expected_realm_id.0 {
                return Err(DataError::new_err(format!(
                    "Invalid realm ID: expected `{}`, got `{}`",
                    expected_realm_id.0, r.0.realm_id
                )));
            }
        }

        if let Some(expected_user_id) = expected_user {
            if r.0.user_id != expected_user_id.0 {
                return Err(DataError::new_err(format!(
                    "Invalid user ID: expected `{}`, got `{}`",
                    expected_user_id.0, r.0.user_id
                )));
            }
        }

        if let Some(expected_role) = expected_role {
            let expected_role = py_to_rs_realm_role(expected_role)?;
            if r.0.role != expected_role {
                return Err(DataError::new_err(format!(
                    "Invalid role: expected `{:?}`, got `{:?}`",
                    expected_role, r.0.role
                )));
            }
        }

        Ok(r)
    }

    fn dump_and_sign<'py>(
        &self,
        author_signkey: &SigningKey,
        py: Python<'py>,
    ) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0)))
    }

    #[classmethod]
    fn unsecure_load(_cls: &PyType, signed: &[u8]) -> PyResult<Self> {
        Ok(Self(
            libparsec::types::RealmRoleCertificate::unsecure_load(signed)
                .map_err(DataError::new_err)?,
        ))
    }

    #[classmethod]
    fn build_realm_root_certif(
        _cls: &PyType,
        author: DeviceID,
        timestamp: DateTime,
        realm_id: RealmID,
    ) -> PyResult<Self> {
        Ok(Self(libparsec::types::RealmRoleCertificate {
            user_id: author.0.user_id.clone(),
            author: CertificateSignerOwned::User(author.0),
            timestamp: timestamp.0,
            realm_id: realm_id.0,
            role: Some(libparsec::types::RealmRole::Owner),
        }))
    }

    #[getter]
    fn author(&self) -> PyResult<Option<DeviceID>> {
        Ok(match &self.0.author {
            CertificateSignerOwned::Root => None,
            CertificateSignerOwned::User(device_id) => Some(DeviceID(device_id.clone())),
        })
    }

    #[getter]
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.timestamp))
    }

    #[getter]
    fn realm_id(&self) -> PyResult<RealmID> {
        Ok(RealmID(self.0.realm_id))
    }

    #[getter]
    fn user_id(&self) -> PyResult<UserID> {
        Ok(UserID(self.0.user_id.clone()))
    }

    #[getter]
    fn role(&self) -> PyResult<Option<Py<PyAny>>> {
        self.0.role.map(|x| rs_to_py_realm_role(&x)).transpose()
    }
}
