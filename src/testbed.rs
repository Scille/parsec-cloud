// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{prelude::*, types::PyBytes, PyResult};
use std::path::PathBuf;

use crate::{
    addrs::BackendAddr,
    api_crypto::{PrivateKey, SecretKey, SigningKey},
    data::{DeviceCertificate, UserCertificate},
    enumerate::UserProfile,
    ids::{DeviceID, DeviceLabel, EntryID, HumanHandle, UserID},
    runtime::FutureIntoCoroutine,
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct TestbedEnv(pub std::sync::Arc<libparsec::testbed::TestbedEnv>);

crate::binding_utils::gen_proto!(TestbedEnv, __repr__);
crate::binding_utils::gen_proto!(TestbedEnv, __copy__);
crate::binding_utils::gen_proto!(TestbedEnv, __deepcopy__);

#[pyclass]
#[derive(Clone)]
pub(crate) struct TestbedDeviceData(pub libparsec::testbed::TestbedDeviceData);

crate::binding_utils::gen_proto!(TestbedDeviceData, __repr__);
crate::binding_utils::gen_proto!(TestbedDeviceData, __copy__);
crate::binding_utils::gen_proto!(TestbedDeviceData, __deepcopy__);

#[pymethods]
impl TestbedDeviceData {
    #[getter]
    fn device_id(&self) -> DeviceID {
        DeviceID(self.0.device_id.clone())
    }

    #[getter]
    fn device_label(&self) -> Option<DeviceLabel> {
        self.0.device_label.as_ref().map(|x| DeviceLabel(x.clone()))
    }

    #[getter]
    fn signing_key(&self) -> SigningKey {
        SigningKey(self.0.signing_key.clone())
    }

    #[getter]
    fn local_symkey(&self) -> SecretKey {
        SecretKey(self.0.local_symkey.clone())
    }

    #[getter]
    fn certif(&self) -> DeviceCertificate {
        DeviceCertificate(self.0.certif.clone())
    }

    #[getter]
    fn raw_certif<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.raw_certif)
    }

    #[getter]
    fn raw_redacted_certif<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.raw_redacted_certif)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct TestbedUserData(pub libparsec::testbed::TestbedUserData);

crate::binding_utils::gen_proto!(TestbedUserData, __repr__);
crate::binding_utils::gen_proto!(TestbedUserData, __copy__);
crate::binding_utils::gen_proto!(TestbedUserData, __deepcopy__);

#[pymethods]
impl TestbedUserData {
    #[getter]
    fn user_id(&self) -> UserID {
        UserID(self.0.user_id.clone())
    }

    #[getter]
    fn human_handle(&self) -> Option<HumanHandle> {
        self.0
            .human_handle
            .as_ref()
            .map(|x| crate::ids::HumanHandle(x.clone()))
    }

    #[getter]
    fn private_key(&self) -> PrivateKey {
        PrivateKey(self.0.private_key.clone())
    }

    #[getter]
    fn profile(&self) -> &'static PyObject {
        UserProfile::from_profile(self.0.profile)
    }

    #[getter]
    fn user_manifest_id(&self) -> EntryID {
        EntryID(self.0.user_manifest_id.clone())
    }

    #[getter]
    fn user_manifest_key(&self) -> SecretKey {
        SecretKey(self.0.user_manifest_key.clone())
    }

    #[getter]
    fn certif(&self) -> UserCertificate {
        UserCertificate(self.0.certif.clone())
    }

    #[getter]
    fn raw_certif<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.raw_certif)
    }

    #[getter]
    fn raw_redacted_certif<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.raw_redacted_certif)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct TestbedTemplate(pub std::sync::Arc<libparsec::testbed::TestbedTemplate>);

crate::binding_utils::gen_proto!(TestbedTemplate, __repr__);
crate::binding_utils::gen_proto!(TestbedTemplate, __copy__);
crate::binding_utils::gen_proto!(TestbedTemplate, __deepcopy__);

#[pymethods]
impl TestbedTemplate {
    #[getter]
    fn id(&self) -> &str {
        self.0.id
    }

    #[getter]
    fn root_signing_key(&self) -> SigningKey {
        SigningKey(self.0.root_signing_key.clone())
    }

    #[getter]
    fn devices(&self) -> Vec<TestbedDeviceData> {
        self.0
            .devices
            .iter()
            .map(|x| TestbedDeviceData(x.clone()))
            .collect()
    }

    #[getter]
    fn users(&self) -> Vec<TestbedUserData> {
        self.0
            .users
            .iter()
            .map(|x| TestbedUserData(x.clone()))
            .collect()
    }

    #[getter]
    fn crc(&self) -> u32 {
        self.0.crc.to_owned()
    }
}

#[pyfunction]
pub(crate) fn test_new_testbed(
    // params are moved into async coroutine, so ownership is needed.
    template: String,
    test_server: Option<BackendAddr>,
) -> PyResult<FutureIntoCoroutine> {
    if &template != "empty" && &template != "coolorg" {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "No template named `{template}`",
        )));
    }

    Ok(FutureIntoCoroutine::from(async move {
        let env = libparsec::testbed::test_new_testbed(
            &template,
            test_server.as_ref().map(|addr| &addr.0),
        )
        .await;
        Ok(TestbedEnv(env))
    }))
}

#[pyfunction]
pub(crate) fn test_drop_testbed(path: PathBuf) -> FutureIntoCoroutine {
    FutureIntoCoroutine::from(async move { Ok(libparsec::testbed::test_drop_testbed(&path).await) })
}

#[pyfunction]
pub(crate) fn test_get_testbed_templates() -> Vec<TestbedTemplate> {
    libparsec::testbed::test_get_testbed_templates()
        .into_iter()
        .map(|x| TestbedTemplate(x))
        .collect()
}
