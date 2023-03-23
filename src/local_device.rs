// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::{PyException, PyValueError},
    prelude::*,
    types::{PyBytes, PyDict, PyType},
};
use std::path::PathBuf;
use std::sync::Arc;

use libparsec::client_types;
use libparsec::platform_device_loader;

use crate::{
    addrs::BackendOrganizationAddr,
    api_crypto::{PrivateKey, PublicKey, SecretKey, SigningKey, VerifyKey},
    binding_utils::PathWrapper,
    enumerate::{DeviceFileType, UserProfile},
    ids::{DeviceID, DeviceLabel, DeviceName, EntryID, HumanHandle, OrganizationID, UserID},
    runtime::FutureIntoCoroutine,
    time::{DateTime, TimeProvider},
};

pub(crate) fn add_mod(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LocalDevice>()?;
    m.add_class::<UserInfo>()?;
    m.add_class::<DeviceInfo>()?;
    m.add_class::<AvailableDevice>()?;

    m.add_function(wrap_pyfunction!(save_device_with_password, m)?)?;
    m.add_function(wrap_pyfunction!(save_device_with_password_in_config, m)?)?;
    m.add_function(wrap_pyfunction!(change_device_password, m)?)?;
    m.add_function(wrap_pyfunction!(list_available_devices, m)?)?;
    m.add_function(wrap_pyfunction!(get_available_device, m)?)?;
    m.add_function(wrap_pyfunction!(load_recovery_device, m)?)?;
    m.add_function(wrap_pyfunction!(save_recovery_device, m)?)?;

    m.add("LocalDeviceError", py.get_type::<LocalDeviceErrorExc>())?;
    m.add(
        "LocalDeviceCryptoError",
        py.get_type::<LocalDeviceCryptoErrorExc>(),
    )?;
    m.add(
        "LocalDeviceValidationError",
        py.get_type::<LocalDeviceValidationErrorExc>(),
    )?;
    m.add(
        "LocalDeviceAlreadyExistsError",
        py.get_type::<LocalDeviceAlreadyExistsErrorExc>(),
    )?;
    m.add(
        "LocalDevicePackingError",
        py.get_type::<LocalDevicePackingErrorExc>(),
    )?;
    m.add(
        "LocalDeviceNotFoundError",
        py.get_type::<LocalDeviceNotFoundErrorExc>(),
    )?;

    Ok(())
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct LocalDevice(pub Arc<libparsec::client_types::LocalDevice>);

crate::binding_utils::gen_proto!(LocalDevice, __repr__);
crate::binding_utils::gen_proto!(LocalDevice, __copy__);
crate::binding_utils::gen_proto!(LocalDevice, __deepcopy__);
crate::binding_utils::gen_proto!(LocalDevice, __richcmp__, eq);

#[pymethods]
impl LocalDevice {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [
                organization_addr: BackendOrganizationAddr,
                "organization_addr"
            ],
            [device_id: DeviceID, "device_id"],
            [device_label: Option<DeviceLabel>, "device_label"],
            [human_handle: Option<HumanHandle>, "human_handle"],
            [signing_key: SigningKey, "signing_key"],
            [private_key: PrivateKey, "private_key"],
            [profile: UserProfile, "profile"],
            [user_manifest_id: EntryID, "user_manifest_id"],
            [user_manifest_key: SecretKey, "user_manifest_key"],
            [local_symkey: SecretKey, "local_symkey"],
            [time_provider: TimeProvider, "time_provider"],
        );
        crate::binding_utils::check_mandatory_kwargs!(
            [organization_addr, "organization_addr"],
            [device_id, "device_id"],
            [device_label, "device_label"],
            [human_handle, "human_handle"],
            [signing_key, "signing_key"],
            [private_key, "private_key"],
            [profile, "profile"],
            [user_manifest_id, "user_manifest_id"],
            [user_manifest_key, "user_manifest_key"],
            [local_symkey, "local_symkey"],
            // time_provider is not mandatory
        );

        let time_provider = time_provider.map(|tp| tp.0).unwrap_or_default();
        let local_device = libparsec::client_types::LocalDevice {
            organization_addr: organization_addr.0,
            device_id: device_id.0,
            device_label: device_label.map(|x| x.0),
            human_handle: human_handle.map(|x| x.0),
            signing_key: signing_key.0,
            private_key: private_key.0,
            profile: profile.0,
            user_manifest_id: user_manifest_id.0,
            user_manifest_key: user_manifest_key.0,
            local_symkey: local_symkey.0,
            time_provider,
        };
        Ok(Self(Arc::new(local_device)))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [
                organization_addr: BackendOrganizationAddr,
                "organization_addr"
            ],
            [device_id: DeviceID, "device_id"],
            [device_label: Option<DeviceLabel>, "device_label"],
            [human_handle: Option<HumanHandle>, "human_handle"],
            [signing_key: SigningKey, "signing_key"],
            [private_key: PrivateKey, "private_key"],
            [profile: UserProfile, "profile"],
            [user_manifest_id: EntryID, "user_manifest_id"],
            [user_manifest_key: SecretKey, "user_manifest_key"],
            [local_symkey: SecretKey, "local_symkey"],
            [time_provider: TimeProvider, "time_provider"],
        );

        let mut r = (*self.0).clone();

        if let Some(v) = organization_addr {
            r.organization_addr = v.0;
        }
        if let Some(v) = device_id {
            r.device_id = v.0;
        }
        if let Some(v) = device_label {
            r.device_label = v.map(|v| v.0);
        }
        if let Some(v) = human_handle {
            r.human_handle = v.map(|v| v.0);
        }
        if let Some(v) = signing_key {
            r.signing_key = v.0;
        }
        if let Some(v) = private_key {
            r.private_key = v.0;
        }
        if let Some(v) = profile {
            r.profile = v.0;
        }
        if let Some(v) = user_manifest_id {
            r.user_manifest_id = v.0;
        }
        if let Some(v) = user_manifest_key {
            r.user_manifest_key = v.0;
        }
        if let Some(v) = local_symkey {
            r.local_symkey = v.0;
        }
        if let Some(v) = time_provider {
            r.time_provider = v.0;
        }

        Ok(Self(Arc::new(r)))
    }

    #[allow(clippy::too_many_arguments)]
    #[classmethod]
    #[args(
        profil = "UserProfile::standard()",
        device_id = "None",
        human_handle = "None",
        device_label = "None",
        signing_key = "None",
        private_key = "None"
    )]
    fn generate_new_device(
        _cls: &PyType,
        organization_addr: BackendOrganizationAddr,
        profile: UserProfile,
        device_id: Option<DeviceID>,
        human_handle: Option<HumanHandle>,
        device_label: Option<DeviceLabel>,
        signing_key: Option<SigningKey>,
        private_key: Option<PrivateKey>,
    ) -> Self {
        let local_device = client_types::LocalDevice::generate_new_device(
            organization_addr.0,
            device_id.map(|d| d.0),
            profile.0,
            human_handle.map(|h| h.0),
            device_label.map(|d| d.0),
            signing_key.map(|s| s.0),
            private_key.map(|p| p.0),
        );
        Self(Arc::new(local_device))
    }

    #[getter]
    fn is_admin(&self) -> PyResult<bool> {
        Ok(self.0.profile == libparsec::types::UserProfile::Admin)
    }

    #[getter]
    fn is_outsider(&self) -> PyResult<bool> {
        Ok(self.0.profile == libparsec::types::UserProfile::Outsider)
    }

    #[getter]
    fn slug(&self) -> PyResult<String> {
        Ok(self.0.slug())
    }

    #[getter]
    fn slughash(&self) -> PyResult<String> {
        Ok(self.0.slughash())
    }

    #[classmethod]
    fn load_slug(_cls: &PyType, slug: &str) -> PyResult<(OrganizationID, DeviceID)> {
        libparsec::client_types::LocalDevice::load_slug(slug)
            .map(|(org_id, device_id)| (OrganizationID(org_id), DeviceID(device_id)))
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[classmethod]
    fn load_device_with_password(
        _cls: &PyType,
        key_file: PathBuf,
        password: &str,
    ) -> LocalDeviceResult<Self> {
        platform_device_loader::load_device_with_password_from_path(&key_file, password)
            .map(|local_device| LocalDevice(Arc::new(local_device)))
            .map_err(|e| e.into())
    }

    #[getter]
    fn root_verify_key(&self) -> PyResult<VerifyKey> {
        Ok(VerifyKey(self.0.root_verify_key().clone()))
    }

    #[getter]
    fn organization_id(&self) -> PyResult<OrganizationID> {
        Ok(OrganizationID(self.0.organization_id().clone()))
    }

    #[getter]
    fn device_name(&self) -> PyResult<DeviceName> {
        Ok(DeviceName(self.0.device_name().clone()))
    }

    #[getter]
    fn user_id(&self) -> PyResult<UserID> {
        Ok(UserID(self.0.user_id().clone()))
    }

    #[getter]
    fn verify_key(&self) -> PyResult<VerifyKey> {
        Ok(VerifyKey(self.0.verify_key()))
    }

    #[getter]
    fn public_key(&self) -> PyResult<PublicKey> {
        Ok(PublicKey(self.0.public_key()))
    }

    #[getter]
    fn user_display(&self) -> PyResult<&str> {
        Ok(self
            .0
            .human_handle
            .as_ref()
            .map(|hh| hh.as_ref())
            .unwrap_or_else(|| self.0.device_id.user_id().as_ref()))
    }

    #[getter]
    fn short_user_display(&self) -> PyResult<&str> {
        Ok(self
            .0
            .human_handle
            .as_ref()
            .map(|hh| hh.label())
            .unwrap_or_else(|| self.0.device_id.user_id().as_ref()))
    }

    #[getter]
    fn device_display(&self) -> PyResult<&str> {
        Ok(self
            .0
            .device_label
            .as_ref()
            .map(|dl| dl.as_ref())
            .unwrap_or_else(|| self.0.device_id.device_name().as_ref()))
    }

    #[getter]
    fn organization_addr(&self) -> PyResult<BackendOrganizationAddr> {
        Ok(BackendOrganizationAddr(self.0.organization_addr.clone()))
    }

    #[getter]
    fn device_id(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.device_id.clone()))
    }

    #[getter]
    fn device_label(&self) -> PyResult<Option<DeviceLabel>> {
        Ok(self.0.device_label.as_ref().map(|x| DeviceLabel(x.clone())))
    }

    #[getter]
    fn human_handle(&self) -> PyResult<Option<HumanHandle>> {
        Ok(self.0.human_handle.as_ref().map(|x| HumanHandle(x.clone())))
    }

    #[getter]
    fn signing_key(&self) -> PyResult<SigningKey> {
        Ok(SigningKey(self.0.signing_key.clone()))
    }

    #[getter]
    fn private_key(&self) -> PyResult<PrivateKey> {
        Ok(PrivateKey(self.0.private_key.clone()))
    }

    #[getter]
    fn profile(&self) -> PyResult<&'static PyObject> {
        Ok(UserProfile::from_profile(self.0.profile))
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
    fn local_symkey(&self) -> PyResult<SecretKey> {
        Ok(SecretKey(self.0.local_symkey.clone()))
    }

    #[getter]
    fn get_time_provider(&self) -> PyResult<TimeProvider> {
        Ok(TimeProvider(self.0.time_provider.clone()))
    }

    // TODO: rename this into `now`
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.now()))
    }

    fn dump<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump()))
    }

    #[classmethod]
    fn load(_cls: &PyType, encrypted: &[u8]) -> PyResult<Self> {
        libparsec::client_types::LocalDevice::load(encrypted)
            .map(|local_device| Self(Arc::new(local_device)))
            .map_err(PyValueError::new_err)
    }
}

#[pyclass]
#[derive(Clone)]
struct UserInfo(pub libparsec::client_types::UserInfo);

crate::binding_utils::gen_proto!(UserInfo, __repr__);
crate::binding_utils::gen_proto!(UserInfo, __copy__);
crate::binding_utils::gen_proto!(UserInfo, __deepcopy__);
crate::binding_utils::gen_proto!(UserInfo, __richcmp__, ord);
crate::binding_utils::gen_proto!(UserInfo, __hash__);

#[pymethods]
impl UserInfo {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [user_id: UserID, "user_id"],
            [human_handle: Option<HumanHandle>, "human_handle"],
            [profile: UserProfile, "profile"],
            [created_on: DateTime, "created_on"],
            [revoked_on: Option<DateTime>, "revoked_on"],
        );

        Ok(Self(libparsec::client_types::UserInfo {
            user_id: user_id.0,
            human_handle: human_handle.map(|x| x.0),
            profile: profile.0,
            created_on: created_on.0,
            revoked_on: revoked_on.map(|x| x.0),
        }))
    }

    #[getter]
    fn user_id(&self) -> PyResult<UserID> {
        Ok(UserID(self.0.user_id.clone()))
    }

    #[getter]
    fn human_handle(&self) -> PyResult<Option<HumanHandle>> {
        Ok(self.0.human_handle.as_ref().map(|x| HumanHandle(x.clone())))
    }

    #[getter]
    fn profile(&self) -> PyResult<&'static PyObject> {
        Ok(UserProfile::from_profile(self.0.profile))
    }

    #[getter]
    fn created_on(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.created_on))
    }

    #[getter]
    fn revoked_on(&self) -> PyResult<Option<DateTime>> {
        Ok(self.0.revoked_on.map(DateTime))
    }

    #[getter]
    fn user_display(&self) -> PyResult<&str> {
        Ok(self.0.user_display())
    }

    #[getter]
    fn short_user_display(&self) -> PyResult<&str> {
        Ok(self.0.short_user_display())
    }

    #[getter]
    fn is_revoked(&self) -> PyResult<bool> {
        Ok(self.0.is_revoked())
    }
}

#[pyclass]
#[derive(Clone)]
struct DeviceInfo(pub libparsec::client_types::DeviceInfo);

crate::binding_utils::gen_proto!(DeviceInfo, __repr__);
crate::binding_utils::gen_proto!(DeviceInfo, __copy__);
crate::binding_utils::gen_proto!(DeviceInfo, __deepcopy__);
crate::binding_utils::gen_proto!(DeviceInfo, __richcmp__, ord);
crate::binding_utils::gen_proto!(DeviceInfo, __hash__);

#[pymethods]
impl DeviceInfo {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [device_id: DeviceID, "device_id"],
            [device_label: Option<DeviceLabel>, "device_label"],
            [created_on: DateTime, "created_on"],
        );

        Ok(Self(libparsec::client_types::DeviceInfo {
            device_id: device_id.0,
            device_label: device_label.map(|x| x.0),
            created_on: created_on.0,
        }))
    }

    #[getter]
    fn device_id(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.device_id.clone()))
    }

    #[getter]
    fn device_label(&self) -> PyResult<Option<DeviceLabel>> {
        Ok(self.0.device_label.as_ref().map(|x| DeviceLabel(x.clone())))
    }

    #[getter]
    fn created_on(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.created_on))
    }

    #[getter]
    fn device_name(&self) -> PyResult<DeviceName> {
        Ok(DeviceName(self.0.device_name().clone()))
    }

    #[getter]
    fn device_display(&self) -> PyResult<&str> {
        Ok(self.0.device_display())
    }
}

pyo3::create_exception!(_parsec, LocalDeviceErrorExc, PyException);
pyo3::create_exception!(_parsec, LocalDeviceCryptoErrorExc, LocalDeviceErrorExc);
pyo3::create_exception!(_parsec, LocalDeviceValidationErrorExc, LocalDeviceErrorExc);
pyo3::create_exception!(
    _parsec,
    LocalDeviceAlreadyExistsErrorExc,
    LocalDeviceErrorExc
);
pyo3::create_exception!(_parsec, LocalDevicePackingErrorExc, LocalDeviceErrorExc);
pyo3::create_exception!(_parsec, LocalDeviceNotFoundErrorExc, LocalDeviceErrorExc);

#[derive(Debug)]
struct LocalDeviceError(client_types::LocalDeviceError);

impl std::fmt::Display for LocalDeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<client_types::LocalDeviceError> for LocalDeviceError {
    fn from(value: client_types::LocalDeviceError) -> Self {
        Self(value)
    }
}

impl From<LocalDeviceError> for PyErr {
    fn from(value: LocalDeviceError) -> Self {
        let internal = value.0;
        match internal {
            client_types::LocalDeviceError::CryptoError { exc } => {
                LocalDeviceCryptoErrorExc::new_err(exc.to_string())
            }
            client_types::LocalDeviceError::Validation { .. } => {
                LocalDeviceValidationErrorExc::new_err(internal.to_string())
            }
            client_types::LocalDeviceError::AlreadyExists(_) => {
                LocalDeviceAlreadyExistsErrorExc::new_err(internal.to_string())
            }
            client_types::LocalDeviceError::Deserialization(_) => {
                LocalDevicePackingErrorExc::new_err(internal.to_string())
            }
            client_types::LocalDeviceError::Access(_)
            | client_types::LocalDeviceError::InvalidSlug
            | client_types::LocalDeviceError::Serialization(_) => {
                LocalDeviceErrorExc::new_err(internal.to_string())
            }
        }
    }
}

type LocalDeviceResult<T> = Result<T, LocalDeviceError>;

#[pyfunction]
fn save_device_with_password(
    key_file: PathBuf,
    device: &LocalDevice,
    password: &str,
    force: bool,
) -> LocalDeviceResult<()> {
    platform_device_loader::save_device_with_password(&key_file, &device.0, password, force)
        .map_err(|e| e.into())
}

#[pyfunction]
fn save_device_with_password_in_config(
    config_dir: PathBuf,
    device: &LocalDevice,
    password: &str,
) -> LocalDeviceResult<PathBuf> {
    platform_device_loader::save_device_with_password_in_config(&config_dir, &device.0, password)
        .map_err(|e| e.into())
}

#[pyfunction]
fn change_device_password(
    key_file: PathBuf,
    old_password: &str,
    new_password: &str,
) -> LocalDeviceResult<()> {
    platform_device_loader::change_device_password(&key_file, old_password, new_password)
        .map_err(|e| e.into())
}

#[pyclass]
struct AvailableDevice(client_types::AvailableDevice);

#[pymethods]
impl AvailableDevice {
    #[new]
    fn new(
        key_file_path: PathBuf,
        organization_id: OrganizationID,
        device_id: DeviceID,
        human_handle: Option<HumanHandle>,
        device_label: Option<DeviceLabel>,
        slug: String,
        r#type: DeviceFileType,
    ) -> Self {
        Self(client_types::AvailableDevice {
            key_file_path,
            organization_id: organization_id.0,
            device_id: device_id.0,
            human_handle: human_handle.map(|h| h.0),
            device_label: device_label.map(|l| l.0),
            slug,
            ty: r#type.0,
        })
    }

    #[classmethod]
    fn load(_cls: &PyType, key_file_path: PathBuf) -> LocalDeviceResult<Self> {
        let value = platform_device_loader::load_available_device(key_file_path)?;
        Ok(Self(value))
    }

    #[getter]
    fn key_file_path(&self) -> PathWrapper {
        PathWrapper(self.0.key_file_path.to_path_buf())
    }

    #[getter]
    fn organization_id(&self) -> OrganizationID {
        OrganizationID(self.0.organization_id.clone())
    }

    #[getter]
    fn device_id(&self) -> DeviceID {
        DeviceID(self.0.device_id.clone())
    }

    #[getter]
    fn human_handle(&self) -> Option<HumanHandle> {
        self.0.human_handle.as_ref().map(|h| HumanHandle(h.clone()))
    }

    #[getter]
    fn device_label(&self) -> Option<DeviceLabel> {
        self.0.device_label.as_ref().map(|d| DeviceLabel(d.clone()))
    }

    #[getter]
    fn slug(&self) -> &str {
        &self.0.slug
    }

    #[getter]
    fn r#type(&self) -> DeviceFileType {
        DeviceFileType(self.0.ty)
    }

    #[getter]
    fn user_display(&self) -> &str {
        self.0.user_display()
    }

    #[getter]
    fn short_user_display(&self) -> &str {
        self.0.short_user_display()
    }

    #[getter]
    fn device_display(&self) -> &str {
        self.0.device_display()
    }

    #[getter]
    fn slughash(&self) -> String {
        self.0.slughash()
    }
}

crate::binding_utils::gen_proto!(AvailableDevice, __richcmp__, eq);
crate::binding_utils::gen_proto!(AvailableDevice, __hash__);

#[pyfunction]
fn list_available_devices(config_dir: PathBuf) -> LocalDeviceResult<Vec<AvailableDevice>> {
    platform_device_loader::list_available_devices_core(&config_dir)
        .map(|devices| devices.iter().map(|d| AvailableDevice(d.clone())).collect())
        .map_err(LocalDeviceError::from)
}

#[pyfunction]
fn get_available_device(config_dir: PathBuf, slug: &str) -> LocalDeviceResult<AvailableDevice> {
    platform_device_loader::get_available_device(&config_dir, slug)
        .map(AvailableDevice)
        .map_err(LocalDeviceError::from)
}

#[pyfunction]
fn load_recovery_device(key_file: PathBuf, password: String) -> FutureIntoCoroutine {
    FutureIntoCoroutine::from(async move {
        platform_device_loader::load_recovery_device(&key_file, &password)
            .await
            .map(Arc::new)
            .map(LocalDevice)
            .map_err(|e| LocalDeviceError::from(e).into())
    })
}

#[pyfunction]
fn save_recovery_device(
    key_file: PathBuf,
    device: LocalDevice,
    force: bool,
) -> FutureIntoCoroutine {
    FutureIntoCoroutine::from(async move {
        platform_device_loader::save_recovery_device(&key_file, &device.0, force)
            .await
            .map_err(|e| LocalDeviceError::from(e).into())
    })
}
