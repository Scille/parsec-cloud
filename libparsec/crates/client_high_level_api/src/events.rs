// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::ClientHandle;

#[derive(Debug)]
pub enum ClientEvent {
    ClientConnectionChanged { client: ClientHandle },
    WorkspaceReencryptionNeeded,
    WorkspaceReencryptionStarted,
    WorkspaceReencryptionEnded,
    // TODO finish me
}
