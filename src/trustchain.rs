// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{import_exception, prelude::*, types::PyTuple};

use crate::{
    api_crypto::VerifyKey,
    certif::{DeviceCertificate, RevokedUserCertificate, UserCertificate},
    ids::{DeviceID, UserID},
    protocol::Trustchain,
    time::TimeProvider,
};

import_exception!(parsec.core.trustchain, TrustchainError);

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
            .map_err(|err| TrustchainError::new_err(err.to_string()))?;

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
            .map_err(|err| TrustchainError::new_err(err.to_string()))?;

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
