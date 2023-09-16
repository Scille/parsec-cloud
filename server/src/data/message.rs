// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::PyNotImplementedError,
    pyclass, pymethods,
    types::{PyBytes, PyString, PyType},
    PyObject, PyRef, PyResult, Python,
};

use crate::{
    DataExc, DateTime, DeviceID, EntryName, PrivateKey, PublicKey, SecretKey, SigningKey,
    VerifyKey, VlobID,
};

use libparsec::low_level::types::IndexInt;

#[pyclass(subclass)]
#[derive(Clone)]
pub(crate) struct MessageContent(pub libparsec::low_level::types::MessageContent);

impl From<libparsec::low_level::types::MessageContent> for MessageContent {
    fn from(value: libparsec::low_level::types::MessageContent) -> Self {
        Self(value)
    }
}

crate::binding_utils::gen_proto!(MessageContent, __repr__);
crate::binding_utils::gen_proto!(MessageContent, __copy__);
crate::binding_utils::gen_proto!(MessageContent, __deepcopy__);
crate::binding_utils::gen_proto!(MessageContent, __richcmp__, eq);

#[pymethods]
impl MessageContent {
    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(match &self.0 {
            libparsec::low_level::types::MessageContent::SharingGranted { author, .. } => {
                DeviceID(author.clone())
            }
            libparsec::low_level::types::MessageContent::SharingReencrypted { author, .. } => {
                DeviceID(author.clone())
            }
            libparsec::low_level::types::MessageContent::SharingRevoked { author, .. } => {
                DeviceID(author.clone())
            }
            libparsec::low_level::types::MessageContent::Ping { author, .. } => {
                DeviceID(author.clone())
            }
        })
    }

    #[getter]
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(match self.0 {
            libparsec::low_level::types::MessageContent::SharingGranted { timestamp, .. } => {
                DateTime(timestamp)
            }
            libparsec::low_level::types::MessageContent::SharingReencrypted {
                timestamp, ..
            } => DateTime(timestamp),
            libparsec::low_level::types::MessageContent::SharingRevoked { timestamp, .. } => {
                DateTime(timestamp)
            }
            libparsec::low_level::types::MessageContent::Ping { timestamp, .. } => {
                DateTime(timestamp)
            }
        })
    }

    #[classmethod]
    fn decrypt_verify_and_load_for(
        _cls: &PyType,
        ciphered: &[u8],
        recipient_privkey: &PrivateKey,
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_timestamp: DateTime,
        py: Python,
    ) -> PyResult<PyObject> {
        let msg = libparsec::low_level::types::MessageContent::decrypt_verify_and_load_for(
            ciphered,
            &recipient_privkey.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp.0,
        )
        .map_err(DataExc::from)?;

        Ok(match msg {
            libparsec::low_level::types::MessageContent::SharingGranted { .. } => {
                crate::binding_utils::py_object!(msg, Self, SharingGrantedMessageContent, py)?
            }
            libparsec::low_level::types::MessageContent::SharingReencrypted { .. } => {
                crate::binding_utils::py_object!(msg, Self, SharingReencryptedMessageContent, py)?
            }
            libparsec::low_level::types::MessageContent::SharingRevoked { .. } => {
                crate::binding_utils::py_object!(msg, Self, SharingRevokedMessageContent, py)?
            }
            libparsec::low_level::types::MessageContent::Ping { .. } => {
                crate::binding_utils::py_object!(msg, Self, PingMessageContent, py)?
            }
        })
    }

    fn dump_sign_and_encrypt_for<'py>(
        &self,
        author_signkey: &SigningKey,
        recipient_pubkey: &PublicKey,
        py: Python<'py>,
    ) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self
                .0
                .dump_sign_and_encrypt_for(&author_signkey.0, &recipient_pubkey.0),
        ))
    }
}

#[pyclass(extends=MessageContent)]
pub(crate) struct SharingGrantedMessageContent;

#[pymethods]
impl SharingGrantedMessageContent {
    #[new]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        name: EntryName,
        id: VlobID,
        encryption_revision: IndexInt,
        encrypted_on: DateTime,
        key: SecretKey,
    ) -> PyResult<(Self, MessageContent)> {
        Ok((
            Self,
            MessageContent(
                libparsec::low_level::types::MessageContent::SharingGranted {
                    author: author.0,
                    timestamp: timestamp.0,
                    name: name.0,
                    id: id.0,
                    encryption_revision,
                    encrypted_on: encrypted_on.0,
                    key: key.0,
                },
            ),
        ))
    }

    #[getter]
    fn name(_self: PyRef<'_, Self>) -> PyResult<EntryName> {
        Ok(match &_self.as_ref().0 {
            libparsec::low_level::types::MessageContent::SharingGranted { name, .. } => {
                EntryName(name.clone())
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn id(_self: PyRef<'_, Self>) -> PyResult<VlobID> {
        Ok(match _self.as_ref().0 {
            libparsec::low_level::types::MessageContent::SharingGranted { id, .. } => VlobID(id),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn encryption_revision(_self: PyRef<'_, Self>) -> PyResult<IndexInt> {
        Ok(match _self.as_ref().0 {
            libparsec::low_level::types::MessageContent::SharingGranted {
                encryption_revision,
                ..
            } => encryption_revision,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn encrypted_on(_self: PyRef<'_, Self>) -> PyResult<DateTime> {
        Ok(match _self.as_ref().0 {
            libparsec::low_level::types::MessageContent::SharingGranted {
                encrypted_on, ..
            } => DateTime(encrypted_on),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn key(_self: PyRef<'_, Self>) -> PyResult<SecretKey> {
        Ok(match &_self.as_ref().0 {
            libparsec::low_level::types::MessageContent::SharingGranted { key, .. } => {
                SecretKey(key.clone())
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass(extends=MessageContent)]
pub(crate) struct SharingReencryptedMessageContent;

#[pymethods]
impl SharingReencryptedMessageContent {
    #[new]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        name: EntryName,
        id: VlobID,
        encryption_revision: IndexInt,
        encrypted_on: DateTime,
        key: SecretKey,
    ) -> PyResult<(Self, MessageContent)> {
        Ok((
            Self,
            MessageContent(
                libparsec::low_level::types::MessageContent::SharingReencrypted {
                    author: author.0,
                    timestamp: timestamp.0,
                    name: name.0,
                    id: id.0,
                    encryption_revision,
                    encrypted_on: encrypted_on.0,
                    key: key.0,
                },
            ),
        ))
    }

    #[getter]
    fn name(_self: PyRef<'_, Self>) -> PyResult<EntryName> {
        Ok(match &_self.as_ref().0 {
            libparsec::low_level::types::MessageContent::SharingReencrypted { name, .. } => {
                EntryName(name.clone())
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn id(_self: PyRef<'_, Self>) -> PyResult<VlobID> {
        Ok(match _self.as_ref().0 {
            libparsec::low_level::types::MessageContent::SharingReencrypted { id, .. } => {
                VlobID(id)
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn encryption_revision(_self: PyRef<'_, Self>) -> PyResult<IndexInt> {
        Ok(match _self.as_ref().0 {
            libparsec::low_level::types::MessageContent::SharingReencrypted {
                encryption_revision,
                ..
            } => encryption_revision,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn encrypted_on(_self: PyRef<'_, Self>) -> PyResult<DateTime> {
        Ok(match _self.as_ref().0 {
            libparsec::low_level::types::MessageContent::SharingReencrypted {
                encrypted_on,
                ..
            } => DateTime(encrypted_on),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn key(_self: PyRef<'_, Self>) -> PyResult<SecretKey> {
        Ok(match &_self.as_ref().0 {
            libparsec::low_level::types::MessageContent::SharingReencrypted { key, .. } => {
                SecretKey(key.clone())
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass(extends=MessageContent)]
pub(crate) struct SharingRevokedMessageContent;

#[pymethods]
impl SharingRevokedMessageContent {
    #[new]
    fn new(author: DeviceID, timestamp: DateTime, id: VlobID) -> PyResult<(Self, MessageContent)> {
        Ok((
            Self,
            MessageContent(
                libparsec::low_level::types::MessageContent::SharingRevoked {
                    author: author.0,
                    timestamp: timestamp.0,
                    id: id.0,
                },
            ),
        ))
    }

    #[getter]
    fn id(_self: PyRef<'_, Self>) -> PyResult<VlobID> {
        Ok(match _self.as_ref().0 {
            libparsec::low_level::types::MessageContent::SharingRevoked { id, .. } => VlobID(id),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass(extends=MessageContent)]
pub(crate) struct PingMessageContent;

#[pymethods]
impl PingMessageContent {
    #[new]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        ping: String,
    ) -> PyResult<(Self, MessageContent)> {
        Ok((
            Self,
            MessageContent(libparsec::low_level::types::MessageContent::Ping {
                author: author.0,
                timestamp: timestamp.0,
                ping,
            }),
        ))
    }

    #[getter]
    fn ping<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyString> {
        Ok(match &_self.as_ref().0 {
            libparsec::low_level::types::MessageContent::Ping { ping, .. } => {
                PyString::new(py, ping)
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}
