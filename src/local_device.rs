// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::{PyException, PyValueError},
    prelude::*,
    types::{PyBytes, PyDict, PyType},
};
use std::path::PathBuf;

use libparsec::client_types;

use crate::{
    addrs::BackendOrganizationAddr,
    api_crypto::{PrivateKey, PublicKey, SecretKey, SigningKey, VerifyKey},
    enumerate::UserProfile,
    ids::{DeviceID, DeviceLabel, DeviceName, EntryID, HumanHandle, OrganizationID, UserID},
    time::{DateTime, TimeProvider},
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct LocalDevice(pub libparsec::client_types::LocalDevice);

crate::binding_utils::gen_proto!(LocalDevice, __repr__);
crate::binding_utils::gen_proto!(LocalDevice, __richcmp__, eq);

#[pymethods]
impl LocalDevice {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
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
        );

        Ok(Self(libparsec::client_types::LocalDevice {
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
            time_provider: libparsec::types::TimeProvider::default(),
        }))
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
        );

        let mut r = self.0.clone();

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

        Ok(Self(r))
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

    #[setter]
    fn set_time_provider(&mut self, value: TimeProvider) -> PyResult<()> {
        self.0.time_provider = value.0;
        Ok(())
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
        match libparsec::client_types::LocalDevice::load(encrypted) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct UserInfo(pub libparsec::client_types::UserInfo);

crate::binding_utils::gen_proto!(UserInfo, __repr__);
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
pub(crate) struct DeviceInfo(pub libparsec::client_types::DeviceInfo);

crate::binding_utils::gen_proto!(DeviceInfo, __repr__);
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

crate::binding_utils::create_exception!(LocalDevice, PyException, client_types::LocalDeviceError);

#[pyfunction]
pub(crate) fn save_device_with_password(
    key_file: PathBuf,
    device: &LocalDevice,
    password: &str,
    force: bool,
) -> LocalDeviceResult<()> {
    client_types::save_device_with_password(&key_file, &device.0, password, force)
        .map_err(|e| e.into())
}

#[pyfunction]
pub(crate) fn save_device_with_password_in_config(
    config_dir: PathBuf,
    device: &LocalDevice,
    password: &str,
) -> LocalDeviceResult<PathBuf> {
    client_types::save_device_with_password_in_config(&config_dir, &device.0, password)
        .map_err(|e| e.into())
}

#[pyfunction]
pub(crate) fn change_device_password(
    key_file: PathBuf,
    old_password: &str,
    new_password: &str,
) -> LocalDeviceResult<()> {
    client_types::change_device_password(&key_file, old_password, new_password)
        .map_err(|e| e.into())
}
