// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use super::{
    AuthenticatedCmds, CertificateOps, DataAccessFetchBlockError, DataAccessFetchManifestError,
};
use crate::server_fetch::{
    server_fetch_block, server_fetch_child_manifest, server_fetch_versions_workspace_manifest,
    server_fetch_workspace_manifest, ServerFetchBlockError, ServerFetchManifestError,
    ServerFetchVersionsManifestError,
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
            // The realm doesn't exist on server side, hence there is no history !
            // Note we shouldn't end up here under normal circumstances, since
            // `get_workspace_manifest_v1` should have been called first during
            // workspace history store initialization, leading to a `NoHistory` error.
            ServerFetchBlockError::RealmNotFound => DataAccessFetchBlockError::BlockNotFound,
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

    pub async fn get_workspace_manifest_v1(
        &self,
    ) -> Result<Arc<FolderManifest>, DataAccessFetchManifestError> {
        let outcome = server_fetch_versions_workspace_manifest(
            &self.cmds,
            &self.certificates_ops,
            self.realm_id,
            &[1],
        )
        .await;

        let workspace_manifest_v1 = match outcome {
            Ok(manifest) => match manifest.into_iter().next() {
                Some(manifest) => manifest,
                // Manifest has never been uploaded to the server yet
                None => return Err(DataAccessFetchManifestError::EntryNotFound),
            },
            Err(err) => {
                return match err {
                    // The realm doesn't exist on server side, hence there is no history !
                    ServerFetchVersionsManifestError::RealmNotFound => {
                        Err(DataAccessFetchManifestError::EntryNotFound)
                    }

                    // Actual errors
                    ServerFetchVersionsManifestError::Stopped => {
                        Err(DataAccessFetchManifestError::Stopped)
                    }
                    ServerFetchVersionsManifestError::Offline => {
                        Err(DataAccessFetchManifestError::Offline)
                    }
                    ServerFetchVersionsManifestError::NoRealmAccess => {
                        Err(DataAccessFetchManifestError::NoRealmAccess)
                    }
                    ServerFetchVersionsManifestError::InvalidKeysBundle(err) => {
                        Err(DataAccessFetchManifestError::InvalidKeysBundle(err))
                    }
                    ServerFetchVersionsManifestError::InvalidCertificate(err) => {
                        Err(DataAccessFetchManifestError::InvalidCertificate(err))
                    }
                    ServerFetchVersionsManifestError::InvalidManifest(err) => {
                        Err(DataAccessFetchManifestError::InvalidManifest(err))
                    }
                    ServerFetchVersionsManifestError::Internal(err) => Err(err.into()),
                };
            }
        };

        Ok(workspace_manifest_v1.into())
    }
}
