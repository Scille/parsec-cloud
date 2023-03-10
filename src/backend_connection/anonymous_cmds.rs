// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError, pyclass, pymethods, IntoPy, PyAny, PyObject, PyResult, Python,
};
use std::sync::Arc;

use libparsec::{client_connection, protocol::anonymous_cmds, types::Maybe};

use crate::{
    addrs::{BackendOrganizationBootstrapAddr, BackendPkiEnrollmentAddr},
    api_crypto::VerifyKey,
    binding_utils::BytesWrapper,
    ids::EnrollmentID,
    protocol::*,
    runtime::FutureIntoCoroutine,
};

#[pyclass]
pub(crate) struct AnonymousCmds(Arc<client_connection::AnonymousCmds>);

#[pymethods]
impl AnonymousCmds {
    #[new]
    fn new(addr: &PyAny) -> PyResult<Self> {
        let addr = if let Ok(addr) = addr.extract::<BackendOrganizationBootstrapAddr>() {
            libparsec::types::BackendAnonymousAddr::BackendOrganizationBootstrapAddr(addr.0)
        } else if let Ok(addr) = addr.extract::<BackendPkiEnrollmentAddr>() {
            libparsec::types::BackendAnonymousAddr::BackendPkiEnrollmentAddr(addr.0)
        } else {
            return Err(PyValueError::new_err("Invalid addr provided: accepted addr are BackendOrganizationAddr or BackendPkiEnrollmentAddr"));
        };

        let anonymous_cmds = client_connection::generate_anonymous_client(addr);
        Ok(Self(Arc::new(anonymous_cmds)))
    }

    #[getter]
    fn addr(&self, py: Python) -> PyObject {
        match self.0.addr() {
            libparsec::types::BackendAnonymousAddr::BackendOrganizationBootstrapAddr(addr) => {
                BackendOrganizationBootstrapAddr(addr.clone()).into_py(py)
            }
            libparsec::types::BackendAnonymousAddr::BackendPkiEnrollmentAddr(addr) => {
                BackendPkiEnrollmentAddr(addr.clone()).into_py(py)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn organization_bootstrap(
        &self,
        bootstrap_token: String,
        device_certificate: BytesWrapper,
        redacted_device_certificate: BytesWrapper,
        redacted_user_certificate: BytesWrapper,
        root_verify_key: VerifyKey,
        sequester_authority_certificate: Option<BytesWrapper>,
        user_certificate: BytesWrapper,
    ) -> FutureIntoCoroutine {
        let anonymous_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(
            device_certificate,
            redacted_device_certificate,
            redacted_user_certificate,
            sequester_authority_certificate,
            user_certificate
        );

        FutureIntoCoroutine::from_raw(async move {
            let root_verify_key = root_verify_key.0;
            let sequester_authority_certificate = Maybe::Present(sequester_authority_certificate);

            let req = anonymous_cmds::v2::organization_bootstrap::Req {
                bootstrap_token,
                device_certificate,
                redacted_device_certificate,
                redacted_user_certificate,
                root_verify_key,
                sequester_authority_certificate,
                user_certificate,
            };

            crate::binding_utils::send_command!(
                anonymous_cmds,
                req,
                anonymous_cmds::v2::organization_bootstrap,
                OrganizationBootstrapRep,
                Ok,
                AlreadyBootstrapped,
                BadTimestamp,
                InvalidCertification,
                InvalidData,
                NotFound,
                UnknownStatus,
                "handle_bad_timestamp",
            )
        })
    }

    fn pki_enrollment_info(&self, enrollment_id: EnrollmentID) -> FutureIntoCoroutine {
        let anonymous_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let enrollment_id = enrollment_id.0;

            let req = anonymous_cmds::v2::pki_enrollment_info::Req { enrollment_id };

            crate::binding_utils::send_command!(
                anonymous_cmds,
                req,
                anonymous_cmds::v2::pki_enrollment_info,
                PkiEnrollmentInfoRep,
                Ok,
                NotFound,
                UnknownStatus,
            )
        })
    }

    fn pki_enrollment_submit(
        &self,
        enrollment_id: EnrollmentID,
        force: bool,
        submit_payload: BytesWrapper,
        submit_payload_signature: BytesWrapper,
        submitter_der_x509_certificate: BytesWrapper,
        submitter_der_x509_certificate_email: Option<String>,
    ) -> FutureIntoCoroutine {
        let anonymous_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(
            submit_payload,
            submit_payload_signature,
            submitter_der_x509_certificate
        );

        FutureIntoCoroutine::from_raw(async move {
            let enrollment_id = enrollment_id.0;

            let req = anonymous_cmds::v2::pki_enrollment_submit::Req {
                enrollment_id,
                force,
                submit_payload,
                submit_payload_signature,
                submitter_der_x509_certificate,
                submitter_der_x509_certificate_email,
            };

            crate::binding_utils::send_command!(
                anonymous_cmds,
                req,
                anonymous_cmds::v2::pki_enrollment_submit,
                PkiEnrollmentSubmitRep,
                Ok,
                AlreadyEnrolled,
                AlreadySubmitted,
                EmailAlreadyUsed,
                IdAlreadyUsed,
                InvalidPayloadData,
                UnknownStatus,
            )
        })
    }
}
