// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::{anyhow, thiserror};

#[derive(Debug, thiserror::Error)]
pub enum ClientInfoError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}
