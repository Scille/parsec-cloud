// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{prelude::*, types::PyBytes, PyResult};
use std::path::PathBuf;

use crate::addrs::BackendAddr;
use crate::runtime::FutureIntoCoroutine;

#[pyclass]
#[derive(Clone)]
pub(crate) struct TestbedEnv(
    #[cfg(feature = "test-utils")] pub std::sync::Arc<libparsec::TestbedEnv>,
);

crate::binding_utils::gen_proto!(TestbedEnv, __repr__);
crate::binding_utils::gen_proto!(TestbedEnv, __copy__);
crate::binding_utils::gen_proto!(TestbedEnv, __deepcopy__);

#[pyclass]
#[derive(Clone)]
pub(crate) struct TestbedDeviceData(
    #[cfg(feature = "test-utils")] pub libparsec::TestbedDeviceData,
);

crate::binding_utils::gen_proto!(TestbedDeviceData, __repr__);
crate::binding_utils::gen_proto!(TestbedDeviceData, __copy__);
crate::binding_utils::gen_proto!(TestbedDeviceData, __deepcopy__);

#[pymethods]
impl TestbedDeviceData {
    #[getter]
    fn device_id(&self) -> crate::ids::DeviceID {
        crate::ids::DeviceID(self.0.device_id.clone())
    }
    #[getter]
    fn device_label(&self) -> Option<crate::ids::DeviceLabel> {
        self.0
            .device_label
            .as_ref()
            .map(|x| crate::ids::DeviceLabel(x.clone()))
    }
    #[getter]
    fn signing_key(&self) -> crate::api_crypto::SigningKey {
        crate::api_crypto::SigningKey(self.0.signing_key.clone())
    }
    #[getter]
    fn local_symkey(&self) -> crate::api_crypto::SecretKey {
        crate::api_crypto::SecretKey(self.0.local_symkey.clone())
    }
    #[getter]
    fn certif(&self) -> crate::data::DeviceCertificate {
        crate::data::DeviceCertificate(self.0.certif.clone())
    }
    #[getter]
    fn raw_certif<'p>(&self, py: Python<'p>) -> &'p PyBytes {
        PyBytes::new(py, &self.0.raw_certif)
    }
    #[getter]
    fn raw_redacted_certif<'p>(&self, py: Python<'p>) -> &'p PyBytes {
        PyBytes::new(py, &self.0.raw_redacted_certif)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct TestbedUserData(#[cfg(feature = "test-utils")] pub libparsec::TestbedUserData);

crate::binding_utils::gen_proto!(TestbedUserData, __repr__);
crate::binding_utils::gen_proto!(TestbedUserData, __copy__);
crate::binding_utils::gen_proto!(TestbedUserData, __deepcopy__);

#[pymethods]
impl TestbedUserData {
    #[getter]
    fn user_id(&self) -> crate::ids::UserID {
        crate::ids::UserID(self.0.user_id.clone())
    }
    #[getter]
    fn human_handle(&self) -> Option<crate::ids::HumanHandle> {
        self.0
            .human_handle
            .as_ref()
            .map(|x| crate::ids::HumanHandle(x.clone()))
    }
    #[getter]
    fn private_key(&self) -> crate::api_crypto::PrivateKey {
        crate::api_crypto::PrivateKey(self.0.private_key.clone())
    }
    #[getter]
    fn profile(&self) -> crate::enumerate::UserProfile {
        crate::enumerate::UserProfile(self.0.profile.clone())
    }
    #[getter]
    fn user_manifest_id(&self) -> crate::ids::EntryID {
        crate::ids::EntryID(self.0.user_manifest_id.clone())
    }
    #[getter]
    fn user_manifest_key(&self) -> crate::api_crypto::SecretKey {
        crate::api_crypto::SecretKey(self.0.user_manifest_key.clone())
    }
    #[getter]
    fn certif(&self) -> crate::data::UserCertificate {
        crate::data::UserCertificate(self.0.certif.clone())
    }
    #[getter]
    fn raw_certif<'p>(&self, py: Python<'p>) -> &'p PyBytes {
        PyBytes::new(py, &self.0.raw_certif)
    }
    #[getter]
    fn raw_redacted_certif<'p>(&self, py: Python<'p>) -> &'p PyBytes {
        PyBytes::new(py, &self.0.raw_redacted_certif)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct TestbedTemplate(
    #[cfg(feature = "test-utils")] pub std::sync::Arc<libparsec::TestbedTemplate>,
);

crate::binding_utils::gen_proto!(TestbedTemplate, __repr__);
crate::binding_utils::gen_proto!(TestbedTemplate, __copy__);
crate::binding_utils::gen_proto!(TestbedTemplate, __deepcopy__);

#[pymethods]
impl TestbedTemplate {
    #[getter]
    fn id(&self) -> String {
        self.0.id.to_owned()
    }
    #[getter]
    fn root_signing_key(&self) -> crate::api_crypto::SigningKey {
        crate::api_crypto::SigningKey(self.0.root_signing_key.clone())
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
    template: String,
    test_server: Option<BackendAddr>,
) -> PyResult<FutureIntoCoroutine> {
    #[cfg(not(feature = "test-utils"))]
    {
        let _test_server = test_server;
        let _template = template;
        Err(pyo3::exceptions::PyRuntimeError::new_err(
            "Test features are disabled !",
        ))
    }
    #[cfg(feature = "test-utils")]
    {
        if &template != "empty" && &template != "coolorg" {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "No template named `{}`",
                template
            )));
        }
        Ok(FutureIntoCoroutine::from(async move {
            let env =
                libparsec::test_new_testbed(&template, test_server.as_ref().map(|addr| &addr.0))
                    .await;
            Ok(TestbedEnv(env))
        }))
    }
}

#[pyfunction]
pub(crate) fn test_drop_testbed(path: PathBuf) -> PyResult<FutureIntoCoroutine> {
    #[cfg(not(feature = "test-utils"))]
    {
        let _path = path;
        Err(pyo3::exceptions::PyRuntimeError::new_err(
            "Test features are disabled !",
        ))
    }

    #[cfg(feature = "test-utils")]
    {
        Ok(FutureIntoCoroutine::from(async move {
            Ok(libparsec::test_drop_testbed(&path).await)
        }))
    }
}

#[pyfunction]
pub(crate) fn test_get_testbed_templates() -> Vec<TestbedTemplate> {
    #[cfg(not(feature = "test-utils"))]
    {
        Err(pyo3::exceptions::PyRuntimeError::new_err(
            "Test features are disabled !",
        ))
    }

    #[cfg(feature = "test-utils")]
    {
        libparsec::test_get_testbed_templates()
            .into_iter()
            .map(|x| TestbedTemplate(x))
            .collect()
    }
}
