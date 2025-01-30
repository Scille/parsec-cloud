// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use super::{
    AuthenticatedCmds, CertificateOps, DataAccessFetchBlockError, DataAccessFetchManifestError,
    DataAccessGetWorkspaceManifestV1TimestampError,
};
use crate::server_fetch::{
    server_fetch_block, server_fetch_child_manifest,
    server_fetch_versions_remote_workspace_manifest, server_fetch_workspace_manifest,
    ServerFetchBlockError, ServerFetchManifestError, ServerFetchVersionsRemoteManifestError,
};

pub(super) struct ServerDataAccess {
    cmds: Arc<AuthenticatedCmds>,
    certificates_ops: Arc<CertificateOps>,
    realm_id: VlobID,
}

impl ServerDataAccess {
    pub fn new(
        cmds: Arc<AuthenticatedCmds>,
        certificates_ops: Arc<CertificateOps>,
        realm_id: VlobID,
    ) -> Self {
        Self {
            cmds,
            certificates_ops,
            realm_id,
        }
    }

    pub async fn fetch_manifest(
        &self,
        at: DateTime,
        entry_id: VlobID,
    ) -> Result<Option<ArcChildManifest>, DataAccessFetchManifestError> {
        let outcome = if self.realm_id == entry_id {
            server_fetch_workspace_manifest(
                &self.cmds,
                &self.certificates_ops,
                self.realm_id,
                Some(at),
            )
            .await
            .map(ChildManifest::Folder)
        } else {
            server_fetch_child_manifest(
                &self.cmds,
                &self.certificates_ops,
                self.realm_id,
                entry_id,
                Some(at),
            )
            .await
        };

        let maybe_manifest = match outcome {
            Ok(ChildManifest::File(manifest)) => Some(Arc::new(manifest).into()),
            Ok(ChildManifest::Folder(manifest)) => Some(Arc::new(manifest).into()),
            // This is unexpected: we got an entry ID from a parent folder/workspace
            // manifest, but this ID points to nothing according to the server :/
            //
            // That could means two things:
            // - the server is lying to us
            // - the client that have uploaded the parent folder/workspace manifest
            //   was buggy and include the ID of a not-yet-synchronized entry
            //
            // We just pretend the entry doesn't exist
            Err(ServerFetchManifestError::VlobNotFound) => None,
            Err(ServerFetchManifestError::Stopped) => {
                return Err(DataAccessFetchManifestError::Stopped);
            }
            Err(ServerFetchManifestError::Offline) => {
                return Err(DataAccessFetchManifestError::Offline);
            }
            // The realm doesn't exist on server side, hence we are it creator and
            // it data only live on our local storage, which we have already checked.
            Err(ServerFetchManifestError::RealmNotFound) => {
                return Err(DataAccessFetchManifestError::EntryNotFound);
            }
            Err(ServerFetchManifestError::NoRealmAccess) => {
                return Err(DataAccessFetchManifestError::NoRealmAccess);
            }
            Err(ServerFetchManifestError::InvalidKeysBundle(err)) => {
                return Err(DataAccessFetchManifestError::InvalidKeysBundle(err));
            }
            Err(ServerFetchManifestError::InvalidCertificate(err)) => {
                return Err(DataAccessFetchManifestError::InvalidCertificate(err));
            }
            Err(ServerFetchManifestError::InvalidManifest(err)) => {
                return Err(DataAccessFetchManifestError::InvalidManifest(err));
            }
            Err(ServerFetchManifestError::Internal(err)) => {
                return Err(err.context("cannot fetch from server").into());
            }
        };

        Ok(maybe_manifest)
    }

    pub async fn fetch_block(
        &self,
        manifest: &FileManifest,
        access: &BlockAccess,
    ) -> Result<Bytes, DataAccessFetchBlockError> {
        server_fetch_block(
            &self.cmds,
            &self.certificates_ops,
            self.realm_id,
            manifest,
            access,
        )
        .await
        .map_err(|err| match err {
            ServerFetchBlockError::Stopped => DataAccessFetchBlockError::Stopped,
            ServerFetchBlockError::Offline => DataAccessFetchBlockError::Offline,
            ServerFetchBlockError::BlockNotFound => DataAccessFetchBlockError::BlockNotFound,
            ServerFetchBlockError::NoRealmAccess => DataAccessFetchBlockError::NoRealmAccess,
            ServerFetchBlockError::StoreUnavailable => DataAccessFetchBlockError::StoreUnavailable,
            ServerFetchBlockError::InvalidBlockAccess(err) => {
                DataAccessFetchBlockError::InvalidBlockAccess(err)
            }
            ServerFetchBlockError::InvalidKeysBundle(err) => {
                DataAccessFetchBlockError::InvalidKeysBundle(err)
            }
            ServerFetchBlockError::InvalidCertificate(err) => {
                DataAccessFetchBlockError::InvalidCertificate(err)
            }
            ServerFetchBlockError::Internal(err) => err.into(),
        })
    }

    pub async fn get_workspace_manifest_v1_timestamp(
        &self,
    ) -> Result<Option<DateTime>, DataAccessGetWorkspaceManifestV1TimestampError> {
        let outcome = server_fetch_versions_remote_workspace_manifest(
            &self.cmds,
            &self.certificates_ops,
            self.realm_id,
            &[1],
        )
        .await;

        let workspace_manifest_v1_timestamp = match outcome {
            Ok(manifest) => match manifest.first() {
                Some(manifest) => manifest.timestamp,
                // Manifest has never been uploaded to the server yet
                None => return Ok(None),
            },
            Err(err) => {
                return match err {
                    // The realm doesn't exist on server side, hence we are its creator and
                    // the manifest has never been uploaded to the server yet.
                    ServerFetchVersionsRemoteManifestError::RealmNotFound => Ok(None),

                    // Actual errors
                    ServerFetchVersionsRemoteManifestError::Stopped => {
                        Err(DataAccessGetWorkspaceManifestV1TimestampError::Stopped)
                    }
                    ServerFetchVersionsRemoteManifestError::Offline => {
                        Err(DataAccessGetWorkspaceManifestV1TimestampError::Offline)
                    }
                    ServerFetchVersionsRemoteManifestError::NoRealmAccess => {
                        Err(DataAccessGetWorkspaceManifestV1TimestampError::NoRealmAccess)
                    }
                    ServerFetchVersionsRemoteManifestError::InvalidKeysBundle(err) => {
                        Err(DataAccessGetWorkspaceManifestV1TimestampError::InvalidKeysBundle(err))
                    }
                    ServerFetchVersionsRemoteManifestError::InvalidCertificate(err) => {
                        Err(DataAccessGetWorkspaceManifestV1TimestampError::InvalidCertificate(err))
                    }
                    ServerFetchVersionsRemoteManifestError::InvalidManifest(err) => {
                        Err(DataAccessGetWorkspaceManifestV1TimestampError::InvalidManifest(err))
                    }
                    ServerFetchVersionsRemoteManifestError::Internal(err) => Err(err.into()),
                };
            }
        };

        Ok(Some(workspace_manifest_v1_timestamp))
    }
}
