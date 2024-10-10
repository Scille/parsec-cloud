// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::tos_cmds;
use libparsec_types::prelude::*;

use super::Client;
use crate::event_bus::EventShouldRetryConnectionNow;

#[derive(Debug, thiserror::Error)]
pub enum ClientGetTosError {
    #[error("There are no Terms of Service defined for the organization")]
    NoTos,
    #[error("Cannot reach the server")]
    Offline,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for ClientGetTosError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
/// Terms of Service
pub struct Tos {
    /// Mapping of locale + URL tuples.
    /// e.g. {"en_US": "https://example.com/tos_en.html", "fr_FR": "https://example.com/tos_fr.html"}
    pub per_locale_urls: HashMap<String, String>,
    pub updated_on: DateTime,
}

pub async fn get_tos(client_ops: &Client) -> Result<Tos, ClientGetTosError> {
    let request = tos_cmds::latest::tos_get::Req;
    let rep = client_ops.cmds.send(request).await?;
    match rep {
        tos_cmds::v4::tos_get::Rep::Ok {
            updated_on,
            per_locale_urls,
        } => Ok(Tos {
            per_locale_urls,
            updated_on,
        }),
        tos_cmds::v4::tos_get::Rep::NoTos => Err(ClientGetTosError::NoTos),
        tos_cmds::v4::tos_get::Rep::UnknownStatus { unknown_status, .. } => {
            Err(ClientGetTosError::Internal(anyhow::anyhow!(
                "Unknown error status `{}` from server",
                unknown_status
            )))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClientAcceptTosError {
    #[error("There are no Terms of Service defined for the organization")]
    NoTos,
    #[error("The Terms of Service have changed on the server")]
    TosMismatch,
    #[error("Cannot reach the server")]
    Offline,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for ClientAcceptTosError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn accept_tos(
    client_ops: &Client,
    tos_updated_on: DateTime,
) -> Result<(), ClientAcceptTosError> {
    let request = tos_cmds::latest::tos_accept::Req { tos_updated_on };
    let rep = client_ops.cmds.send(request).await?;
    match rep {
        tos_cmds::v4::tos_accept::Rep::Ok => {
            // If TOS acceptance was required, it most likely means the connection
            // monitor is currently polling the server from time to time in order to
            // detect when the TOS have been accepted.
            // So we send a dedicated event to speed up the process.
            client_ops.event_bus.send(&EventShouldRetryConnectionNow);
            Ok(())
        }
        tos_cmds::v4::tos_accept::Rep::TosMismatch => Err(ClientAcceptTosError::TosMismatch),
        tos_cmds::v4::tos_accept::Rep::NoTos => Err(ClientAcceptTosError::NoTos),
        tos_cmds::v4::tos_accept::Rep::UnknownStatus { unknown_status, .. } => {
            Err(ClientAcceptTosError::Internal(anyhow::anyhow!(
                "Unknown error status `{}` from server",
                unknown_status
            )))
        }
    }
}
