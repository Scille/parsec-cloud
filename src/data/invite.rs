// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    prelude::{pyclass, pyfunction, pymethods, IntoPy, PyObject, PyResult, Python},
    types::{PyBytes, PyDict, PyList, PyTuple, PyType},
};

use crate::{
    api_crypto::{PrivateKey, PublicKey, SecretKey, VerifyKey},
    enumerate::UserProfile,
    ids::{DeviceID, DeviceLabel, EntryID, HumanHandle},
};

/*
* SASCode
*/

#[pyclass]
pub(crate) struct SASCode(pub libparsec::types::SASCode);

crate::binding_utils::gen_proto!(SASCode, __repr__);
crate::binding_utils::gen_proto!(SASCode, __copy__);
crate::binding_utils::gen_proto!(SASCode, __deepcopy__);
crate::binding_utils::gen_proto!(SASCode, __str__);
crate::binding_utils::gen_proto!(SASCode, __richcmp__, ord);
crate::binding_utils::gen_proto!(SASCode, __hash__);

#[pymethods]
impl SASCode {
    #[new]
    fn new(code: &str) -> PyResult<Self> {
        match code.parse::<libparsec::types::SASCode>() {
            Ok(sas) => Ok(Self(sas)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    fn from_int(_cls: &PyType, num: u32) -> PyResult<Self> {
        match libparsec::types::SASCode::try_from(num) {
            Ok(sas) => Ok(Self(sas)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}

#[pyfunction]
pub(crate) fn generate_sas_codes<'p>(
    py: Python<'p>,
    claimer_nonce: &PyBytes,
    greeter_nonce: &PyBytes,
    shared_secret_key: &SecretKey,
) -> PyResult<&'p PyTuple> {
    let (claimer_sas, greeter_sas) = libparsec::types::SASCode::generate_sas_codes(
        claimer_nonce.as_bytes(),
        greeter_nonce.as_bytes(),
        &shared_secret_key.0,
    );
    Ok(PyTuple::new(
        py,
        vec![
            SASCode(claimer_sas).into_py(py),
            SASCode(greeter_sas).into_py(py),
        ],
    ))
}

#[pyfunction]
pub(crate) fn generate_sas_code_candidates<'p>(
    py: Python<'p>,
    valid_sas: &SASCode,
    size: usize,
) -> PyResult<&'p PyList> {
    let candidates: Vec<libparsec::types::SASCode> = valid_sas.0.generate_sas_code_candidates(size);
    let py_candidates: Vec<PyObject> = candidates
        .iter()
        .map(|v| SASCode::new(v.as_ref()).unwrap().into_py(py))
        .collect();

    Ok(PyList::new(py, py_candidates))
}

/*
* InviteUserData
*/

#[pyclass]
pub(crate) struct InviteUserData(pub libparsec::types::InviteUserData);

crate::binding_utils::gen_proto!(InviteUserData, __repr__);
crate::binding_utils::gen_proto!(InviteUserData, __copy__);
crate::binding_utils::gen_proto!(InviteUserData, __deepcopy__);

#[pymethods]
impl InviteUserData {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [
                requested_device_label: Option<DeviceLabel>,
                "requested_device_label"
            ],
            [
                requested_human_handle: Option<HumanHandle>,
                "requested_human_handle"
            ],
            [public_key: PublicKey, "public_key"],
            [verify_key: VerifyKey, "verify_key"],
        );

        Ok(Self(libparsec::types::InviteUserData {
            requested_device_label: requested_device_label.map(|d| d.0),
            requested_human_handle: requested_human_handle.map(|h| h.0),
            public_key: public_key.0,
            verify_key: verify_key.0,
        }))
    }

    #[getter]
    fn requested_human_handle(&self) -> PyResult<Option<HumanHandle>> {
        match &self.0.requested_human_handle {
            Some(hh) => Ok(Some(HumanHandle(hh.clone()))),
            None => Ok(None),
        }
    }

    #[getter]
    fn requested_device_label(&self) -> PyResult<Option<DeviceLabel>> {
        match &self.0.requested_device_label {
            Some(dl) => Ok(Some(DeviceLabel(dl.clone()))),
            None => Ok(None),
        }
    }

    #[getter]
    fn public_key(&self) -> PyResult<PublicKey> {
        Ok(PublicKey(self.0.public_key.clone()))
    }

    #[getter]
    fn verify_key(&self) -> PyResult<VerifyKey> {
        Ok(VerifyKey(self.0.verify_key.clone()))
    }

    fn dump_and_encrypt<'p>(&self, py: Python<'p>, key: &SecretKey) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_encrypt(&key.0)))
    }

    #[classmethod]
    fn decrypt_and_load(_cls: &PyType, encrypted: &[u8], key: &SecretKey) -> PyResult<Self> {
        match libparsec::types::InviteUserData::decrypt_and_load(encrypted, &key.0) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
}

/*
* InviteUserConfirmation
*/

#[pyclass]
pub(crate) struct InviteUserConfirmation(pub libparsec::types::InviteUserConfirmation);

crate::binding_utils::gen_proto!(InviteUserConfirmation, __repr__);
crate::binding_utils::gen_proto!(InviteUserConfirmation, __copy__);
crate::binding_utils::gen_proto!(InviteUserConfirmation, __deepcopy__);

#[pymethods]
impl InviteUserConfirmation {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [device_id: DeviceID, "device_id"],
            [device_label: Option<DeviceLabel>, "device_label"],
            [human_handle: Option<HumanHandle>, "human_handle"],
            [profile: UserProfile, "profile"],
            [root_verify_key: VerifyKey, "root_verify_key"],
        );

        Ok(Self(libparsec::types::InviteUserConfirmation {
            device_id: device_id.0,
            device_label: device_label.map(|d| d.0),
            human_handle: human_handle.map(|h| h.0),
            profile: profile.0,
            root_verify_key: root_verify_key.0,
        }))
    }

    #[getter]
    fn human_handle(&self) -> PyResult<Option<HumanHandle>> {
        match &self.0.human_handle {
            Some(hh) => Ok(Some(HumanHandle(hh.clone()))),
            None => Ok(None),
        }
    }

    #[getter]
    fn device_label(&self) -> PyResult<Option<DeviceLabel>> {
        match &self.0.device_label {
            Some(dl) => Ok(Some(DeviceLabel(dl.clone()))),
            None => Ok(None),
        }
    }

    #[getter]
    fn device_id(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.device_id.clone()))
    }

    #[getter]
    fn profile(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            let cls = py.import("parsec.api.protocol")?.getattr("UserProfile")?;
            let name = self.0.profile.to_string();
            let obj = cls.getattr(&name as &str)?;
            Ok(obj.into_py(py))
        })
    }

    #[getter]
    fn root_verify_key(&self) -> PyResult<VerifyKey> {
        Ok(VerifyKey(self.0.root_verify_key.clone()))
    }

    fn dump_and_encrypt<'p>(&self, py: Python<'p>, key: &SecretKey) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_encrypt(&key.0)))
    }

    #[classmethod]
    fn decrypt_and_load(_cls: &PyType, encrypted: &[u8], key: &SecretKey) -> PyResult<Self> {
        match libparsec::types::InviteUserConfirmation::decrypt_and_load(encrypted, &key.0) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
}

/*
* InviteDeviceData
*/

#[pyclass]
pub(crate) struct InviteDeviceData(pub libparsec::types::InviteDeviceData);

crate::binding_utils::gen_proto!(InviteDeviceData, __repr__);
crate::binding_utils::gen_proto!(InviteDeviceData, __copy__);
crate::binding_utils::gen_proto!(InviteDeviceData, __deepcopy__);

#[pymethods]
impl InviteDeviceData {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [
                requested_device_label: Option<DeviceLabel>,
                "requested_device_label"
            ],
            [verify_key: VerifyKey, "verify_key"],
        );

        Ok(Self(libparsec::types::InviteDeviceData {
            requested_device_label: requested_device_label.map(|d| d.0),
            verify_key: verify_key.0,
        }))
    }

    #[getter]
    fn requested_device_label(&self) -> PyResult<Option<DeviceLabel>> {
        match &self.0.requested_device_label {
            Some(dl) => Ok(Some(DeviceLabel(dl.clone()))),
            None => Ok(None),
        }
    }

    #[getter]
    fn verify_key(&self) -> PyResult<VerifyKey> {
        Ok(VerifyKey(self.0.verify_key.clone()))
    }

    fn dump_and_encrypt<'p>(&self, py: Python<'p>, key: &SecretKey) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_encrypt(&key.0)))
    }

    #[classmethod]
    fn decrypt_and_load(_cls: &PyType, encrypted: &[u8], key: &SecretKey) -> PyResult<Self> {
        match libparsec::types::InviteDeviceData::decrypt_and_load(encrypted, &key.0) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
}

/*
* InviteDeviceConfirmation
*/

#[pyclass]
pub(crate) struct InviteDeviceConfirmation(pub libparsec::types::InviteDeviceConfirmation);

crate::binding_utils::gen_proto!(InviteDeviceConfirmation, __repr__);
crate::binding_utils::gen_proto!(InviteDeviceConfirmation, __copy__);
crate::binding_utils::gen_proto!(InviteDeviceConfirmation, __deepcopy__);

#[pymethods]
impl InviteDeviceConfirmation {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [device_id: DeviceID, "device_id"],
            [device_label: Option<DeviceLabel>, "device_label"],
            [human_handle: Option<HumanHandle>, "human_handle"],
            [profile: UserProfile, "profile"],
            [private_key: PrivateKey, "private_key"],
            [root_verify_key: VerifyKey, "root_verify_key"],
            [user_manifest_id: EntryID, "user_manifest_id"],
            [user_manifest_key: SecretKey, "user_manifest_key"],
        );

        Ok(Self(libparsec::types::InviteDeviceConfirmation {
            device_id: device_id.0,
            device_label: device_label.map(|d| d.0),
            human_handle: human_handle.map(|h| h.0),
            profile: profile.0,
            private_key: private_key.0,
            user_manifest_id: user_manifest_id.0,
            user_manifest_key: user_manifest_key.0,
            root_verify_key: root_verify_key.0,
        }))
    }

    #[getter]
    fn human_handle(&self) -> PyResult<Option<HumanHandle>> {
        match &self.0.human_handle {
            Some(hh) => Ok(Some(HumanHandle(hh.clone()))),
            None => Ok(None),
        }
    }

    #[getter]
    fn device_label(&self) -> PyResult<Option<DeviceLabel>> {
        match &self.0.device_label {
            Some(dl) => Ok(Some(DeviceLabel(dl.clone()))),
            None => Ok(None),
        }
    }

    #[getter]
    fn device_id(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.device_id.clone()))
    }

    #[getter]
    fn profile(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            let cls = py.import("parsec.api.protocol")?.getattr("UserProfile")?;
            let name = self.0.profile.to_string();
            let obj = cls.getattr(&name as &str)?;
            Ok(obj.into_py(py))
        })
    }

    #[getter]
    fn root_verify_key(&self) -> PyResult<VerifyKey> {
        Ok(VerifyKey(self.0.root_verify_key.clone()))
    }

    #[getter]
    fn user_manifest_id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.user_manifest_id))
    }

    #[getter]
    fn user_manifest_key(&self) -> PyResult<SecretKey> {
        Ok(SecretKey(self.0.user_manifest_key.clone()))
    }

    #[getter]
    fn private_key(&self) -> PyResult<PrivateKey> {
        Ok(PrivateKey(self.0.private_key.clone()))
    }

    fn dump_and_encrypt<'p>(&self, py: Python<'p>, key: &SecretKey) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_encrypt(&key.0)))
    }

    #[classmethod]
    fn decrypt_and_load(_cls: &PyType, encrypted: &[u8], key: &SecretKey) -> PyResult<Self> {
        match libparsec::types::InviteDeviceConfirmation::decrypt_and_load(encrypted, &key.0) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
}
