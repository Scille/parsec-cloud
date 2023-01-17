// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use pyo3::{
    pyclass, pyclass_init::PyObjectInit, pymethods, PyClassInitializer, PyErr, PyObject, PyResult,
    PyTypeInfo, Python,
};

use libparsec::{client_connection, protocol::authenticated_cmds};

use crate::{
    addrs::BackendOrganizationAddr,
    api_crypto::SigningKey,
    ids::DeviceID,
    protocol::{AuthenticatedPingRep, AuthenticatedPingRepOk, AuthenticatedPingRepUnknownStatus},
    runtime::FutureIntoCoroutine,
};

use super::CommandErrorExc;

#[pyclass]
pub(crate) struct AuthenticatedCmds(Arc<client_connection::AuthenticatedCmds>);

#[pymethods]
impl AuthenticatedCmds {
    #[new]
    fn new(
        server_url: BackendOrganizationAddr,
        device_id: DeviceID,
        signing_key: SigningKey,
    ) -> PyResult<Self> {
        let auth_cmds =
            client_connection::client::generate_client(signing_key.0, device_id.0, server_url.0);
        Ok(Self(Arc::new(auth_cmds)))
    }

    fn ping(&self, ping: String) -> FutureIntoCoroutine {
        let auth_cmds = self.0.clone();

        FutureIntoCoroutine::from(async move {
            auth_cmds
                .ping(ping)
                .await
                .map(AuthenticatedPingRep)
                .map(|r| {
                    Python::with_gil(|py| match &r.0 {
                        authenticated_cmds::v3::ping::Rep::Ok { .. } => {
                            crate::binding_utils::py_object!(
                                r,
                                AuthenticatedPingRepOk,
                                py,
                                init_non_self
                            )
                        }
                        authenticated_cmds::v3::ping::Rep::UnknownStatus { .. } => {
                            crate::binding_utils::py_object!(
                                r,
                                AuthenticatedPingRepUnknownStatus,
                                py,
                                init_non_self
                            )
                        }
                    })
                })
                .map_err(|e| Into::<PyErr>::into(Into::<CommandErrorExc>::into(e)))
        })
    }
}
