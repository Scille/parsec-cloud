// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use crate::{
    api_crypto::VerifyKey,
    binding_utils::{gen_proto, to_trustchain_error_exception},
    certif::{DeviceCertificate, RevokedUserCertificate, UserCertificate},
    ids::{self, DeviceID, UserID},
    protocol::Trustchain,
    time::{self, DateTime, TimeProvider},
};

use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyType};
use pyo3::{
    create_exception,
    exceptions::{PyAttributeError, PyException, PyValueError},
};

#[pyclass]
pub(crate) struct TrustchainContext(pub libparsec::core::TrustchainContext);

#[pymethods]
impl TrustchainContext {
    #[new]
    fn new(
        root_verify_key: &VerifyKey,
        time_provider: TimeProvider,
        cache_validity: i64,
    ) -> PyResult<Self> {
        Ok(Self(libparsec::core::TrustchainContext::new(
            root_verify_key.0.clone(),
            time_provider.0,
            cache_validity,
        )))
    }

    #[getter]
    fn cache_validity(&self) -> PyResult<i64> {
        Ok(self.0.cache_validity())
    }

    fn invalidate_user_cache(&mut self, user_id: &UserID) -> PyResult<()> {
        self.0.invalidate_user_cache(&user_id.0);
        Ok(())
    }

    fn get_user(&self, user_id: &UserID) -> PyResult<Option<UserCertificate>> {
        Ok(self.0.get_user(&user_id.0).cloned().map(UserCertificate))
    }

    fn get_revoked_user(&self, user_id: &UserID) -> PyResult<Option<RevokedUserCertificate>> {
        Ok(self
            .0
            .get_revoked_user(&user_id.0)
            .cloned()
            .map(RevokedUserCertificate))
    }

    fn get_device(&self, device_id: &DeviceID) -> PyResult<Option<DeviceCertificate>> {
        Ok(self
            .0
            .get_device(&device_id.0)
            .cloned()
            .map(DeviceCertificate))
    }

    fn load_trustchain<'py>(
        &mut self,
        users: Vec<Vec<u8>>,
        revoked_users: Vec<Vec<u8>>,
        devices: Vec<Vec<u8>>,
        py: Python<'py>,
    ) -> PyResult<(&'py PyTuple, &'py PyTuple, &'py PyTuple)> {
        let (users, revoked_users, devices) = self
            .0
            .load_trustchain(&users, &revoked_users, &devices)
            .map_err(to_trustchain_error_exception)?;

        let users = users.into_iter().map(|x| UserCertificate(x).into_py(py));
        let revoked_users = revoked_users
            .into_iter()
            .map(|x| RevokedUserCertificate(x).into_py(py));
        let devices = devices
            .into_iter()
            .map(|x| DeviceCertificate(x).into_py(py));

        Ok((
            PyTuple::new(py, users),
            PyTuple::new(py, revoked_users),
            PyTuple::new(py, devices),
        ))
    }

    fn load_user_and_devices<'py>(
        &mut self,
        trustchain: Trustchain,
        user_certif: Vec<u8>,
        revoked_user_certif: Option<Vec<u8>>,
        devices_certifs: Vec<Vec<u8>>,
        expected_user_id: Option<UserID>,
        py: Python<'py>,
    ) -> PyResult<(
        UserCertificate,
        Option<RevokedUserCertificate>,
        &'py PyTuple,
    )> {
        let (user, revoked_user, devices) = self
            .0
            .load_user_and_devices(
                trustchain.0,
                user_certif,
                revoked_user_certif,
                devices_certifs,
                expected_user_id.map(|user_id| user_id.0),
            )
            .map_err(to_trustchain_error_exception)?;

        let devices = devices
            .into_iter()
            .map(|x| DeviceCertificate(x).into_py(py));

        Ok((
            UserCertificate(user),
            revoked_user.map(RevokedUserCertificate),
            PyTuple::new(py, devices),
        ))
    }
}

#[pyclass]
#[derive(PartialEq)]
pub(crate) struct TrustchainError(pub(crate) libparsec::core::TrustchainError);

// This object is only here to wrap our `TrustchainError` in a python exception
// object. Without this object we can't raise or except `TrustchainError` in
// python code
create_exception!(_parsec, TrustchainErrorException, PyException);

#[pymethods]
impl TrustchainError {
    #[classmethod]
    fn invalid_certificate(_cls: &PyType, path: String, exc: String) -> Self {
        Self(libparsec::core::TrustchainError::InvalidCertificate { path, exc })
    }

    #[classmethod]
    fn invalid_self_signed_user_certificate(_cls: &PyType, user_id: UserID) -> Self {
        Self(
            libparsec::core::TrustchainError::InvalidSelfSignedUserCertificate {
                user_id: user_id.0,
            },
        )
    }

    #[classmethod]
    fn invalid_self_signed_revocation_certificate(_cls: &PyType, user_id: UserID) -> Self {
        Self(
            libparsec::core::TrustchainError::InvalidSelfSignedUserRevocationCertificate {
                user_id: user_id.0,
            },
        )
    }

    #[classmethod]
    fn invalid_signature_given(_cls: &PyType, path: String, user_id: UserID) -> Self {
        Self(libparsec::core::TrustchainError::InvalidSignatureGiven {
            path,
            user_id: user_id.0,
        })
    }

    #[classmethod]
    fn invalid_signature_loop_detected(_cls: &PyType, path: String) -> Self {
        Self(libparsec::core::TrustchainError::InvalidSignatureLoopDetected { path })
    }

    #[classmethod]
    fn missing_device_certificate(_cls: &PyType, path: String, device_id: DeviceID) -> Self {
        Self(libparsec::core::TrustchainError::MissingDeviceCertificate {
            path,
            device_id: device_id.0,
        })
    }

    #[classmethod]
    fn missing_user_certificate(_cls: &PyType, path: String, user_id: UserID) -> Self {
        Self(libparsec::core::TrustchainError::MissingUserCertificate {
            path,
            user_id: user_id.0,
        })
    }

    #[classmethod]
    fn signature_posterior_user_revocation(
        _cls: &PyType,
        path: String,
        verified_timestamp: DateTime,
        user_timestamp: DateTime,
    ) -> Self {
        Self(
            libparsec::core::TrustchainError::SignaturePosteriorUserRevocation {
                path,
                verified_timestamp: verified_timestamp.0,
                user_timestamp: user_timestamp.0,
            },
        )
    }

    #[classmethod]
    fn unexpected_certificate(_cls: &PyType, expected: UserID, got: UserID) -> Self {
        Self(libparsec::core::TrustchainError::UnexpectedCertificate {
            expected: expected.0,
            got: got.0,
        })
    }

    #[getter]
    fn path(&self) -> PyResult<String> {
        match &self.0 {
            libparsec::core::TrustchainError::InvalidCertificate { path, .. } => Ok(path.clone()),
            libparsec::core::TrustchainError::InvalidSignatureGiven { path, .. } => {
                Ok(path.clone())
            }
            libparsec::core::TrustchainError::InvalidSignatureLoopDetected { path } => {
                Ok(path.clone())
            }
            libparsec::core::TrustchainError::MissingDeviceCertificate { path, .. } => {
                Ok(path.clone())
            }
            libparsec::core::TrustchainError::MissingUserCertificate { path, .. } => {
                Ok(path.clone())
            }
            libparsec::core::TrustchainError::SignaturePosteriorUserRevocation { path, .. } => {
                Ok(path.clone())
            }
            _ => Err(PyAttributeError::new_err("No such attribute `path`")),
        }
    }

    #[getter]
    fn exc(&self) -> PyResult<String> {
        if let libparsec::core::TrustchainError::InvalidCertificate { exc, .. } = &self.0 {
            Ok(exc.clone())
        } else {
            Err(PyAttributeError::new_err("No such attribute `exc`"))
        }
    }

    #[getter]
    fn user_id(&self) -> PyResult<UserID> {
        match &self.0 {
            libparsec::core::TrustchainError::InvalidSelfSignedUserCertificate { user_id } => {
                Ok(ids::UserID(user_id.clone()))
            }
            libparsec::core::TrustchainError::InvalidSelfSignedUserRevocationCertificate {
                user_id,
            } => Ok(ids::UserID(user_id.clone())),
            libparsec::core::TrustchainError::InvalidSignatureGiven { user_id, .. } => {
                Ok(ids::UserID(user_id.clone()))
            }
            libparsec::core::TrustchainError::MissingUserCertificate { user_id, .. } => {
                Ok(ids::UserID(user_id.clone()))
            }
            _ => Err(PyAttributeError::new_err("No such attribute `user_id`")),
        }
    }

    #[getter]
    fn device_id(&self) -> PyResult<ids::DeviceID> {
        if let libparsec::core::TrustchainError::MissingDeviceCertificate { device_id, .. } =
            &self.0
        {
            Ok(ids::DeviceID(device_id.clone()))
        } else {
            Err(PyAttributeError::new_err("No such attribute `device_id`"))
        }
    }

    #[getter]
    fn verified_timestamp(&self) -> PyResult<DateTime> {
        if let libparsec::core::TrustchainError::SignaturePosteriorUserRevocation {
            verified_timestamp,
            ..
        } = &self.0
        {
            Ok(time::DateTime(*verified_timestamp))
        } else {
            Err(PyAttributeError::new_err(
                "No such attribute `verified_timestamp`",
            ))
        }
    }

    #[getter]
    fn user_timestamp(&self) -> PyResult<DateTime> {
        if let libparsec::core::TrustchainError::SignaturePosteriorUserRevocation {
            user_timestamp,
            ..
        } = &self.0
        {
            Ok(time::DateTime(*user_timestamp))
        } else {
            Err(PyAttributeError::new_err(
                "No such attribute `user_timestamp`",
            ))
        }
    }

    #[getter]
    fn expected(&self) -> PyResult<ids::UserID> {
        if let libparsec::core::TrustchainError::UnexpectedCertificate { expected, .. } = &self.0 {
            Ok(ids::UserID(expected.clone()))
        } else {
            Err(PyAttributeError::new_err("No such attribute `expected`"))
        }
    }

    #[getter]
    fn got(&self) -> PyResult<ids::UserID> {
        if let libparsec::core::TrustchainError::UnexpectedCertificate { got, .. } = &self.0 {
            Ok(ids::UserID(got.clone()))
        } else {
            Err(PyAttributeError::new_err("No such attribute `got`"))
        }
    }
}

gen_proto!(TrustchainError, __richcmp__, eq);
gen_proto!(TrustchainError, __str__); // Needed for python's exceptions
gen_proto!(TrustchainError, __repr__);

impl From<TrustchainError> for PyErr {
    fn from(err: TrustchainError) -> Self {
        match err.0 {
            libparsec::core::TrustchainError::InvalidCertificate { .. } => {
                PyValueError::new_err("Invalid certificate")
            }
            libparsec::core::TrustchainError::InvalidSelfSignedUserCertificate { .. } => {
                PyValueError::new_err("Invalid self signed user certificate")
            }
            libparsec::core::TrustchainError::InvalidSelfSignedUserRevocationCertificate {
                ..
            } => PyValueError::new_err("Invalid self signed user revocation certificate"),
            libparsec::core::TrustchainError::InvalidSignatureGiven { .. } => {
                PyValueError::new_err("Invalid signature given")
            }
            libparsec::core::TrustchainError::InvalidSignatureLoopDetected { .. } => {
                PyValueError::new_err("Invalid signature loop detected")
            }
            libparsec::core::TrustchainError::MissingDeviceCertificate { .. } => {
                PyValueError::new_err("Missing device certificate")
            }
            libparsec::core::TrustchainError::MissingUserCertificate { .. } => {
                PyValueError::new_err("Missing user certificate")
            }
            libparsec::core::TrustchainError::SignaturePosteriorUserRevocation { .. } => {
                PyValueError::new_err("Signature posterior user revocation")
            }
            libparsec::core::TrustchainError::UnexpectedCertificate { .. } => {
                PyValueError::new_err("Unexpected certificate")
            }
        }
    }
}
