// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyBytes, PyDict, PyList, PyString, PyTuple, PyType};
use uuid::Uuid;

use crate::binding_utils::{comp_op, hash_generic, kwargs_extract_required};
use crate::crypto::{PrivateKey, PublicKey, SecretKey, VerifyKey};
use crate::ids::{DeviceID, DeviceLabel, EntryID, HumanHandle};

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct InvitationToken(pub parsec_api_types::InvitationToken);

#[pymethods]
impl InvitationToken {
    #[new]
    pub fn new(uuid: &PyAny) -> PyResult<Self> {
        match Uuid::parse_str(uuid.getattr("hex")?.extract::<&str>()?) {
            Ok(u) => Ok(Self(parsec_api_types::InvitationToken::from(u))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[getter]
    fn uuid<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let uuid = PyModule::import(py, "uuid")?;
        let kwargs = vec![("hex", self.hex().unwrap())].into_py_dict(py);
        match uuid.getattr("UUID")?.call((), Some(kwargs)) {
            Ok(any) => Ok(any),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(parsec_api_types::InvitationToken::default()))
    }

    #[getter]
    fn bytes<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_bytes()))
    }

    #[getter]
    fn hex(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __richcmp__(&self, py: Python, other: &InvitationToken, op: CompareOp) -> PyResult<bool> {
        let h1 = self.__hash__(py).unwrap();
        let h2 = other.__hash__(py).unwrap();
        comp_op(op, h1, h2)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.as_hyphenated())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<EntryID '{}'>", self.0.as_hyphenated()))
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.as_hyphenated(), py)
    }

    #[classmethod]
    fn from_bytes(_cls: &PyType, bytes: &PyBytes) -> PyResult<Self> {
        let b = bytes.as_bytes();
        match uuid::Uuid::from_slice(b) {
            Ok(uuid) => Ok(Self(parsec_api_types::InvitationToken::from(uuid))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[classmethod]
    fn from_hex(_cls: &PyType, hex: &PyString) -> PyResult<Self> {
        match hex.to_string().parse::<parsec_api_types::InvitationToken>() {
            Ok(entry_id) => Ok(Self(entry_id)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
}

/*
* SASCode
*/

#[pyclass]
#[derive(PartialEq, Eq)]
pub struct SASCode(pub parsec_api_types::SASCode);

#[pymethods]
impl SASCode {
    #[new]
    pub fn new(code: &str) -> PyResult<Self> {
        match code.parse::<parsec_api_types::SASCode>() {
            Ok(sas) => Ok(Self(sas)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    pub fn from_int(_cls: &PyType, num: u32) -> PyResult<Self> {
        match parsec_api_types::SASCode::try_from(num) {
            Ok(sas) => Ok(Self(sas)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    fn __richcmp__(&self, py: Python, other: &SASCode, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            CompareOp::Lt => (self.0 < other.0).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.to_string(), py)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<SASCode {}>", self.0))
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}

#[pyfunction]
pub fn generate_sas_codes<'p>(
    py: Python<'p>,
    claimer_nonce: &PyBytes,
    greeter_nonce: &PyBytes,
    shared_secret_key: &SecretKey,
) -> PyResult<&'p PyTuple> {
    let (claimer_sas, greeter_sas) = parsec_api_types::SASCode::generate_sas_codes(
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
pub fn generate_sas_code_candidates<'p>(
    py: Python<'p>,
    valid_sas: &SASCode,
    size: usize,
) -> PyResult<&'p PyList> {
    let candidates: Vec<parsec_api_types::SASCode> = valid_sas.0.generate_sas_code_candidates(size);
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
#[derive(PartialEq, Eq)]
pub struct InviteUserData(pub parsec_api_types::InviteUserData);

#[pymethods]
impl InviteUserData {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let public_key = kwargs_extract_required::<PublicKey>(args, "public_key")?;
        let verify_key = kwargs_extract_required::<VerifyKey>(args, "verify_key")?;
        /*
            Both `requested_device_label` and `requested_human_handle` are marked as optional, but they have to be set
            explicitly to None by the user. That's why we're using `kwargs_extract_required` and extracting an Option, rather
            that `kwargs_extract_optional`.
        */
        let requested_device_label =
            match kwargs_extract_required::<Option<DeviceLabel>>(args, "requested_device_label")? {
                Some(rdl) => Some(rdl.0),
                None => None,
            };
        let requested_human_handle =
            match kwargs_extract_required::<Option<HumanHandle>>(args, "requested_human_handle")? {
                Some(rhh) => Some(rhh.0),
                None => None,
            };
        Ok(Self(parsec_api_types::InviteUserData {
            requested_device_label,
            requested_human_handle,
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
        match parsec_api_types::InviteUserData::decrypt_and_load(encrypted, &key.0) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
}

/*
* InviteUserConfirmation
*/

#[pyclass]
#[derive(PartialEq, Eq)]
pub struct InviteUserConfirmation(pub parsec_api_types::InviteUserConfirmation);

#[pymethods]
impl InviteUserConfirmation {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let device_id = kwargs_extract_required::<DeviceID>(args, "device_id")?;

        /*
            Both `device_label` and `human_handle` are marked as optional, but they have to be set
            explicitly to None by the user. That's why we're using `kwargs_extract_required` and extracting an Option, rather
            that `kwargs_extract_optional`.
        */
        let device_label =
            match kwargs_extract_required::<Option<DeviceLabel>>(args, "device_label")? {
                Some(rdl) => Some(rdl.0),
                None => None,
            };
        let human_handle =
            match kwargs_extract_required::<Option<HumanHandle>>(args, "human_handle")? {
                Some(rhh) => Some(rhh.0),
                None => None,
            };

        let profile = match args.get_item("profile") {
            Some(p) => {
                match p
                    .getattr("name")?
                    .extract::<&str>()?
                    .parse::<parsec_api_types::UserProfile>()
                {
                    Ok(p) => p,
                    Err(err) => return Err(PyValueError::new_err(err)),
                }
            }
            None => return Err(PyValueError::new_err("Missing `profile` argument")),
        };

        let root_verify_key = kwargs_extract_required::<VerifyKey>(args, "root_verify_key")?;

        Ok(Self(parsec_api_types::InviteUserConfirmation {
            device_id: device_id.0,
            device_label,
            human_handle,
            profile,
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
            let obj = cls.getattr(name)?;
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
        match parsec_api_types::InviteUserConfirmation::decrypt_and_load(encrypted, &key.0) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
}

/*
* InviteDeviceData
*/

#[pyclass]
#[derive(PartialEq, Eq)]
pub struct InviteDeviceData(pub parsec_api_types::InviteDeviceData);

#[pymethods]
impl InviteDeviceData {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let requested_device_label =
            match kwargs_extract_required::<Option<DeviceLabel>>(args, "requested_device_label")? {
                Some(rdl) => Some(rdl.0),
                None => None,
            };

        let verify_key = kwargs_extract_required::<VerifyKey>(args, "verify_key")?;

        Ok(Self(parsec_api_types::InviteDeviceData {
            requested_device_label,
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
        match parsec_api_types::InviteDeviceData::decrypt_and_load(encrypted, &key.0) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
}

/*
* InviteDeviceConfirmation
*/

#[pyclass]
#[derive(PartialEq, Eq)]
pub struct InviteDeviceConfirmation(pub parsec_api_types::InviteDeviceConfirmation);

#[pymethods]
impl InviteDeviceConfirmation {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let device_id = kwargs_extract_required::<DeviceID>(args, "device_id")?;

        /*
            Both `device_label` and `human_handle` are marked as optional, but they have to be set
            explicitly to None by the user. That's why we're using `kwargs_extract_required` and extracting an Option, rather
            that `kwargs_extract_optional`.
        */
        let device_label =
            match kwargs_extract_required::<Option<DeviceLabel>>(args, "device_label")? {
                Some(rdl) => Some(rdl.0),
                None => None,
            };
        let human_handle =
            match kwargs_extract_required::<Option<HumanHandle>>(args, "human_handle")? {
                Some(rhh) => Some(rhh.0),
                None => None,
            };

        let profile = match args.get_item("profile") {
            Some(p) => {
                match p
                    .getattr("name")?
                    .extract::<&str>()?
                    .parse::<parsec_api_types::UserProfile>()
                {
                    Ok(p) => p,
                    Err(err) => return Err(PyValueError::new_err(err)),
                }
            }
            None => return Err(PyValueError::new_err("Missing `profile` argument")),
        };
        let private_key = kwargs_extract_required::<PrivateKey>(args, "private_key")?;
        let root_verify_key = kwargs_extract_required::<VerifyKey>(args, "root_verify_key")?;
        let user_manifest_id = kwargs_extract_required::<EntryID>(args, "user_manifest_id")?;
        let user_manifest_key = kwargs_extract_required::<SecretKey>(args, "user_manifest_key")?;

        Ok(Self(parsec_api_types::InviteDeviceConfirmation {
            device_id: device_id.0,
            device_label,
            human_handle,
            profile,
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
            let obj = cls.getattr(name)?;
            Ok(obj.into_py(py))
        })
    }

    #[getter]
    fn root_verify_key(&self) -> PyResult<VerifyKey> {
        Ok(VerifyKey(self.0.root_verify_key.clone()))
    }

    #[getter]
    fn user_manifest_id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.user_manifest_id.clone()))
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
        match parsec_api_types::InviteDeviceConfirmation::decrypt_and_load(encrypted, &key.0) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
}
