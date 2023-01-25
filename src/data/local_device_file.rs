// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::{PyAttributeError, PyValueError},
    prelude::*,
    types::{PyBytes, PyDict, PyType},
};

use crate::{
    binding_utils::BytesWrapper,
    enumerate::DeviceFileType,
    ids::{DeviceID, DeviceLabel, HumanHandle, OrganizationID},
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct DeviceFile(pub libparsec::client_types::DeviceFile);

crate::binding_utils::gen_proto!(DeviceFile, __repr__);
crate::binding_utils::gen_proto!(DeviceFile, __richcmp__, eq);

#[pymethods]
impl DeviceFile {
    #[new]
    #[args(
        salt = "None",
        encrypted_key = "None",
        certificate_id = "None",
        certificate_sha1 = "None",
        py_kwargs = "**"
    )]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [ty: DeviceFileType, "type"],
            [ciphertext: BytesWrapper, "ciphertext"],
            [human_handle: Option<HumanHandle>, "human_handle"],
            [device_label: Option<DeviceLabel>, "device_label"],
            [device_id: DeviceID, "device_id"],
            [organization_id: OrganizationID, "organization_id"],
            [slug: String, "slug"],
            [salt: Option<BytesWrapper>, "salt"],
            [encrypted_key: Option<BytesWrapper>, "encrypted_key"],
            [certificate_id: Option<String>, "certificate_id"],
            [certificate_sha1: Option<BytesWrapper>, "certificate_sha1"],
        );
        crate::binding_utils::unwrap_bytes!(ciphertext, salt, encrypted_key, certificate_sha1);

        let device = match ty {
            DeviceFileType(libparsec::client_types::DeviceFileType::Password) => {
                if encrypted_key.is_some() {
                    return Err(PyAttributeError::new_err(
                        "Found encrypted_key attribute for password",
                    ));
                } else if certificate_id.is_some() {
                    return Err(PyAttributeError::new_err(
                        "Found certificate_id attribute for password",
                    ));
                } else if certificate_sha1.is_some() {
                    return Err(PyAttributeError::new_err(
                        "Found certificate_sha1 attribute for password",
                    ));
                }

                libparsec::client_types::DeviceFile::Password(
                    libparsec::client_types::DeviceFilePassword {
                        ciphertext,
                        human_handle: human_handle.map(|x| x.0),
                        device_label: device_label.map(|x| x.0),
                        device_id: device_id.0,
                        organization_id: organization_id.0,
                        slug,
                        salt: salt
                            .ok_or_else(|| PyAttributeError::new_err("Missing salt attribute"))?,
                    },
                )
            }
            DeviceFileType(libparsec::client_types::DeviceFileType::Recovery) => {
                if salt.is_some() {
                    return Err(PyAttributeError::new_err(
                        "Found salt attribute for recovery",
                    ));
                } else if encrypted_key.is_some() {
                    return Err(PyAttributeError::new_err(
                        "Found encrypted_key attribute for recovery",
                    ));
                } else if certificate_id.is_some() {
                    return Err(PyAttributeError::new_err(
                        "Found certificate_id attribute for recovery",
                    ));
                } else if certificate_sha1.is_some() {
                    return Err(PyAttributeError::new_err(
                        "Found certificate_sha1 attribute for recovery",
                    ));
                }

                libparsec::client_types::DeviceFile::Recovery(
                    libparsec::client_types::DeviceFileRecovery {
                        ciphertext,
                        human_handle: human_handle.map(|x| x.0),
                        device_label: device_label.map(|x| x.0),
                        device_id: device_id.0,
                        organization_id: organization_id.0,
                        slug,
                    },
                )
            }
            DeviceFileType(libparsec::client_types::DeviceFileType::Smartcard) => {
                if salt.is_some() {
                    return Err(PyAttributeError::new_err(
                        "Found salt attribute for smartcard",
                    ));
                }

                libparsec::client_types::DeviceFile::Smartcard(
                    libparsec::client_types::DeviceFileSmartcard {
                        ciphertext,
                        human_handle: human_handle.map(|x| x.0),
                        device_label: device_label.map(|x| x.0),
                        device_id: device_id.0,
                        organization_id: organization_id.0,
                        slug,
                        encrypted_key: encrypted_key.ok_or_else(|| {
                            PyAttributeError::new_err("Missing encrypted_key attribute")
                        })?,
                        certificate_id: certificate_id.ok_or_else(|| {
                            PyAttributeError::new_err("Missing certificate_id attribute")
                        })?,
                        certificate_sha1,
                    },
                )
            }
        };

        Ok(Self(device))
    }

    #[getter]
    fn r#type(&self) -> &'static pyo3::Py<pyo3::PyAny> {
        match self.0 {
            libparsec::client_types::DeviceFile::Password(_) => DeviceFileType::password(),
            libparsec::client_types::DeviceFile::Recovery(_) => DeviceFileType::recovery(),
            libparsec::client_types::DeviceFile::Smartcard(_) => DeviceFileType::smartcard(),
        }
    }

    #[getter]
    fn ciphertext<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(
            py,
            match &self.0 {
                libparsec::client_types::DeviceFile::Password(device) => &device.ciphertext,
                libparsec::client_types::DeviceFile::Recovery(device) => &device.ciphertext,
                libparsec::client_types::DeviceFile::Smartcard(device) => &device.ciphertext,
            },
        )
    }

    #[getter]
    fn human_handle(&self) -> Option<HumanHandle> {
        match &self.0 {
            libparsec::client_types::DeviceFile::Password(device) => {
                device.human_handle.clone().map(HumanHandle)
            }
            libparsec::client_types::DeviceFile::Recovery(device) => {
                device.human_handle.clone().map(HumanHandle)
            }
            libparsec::client_types::DeviceFile::Smartcard(device) => {
                device.human_handle.clone().map(HumanHandle)
            }
        }
    }

    #[getter]
    fn device_label(&self) -> Option<DeviceLabel> {
        match &self.0 {
            libparsec::client_types::DeviceFile::Password(device) => {
                device.device_label.clone().map(DeviceLabel)
            }
            libparsec::client_types::DeviceFile::Recovery(device) => {
                device.device_label.clone().map(DeviceLabel)
            }
            libparsec::client_types::DeviceFile::Smartcard(device) => {
                device.device_label.clone().map(DeviceLabel)
            }
        }
    }

    #[getter]
    fn device_id(&self) -> DeviceID {
        DeviceID(match &self.0 {
            libparsec::client_types::DeviceFile::Password(device) => device.device_id.clone(),
            libparsec::client_types::DeviceFile::Recovery(device) => device.device_id.clone(),
            libparsec::client_types::DeviceFile::Smartcard(device) => device.device_id.clone(),
        })
    }

    #[getter]
    fn organization_id(&self) -> OrganizationID {
        OrganizationID(match &self.0 {
            libparsec::client_types::DeviceFile::Password(device) => device.organization_id.clone(),
            libparsec::client_types::DeviceFile::Recovery(device) => device.organization_id.clone(),
            libparsec::client_types::DeviceFile::Smartcard(device) => {
                device.organization_id.clone()
            }
        })
    }

    #[getter]
    fn slug(&self) -> &str {
        match &self.0 {
            libparsec::client_types::DeviceFile::Password(device) => &device.slug,
            libparsec::client_types::DeviceFile::Recovery(device) => &device.slug,
            libparsec::client_types::DeviceFile::Smartcard(device) => &device.slug,
        }
    }

    #[getter]
    fn salt<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            match &self.0 {
                libparsec::client_types::DeviceFile::Password(device) => &device.salt,
                _ => return Err(PyAttributeError::new_err("No such attribute")),
            },
        ))
    }

    #[getter]
    fn encrypted_key<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            match &self.0 {
                libparsec::client_types::DeviceFile::Smartcard(device) => &device.encrypted_key,
                _ => return Err(PyAttributeError::new_err("No such attribute")),
            },
        ))
    }

    #[getter]
    fn certificate_id(&self) -> PyResult<&str> {
        Ok(match &self.0 {
            libparsec::client_types::DeviceFile::Smartcard(device) => &device.certificate_id,
            _ => return Err(PyAttributeError::new_err("No such attribute")),
        })
    }

    #[getter]
    fn certificate_sha1<'py>(&self, py: Python<'py>) -> PyResult<Option<&'py PyBytes>> {
        Ok(match &self.0 {
            libparsec::client_types::DeviceFile::Smartcard(device) => device
                .certificate_sha1
                .as_ref()
                .map(|x| PyBytes::new(py, x)),
            _ => return Err(PyAttributeError::new_err("No such attribute")),
        })
    }

    #[classmethod]
    fn load(_cls: &PyType, data: &[u8]) -> PyResult<Self> {
        libparsec::client_types::DeviceFile::load(data)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn dump<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump())
    }
}
