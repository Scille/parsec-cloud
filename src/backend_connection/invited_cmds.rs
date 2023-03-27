// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{exceptions::PyValueError, pyclass, pymethods, PyResult};
use std::sync::Arc;

use libparsec::{client_connection, protocol::invited_cmds};

use crate::{
    addrs::BackendInvitationAddr,
    api_crypto::{HashDigest, PublicKey},
    binding_utils::BytesWrapper,
    protocol::*,
    runtime::FutureIntoCoroutine,
};

#[pyclass]
pub(crate) struct InvitedCmds(pub Arc<client_connection::InvitedCmds>);

#[pymethods]
impl InvitedCmds {
    #[new]
    fn new(addr: BackendInvitationAddr) -> PyResult<Self> {
        let client_config = client_connection::ProxyConfig::new_from_env();

        client_connection::generate_invited_client(addr.0, client_config)
            .map_err(|e| PyValueError::new_err(format!("Fail to generate an invited client: {e}")))
            .map(|client| Self(Arc::new(client)))
    }

    #[getter]
    fn addr(&self) -> BackendInvitationAddr {
        BackendInvitationAddr(self.0.addr().clone())
    }

    fn invite_1_claimer_wait_peer(&self, claimer_public_key: PublicKey) -> FutureIntoCoroutine {
        let invited_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let claimer_public_key = claimer_public_key.0;

            let req = invited_cmds::v2::invite_1_claimer_wait_peer::Req { claimer_public_key };

            crate::binding_utils::send_command!(
                invited_cmds,
                req,
                invited_cmds::v2::invite_1_claimer_wait_peer,
                Invite1ClaimerWaitPeerRep,
                Ok,
                AlreadyDeleted,
                NotFound,
                InvalidState,
                UnknownStatus
            )
        })
    }

    fn invite_2a_claimer_send_hashed_nonce(
        &self,
        claimer_hashed_nonce: HashDigest,
    ) -> FutureIntoCoroutine {
        let invited_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let claimer_hashed_nonce = claimer_hashed_nonce.0;

            let req = invited_cmds::v2::invite_2a_claimer_send_hashed_nonce::Req {
                claimer_hashed_nonce,
            };

            crate::binding_utils::send_command!(
                invited_cmds,
                req,
                invited_cmds::v2::invite_2a_claimer_send_hashed_nonce,
                Invite2aClaimerSendHashedNonceRep,
                Ok,
                AlreadyDeleted,
                NotFound,
                InvalidState,
                UnknownStatus
            )
        })
    }

    fn invite_2b_claimer_send_nonce(&self, claimer_nonce: BytesWrapper) -> FutureIntoCoroutine {
        let invited_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(claimer_nonce);

        FutureIntoCoroutine::from_raw(async move {
            let req = invited_cmds::v2::invite_2b_claimer_send_nonce::Req { claimer_nonce };

            crate::binding_utils::send_command!(
                invited_cmds,
                req,
                invited_cmds::v2::invite_2b_claimer_send_nonce,
                Invite2bClaimerSendNonceRep,
                Ok,
                AlreadyDeleted,
                NotFound,
                InvalidState,
                UnknownStatus
            )
        })
    }

    fn invite_3a_claimer_signify_trust(&self) -> FutureIntoCoroutine {
        let invited_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = invited_cmds::v2::invite_3a_claimer_signify_trust::Req;

            crate::binding_utils::send_command!(
                invited_cmds,
                req,
                invited_cmds::v2::invite_3a_claimer_signify_trust,
                Invite3aClaimerSignifyTrustRep,
                Ok,
                AlreadyDeleted,
                NotFound,
                InvalidState,
                UnknownStatus
            )
        })
    }

    fn invite_3b_claimer_wait_peer_trust(&self) -> FutureIntoCoroutine {
        let invited_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = invited_cmds::v2::invite_3b_claimer_wait_peer_trust::Req;

            crate::binding_utils::send_command!(
                invited_cmds,
                req,
                invited_cmds::v2::invite_3b_claimer_wait_peer_trust,
                Invite3bClaimerWaitPeerTrustRep,
                Ok,
                AlreadyDeleted,
                NotFound,
                InvalidState,
                UnknownStatus
            )
        })
    }

    fn invite_4_claimer_communicate(&self, payload: BytesWrapper) -> FutureIntoCoroutine {
        let invited_cmds = self.0.clone();

        crate::binding_utils::unwrap_bytes!(payload);

        FutureIntoCoroutine::from_raw(async move {
            let req = invited_cmds::v2::invite_4_claimer_communicate::Req { payload };

            crate::binding_utils::send_command!(
                invited_cmds,
                req,
                invited_cmds::v2::invite_4_claimer_communicate,
                Invite4ClaimerCommunicateRep,
                Ok,
                AlreadyDeleted,
                NotFound,
                InvalidState,
                UnknownStatus
            )
        })
    }

    fn invite_info(&self) -> FutureIntoCoroutine {
        let invited_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = invited_cmds::v2::invite_info::Req;

            crate::binding_utils::send_command!(
                invited_cmds,
                req,
                invited_cmds::v2::invite_info,
                InviteInfoRep,
                Ok,
                UnknownStatus
            )
        })
    }

    #[args(ping = "String::new()")]
    fn ping(&self, ping: String) -> FutureIntoCoroutine {
        let invited_cmds = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let req = invited_cmds::v2::ping::Req { ping };

            crate::binding_utils::send_command!(
                invited_cmds,
                req,
                invited_cmds::v2::ping,
                InvitedPingRep,
                Ok,
                UnknownStatus
            )
        })
    }
}
