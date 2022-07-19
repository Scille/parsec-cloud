// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::exceptions::PyValueError;
use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};

use libparsec::protocol::authenticated_cmds::{
    invite_1_greeter_wait_peer, invite_2a_greeter_get_hashed_nonce, invite_2b_greeter_send_nonce,
    invite_3a_greeter_wait_peer_trust, invite_3b_greeter_signify_trust,
    invite_4_greeter_communicate, invite_delete, invite_list, invite_new,
};
use libparsec::protocol::invited_cmds::{
    invite_1_claimer_wait_peer, invite_2a_claimer_send_hashed_nonce_hash_nonce,
    invite_2b_claimer_send_nonce, invite_3a_claimer_signify_trust,
    invite_3b_claimer_wait_peer_trust, invite_4_claimer_communicate, invite_info,
};

use crate::api_crypto::{HashDigest, PublicKey};
use crate::binding_utils::py_to_rs_invitation_status;
use crate::ids::{HumanHandle, UserID};
use crate::invite::InvitationToken;
use crate::time::DateTime;

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct InviteNewReq(pub invite_new::Req);

#[pymethods]
impl InviteNewReq {
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

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

fn py_to_rs_invitation_email_sent_status(
    email_sent: &PyAny,
) -> PyResult<invite_new::InvitationEmailSentStatus> {
    use invite_new::InvitationEmailSentStatus::*;
    Ok(match email_sent.getattr("name")?.extract::<&str>()? {
        "SUCCESS" => Success,
        "NOT_AVAILABLE" => NotAvailable,
        "BAD_RECIPIENT" => BadRecipient,
        _ => unreachable!(),
    })
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct InviteNewRep(pub invite_new::Rep);

#[pymethods]
impl InviteNewRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, token: InvitationToken, email_sent: &PyAny) -> PyResult<Self> {
        let token = token.0;
        let email_sent = match py_to_rs_invitation_email_sent_status(email_sent) {
            Ok(email_sent) => libparsec::types::Maybe::Present(Some(email_sent)),
            _ => libparsec::types::Maybe::Absent,
        };
        Ok(Self(invite_new::Rep::Ok { token, email_sent }))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_new::Rep::NotAllowed))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyMember")]
    fn already_member(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_new::Rep::AlreadyMember))
    }

    #[classmethod]
    #[pyo3(name = "NotAvailable")]
    fn not_available(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_new::Rep::NotAvailable))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_new::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

fn py_to_rs_invitation_deleted_reason(
    reason: &PyAny,
) -> PyResult<invite_delete::InvitationDeletedReason> {
    use invite_delete::InvitationDeletedReason::*;
    Ok(match reason.getattr("name")?.extract::<&str>()? {
        "FINISHED" => Finished,
        "CANCELLED" => Cancelled,
        "ROTTEN" => Rotten,
        _ => unreachable!(),
    })
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct InviteDeleteReq(pub invite_delete::Req);

#[pymethods]
impl InviteDeleteReq {
    #[new]
    fn new(token: InvitationToken, reason: &PyAny) -> PyResult<Self> {
        let token = token.0;
        let reason = py_to_rs_invitation_deleted_reason(reason)?;
        Ok(Self(invite_delete::Req { token, reason }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct InviteDeleteRep(pub invite_delete::Rep);

#[pymethods]
impl InviteDeleteRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_delete::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_delete::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyDeleted")]
    fn already_deleted(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_delete::Rep::AlreadyDeleted))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_delete::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct InviteListReq(pub invite_list::Req);

#[pymethods]
impl InviteListReq {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(invite_list::Req))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct InviteListItem(pub invite_list::InviteListItem);

#[pymethods]
impl InviteListItem {
    #[classmethod]
    #[pyo3(name = "User")]
    fn user(
        _cls: &PyType,
        token: InvitationToken,
        created_on: DateTime,
        claimer_email: String,
        status: &PyAny,
    ) -> PyResult<Self> {
        let token = token.0;
        let created_on = created_on.0;
        let status = py_to_rs_invitation_status(status)?;
        Ok(Self(invite_list::InviteListItem::User {
            token,
            created_on,
            claimer_email,
            status,
        }))
    }

    #[classmethod]
    #[pyo3(name = "device")]
    fn device(
        _cls: &PyType,
        token: InvitationToken,
        created_on: DateTime,
        status: &PyAny,
    ) -> PyResult<Self> {
        let token = token.0;
        let created_on = created_on.0;
        let status = py_to_rs_invitation_status(status)?;
        Ok(Self(invite_list::InviteListItem::Device {
            token,
            created_on,
            status,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct InviteListRep(pub invite_list::Rep);

#[pymethods]
impl InviteListRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, invitations: Vec<InviteListItem>) -> PyResult<Self> {
        let invitations = invitations.into_iter().map(|inv| inv.0).collect();
        Ok(Self(invite_list::Rep::Ok { invitations }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_list::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct InviteInfoReq(pub invite_info::Req);

#[pymethods]
impl InviteInfoReq {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(invite_info::Req))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct InviteInfoRep(pub invite_info::Rep);

#[pymethods]
impl InviteInfoRep {
    #[classmethod]
    #[pyo3(name = "OkUser")]
    fn ok_user(
        _cls: &PyType,
        claimer_email: String,
        greeter_user_id: UserID,
        greeter_human_handle: HumanHandle,
    ) -> PyResult<Self> {
        let greeter_user_id = greeter_user_id.0;
        let greeter_human_handle = greeter_human_handle.0;
        Ok(Self(invite_info::Rep::Ok(
            invite_info::UserOrDevice::User {
                claimer_email,
                greeter_user_id,
                greeter_human_handle,
            },
        )))
    }

    #[classmethod]
    #[pyo3(name = "OkDevice")]
    fn ok_device(
        _cls: &PyType,
        greeter_user_id: UserID,
        greeter_human_handle: HumanHandle,
    ) -> PyResult<Self> {
        let greeter_user_id = greeter_user_id.0;
        let greeter_human_handle = greeter_human_handle.0;
        Ok(Self(invite_info::Rep::Ok(
            invite_info::UserOrDevice::Device {
                greeter_user_id,
                greeter_human_handle,
            },
        )))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_info::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct Invite1ClaimerWaitPeerReq(pub invite_1_claimer_wait_peer::Req);

#[pymethods]
impl Invite1ClaimerWaitPeerReq {
    #[new]
    fn new(claimer_public_key: PublicKey) -> PyResult<Self> {
        let claimer_public_key = claimer_public_key.0;
        Ok(Self(invite_1_claimer_wait_peer::Req { claimer_public_key }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Invite1ClaimerWaitPeerRep(pub invite_1_claimer_wait_peer::Rep);

#[pymethods]
impl Invite1ClaimerWaitPeerRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, greeter_public_key: PublicKey) -> PyResult<Self> {
        let greeter_public_key = greeter_public_key.0;
        Ok(Self(invite_1_claimer_wait_peer::Rep::Ok {
            greeter_public_key,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_1_claimer_wait_peer::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "InvalidState")]
    fn invalid_state(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_1_claimer_wait_peer::Rep::InvalidState))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_1_claimer_wait_peer::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct Invite1GreeterWaitPeerReq(pub invite_1_greeter_wait_peer::Req);

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

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Invite1GreeterWaitPeerRep(pub invite_1_greeter_wait_peer::Rep);

#[pymethods]
impl Invite1GreeterWaitPeerRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, claimer_public_key: PublicKey) -> PyResult<Self> {
        let claimer_public_key = claimer_public_key.0;
        Ok(Self(invite_1_greeter_wait_peer::Rep::Ok {
            claimer_public_key,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_1_greeter_wait_peer::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyDeleted")]
    fn already_deleted(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_1_greeter_wait_peer::Rep::AlreadyDeleted))
    }

    #[classmethod]
    #[pyo3(name = "InvalidState")]
    fn invalid_state(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_1_greeter_wait_peer::Rep::InvalidState))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_1_greeter_wait_peer::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct Invite2aClaimerSendHashedNonceHashNonceReq(
    pub invite_2a_claimer_send_hashed_nonce_hash_nonce::Req,
);

#[pymethods]
impl Invite2aClaimerSendHashedNonceHashNonceReq {
    #[new]
    fn new(claimer_hashed_nonce: HashDigest) -> PyResult<Self> {
        let claimer_hashed_nonce = claimer_hashed_nonce.0;
        Ok(Self(invite_2a_claimer_send_hashed_nonce_hash_nonce::Req {
            claimer_hashed_nonce,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Invite2aClaimerSendHashedNonceHashNonceRep(
    pub invite_2a_claimer_send_hashed_nonce_hash_nonce::Rep,
);

#[pymethods]
impl Invite2aClaimerSendHashedNonceHashNonceRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, greeter_nonce: Vec<u8>) -> PyResult<Self> {
        Ok(Self(
            invite_2a_claimer_send_hashed_nonce_hash_nonce::Rep::Ok { greeter_nonce },
        ))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(
            invite_2a_claimer_send_hashed_nonce_hash_nonce::Rep::NotFound,
        ))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyDeleted")]
    fn already_deleted(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(
            invite_2a_claimer_send_hashed_nonce_hash_nonce::Rep::AlreadyDeleted,
        ))
    }

    #[classmethod]
    #[pyo3(name = "InvalidState")]
    fn invalid_state(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(
            invite_2a_claimer_send_hashed_nonce_hash_nonce::Rep::InvalidState,
        ))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_2a_claimer_send_hashed_nonce_hash_nonce::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct Invite2aGreeterGetHashedNonceReq(pub invite_2a_greeter_get_hashed_nonce::Req);

#[pymethods]
impl Invite2aGreeterGetHashedNonceReq {
    #[new]
    fn new(token: InvitationToken) -> PyResult<Self> {
        let token = token.0;
        Ok(Self(invite_2a_greeter_get_hashed_nonce::Req { token }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Invite2aGreeterGetHashedNonceRep(pub invite_2a_greeter_get_hashed_nonce::Rep);

#[pymethods]
impl Invite2aGreeterGetHashedNonceRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, claimer_hashed_nonce: HashDigest) -> PyResult<Self> {
        let claimer_hashed_nonce = claimer_hashed_nonce.0;
        Ok(Self(invite_2a_greeter_get_hashed_nonce::Rep::Ok {
            claimer_hashed_nonce,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_2a_greeter_get_hashed_nonce::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyDeleted")]
    fn already_deleted(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(
            invite_2a_greeter_get_hashed_nonce::Rep::AlreadyDeleted,
        ))
    }

    #[classmethod]
    #[pyo3(name = "InvalidState")]
    fn invalid_state(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_2a_greeter_get_hashed_nonce::Rep::InvalidState))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_2a_greeter_get_hashed_nonce::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct Invite2bClaimerSendNonceReq(pub invite_2b_claimer_send_nonce::Req);

#[pymethods]
impl Invite2bClaimerSendNonceReq {
    #[new]
    fn new(claimer_nonce: Vec<u8>) -> PyResult<Self> {
        Ok(Self(invite_2b_claimer_send_nonce::Req { claimer_nonce }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Invite2bClaimerSendNonceRep(pub invite_2b_claimer_send_nonce::Rep);

#[pymethods]
impl Invite2bClaimerSendNonceRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_2b_claimer_send_nonce::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_2b_claimer_send_nonce::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "InvalidState")]
    fn invalid_state(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_2b_claimer_send_nonce::Rep::InvalidState))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_2b_claimer_send_nonce::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct Invite2bGreeterSendNonceReq(pub invite_2b_greeter_send_nonce::Req);

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

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Invite2bGreeterSendNonceRep(pub invite_2b_greeter_send_nonce::Rep);

#[pymethods]
impl Invite2bGreeterSendNonceRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, claimer_nonce: Vec<u8>) -> PyResult<Self> {
        Ok(Self(invite_2b_greeter_send_nonce::Rep::Ok {
            claimer_nonce,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_2b_greeter_send_nonce::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyDeleted")]
    fn already_deleted(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_2b_greeter_send_nonce::Rep::AlreadyDeleted))
    }

    #[classmethod]
    #[pyo3(name = "InvalidState")]
    fn invalid_state(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_2b_greeter_send_nonce::Rep::InvalidState))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_2b_greeter_send_nonce::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct Invite3aClaimerSignifyTrustReq(pub invite_3a_claimer_signify_trust::Req);

#[pymethods]
impl Invite3aClaimerSignifyTrustReq {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(invite_3a_claimer_signify_trust::Req))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Invite3aClaimerSignifyTrustRep(pub invite_3a_claimer_signify_trust::Rep);

#[pymethods]
impl Invite3aClaimerSignifyTrustRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3a_claimer_signify_trust::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3a_claimer_signify_trust::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "InvalidState")]
    fn invalid_state(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3a_claimer_signify_trust::Rep::InvalidState))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_3a_claimer_signify_trust::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct Invite3aGreeterWaitPeerTrustReq(pub invite_3a_greeter_wait_peer_trust::Req);

#[pymethods]
impl Invite3aGreeterWaitPeerTrustReq {
    #[new]
    fn new(token: InvitationToken) -> PyResult<Self> {
        let token = token.0;
        Ok(Self(invite_3a_greeter_wait_peer_trust::Req { token }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Invite3aGreeterWaitPeerTrustRep(pub invite_3a_greeter_wait_peer_trust::Rep);

#[pymethods]
impl Invite3aGreeterWaitPeerTrustRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3a_greeter_wait_peer_trust::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3a_greeter_wait_peer_trust::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyDeleted")]
    fn already_deleted(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3a_greeter_wait_peer_trust::Rep::AlreadyDeleted))
    }

    #[classmethod]
    #[pyo3(name = "InvalidState")]
    fn invalid_state(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3a_greeter_wait_peer_trust::Rep::InvalidState))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_3a_greeter_wait_peer_trust::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct Invite3bClaimerWaitPeerTrustReq(pub invite_3b_claimer_wait_peer_trust::Req);

#[pymethods]
impl Invite3bClaimerWaitPeerTrustReq {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(invite_3b_claimer_wait_peer_trust::Req))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Invite3bClaimerWaitPeerTrustRep(pub invite_3b_claimer_wait_peer_trust::Rep);

#[pymethods]
impl Invite3bClaimerWaitPeerTrustRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3b_claimer_wait_peer_trust::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3b_claimer_wait_peer_trust::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "InvalidState")]
    fn invalid_state(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3b_claimer_wait_peer_trust::Rep::InvalidState))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_3b_claimer_wait_peer_trust::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct Invite3bGreeterSignifyTrustReq(pub invite_3b_greeter_signify_trust::Req);

#[pymethods]
impl Invite3bGreeterSignifyTrustReq {
    #[new]
    fn new(token: InvitationToken) -> PyResult<Self> {
        let token = token.0;
        Ok(Self(invite_3b_greeter_signify_trust::Req { token }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Invite3bGreeterSignifyTrustRep(pub invite_3b_greeter_signify_trust::Rep);

#[pymethods]
impl Invite3bGreeterSignifyTrustRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3b_greeter_signify_trust::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3b_greeter_signify_trust::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyDeleted")]
    fn already_deleted(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3b_greeter_signify_trust::Rep::AlreadyDeleted))
    }

    #[classmethod]
    #[pyo3(name = "InvalidState")]
    fn invalid_state(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_3b_greeter_signify_trust::Rep::InvalidState))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_3b_greeter_signify_trust::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct Invite4ClaimerCommunicateReq(pub invite_4_claimer_communicate::Req);

#[pymethods]
impl Invite4ClaimerCommunicateReq {
    #[new]
    fn new(payload: Vec<u8>) -> PyResult<Self> {
        Ok(Self(invite_4_claimer_communicate::Req { payload }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Invite4ClaimerCommunicateRep(pub invite_4_claimer_communicate::Rep);

#[pymethods]
impl Invite4ClaimerCommunicateRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, payload: Vec<u8>) -> PyResult<Self> {
        Ok(Self(invite_4_claimer_communicate::Rep::Ok { payload }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_4_claimer_communicate::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "InvalidState")]
    fn invalid_state(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_4_claimer_communicate::Rep::InvalidState))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_4_claimer_communicate::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct Invite4GreeterCommunicateReq(pub invite_4_greeter_communicate::Req);

#[pymethods]
impl Invite4GreeterCommunicateReq {
    #[new]
    fn new(token: InvitationToken, payload: Vec<u8>) -> PyResult<Self> {
        let token = token.0;
        Ok(Self(invite_4_greeter_communicate::Req { token, payload }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Invite4GreeterCommunicateRep(pub invite_4_greeter_communicate::Rep);

#[pymethods]
impl Invite4GreeterCommunicateRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, payload: Vec<u8>) -> PyResult<Self> {
        Ok(Self(invite_4_greeter_communicate::Rep::Ok { payload }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_4_greeter_communicate::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyDeleted")]
    fn already_deleted(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_4_greeter_communicate::Rep::AlreadyDeleted))
    }

    #[classmethod]
    #[pyo3(name = "InvalidState")]
    fn invalid_state(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(invite_4_greeter_communicate::Rep::InvalidState))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invite_4_greeter_communicate::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
