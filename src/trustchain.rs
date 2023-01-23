// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use crate::{
    api_crypto::VerifyKey,
    binding_utils::{gen_proto, BytesWrapper},
    data::{DeviceCertificate, RevokedUserCertificate, UserCertificate},
    ids::{DeviceID, UserID},
    protocol::Trustchain,
    time::{self, DateTime, TimeProvider},
};

use pyo3::{
    create_exception,
    exceptions::{PyAttributeError, PyException},
    prelude::*,
    types::PyTuple,
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
        users: Vec<BytesWrapper>,
        revoked_users: Vec<BytesWrapper>,
        devices: Vec<BytesWrapper>,
        py: Python<'py>,
    ) -> Result<(&'py PyTuple, &'py PyTuple, &'py PyTuple), TrustchainError> {
        crate::binding_utils::unwrap_bytes!(users, revoked_users, devices);
        let (users, revoked_users, devices) =
            self.0.load_trustchain(&users, &revoked_users, &devices)?;

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
        user_certif: BytesWrapper,
        revoked_user_certif: Option<BytesWrapper>,
        devices_certifs: Vec<BytesWrapper>,
        expected_user_id: Option<&UserID>,
        py: Python<'py>,
    ) -> Result<
        (
            UserCertificate,
            Option<RevokedUserCertificate>,
            &'py PyTuple,
        ),
        TrustchainError,
    > {
        crate::binding_utils::unwrap_bytes!(user_certif, revoked_user_certif, devices_certifs);
        let (user, revoked_user, devices) = self.0.load_user_and_devices(
            trustchain.0,
            user_certif,
            revoked_user_certif,
            devices_certifs,
            expected_user_id.map(|user_id| &user_id.0),
        )?;

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
pub(crate) struct TrustchainError(pub libparsec::core::TrustchainError);

// This object is only here to wrap our `TrustchainError` in a python exception
// object. Without this object we can't raise or except `TrustchainError` in
// python code
create_exception!(_parsec, TrustchainErrorException, PyException);

#[pymethods]
impl TrustchainError {
    #[getter]
    fn path(&self) -> PyResult<&str> {
        match &self.0 {
            libparsec::core::TrustchainError::InvalidCertificate { path, .. } => Ok(path),
            libparsec::core::TrustchainError::InvalidSignatureGiven { path, .. } => Ok(path),
            libparsec::core::TrustchainError::InvalidSignatureLoopDetected { path } => Ok(path),
            libparsec::core::TrustchainError::MissingDeviceCertificate { path, .. } => Ok(path),
            libparsec::core::TrustchainError::MissingUserCertificate { path, .. } => Ok(path),
            libparsec::core::TrustchainError::SignaturePosteriorUserRevocation { path, .. } => {
                Ok(path)
            }
            _ => Err(PyAttributeError::new_err("No such attribute `path`")),
        }
    }

    #[getter]
    fn exc(&self) -> PyResult<&str> {
        if let libparsec::core::TrustchainError::InvalidCertificate { exc, .. } = &self.0 {
            Ok(exc)
        } else {
            Err(PyAttributeError::new_err("No such attribute `exc`"))
        }
    }

    #[getter]
    fn user_id(&self) -> PyResult<UserID> {
        match &self.0 {
            libparsec::core::TrustchainError::InvalidSelfSignedUserCertificate { user_id } => {
                Ok(UserID(user_id.clone()))
            }
            libparsec::core::TrustchainError::InvalidSelfSignedUserRevocationCertificate {
                user_id,
            } => Ok(UserID(user_id.clone())),
            libparsec::core::TrustchainError::InvalidSignatureGiven { user_id, .. } => {
                Ok(UserID(user_id.clone()))
            }
            libparsec::core::TrustchainError::MissingUserCertificate { user_id, .. } => {
                Ok(UserID(user_id.clone()))
            }
            _ => Err(PyAttributeError::new_err("No such attribute `user_id`")),
        }
    }

    #[getter]
    fn device_id(&self) -> PyResult<DeviceID> {
        if let libparsec::core::TrustchainError::MissingDeviceCertificate { device_id, .. } =
            &self.0
        {
            Ok(DeviceID(device_id.clone()))
        } else {
            Err(PyAttributeError::new_err("No such attribute `device_id`"))
        }
    }

    #[getter]
    fn verified_timestamp(&self) -> PyResult<DateTime> {
        if let libparsec::core::TrustchainError::SignaturePosteriorUserRevocation {
            verified_timestamp,
            ..
        } = self.0
        {
            Ok(time::DateTime(verified_timestamp))
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
        } = self.0
        {
            Ok(time::DateTime(user_timestamp))
        } else {
            Err(PyAttributeError::new_err(
                "No such attribute `user_timestamp`",
            ))
        }
    }

    #[getter]
    fn expected(&self) -> PyResult<UserID> {
        if let libparsec::core::TrustchainError::UnexpectedCertificate { expected, .. } = &self.0 {
            Ok(UserID(expected.clone()))
        } else {
            Err(PyAttributeError::new_err("No such attribute `expected`"))
        }
    }

    #[getter]
    fn got(&self) -> PyResult<UserID> {
        if let libparsec::core::TrustchainError::UnexpectedCertificate { got, .. } = &self.0 {
            Ok(UserID(got.clone()))
        } else {
            Err(PyAttributeError::new_err("No such attribute `got`"))
        }
    }
}

gen_proto!(TrustchainError, __richcmp__, eq);
gen_proto!(TrustchainError, __str__); // Needed for python's exceptions
gen_proto!(TrustchainError, __repr__);
gen_proto!(TrustchainError, __copy__);
gen_proto!(TrustchainError, __deepcopy__);

impl From<TrustchainError> for PyErr {
    fn from(err: TrustchainError) -> Self {
        TrustchainErrorException::new_err(err)
    }
}

impl From<libparsec::core::TrustchainError> for TrustchainError {
    fn from(err: libparsec::core::TrustchainError) -> Self {
        TrustchainError(err)
    }
}
