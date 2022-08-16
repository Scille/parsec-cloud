// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use crate::api_crypto::VerifyKey;
use crate::certif::{DeviceCertificate, RevokedUserCertificate, UserCertificate};
use crate::ids::{DeviceID, UserID};
use crate::time::DateTime;

import_exception!(parsec.core.trustchain, TrustchainError);

#[pyclass]
pub(crate) struct TrustchainContext(pub libparsec::core::TrustchainContext);

#[pymethods]
impl TrustchainContext {
    #[new]
    fn new(root_verify_key: &VerifyKey, cache_validity: i64) -> PyResult<Self> {
        Ok(Self(libparsec::core::TrustchainContext::new(
            root_verify_key.0.clone(),
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

    fn get_user(
        &self,
        user_id: &UserID,
        now: Option<DateTime>,
    ) -> PyResult<Option<UserCertificate>> {
        let now = now.map(|now| now.0);
        Ok(self
            .0
            .get_user(&user_id.0, now)
            .cloned()
            .map(UserCertificate))
    }

    fn get_revoked_user(
        &self,
        user_id: &UserID,
        now: Option<DateTime>,
    ) -> PyResult<Option<RevokedUserCertificate>> {
        let now = now.map(|now| now.0);
        Ok(self
            .0
            .get_revoked_user(&user_id.0, now)
            .cloned()
            .map(RevokedUserCertificate))
    }

    fn get_device(
        &self,
        device_id: &DeviceID,
        now: Option<DateTime>,
    ) -> PyResult<Option<DeviceCertificate>> {
        let now = now.map(|now| now.0);
        Ok(self
            .0
            .get_device(&device_id.0, now)
            .cloned()
            .map(DeviceCertificate))
    }

    fn load_trustchain<'py>(
        &mut self,
        users: Vec<Vec<u8>>,
        revoked_users: Vec<Vec<u8>>,
        devices: Vec<Vec<u8>>,
        now: Option<DateTime>,
        py: Python<'py>,
    ) -> PyResult<(&'py PyTuple, &'py PyTuple, &'py PyTuple)> {
        let now = now.map(|now| now.0);
        let (users, revoked_users, devices) = self
            .0
            .load_trustchain(&users, &revoked_users, &devices, now)
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
        trustchain: Option<&PyDict>,
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
        crate::binding_utils::parse_kwargs!(
            trustchain,
            [users: Vec<Vec<u8>>, "users"],
            [devices: Vec<Vec<u8>>, "devices"],
            [revoked_users: Vec<Vec<u8>>, "revoked_users"]
        );

        let trustchain = libparsec::protocol::authenticated_cmds::user_get::Trustchain {
            users,
            devices,
            revoked_users,
        };

        let (user, revoked_user, devices) = self
            .0
            .load_user_and_devices(
                trustchain,
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
