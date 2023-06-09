// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[derive(Debug, thiserror::Error)]
pub enum FetchRemoteUserManifestError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Cannot access workspace's data while it is in maintenance")]
    InMaintenance,
    #[error("Server has no such version for this user manifest")]
    BadVersion,
    // #[error("A certificate provided by the server is invalid: {0}")]
    // InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for FetchRemoteUserManifestError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

impl super::UserOps {
    // pub async fn sync(&self) -> Result<(), DynError> {
    //     let user_manifest = self.storage.get_user_manifest();
    //     if user_manifest.need_sync {
    //         self._outbound_sync().await
    //     } else {
    //         self._inbound_sync().await
    //     }
    // }

    // async fn _outbound_sync(&self) -> Result<(), DynError> {
    //     todo!()
    // }

    // async fn _inbound_sync(&self) -> Result<(), DynError> {
    //     // Retrieve remote
    //     let _target_um = self._fetch_remote_user_manifest(None).await;
    //     // diverged_um = self.get_user_manifest()
    //     // if target_um.version == diverged_um.base_version:
    //     //     # Nothing new
    //     //     return

    //     // # New things in remote, merge is needed
    //     // async with self._update_user_manifest_lock:
    //     //     diverged_um = self.get_user_manifest()
    //     //     if target_um.version <= diverged_um.base_version:
    //     //         # Sync already achieved by a concurrent operation
    //     //         return
    //     //     merged_um = merge_local_user_manifests(diverged_um, target_um)
    //     //     await self.set_user_manifest(merged_um)
    //     //     # In case we weren't online when the sharing message arrived,
    //     //     # we will learn about the change in the sharing only now.
    //     //     # Hence send the corresponding events !
    //     //     self._detect_and_send_shared_events(diverged_um, merged_um)
    //     //     # TODO: deprecated event ?
    //     //     self.event_bus.send(
    //     //         CoreEvent.FS_ENTRY_REMOTE_CHANGED, path="/", id=self.user_manifest_id
    //     //     )
    //     //     return
    //     todo!()
    // }

    // async fn _fetch_remote_user_manifest(
    //     &self,
    //     version: Option<VersionInt>,
    // ) -> Result<UserManifest, FetchRemoteUserManifestError> {
    //     use authenticated_cmds::latest::vlob_read;

    //     let req = vlob_read::Req {
    //         // `encryption_revision` is always 1 given we never re-encrypt the user manifest's realm
    //         encryption_revision: 1,
    //         timestamp: None,
    //         version: version.clone(),
    //         vlob_id: VlobID::from(self.device.user_manifest_id.as_ref().to_owned()),
    //     };
    //     // TODO: handle errors !
    //     let rep = self.cmds.send(req).await?;
    //     match rep {
    //         vlob_read::Rep::Ok { author: expected_author, timestamp: expected_timestamp, version: version_according_to_server, author_last_role_granted_on, blob } => {
    //             let expected_version = match version {
    //                 Some(version) => version,
    //                 None => version_according_to_server,
    //             };
    //             let manifest = self.certificates_ops.decrypt_verify_and_load_user_manifest(
    //                 blob,
    //                 self.device.user_manifest_key,
    //                 &self.device.user_manifest_id,
    //                 expected_author,
    //                 expected_timestamp,
    //                 expected_version,
    //             )?;
    //             Ok(manifest)
    //         },
    //         // Expected errors
    //         vlob_read::Rep::InMaintenance => {
    //             return Err(FetchRemoteUserManifestError::InMaintenance);
    //         }
    //         rep @ vlob_read::Rep::BadVersion if version.is_some() => {
    //             return Err(FetchRemoteUserManifestError::BadVersion);
    //         }
    //         // Unexpected errors :(
    //         rep @ (
    //             // We didn't specified a `version` argument in the request
    //             vlob_read::Rep::BadVersion |
    //             // User never lose access to it user manifest's workspace
    //             vlob_read::Rep::NotAllowed |
    //             // User manifest's workspace never get reencrypted !
    //             vlob_read::Rep::BadEncryptionRevision |
    //             // User manifest's vlob is supposed to exists !
    //             vlob_read::Rep::NotFound { .. }
    //         ) => {
    //             return Err(anyhow::anyhow!("Unexpected response `{:?}` from server", rep).into());
    //         }
    //         vlob_read::Rep::UnknownStatus { unknown_status, .. } => {
    //             return Err(anyhow::anyhow!("Unknown error status `{}` from server", unknown_status).into());
    //         }
    //     }
    // }
}
