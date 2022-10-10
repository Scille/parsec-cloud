use pyo3::{
    exceptions::{PyNotImplementedError, PyValueError},
    pyclass,
    pyclass_init::PyObjectInit,
    pymethods,
    types::{PyBytes, PyString, PyType},
    PyClassInitializer, PyObject, PyRef, PyResult, PyTypeInfo, Python,
};

use crate::{
    api_crypto::{PrivateKey, PublicKey, SecretKey, SigningKey, VerifyKey},
    ids::{DeviceID, EntryID},
    manifest::EntryName,
    time::DateTime,
};

#[pyclass(subclass)]
#[derive(Clone)]
pub(crate) struct MessageContent(pub libparsec::types::MessageContent);

crate::binding_utils::gen_proto!(MessageContent, __repr__);
crate::binding_utils::gen_proto!(MessageContent, __richcmp__, eq);

#[pymethods]
impl MessageContent {
    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(match &self.0 {
            libparsec::types::MessageContent::SharingGranted { author, .. } => {
                DeviceID(author.clone())
            }
            libparsec::types::MessageContent::SharingReencrypted { author, .. } => {
                DeviceID(author.clone())
            }
            libparsec::types::MessageContent::SharingRevoked { author, .. } => {
                DeviceID(author.clone())
            }
            libparsec::types::MessageContent::Ping { author, .. } => DeviceID(author.clone()),
        })
    }

    #[getter]
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(match self.0 {
            libparsec::types::MessageContent::SharingGranted { timestamp, .. } => {
                DateTime(timestamp)
            }
            libparsec::types::MessageContent::SharingReencrypted { timestamp, .. } => {
                DateTime(timestamp)
            }
            libparsec::types::MessageContent::SharingRevoked { timestamp, .. } => {
                DateTime(timestamp)
            }
            libparsec::types::MessageContent::Ping { timestamp, .. } => DateTime(timestamp),
        })
    }

    #[classmethod]
    pub fn decrypt_verify_and_load_for(
        _cls: &PyType,
        ciphered: &[u8],
        recipient_privkey: &PrivateKey,
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_timestamp: DateTime,
        py: Python,
    ) -> PyResult<PyObject> {
        let msg = libparsec::types::MessageContent::decrypt_verify_and_load_for(
            ciphered,
            &recipient_privkey.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp.0,
        );

        Ok(match msg {
            Ok(msg @ libparsec::types::MessageContent::SharingGranted { .. }) => {
                let initializer =
                    PyClassInitializer::from((SharingGrantedMessageContent, Self(msg)));
                // This block is safe because `initializer` has the correct type
                unsafe {
                    let ptr = initializer
                        .into_new_object(py, SharingGrantedMessageContent::type_object_raw(py))?;
                    PyObject::from_owned_ptr(py, ptr)
                }
            }
            Ok(msg @ libparsec::types::MessageContent::SharingReencrypted { .. }) => {
                let initializer =
                    PyClassInitializer::from((SharingReencryptedMessageContent, Self(msg)));
                // This block is safe because `initializer` has the correct type
                unsafe {
                    let ptr = initializer.into_new_object(
                        py,
                        SharingReencryptedMessageContent::type_object_raw(py),
                    )?;
                    PyObject::from_owned_ptr(py, ptr)
                }
            }
            Ok(msg @ libparsec::types::MessageContent::SharingRevoked { .. }) => {
                let initializer =
                    PyClassInitializer::from((SharingRevokedMessageContent, Self(msg)));
                // This block is safe because `initializer` has the correct type
                unsafe {
                    let ptr = initializer
                        .into_new_object(py, SharingRevokedMessageContent::type_object_raw(py))?;
                    PyObject::from_owned_ptr(py, ptr)
                }
            }
            Ok(msg @ libparsec::types::MessageContent::Ping { .. }) => {
                let initializer = PyClassInitializer::from((PingMessageContent, Self(msg)));
                // This block is safe because `initializer` has the correct type
                unsafe {
                    let ptr =
                        initializer.into_new_object(py, PingMessageContent::type_object_raw(py))?;
                    PyObject::from_owned_ptr(py, ptr)
                }
            }
            Err(err) => return Err(PyValueError::new_err(err)),
        })
    }

    pub fn dump_sign_and_encrypt_for<'py>(
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
        id: EntryID,
        encryption_revision: u32,
        encrypted_on: DateTime,
        key: SecretKey,
    ) -> PyResult<(Self, MessageContent)> {
        Ok((
            Self,
            MessageContent(libparsec::types::MessageContent::SharingGranted {
                author: author.0,
                timestamp: timestamp.0,
                name: name.0,
                id: id.0,
                encryption_revision,
                encrypted_on: encrypted_on.0,
                key: key.0,
            }),
        ))
    }

    #[getter]
    fn name(_self: PyRef<'_, Self>) -> PyResult<EntryName> {
        Ok(match &_self.as_ref().0 {
            libparsec::types::MessageContent::SharingGranted { name, .. } => {
                EntryName(name.clone())
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn id(_self: PyRef<'_, Self>) -> PyResult<EntryID> {
        Ok(match _self.as_ref().0 {
            libparsec::types::MessageContent::SharingGranted { id, .. } => EntryID(id),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn encryption_revision(_self: PyRef<'_, Self>) -> PyResult<u32> {
        Ok(match _self.as_ref().0 {
            libparsec::types::MessageContent::SharingGranted {
                encryption_revision,
                ..
            } => encryption_revision,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn encrypted_on(_self: PyRef<'_, Self>) -> PyResult<DateTime> {
        Ok(match _self.as_ref().0 {
            libparsec::types::MessageContent::SharingGranted { encrypted_on, .. } => {
                DateTime(encrypted_on)
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn key(_self: PyRef<'_, Self>) -> PyResult<SecretKey> {
        Ok(match &_self.as_ref().0 {
            libparsec::types::MessageContent::SharingGranted { key, .. } => SecretKey(key.clone()),
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
        id: EntryID,
        encryption_revision: u32,
        encrypted_on: DateTime,
        key: SecretKey,
    ) -> PyResult<(Self, MessageContent)> {
        Ok((
            Self,
            MessageContent(libparsec::types::MessageContent::SharingReencrypted {
                author: author.0,
                timestamp: timestamp.0,
                name: name.0,
                id: id.0,
                encryption_revision,
                encrypted_on: encrypted_on.0,
                key: key.0,
            }),
        ))
    }

    #[getter]
    fn name(_self: PyRef<'_, Self>) -> PyResult<EntryName> {
        Ok(match &_self.as_ref().0 {
            libparsec::types::MessageContent::SharingReencrypted { name, .. } => {
                EntryName(name.clone())
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn id(_self: PyRef<'_, Self>) -> PyResult<EntryID> {
        Ok(match _self.as_ref().0 {
            libparsec::types::MessageContent::SharingReencrypted { id, .. } => EntryID(id),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn encryption_revision(_self: PyRef<'_, Self>) -> PyResult<u32> {
        Ok(match _self.as_ref().0 {
            libparsec::types::MessageContent::SharingReencrypted {
                encryption_revision,
                ..
            } => encryption_revision,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn encrypted_on(_self: PyRef<'_, Self>) -> PyResult<DateTime> {
        Ok(match _self.as_ref().0 {
            libparsec::types::MessageContent::SharingReencrypted { encrypted_on, .. } => {
                DateTime(encrypted_on)
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn key(_self: PyRef<'_, Self>) -> PyResult<SecretKey> {
        Ok(match &_self.as_ref().0 {
            libparsec::types::MessageContent::SharingReencrypted { key, .. } => {
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
    fn new(author: DeviceID, timestamp: DateTime, id: EntryID) -> PyResult<(Self, MessageContent)> {
        Ok((
            Self,
            MessageContent(libparsec::types::MessageContent::SharingRevoked {
                author: author.0,
                timestamp: timestamp.0,
                id: id.0,
            }),
        ))
    }

    #[getter]
    fn id(_self: PyRef<'_, Self>) -> PyResult<EntryID> {
        Ok(match _self.as_ref().0 {
            libparsec::types::MessageContent::SharingRevoked { id, .. } => EntryID(id),
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
            MessageContent(libparsec::types::MessageContent::Ping {
                author: author.0,
                timestamp: timestamp.0,
                ping,
            }),
        ))
    }

    #[getter]
    fn ping<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyString> {
        Ok(match &_self.as_ref().0 {
            libparsec::types::MessageContent::Ping { ping, .. } => PyString::new(py, ping),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}
