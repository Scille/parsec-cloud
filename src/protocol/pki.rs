// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyAttributeError,
    pyclass, pymethods,
    types::{PyBytes, PyType},
    IntoPy, PyClassInitializer, PyObject, PyRef, PyResult, Python,
};

use libparsec::protocol::{
    anonymous_cmds::v2::{pki_enrollment_info, pki_enrollment_submit},
    authenticated_cmds::{
        v2::pki_enrollment_accept,
        v2::{pki_enrollment_list, pki_enrollment_reject},
    },
    Request,
};

use crate::{
    binding_utils::BytesWrapper,
    ids::EnrollmentID,
    protocol::{
        error::{ProtocolError, ProtocolErrorFields, ProtocolResult},
        Reason,
    },
    time::DateTime,
};

use super::gen_rep;

#[pyclass]
#[derive(Clone)]
pub(crate) struct PkiEnrollmentAcceptReq(pub pki_enrollment_accept::Req);

#[pymethods]
impl PkiEnrollmentAcceptReq {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        accept_payload: BytesWrapper,
        accept_payload_signature: BytesWrapper,
        accepter_der_x509_certificate: BytesWrapper,
        enrollment_id: &EnrollmentID,
        device_certificate: BytesWrapper,
        user_certificate: BytesWrapper,
        redacted_device_certificate: BytesWrapper,
        redacted_user_certificate: BytesWrapper,
    ) -> Self {
        crate::binding_utils::unwrap_bytes!(
            accept_payload,
            accept_payload_signature,
            accepter_der_x509_certificate,
            device_certificate,
            user_certificate,
            redacted_device_certificate,
            redacted_user_certificate
        );
        Self(pki_enrollment_accept::Req {
            accept_payload,
            accept_payload_signature,
            accepter_der_x509_certificate,
            device_certificate,
            enrollment_id: enrollment_id.0,
            redacted_device_certificate,
            redacted_user_certificate,
            user_certificate,
        })
    }

    #[getter]
    fn accept_payload<'py>(&'py self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.accept_payload)
    }

    #[getter]
    fn accept_payload_signature<'py>(&'py self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.accept_payload_signature)
    }

    #[getter]
    fn accepter_der_x509_certificate<'py>(&'py self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.accepter_der_x509_certificate)
    }

    #[getter]
    fn device_certificate<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.device_certificate)
    }

    #[getter]
    fn user_certificate<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.user_certificate)
    }

    #[getter]
    fn redacted_user_certificate<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.redacted_user_certificate)
    }

    #[getter]
    fn redacted_device_certificate<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.redacted_device_certificate)
    }

    #[getter]
    fn enrollment_id(&self) -> EnrollmentID {
        EnrollmentID(self.0.enrollment_id)
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }
}

gen_rep!(
    pki_enrollment_accept,
    PkiEnrollmentAcceptRep,
    [NotAllowed, reason: Reason],
    [InvalidPayloadData, reason: Reason],
    [InvalidCertification, reason: Reason],
    [InvalidData, reason: Reason],
    [NotFound, reason: Reason],
    [NoLongerAvailable, reason: Reason],
    [AlreadyExists, reason: Reason],
    [ActiveUsersLimitReached],
);

#[pyclass(extends=PkiEnrollmentAcceptRep)]
pub(crate) struct PkiEnrollmentAcceptRepOk;

#[pymethods]
impl PkiEnrollmentAcceptRepOk {
    #[new]
    fn new() -> (Self, PkiEnrollmentAcceptRep) {
        (Self, PkiEnrollmentAcceptRep(pki_enrollment_accept::Rep::Ok))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) enum PkiEnrollmentStatus {
    #[pyo3(name = "SUBMITTED")]
    Submitted,
    #[pyo3(name = "ACCEPTED")]
    Accepted,
    #[pyo3(name = "REJECTED")]
    Rejected,
    #[pyo3(name = "CANCELLED")]
    Cancelled,
}

#[pymethods]
impl PkiEnrollmentStatus {
    /// Behave like python's `value` attribute
    #[getter]
    fn value(&self) -> &'static str {
        match self {
            PkiEnrollmentStatus::Submitted => "SUBMITTED",
            PkiEnrollmentStatus::Accepted => "ACCEPTED",
            PkiEnrollmentStatus::Rejected => "REJECTED",
            PkiEnrollmentStatus::Cancelled => "CANCELLED",
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct PkiEnrollmentInfoStatus(pki_enrollment_info::PkiEnrollmentInfoStatus);

#[pymethods]
impl PkiEnrollmentInfoStatus {
    #[classmethod]
    #[pyo3(name = "Submitted")]
    fn submitted(_cls: &PyType, submitted_on: DateTime) -> Self {
        Self(pki_enrollment_info::PkiEnrollmentInfoStatus::Submitted {
            submitted_on: submitted_on.0,
        })
    }

    #[classmethod]
    #[pyo3(name = "Accepted")]
    fn accepted(
        _cls: &PyType,
        submitted_on: DateTime,
        accepted_on: DateTime,
        accepter_der_x509_certificate: BytesWrapper,
        accept_payload_signature: BytesWrapper,
        accept_payload: BytesWrapper,
    ) -> Self {
        crate::binding_utils::unwrap_bytes!(
            accepter_der_x509_certificate,
            accept_payload_signature,
            accept_payload
        );
        Self(pki_enrollment_info::PkiEnrollmentInfoStatus::Accepted {
            submitted_on: submitted_on.0,
            accepted_on: accepted_on.0,
            accepter_der_x509_certificate,
            accept_payload,
            accept_payload_signature,
        })
    }

    #[classmethod]
    #[pyo3(name = "Rejected")]
    fn rejected(_cls: &PyType, submitted_on: DateTime, rejected_on: DateTime) -> Self {
        Self(pki_enrollment_info::PkiEnrollmentInfoStatus::Rejected {
            submitted_on: submitted_on.0,
            rejected_on: rejected_on.0,
        })
    }

    #[classmethod]
    #[pyo3(name = "Cancelled")]
    fn cancelled(_cls: &PyType, submitted_on: DateTime, cancelled_on: DateTime) -> Self {
        Self(pki_enrollment_info::PkiEnrollmentInfoStatus::Cancelled {
            submitted_on: submitted_on.0,
            cancelled_on: cancelled_on.0,
        })
    }

    #[getter]
    fn status(&self) -> PkiEnrollmentStatus {
        match self.0 {
            pki_enrollment_info::PkiEnrollmentInfoStatus::Accepted { .. } => {
                PkiEnrollmentStatus::Accepted
            }
            pki_enrollment_info::PkiEnrollmentInfoStatus::Cancelled { .. } => {
                PkiEnrollmentStatus::Cancelled
            }
            pki_enrollment_info::PkiEnrollmentInfoStatus::Submitted { .. } => {
                PkiEnrollmentStatus::Submitted
            }
            pki_enrollment_info::PkiEnrollmentInfoStatus::Rejected { .. } => {
                PkiEnrollmentStatus::Rejected
            }
        }
    }
}

#[pyclass]
pub(crate) struct PkiEnrollmentListReq(pub pki_enrollment_list::Req);

#[pymethods]
impl PkiEnrollmentListReq {
    #[new]
    fn new() -> Self {
        Self(pki_enrollment_list::Req)
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct PkiEnrollmentListItem(pki_enrollment_list::PkiEnrollmentListItem);

#[pymethods]
impl PkiEnrollmentListItem {
    #[new]
    fn new(
        enrollment_id: EnrollmentID,
        submit_payload: BytesWrapper,
        submit_payload_signature: BytesWrapper,
        submitted_on: DateTime,
        submitter_der_x509_certificate: BytesWrapper,
    ) -> Self {
        crate::binding_utils::unwrap_bytes!(
            submit_payload,
            submit_payload_signature,
            submitter_der_x509_certificate
        );
        Self(pki_enrollment_list::PkiEnrollmentListItem {
            enrollment_id: enrollment_id.0,
            submit_payload,
            submit_payload_signature,
            submitted_on: submitted_on.0,
            submitter_der_x509_certificate,
        })
    }

    #[getter]
    fn enrollment_id(&self) -> EnrollmentID {
        EnrollmentID(self.0.enrollment_id)
    }

    #[getter]
    fn submit_payload<'py>(&'py self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.submit_payload)
    }

    #[getter]
    fn submit_payload_signature<'py>(&'py self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.submit_payload_signature)
    }

    #[getter]
    fn submitted_on(&self) -> DateTime {
        DateTime(self.0.submitted_on)
    }

    #[getter]
    fn submitter_der_x509_certificate<'py>(&'py self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.submitter_der_x509_certificate)
    }
}

gen_rep!(
    pki_enrollment_list,
    PkiEnrollmentListRep,
    { .. },
    [NotAllowed, reason: Reason]
);

#[pyclass(extends = PkiEnrollmentListRep)]
pub(crate) struct PkiEnrollmentListRepOk;

#[pymethods]
impl PkiEnrollmentListRepOk {
    #[new]
    fn new(enrollments: Vec<PkiEnrollmentListItem>) -> (Self, PkiEnrollmentListRep) {
        (
            Self,
            PkiEnrollmentListRep(pki_enrollment_list::Rep::Ok {
                enrollments: enrollments.into_iter().map(|e| e.0).collect(),
            }),
        )
    }

    #[getter]
    fn enrollments(_self: PyRef<'_, Self>) -> PyResult<Vec<PkiEnrollmentListItem>> {
        if let pki_enrollment_list::Rep::Ok { enrollments } = &_self.as_ref().0 {
            Ok(enrollments
                .iter()
                .map(|e| PkiEnrollmentListItem(e.clone()))
                .collect())
        } else {
            Err(PyAttributeError::new_err("No such attribute `enrollments`"))
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct PkiEnrollmentRejectReq(pub pki_enrollment_reject::Req);

#[pymethods]
impl PkiEnrollmentRejectReq {
    #[new]
    fn new(enrollment_id: EnrollmentID) -> Self {
        Self(pki_enrollment_reject::Req {
            enrollment_id: enrollment_id.0,
        })
    }

    #[getter]
    fn enrollment_id(&self) -> EnrollmentID {
        EnrollmentID(self.0.enrollment_id)
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }
}

gen_rep!(
    pki_enrollment_reject,
    PkiEnrollmentRejectRep,
    { .. },
    [NotAllowed, reason: Reason],
    [NotFound, reason: Reason],
    [NoLongerAvailable, reason: Reason]
);

#[pyclass(extends = PkiEnrollmentRejectRep)]
#[derive(Clone)]
pub(crate) struct PkiEnrollmentRejectRepOk;

#[pymethods]
impl PkiEnrollmentRejectRepOk {
    #[new]
    fn new() -> (Self, PkiEnrollmentRejectRep) {
        (Self, PkiEnrollmentRejectRep(pki_enrollment_reject::Rep::Ok))
    }
}

#[derive(Clone)]
#[pyclass]
pub(crate) struct PkiEnrollmentSubmitReq(pub pki_enrollment_submit::Req);

#[pymethods]
impl PkiEnrollmentSubmitReq {
    #[new]
    #[args(submitter_der_x509_certificate_email = "None")]
    fn new(
        enrollment_id: EnrollmentID,
        force: bool,
        submitter_der_x509_certificate: BytesWrapper,
        submit_payload_signature: BytesWrapper,
        submit_payload: BytesWrapper,
        submitter_der_x509_certificate_email: Option<String>,
    ) -> Self {
        crate::binding_utils::unwrap_bytes!(
            submitter_der_x509_certificate,
            submit_payload_signature,
            submit_payload
        );
        Self(pki_enrollment_submit::Req {
            enrollment_id: enrollment_id.0,
            force,
            submitter_der_x509_certificate,
            submitter_der_x509_certificate_email,
            submit_payload_signature,
            submit_payload,
        })
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }

    #[getter]
    fn enrollment_id(&self) -> EnrollmentID {
        EnrollmentID(self.0.enrollment_id)
    }

    #[getter]
    fn force(&self) -> bool {
        self.0.force
    }

    #[getter]
    fn submitter_der_x509_certificate<'py>(&self, python: Python<'py>) -> &'py PyBytes {
        PyBytes::new(python, &self.0.submitter_der_x509_certificate)
    }

    #[getter]
    fn submitter_der_x509_certificate_email(&self) -> Option<&'_ String> {
        self.0.submitter_der_x509_certificate_email.as_ref()
    }

    #[getter]
    fn submit_payload_signature<'py>(&self, python: Python<'py>) -> &'py PyBytes {
        PyBytes::new(python, &self.0.submit_payload_signature)
    }

    #[getter]
    fn submit_payload<'py>(&self, python: Python<'py>) -> &'py PyBytes {
        PyBytes::new(python, &self.0.submit_payload)
    }
}

gen_rep!(
    pki_enrollment_submit,
    PkiEnrollmentSubmitRep,
    { .. },
    [AlreadySubmitted, submitted_on: DateTime],
    [InvalidPayloadData, reason: Reason],
    [IdAlreadyUsed],
    [EmailAlreadyUsed],
    [AlreadyEnrolled],
);

#[derive(Clone)]
#[pyclass(extends = PkiEnrollmentSubmitRep)]
pub(crate) struct PkiEnrollmentSubmitRepOk;

#[pymethods]
impl PkiEnrollmentSubmitRepOk {
    #[new]
    fn new(submitted_on: DateTime) -> (Self, PkiEnrollmentSubmitRep) {
        (
            Self,
            PkiEnrollmentSubmitRep(pki_enrollment_submit::Rep::Ok {
                submitted_on: submitted_on.0,
            }),
        )
    }

    #[getter]
    fn submitted_on(_self: PyRef<Self, '_>) -> PyResult<DateTime> {
        if let pki_enrollment_submit::Rep::Ok { submitted_on } = _self.as_ref().0 {
            Ok(DateTime(submitted_on))
        } else {
            Err(PyAttributeError::new_err(
                "No such attribute `submitted_on`",
            ))
        }
    }
}

#[pyclass]
pub(crate) struct PkiEnrollmentInfoReq(pub pki_enrollment_info::Req);

#[pymethods]
impl PkiEnrollmentInfoReq {
    #[new]
    fn new(enrollment_id: EnrollmentID) -> Self {
        PkiEnrollmentInfoReq(pki_enrollment_info::Req {
            enrollment_id: enrollment_id.0,
        })
    }

    #[getter]
    fn enrollment_id(&self) -> EnrollmentID {
        EnrollmentID(self.0.enrollment_id)
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }
}

gen_rep!(
    pki_enrollment_info,
    PkiEnrollmentInfoRep,
    { .. },
    [NotFound, reason: Reason],
);

#[pyclass(extends = PkiEnrollmentInfoRep)]
#[derive(Clone)]
pub(crate) struct PkiEnrollmentInfoRepOk;

#[pymethods]
impl PkiEnrollmentInfoRepOk {
    #[new]
    fn new(info_status: PkiEnrollmentInfoStatus) -> (Self, PkiEnrollmentInfoRep) {
        (
            Self,
            PkiEnrollmentInfoRep(pki_enrollment_info::Rep::Ok(info_status.0)),
        )
    }

    #[getter]
    fn enrollment_status(_self: PyRef<'_, Self>) -> PyResult<PkiEnrollmentInfoStatus> {
        if let pki_enrollment_info::Rep::Ok(infos) = &_self.as_ref().0 {
            Ok(PkiEnrollmentInfoStatus(infos.clone()))
        } else {
            Err(PyAttributeError::new_err(
                "No such field 'enrollment_status'",
            ))
        }
    }

    #[getter]
    fn accepter_der_x509_certificate<'py>(
        _self: PyRef<'py, Self>,
        python: Python<'py>,
    ) -> PyResult<&'py PyBytes> {
        if let pki_enrollment_info::Rep::Ok(
            pki_enrollment_info::PkiEnrollmentInfoStatus::Accepted {
                accepter_der_x509_certificate,
                ..
            },
        ) = &_self.as_ref().0
        {
            Ok(PyBytes::new(python, accepter_der_x509_certificate))
        } else {
            Err(PyAttributeError::new_err(
                "No such field 'accepter_der_x509_certificate'",
            ))
        }
    }

    #[getter]
    fn accept_payload_signature<'py>(
        _self: PyRef<'py, Self>,
        python: Python<'py>,
    ) -> PyResult<&'py PyBytes> {
        if let pki_enrollment_info::Rep::Ok(
            pki_enrollment_info::PkiEnrollmentInfoStatus::Accepted {
                accept_payload_signature,
                ..
            },
        ) = &_self.as_ref().0
        {
            Ok(PyBytes::new(python, accept_payload_signature))
        } else {
            Err(PyAttributeError::new_err(
                "No such field 'accept_payload_signature'",
            ))
        }
    }

    #[getter]
    fn accept_payload<'py>(_self: PyRef<'py, Self>, python: Python<'py>) -> PyResult<&'py PyBytes> {
        if let pki_enrollment_info::Rep::Ok(
            pki_enrollment_info::PkiEnrollmentInfoStatus::Accepted { accept_payload, .. },
        ) = &_self.as_ref().0
        {
            Ok(PyBytes::new(python, accept_payload))
        } else {
            Err(PyAttributeError::new_err("No such field 'accept_payload'"))
        }
    }

    #[getter]
    fn accepted_on(_self: PyRef<'_, Self>) -> PyResult<DateTime> {
        if let pki_enrollment_info::Rep::Ok(
            pki_enrollment_info::PkiEnrollmentInfoStatus::Accepted { accepted_on, .. },
        ) = &_self.as_ref().0
        {
            Ok(DateTime(*accepted_on))
        } else {
            Err(PyAttributeError::new_err("No such field 'accept_payload'"))
        }
    }

    #[getter]
    fn rejected_on(_self: PyRef<'_, Self>) -> PyResult<DateTime> {
        if let pki_enrollment_info::Rep::Ok(
            pki_enrollment_info::PkiEnrollmentInfoStatus::Rejected { rejected_on, .. },
        ) = &_self.as_ref().0
        {
            Ok(DateTime(*rejected_on))
        } else {
            Err(PyAttributeError::new_err("No such field 'rejected_on'"))
        }
    }

    #[getter]
    fn cancelled_on(_self: PyRef<'_, Self>) -> PyResult<DateTime> {
        if let pki_enrollment_info::Rep::Ok(
            pki_enrollment_info::PkiEnrollmentInfoStatus::Cancelled { cancelled_on, .. },
        ) = &_self.as_ref().0
        {
            Ok(DateTime(*cancelled_on))
        } else {
            Err(PyAttributeError::new_err("No such field 'cancelled_on'"))
        }
    }
}
