// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::{PyAttributeError, PyNotImplementedError, PyValueError},
    import_exception,
    prelude::*,
    types::{PyBytes, PyList, PyType},
};

use libparsec::protocol::authenticated_cmds::{
    invite_1_greeter_wait_peer, invite_2a_greeter_get_hashed_nonce, invite_2b_greeter_send_nonce,
    invite_3a_greeter_wait_peer_trust, invite_3b_greeter_signify_trust,
    invite_4_greeter_communicate, invite_delete, invite_list, invite_new,
};
use libparsec::protocol::invited_cmds::{
    invite_1_claimer_wait_peer, invite_2a_claimer_send_hashed_nonce, invite_2b_claimer_send_nonce,
    invite_3a_claimer_signify_trust, invite_3b_claimer_wait_peer_trust,
    invite_4_claimer_communicate, invite_info,
};

use crate::{
    api_crypto,
    api_crypto::{HashDigest, PublicKey},
    ids::{HumanHandle, UserID},
    invite,
    invite::InvitationToken,
    protocol::gen_rep,
    time::DateTime,
};

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(Clone)]
pub(crate) struct InvitationType(pub libparsec::types::InvitationType);

crate::binding_utils::gen_proto!(InvitationType, __repr__);
crate::binding_utils::gen_proto!(InvitationType, __richcmp__, eq);
crate::binding_utils::gen_proto!(InvitationType, __hash__);

#[pymethods]
impl InvitationType {
    #[classattr]
    #[pyo3(name = "DEVICE")]
    fn device() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    InvitationType(libparsec::types::InvitationType::Device).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classattr]
    #[pyo3(name = "USER")]
    fn user() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    InvitationType(libparsec::types::InvitationType::User).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classmethod]
    fn values<'py>(_cls: &'py PyType, py: Python<'py>) -> &'py PyAny {
        PyList::new(py, &[Self::device(), Self::user()]).as_ref()
    }

    #[getter]
    fn str(&self) -> &str {
        match self.0 {
            libparsec::types::InvitationType::Device => "DEVICE",
            libparsec::types::InvitationType::User => "USER",
        }
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<&'static PyObject> {
        match value {
            "DEVICE" => Ok(Self::device()),
            "USER" => Ok(Self::user()),
            _ => Err(PyValueError::new_err("")),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InvitationEmailSentStatus(invite_new::InvitationEmailSentStatus);

crate::binding_utils::gen_proto!(InvitationEmailSentStatus, __repr__);
crate::binding_utils::gen_proto!(InvitationEmailSentStatus, __richcmp__, eq);

#[pymethods]
impl InvitationEmailSentStatus {
    #[classattr]
    #[pyo3(name = "SUCCESS")]
    fn success() -> PyResult<&'static PyObject> {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     InvitationEmailSentStatus(invite_new::InvitationEmailSentStatus::Success).into_py(py)
                })
            };
        };

        Ok(&VALUE)
    }

    #[classattr]
    #[pyo3(name = "NOT_AVAILABLE")]
    fn not_available() -> PyResult<&'static PyObject> {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     InvitationEmailSentStatus(invite_new::InvitationEmailSentStatus::NotAvailable).into_py(py)
                })
            };
        };

        Ok(&VALUE)
    }

    #[classattr]
    #[pyo3(name = "BAD_RECIPIENT")]
    fn bad_recipient() -> PyResult<&'static PyObject> {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     InvitationEmailSentStatus(invite_new::InvitationEmailSentStatus::BadRecipient).into_py(py)
                })
            };
        };

        Ok(&VALUE)
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<Self> {
        match value {
            "SUCCESS" => Ok(Self(invite_new::InvitationEmailSentStatus::Success)),
            "NOT_AVAILABLE" => Ok(Self(invite_new::InvitationEmailSentStatus::NotAvailable)),
            "BAD_RECIPIENT" => Ok(Self(invite_new::InvitationEmailSentStatus::BadRecipient)),
            _ => Err(PyValueError::new_err(format!("Invalid value `{}`", value))),
        }
    }

    fn __str__(&self) -> &str {
        match self.0 {
            invite_new::InvitationEmailSentStatus::Success => "SUCCESS",
            invite_new::InvitationEmailSentStatus::NotAvailable => "NOT_AVAILABLE",
            invite_new::InvitationEmailSentStatus::BadRecipient => "BAD_RECIPIENT",
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InvitationDeletedReason(pub invite_delete::InvitationDeletedReason);

crate::binding_utils::gen_proto!(InvitationDeletedReason, __repr__);
crate::binding_utils::gen_proto!(InvitationDeletedReason, __richcmp__, eq);

#[pymethods]
impl InvitationDeletedReason {
    #[classattr]
    #[pyo3(name = "FINISHED")]
    fn finished() -> PyResult<&'static PyObject> {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject =  {
                Python::with_gil(|py| {
                    InvitationDeletedReason(invite_delete::InvitationDeletedReason::Finished).into_py(py)
                })
            };
        };

        Ok(&VALUE)
    }

    #[classattr]
    #[pyo3(name = "CANCELLED")]
    fn cancelled() -> PyResult<&'static PyObject> {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject =  {
                Python::with_gil(|py| {
                    InvitationDeletedReason(invite_delete::InvitationDeletedReason::Cancelled).into_py(py)
                })
            };
        };

        Ok(&VALUE)
    }

    #[classattr]
    #[pyo3(name = "ROTTEN")]
    fn rotten() -> PyResult<&'static PyObject> {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject =  {
                Python::with_gil(|py| {
                    InvitationDeletedReason(invite_delete::InvitationDeletedReason::Rotten).into_py(py)
                })
            };
        };

        Ok(&VALUE)
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<Self> {
        match value {
            "FINISHED" => Ok(Self(invite_delete::InvitationDeletedReason::Finished)),
            "CANCELLED" => Ok(Self(invite_delete::InvitationDeletedReason::Cancelled)),
            "ROTTEN" => Ok(Self(invite_delete::InvitationDeletedReason::Rotten)),
            _ => Err(PyValueError::new_err(format!("Invalid value `{}`", value))),
        }
    }

    #[getter]
    fn value(&self) -> &'static str {
        match self.0 {
            invite_delete::InvitationDeletedReason::Finished => "FINISHED",
            invite_delete::InvitationDeletedReason::Cancelled => "CANCELLED",
            invite_delete::InvitationDeletedReason::Rotten => "ROTTEN",
        }
    }

    fn __str__(&self) -> &str {
        match self.0 {
            invite_delete::InvitationDeletedReason::Finished => "FINISHED",
            invite_delete::InvitationDeletedReason::Cancelled => "CANCELLED",
            invite_delete::InvitationDeletedReason::Rotten => "ROTTEN",
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InvitationStatus(pub libparsec::types::InvitationStatus);

crate::binding_utils::gen_proto!(InvitationStatus, __repr__);
crate::binding_utils::gen_proto!(InvitationStatus, __richcmp__, eq);
crate::binding_utils::gen_proto!(InvitationStatus, __hash__);

#[pymethods]
impl InvitationStatus {
    #[classattr]
    #[pyo3(name = "IDLE")]
    fn idle() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    InvitationStatus(libparsec::types::InvitationStatus::Idle).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classattr]
    #[pyo3(name = "READY")]
    fn ready() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    InvitationStatus(libparsec::types::InvitationStatus::Ready).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classattr]
    #[pyo3(name = "DELETED")]
    fn deleted() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    InvitationStatus(libparsec::types::InvitationStatus::Deleted).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classmethod]
    fn values<'py>(_cls: &'py PyType, py: Python<'py>) -> &'py PyAny {
        PyList::new(py, &[Self::idle(), Self::ready(), Self::deleted()]).as_ref()
    }

    #[getter]
    fn str(&self) -> &str {
        match self.0 {
            libparsec::types::InvitationStatus::Idle => "IDLE",
            libparsec::types::InvitationStatus::Ready => "READY",
            libparsec::types::InvitationStatus::Deleted => "DELETED",
        }
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<&'static PyObject> {
        match value {
            "IDLE" => Ok(Self::idle()),
            "READY" => Ok(Self::ready()),
            "DELETED" => Ok(Self::deleted()),
            _ => Err(PyValueError::new_err("")),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InviteListItem(pub invite_list::InviteListItem);

crate::binding_utils::gen_proto!(InviteListItem, __repr__);
crate::binding_utils::gen_proto!(InviteListItem, __richcmp__, eq);

#[pymethods]
impl InviteListItem {
    #[classmethod]
    #[pyo3(name = "User")]
    fn user(
        _cls: &PyType,
        token: InvitationToken,
        created_on: DateTime,
        claimer_email: String,
        status: InvitationStatus,
    ) -> PyResult<Self> {
        let token = token.0;
        let created_on = created_on.0;
        Ok(Self(invite_list::InviteListItem::User {
            token,
            created_on,
            claimer_email,
            status: status.0,
        }))
    }

    #[classmethod]
    #[pyo3(name = "Device")]
    fn device(
        _cls: &PyType,
        token: InvitationToken,
        created_on: DateTime,
        status: InvitationStatus,
    ) -> PyResult<Self> {
        let token = token.0;
        let created_on = created_on.0;
        Ok(Self(invite_list::InviteListItem::Device {
            token,
            created_on,
            status: status.0,
        }))
    }

    #[getter]
    #[pyo3(name = "r#type")]
    fn r#type(&self) -> PyResult<InvitationType> {
        Ok(InvitationType(match self.0 {
            invite_list::InviteListItem::User { .. } => libparsec::types::InvitationType::User,
            invite_list::InviteListItem::Device { .. } => libparsec::types::InvitationType::Device,
        }))
    }

    #[getter]
    fn token(&self) -> PyResult<InvitationToken> {
        Ok(InvitationToken(match self.0 {
            invite_list::InviteListItem::User { token, .. } => token,
            invite_list::InviteListItem::Device { token, .. } => token,
        }))
    }

    #[getter]
    fn created_on(&self) -> PyResult<DateTime> {
        Ok(DateTime(match self.0 {
            invite_list::InviteListItem::User { created_on, .. } => created_on,
            invite_list::InviteListItem::Device { created_on, .. } => created_on,
        }))
    }

    #[getter]
    fn claimer_email(&self) -> PyResult<&str> {
        match &self.0 {
            invite_list::InviteListItem::User { claimer_email, .. } => Ok(claimer_email),
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn status(&self) -> PyResult<InvitationStatus> {
        Ok(InvitationStatus(match &self.0 {
            invite_list::InviteListItem::User { status, .. } => status.clone(),
            invite_list::InviteListItem::Device { status, .. } => status.clone(),
        }))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InviteNewReq(pub invite_new::Req);

crate::binding_utils::gen_proto!(InviteNewReq, __repr__);
crate::binding_utils::gen_proto!(InviteNewReq, __richcmp__, eq);

#[pymethods]
impl InviteNewReq {
    #[new]
    fn new(
        r#type: InvitationType,
        claimer_email: Option<String>,
        send_email: bool,
    ) -> PyResult<Self> {
        Ok(InviteNewReq(match r#type.0 {
            libparsec::types::InvitationType::Device => {
                invite_new::Req(invite_new::UserOrDevice::Device { send_email })
            }
            libparsec::types::InvitationType::User => {
                invite_new::Req(invite_new::UserOrDevice::User {
                    claimer_email: claimer_email.expect("Missing claimer_email_argument"),
                    send_email,
                })
            }
        }))
    }

    #[classmethod]
    #[pyo3(name = "User")]
    fn user(_cls: &PyType, claimer_email: String, send_email: bool) -> PyResult<Self> {
        Ok(InviteNewReq(invite_new::Req(
            invite_new::UserOrDevice::User {
                claimer_email,
                send_email,
            },
        )))
    }

    #[classmethod]
    #[pyo3(name = "Device")]
    fn device(_cls: &PyType, send_email: bool) -> PyResult<Self> {
        Ok(Self(invite_new::Req(invite_new::UserOrDevice::Device {
            send_email,
        })))
    }

    #[getter]
    #[pyo3(name = "r#type")]
    fn invitation_type(&self) -> PyResult<InvitationType> {
        Ok(InvitationType(match self.0 {
            invite_new::Req(invite_new::UserOrDevice::Device { .. }) => {
                libparsec::types::InvitationType::Device
            }
            invite_new::Req(invite_new::UserOrDevice::User { .. }) => {
                libparsec::types::InvitationType::User
            }
        }))
    }

    #[getter]
    fn claimer_email(&self) -> PyResult<&str> {
        match &self.0 {
            invite_new::Req(invite_new::UserOrDevice::User { claimer_email, .. }) => {
                Ok(claimer_email)
            }
            _ => Err(PyAttributeError::new_err("No claimer_email attribute")),
        }
    }

    #[getter]
    fn send_email(&self) -> PyResult<bool> {
        Ok(match self.0 {
            invite_new::Req(invite_new::UserOrDevice::User { send_email, .. }) => send_email,
            invite_new::Req(invite_new::UserOrDevice::Device { send_email }) => send_email,
        })
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

gen_rep!(
    invite_new,
    InviteNewRep,
    { .. },
    [NotAllowed],
    [AlreadyMember],
    [NotAvailable]
);

#[pyclass(extends=InviteNewRep)]
pub(crate) struct InviteNewRepOk;

#[pymethods]
impl InviteNewRepOk {
    #[new]
    pub fn new(
        token: InvitationToken,
        email_sent: InvitationEmailSentStatus,
    ) -> PyResult<(Self, InviteNewRep)> {
        let token = token.0;
        Ok((
            Self,
            InviteNewRep(invite_new::Rep::Ok {
                token,
                email_sent: libparsec::types::Maybe::Present(email_sent.0),
            }),
        ))
    }

    #[getter]
    fn token(_self: PyRef<'_, Self>) -> PyResult<InvitationToken> {
        match &_self.as_ref().0 {
            invite_new::Rep::Ok { token, .. } => Ok(invite::InvitationToken(*token)),
            _ => Err(PyAttributeError::new_err("No attribute token")),
        }
    }

    #[getter]
    fn email_sent(_self: PyRef<'_, Self>) -> PyResult<InvitationEmailSentStatus> {
        match &_self.as_ref().0 {
            invite_new::Rep::Ok { email_sent, .. } => match email_sent {
                libparsec::types::Maybe::Present(p) => Ok(InvitationEmailSentStatus(p.clone())),
                libparsec::types::Maybe::Absent => Err(PyAttributeError::new_err("")),
            },
            _ => Err(PyAttributeError::new_err("")),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InviteDeleteReq(pub invite_delete::Req);

crate::binding_utils::gen_proto!(InviteDeleteReq, __repr__);
crate::binding_utils::gen_proto!(InviteDeleteReq, __richcmp__, eq);

#[pymethods]
impl InviteDeleteReq {
    #[new]
    fn new(token: InvitationToken, reason: InvitationDeletedReason) -> PyResult<Self> {
        let token = token.0;
        Ok(Self(invite_delete::Req {
            token,
            reason: reason.0,
        }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn token(&self) -> PyResult<InvitationToken> {
        Ok(invite::InvitationToken(self.0.token))
    }

    #[getter]
    fn reason(&self) -> PyResult<InvitationDeletedReason> {
        Ok(InvitationDeletedReason(self.0.reason.clone()))
    }
}

gen_rep!(
    invite_delete,
    InviteDeleteRep,
    { .. },
    [AlreadyDeleted],
    [NotFound]
);

#[pyclass(extends=InviteDeleteRep)]
pub(crate) struct InviteDeleteRepOk;

#[pymethods]
impl InviteDeleteRepOk {
    #[new]
    fn new() -> PyResult<(Self, InviteDeleteRep)> {
        Ok((Self, InviteDeleteRep(invite_delete::Rep::Ok)))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InviteListReq(pub invite_list::Req);

crate::binding_utils::gen_proto!(InviteListReq, __repr__);
crate::binding_utils::gen_proto!(InviteListReq, __richcmp__, eq);

#[pymethods]
impl InviteListReq {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(invite_list::Req))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

gen_rep!(invite_list, InviteListRep, { .. });

#[pyclass(extends=InviteListRep)]
pub(crate) struct InviteListRepOk;

#[pymethods]
impl InviteListRepOk {
    #[new]
    fn new(invitations: Vec<InviteListItem>) -> PyResult<(Self, InviteListRep)> {
        let invitations = invitations.into_iter().map(|inv| inv.0).collect();
        Ok((Self, InviteListRep(invite_list::Rep::Ok { invitations })))
    }

    #[getter]
    fn invitations(_self: PyRef<'_, Self>) -> PyResult<Vec<InviteListItem>> {
        match &_self.as_ref().0 {
            invite_list::Rep::Ok { invitations } => Ok(invitations
                .iter()
                .map(|f| InviteListItem(f.clone()))
                .collect()),
            _ => Err(PyAttributeError::new_err("")),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InviteInfoReq(pub invite_info::Req);

crate::binding_utils::gen_proto!(InviteInfoReq, __repr__);
crate::binding_utils::gen_proto!(InviteInfoReq, __richcmp__, eq);

#[pymethods]
impl InviteInfoReq {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(invite_info::Req))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

gen_rep!(invite_info, InviteInfoRep, { .. });

#[pyclass(extends=InviteInfoRep)]
pub(crate) struct InviteInfoRepOk;

#[pymethods]
impl InviteInfoRepOk {
    #[new]
    fn new(
        r#type: InvitationType,
        claimer_email: Option<String>,
        greeter_user_id: UserID,
        greeter_human_handle: Option<HumanHandle>,
    ) -> PyResult<(Self, InviteInfoRep)> {
        let greeter_user_id = greeter_user_id.0;
        let greeter_human_handle = greeter_human_handle.map(|inner| inner.0);
        match r#type {
            InvitationType(libparsec::types::InvitationType::Device) => Ok((
                Self,
                InviteInfoRep(invite_info::Rep::Ok(invite_info::UserOrDevice::Device {
                    greeter_user_id,
                    greeter_human_handle,
                })),
            )),
            InvitationType(libparsec::types::InvitationType::User) => Ok((
                Self,
                InviteInfoRep(invite_info::Rep::Ok(invite_info::UserOrDevice::User {
                    claimer_email: claimer_email
                        .expect("Missing claimer_email for InviteInfoRep[User]"),
                    greeter_user_id,
                    greeter_human_handle,
                })),
            )),
        }
    }

    #[getter]
    fn r#type(_self: PyRef<'_, Self>) -> PyResult<InvitationType> {
        match &_self.as_ref().0 {
            invite_info::Rep::Ok(invite_info::UserOrDevice::Device { .. }) => {
                Ok(InvitationType(libparsec::types::InvitationType::Device))
            }
            invite_info::Rep::Ok(invite_info::UserOrDevice::User { .. }) => {
                Ok(InvitationType(libparsec::types::InvitationType::User))
            }
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn greeter_user_id(_self: PyRef<'_, Self>) -> PyResult<UserID> {
        match &_self.as_ref().0 {
            invite_info::Rep::Ok(invite_info::UserOrDevice::Device {
                greeter_user_id, ..
            }) => Ok(UserID(greeter_user_id.clone())),
            invite_info::Rep::Ok(invite_info::UserOrDevice::User {
                greeter_user_id, ..
            }) => Ok(UserID(greeter_user_id.clone())),
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn claimer_email(_self: PyRef<'_, Self>) -> PyResult<String> {
        match &_self.as_ref().0 {
            invite_info::Rep::Ok(invite_info::UserOrDevice::User { claimer_email, .. }) => {
                Ok(claimer_email.clone())
            }
            _ => Err(PyAttributeError::new_err(
                "no claimer_email in non device invitation",
            )),
        }
    }

    #[getter]
    fn greeter_human_handle(_self: PyRef<'_, Self>) -> PyResult<HumanHandle> {
        match &_self.as_ref().0 {
            invite_info::Rep::Ok(invite_info::UserOrDevice::Device {
                greeter_human_handle: Some(handle),
                ..
            }) => Ok(HumanHandle(handle.clone())),
            invite_info::Rep::Ok(invite_info::UserOrDevice::User {
                greeter_human_handle: Some(handle),
                ..
            }) => Ok(HumanHandle(handle.clone())),
            _ => Err(PyAttributeError::new_err("rep in not ok")),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Invite1ClaimerWaitPeerReq(pub invite_1_claimer_wait_peer::Req);

crate::binding_utils::gen_proto!(Invite1ClaimerWaitPeerReq, __repr__);
crate::binding_utils::gen_proto!(Invite1ClaimerWaitPeerReq, __richcmp__, eq);

#[pymethods]
impl Invite1ClaimerWaitPeerReq {
    #[new]
    fn new(claimer_public_key: PublicKey) -> PyResult<Self> {
        let claimer_public_key = claimer_public_key.0;
        Ok(Self(invite_1_claimer_wait_peer::Req { claimer_public_key }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn claimer_public_key(_self: PyRef<'_, Self>) -> PyResult<PublicKey> {
        Ok(api_crypto::PublicKey(_self.0.claimer_public_key.clone()))
    }
}

gen_rep!(
    invite_1_claimer_wait_peer,
    Invite1ClaimerWaitPeerRep,
    { .. },
    [NotFound],
    [InvalidState]
);

#[pyclass(extends=Invite1ClaimerWaitPeerRep)]
pub(crate) struct Invite1ClaimerWaitPeerRepOk;

#[pymethods]
impl Invite1ClaimerWaitPeerRepOk {
    #[new]
    fn new(greeter_public_key: PublicKey) -> PyResult<(Self, Invite1ClaimerWaitPeerRep)> {
        let greeter_public_key = greeter_public_key.0;
        Ok((
            Self,
            Invite1ClaimerWaitPeerRep(invite_1_claimer_wait_peer::Rep::Ok { greeter_public_key }),
        ))
    }

    #[getter]
    fn greeter_public_key(_self: PyRef<'_, Self>) -> PyResult<PublicKey> {
        match &_self.as_ref().0 {
            invite_1_claimer_wait_peer::Rep::Ok { greeter_public_key } => {
                Ok(api_crypto::PublicKey(greeter_public_key.clone()))
            }
            _ => Err(PyNotImplementedError::new_err("")),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Invite1GreeterWaitPeerReq(pub invite_1_greeter_wait_peer::Req);

crate::binding_utils::gen_proto!(Invite1GreeterWaitPeerReq, __repr__);
crate::binding_utils::gen_proto!(Invite1GreeterWaitPeerReq, __richcmp__, eq);

#[pymethods]
impl Invite1GreeterWaitPeerReq {
    #[new]
    fn new(token: InvitationToken, greeter_public_key: PublicKey) -> PyResult<Self> {
        let token = token.0;
        let greeter_public_key = greeter_public_key.0;
        Ok(Self(invite_1_greeter_wait_peer::Req {
            token,
            greeter_public_key,
        }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn token(&self) -> PyResult<InvitationToken> {
        Ok(invite::InvitationToken(self.0.token))
    }

    #[getter]
    fn greeter_public_key(_self: PyRef<'_, Self>) -> PyResult<PublicKey> {
        Ok(api_crypto::PublicKey(_self.0.greeter_public_key.clone()))
    }
}

gen_rep!(
    invite_1_greeter_wait_peer,
    Invite1GreeterWaitPeerRep,
    { .. },
    [NotFound],
    [AlreadyDeleted],
    [InvalidState],
);

#[pyclass(extends=Invite1GreeterWaitPeerRep)]
pub(crate) struct Invite1GreeterWaitPeerRepOk;

#[pymethods]
impl Invite1GreeterWaitPeerRepOk {
    #[new]
    fn new(claimer_public_key: PublicKey) -> PyResult<(Self, Invite1GreeterWaitPeerRep)> {
        let claimer_public_key = claimer_public_key.0;
        Ok((
            Self,
            Invite1GreeterWaitPeerRep(invite_1_greeter_wait_peer::Rep::Ok { claimer_public_key }),
        ))
    }

    #[getter]
    fn claimer_public_key(_self: PyRef<'_, Self>) -> PyResult<PublicKey> {
        match &_self.as_ref().0 {
            invite_1_greeter_wait_peer::Rep::Ok { claimer_public_key } => {
                Ok(api_crypto::PublicKey(claimer_public_key.clone()))
            }
            _ => Err(PyNotImplementedError::new_err("")),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Invite2aClaimerSendHashedNonceReq(pub invite_2a_claimer_send_hashed_nonce::Req);

crate::binding_utils::gen_proto!(Invite2aClaimerSendHashedNonceReq, __repr__);
crate::binding_utils::gen_proto!(Invite2aClaimerSendHashedNonceReq, __richcmp__, eq);

#[pymethods]
impl Invite2aClaimerSendHashedNonceReq {
    #[new]
    fn new(claimer_hashed_nonce: HashDigest) -> PyResult<Self> {
        let claimer_hashed_nonce = claimer_hashed_nonce.0;
        Ok(Self(invite_2a_claimer_send_hashed_nonce::Req {
            claimer_hashed_nonce,
        }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn claimer_hashed_nonce(_self: PyRef<'_, Self>) -> PyResult<HashDigest> {
        Ok(api_crypto::HashDigest(_self.0.claimer_hashed_nonce.clone()))
    }
}

gen_rep!(
    invite_2a_claimer_send_hashed_nonce,
    Invite2aClaimerSendHashedNonceRep,
    { .. },
    [NotFound],
    [AlreadyDeleted],
    [InvalidState],
);

#[pyclass(extends=Invite2aClaimerSendHashedNonceRep)]
pub(crate) struct Invite2aClaimerSendHashedNonceRepOk;

#[pymethods]
impl Invite2aClaimerSendHashedNonceRepOk {
    #[new]
    fn new(greeter_nonce: Vec<u8>) -> PyResult<(Self, Invite2aClaimerSendHashedNonceRep)> {
        Ok((
            Self,
            Invite2aClaimerSendHashedNonceRep(invite_2a_claimer_send_hashed_nonce::Rep::Ok {
                greeter_nonce,
            }),
        ))
    }

    #[getter]
    fn greeter_nonce<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyBytes> {
        let greeter_nonce = match &_self.as_ref().0 {
            invite_2a_claimer_send_hashed_nonce::Rep::Ok { greeter_nonce } => greeter_nonce,
            _ => return Err(PyNotImplementedError::new_err("")),
        };

        Ok(PyBytes::new(py, greeter_nonce))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Invite2aGreeterGetHashedNonceReq(pub invite_2a_greeter_get_hashed_nonce::Req);

crate::binding_utils::gen_proto!(Invite2aGreeterGetHashedNonceReq, __repr__);
crate::binding_utils::gen_proto!(Invite2aGreeterGetHashedNonceReq, __richcmp__, eq);

#[pymethods]
impl Invite2aGreeterGetHashedNonceReq {
    #[new]
    fn new(token: InvitationToken) -> PyResult<Self> {
        let token = token.0;
        Ok(Self(invite_2a_greeter_get_hashed_nonce::Req { token }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn token(&self) -> PyResult<InvitationToken> {
        Ok(invite::InvitationToken(self.0.token))
    }
}

gen_rep!(
    invite_2a_greeter_get_hashed_nonce,
    Invite2aGreeterGetHashedNonceRep,
    { .. },
    [NotFound],
    [AlreadyDeleted],
    [InvalidState]
);

#[pyclass(extends=Invite2aGreeterGetHashedNonceRep)]
pub(crate) struct Invite2aGreeterGetHashedNonceRepOk;

#[pymethods]
impl Invite2aGreeterGetHashedNonceRepOk {
    #[new]
    fn new(claimer_hashed_nonce: HashDigest) -> PyResult<(Self, Invite2aGreeterGetHashedNonceRep)> {
        let claimer_hashed_nonce = claimer_hashed_nonce.0;
        Ok((
            Self,
            Invite2aGreeterGetHashedNonceRep(invite_2a_greeter_get_hashed_nonce::Rep::Ok {
                claimer_hashed_nonce,
            }),
        ))
    }

    #[getter]
    fn claimer_hashed_nonce(_self: PyRef<'_, Self>) -> PyResult<HashDigest> {
        match &_self.as_ref().0 {
            invite_2a_greeter_get_hashed_nonce::Rep::Ok {
                claimer_hashed_nonce,
            } => Ok(api_crypto::HashDigest(claimer_hashed_nonce.clone())),
            _ => Err(PyNotImplementedError::new_err("")),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Invite2bClaimerSendNonceReq(pub invite_2b_claimer_send_nonce::Req);

crate::binding_utils::gen_proto!(Invite2bClaimerSendNonceReq, __repr__);
crate::binding_utils::gen_proto!(Invite2bClaimerSendNonceReq, __richcmp__, eq);

#[pymethods]
impl Invite2bClaimerSendNonceReq {
    #[new]
    fn new(claimer_nonce: Vec<u8>) -> PyResult<Self> {
        Ok(Self(invite_2b_claimer_send_nonce::Req { claimer_nonce }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn claimer_nonce<'py>(_self: PyRef<'_, Self>, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(py, &_self.0.claimer_nonce))
    }
}

gen_rep!(
    invite_2b_claimer_send_nonce,
    Invite2bClaimerSendNonceRep,
    { .. },
    [NotFound],
    [InvalidState]
);

#[pyclass(extends=Invite2bClaimerSendNonceRep)]
pub(crate) struct Invite2bClaimerSendNonceRepOk;

#[pymethods]
impl Invite2bClaimerSendNonceRepOk {
    #[new]
    fn new() -> PyResult<(Self, Invite2bClaimerSendNonceRep)> {
        Ok((
            Self,
            Invite2bClaimerSendNonceRep(invite_2b_claimer_send_nonce::Rep::Ok),
        ))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Invite2bGreeterSendNonceReq(pub invite_2b_greeter_send_nonce::Req);

crate::binding_utils::gen_proto!(Invite2bGreeterSendNonceReq, __repr__);
crate::binding_utils::gen_proto!(Invite2bGreeterSendNonceReq, __richcmp__, eq);

#[pymethods]
impl Invite2bGreeterSendNonceReq {
    #[new]
    fn new(token: InvitationToken, greeter_nonce: Vec<u8>) -> PyResult<Self> {
        let token = token.0;
        Ok(Self(invite_2b_greeter_send_nonce::Req {
            token,
            greeter_nonce,
        }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn token(_self: PyRef<'_, Self>) -> PyResult<InvitationToken> {
        Ok(invite::InvitationToken(_self.0.token))
    }

    #[getter]
    fn greeter_nonce<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(py, &_self.0.greeter_nonce))
    }
}

gen_rep!(
    invite_2b_greeter_send_nonce,
    Invite2bGreeterSendNonceRep,
    { .. },
    [NotFound],
    [AlreadyDeleted],
    [InvalidState]
);

#[pyclass(extends=Invite2bGreeterSendNonceRep)]
pub(crate) struct Invite2bGreeterSendNonceRepOk;

#[pymethods]
impl Invite2bGreeterSendNonceRepOk {
    #[new]
    fn new(claimer_nonce: Vec<u8>) -> PyResult<(Self, Invite2bGreeterSendNonceRep)> {
        Ok((
            Self,
            Invite2bGreeterSendNonceRep(invite_2b_greeter_send_nonce::Rep::Ok { claimer_nonce }),
        ))
    }

    #[getter]
    fn claimer_nonce<'py>(_self: PyRef<'_, Self>, py: Python<'py>) -> PyResult<&'py PyBytes> {
        let claimer_nonce = match &_self.as_ref().0 {
            invite_2b_greeter_send_nonce::Rep::Ok { claimer_nonce } => claimer_nonce,
            _ => return Err(PyNotImplementedError::new_err("")),
        };

        Ok(PyBytes::new(py, claimer_nonce))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Invite3aClaimerSignifyTrustReq(pub invite_3a_claimer_signify_trust::Req);

crate::binding_utils::gen_proto!(Invite3aClaimerSignifyTrustReq, __repr__);
crate::binding_utils::gen_proto!(Invite3aClaimerSignifyTrustReq, __richcmp__, eq);

#[pymethods]
impl Invite3aClaimerSignifyTrustReq {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(invite_3a_claimer_signify_trust::Req))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

gen_rep!(
    invite_3a_claimer_signify_trust,
    Invite3aClaimerSignifyTrustRep,
    { .. },
    [NotFound],
    [InvalidState]
);

#[pyclass(extends=Invite3aClaimerSignifyTrustRep)]
pub(crate) struct Invite3aClaimerSignifyTrustRepOk;

#[pymethods]
impl Invite3aClaimerSignifyTrustRepOk {
    #[new]
    fn new() -> PyResult<(Self, Invite3aClaimerSignifyTrustRep)> {
        Ok((
            Self,
            Invite3aClaimerSignifyTrustRep(invite_3a_claimer_signify_trust::Rep::Ok),
        ))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Invite3aGreeterWaitPeerTrustReq(pub invite_3a_greeter_wait_peer_trust::Req);

crate::binding_utils::gen_proto!(Invite3aGreeterWaitPeerTrustReq, __repr__);
crate::binding_utils::gen_proto!(Invite3aGreeterWaitPeerTrustReq, __richcmp__, eq);

#[pymethods]
impl Invite3aGreeterWaitPeerTrustReq {
    #[new]
    fn new(token: InvitationToken) -> PyResult<Self> {
        let token = token.0;
        Ok(Self(invite_3a_greeter_wait_peer_trust::Req { token }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn token(_self: PyRef<'_, Self>) -> PyResult<InvitationToken> {
        Ok(invite::InvitationToken(_self.0.token))
    }
}

gen_rep!(
    invite_3a_greeter_wait_peer_trust,
    Invite3aGreeterWaitPeerTrustRep,
    { .. },
    [NotFound],
    [AlreadyDeleted],
    [InvalidState]
);

#[pyclass(extends=Invite3aGreeterWaitPeerTrustRep)]
pub(crate) struct Invite3aGreeterWaitPeerTrustRepOk;

#[pymethods]
impl Invite3aGreeterWaitPeerTrustRepOk {
    #[new]
    fn new() -> PyResult<(Self, Invite3aGreeterWaitPeerTrustRep)> {
        Ok((
            Self,
            Invite3aGreeterWaitPeerTrustRep(invite_3a_greeter_wait_peer_trust::Rep::Ok),
        ))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Invite3bClaimerWaitPeerTrustReq(pub invite_3b_claimer_wait_peer_trust::Req);

crate::binding_utils::gen_proto!(Invite3bClaimerWaitPeerTrustReq, __repr__);
crate::binding_utils::gen_proto!(Invite3bClaimerWaitPeerTrustReq, __richcmp__, eq);

#[pymethods]
impl Invite3bClaimerWaitPeerTrustReq {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(invite_3b_claimer_wait_peer_trust::Req))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

gen_rep!(
    invite_3b_claimer_wait_peer_trust,
    Invite3bClaimerWaitPeerTrustRep,
    { .. },
    [NotFound],
    [InvalidState]
);

#[pyclass(extends=Invite3bClaimerWaitPeerTrustRep)]
pub(crate) struct Invite3bClaimerWaitPeerTrustRepOk;

#[pymethods]
impl Invite3bClaimerWaitPeerTrustRepOk {
    #[new]
    fn new() -> PyResult<(Self, Invite3bClaimerWaitPeerTrustRep)> {
        Ok((
            Self,
            Invite3bClaimerWaitPeerTrustRep(invite_3b_claimer_wait_peer_trust::Rep::Ok),
        ))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Invite3bGreeterSignifyTrustReq(pub invite_3b_greeter_signify_trust::Req);

crate::binding_utils::gen_proto!(Invite3bGreeterSignifyTrustReq, __repr__);
crate::binding_utils::gen_proto!(Invite3bGreeterSignifyTrustReq, __richcmp__, eq);

#[pymethods]
impl Invite3bGreeterSignifyTrustReq {
    #[new]
    fn new(token: InvitationToken) -> PyResult<Self> {
        let token = token.0;
        Ok(Self(invite_3b_greeter_signify_trust::Req { token }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn token(_self: PyRef<'_, Self>) -> PyResult<InvitationToken> {
        Ok(invite::InvitationToken(_self.0.token))
    }
}

gen_rep!(
    invite_3b_greeter_signify_trust,
    Invite3bGreeterSignifyTrustRep,
    { .. },
    [NotFound],
    [AlreadyDeleted],
    [InvalidState]
);

#[pyclass(extends=Invite3bGreeterSignifyTrustRep)]
pub(crate) struct Invite3bGreeterSignifyTrustRepOk;

#[pymethods]
impl Invite3bGreeterSignifyTrustRepOk {
    #[new]
    fn new() -> PyResult<(Self, Invite3bGreeterSignifyTrustRep)> {
        Ok((
            Self,
            Invite3bGreeterSignifyTrustRep(invite_3b_greeter_signify_trust::Rep::Ok),
        ))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Invite4ClaimerCommunicateReq(pub invite_4_claimer_communicate::Req);

crate::binding_utils::gen_proto!(Invite4ClaimerCommunicateReq, __repr__);
crate::binding_utils::gen_proto!(Invite4ClaimerCommunicateReq, __richcmp__, eq);

#[pymethods]
impl Invite4ClaimerCommunicateReq {
    #[new]
    fn new(payload: Vec<u8>) -> PyResult<Self> {
        Ok(Self(invite_4_claimer_communicate::Req { payload }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn payload<'py>(_self: PyRef<'_, Self>, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(py, &_self.0.payload))
    }
}

gen_rep!(
    invite_4_claimer_communicate,
    Invite4ClaimerCommunicateRep,
    { .. },
    [NotFound],
    [InvalidState],
);

#[pyclass(extends=Invite4ClaimerCommunicateRep)]
pub(crate) struct Invite4ClaimerCommunicateRepOk;

#[pymethods]
impl Invite4ClaimerCommunicateRepOk {
    #[new]
    fn new(payload: Vec<u8>) -> PyResult<(Self, Invite4ClaimerCommunicateRep)> {
        Ok((
            Self,
            Invite4ClaimerCommunicateRep(invite_4_claimer_communicate::Rep::Ok { payload }),
        ))
    }

    #[getter]
    fn payload<'py>(_self: PyRef<'_, Self>, py: Python<'py>) -> PyResult<&'py PyBytes> {
        let payload = match &_self.as_ref().0 {
            invite_4_claimer_communicate::Rep::Ok { payload } => payload,
            _ => return Err(PyNotImplementedError::new_err("")),
        };

        Ok(PyBytes::new(py, payload))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Invite4GreeterCommunicateReq(pub invite_4_greeter_communicate::Req);

crate::binding_utils::gen_proto!(Invite4GreeterCommunicateReq, __repr__);
crate::binding_utils::gen_proto!(Invite4GreeterCommunicateReq, __richcmp__, eq);

#[pymethods]
impl Invite4GreeterCommunicateReq {
    #[new]
    fn new(token: InvitationToken, payload: Vec<u8>) -> PyResult<Self> {
        let token = token.0;
        Ok(Self(invite_4_greeter_communicate::Req { token, payload }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn token(_self: PyRef<'_, Self>) -> PyResult<InvitationToken> {
        Ok(invite::InvitationToken(_self.0.token))
    }

    #[getter]
    fn payload<'py>(_self: PyRef<'_, Self>, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(py, &_self.0.payload))
    }
}

gen_rep!(
    invite_4_greeter_communicate,
    Invite4GreeterCommunicateRep,
    { .. },
    [NotFound],
    [AlreadyDeleted],
    [InvalidState]
);

#[pyclass(extends=Invite4GreeterCommunicateRep)]
pub(crate) struct Invite4GreeterCommunicateRepOk;

#[pymethods]
impl Invite4GreeterCommunicateRepOk {
    #[new]
    fn new(payload: Vec<u8>) -> PyResult<(Self, Invite4GreeterCommunicateRep)> {
        Ok((
            Self,
            Invite4GreeterCommunicateRep(invite_4_greeter_communicate::Rep::Ok { payload }),
        ))
    }

    #[getter]
    fn payload<'py>(_self: PyRef<'_, Self>, py: Python<'py>) -> PyResult<&'py PyBytes> {
        let payload = match &_self.as_ref().0 {
            invite_4_greeter_communicate::Rep::Ok { payload } => payload,
            _ => return Err(PyNotImplementedError::new_err("")),
        };

        Ok(PyBytes::new(py, payload))
    }
}
