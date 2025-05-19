// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_serialization_format::parsec_data;
use serde::{Deserialize, Serialize};

use crate::{
    self as libparsec_types, DataResult, DateTime, DeviceID, UserManifest, VlobID,
    impl_transparent_data_format_conversion,
};

use super::{impl_local_manifest_dump, impl_local_manifest_load};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalUserManifestData", from = "LocalUserManifestData")]
pub struct LocalUserManifest {
    pub base: UserManifest,
    pub need_sync: bool,
    pub updated: DateTime,
    /// This field is used to store the name of the realm:
    /// - When the realm got created, its name is stored here until the initial
    ///   `RealmNameCertificate` is uploaded (which can take time, e.g. if the
    ///   client is offline).
    /// - After that, to access the workspace name even when the client is offline (given
    ///   `RealmNameCertificate` contains the name encrypted, but the decryption key
    ///   must be fetched by `realm_get_keys_bundle` (which cannot be done while offline).
    pub local_workspaces: Vec<LocalUserManifestWorkspaceEntry>,
    /// Speculative placeholders are created when we want to access the
    /// user manifest but didn't retrieve it from server yet. This implies:
    /// - non-placeholders cannot be speculative
    /// - the only non-speculative placeholder is the placeholder initialized
    ///   during the initial user claim (by opposition of subsequent device
    ///   claims on the same user)
    ///
    /// This speculative information is useful during merge to understand if
    /// a data is not present in the placeholder compared with a remote because:
    /// a) the data is not locally known (speculative is True)
    /// b) the data is known, but has been locally removed (speculative is False)
    pub speculative: bool,
}

impl_local_manifest_dump!(LocalUserManifest);
impl_local_manifest_load!(LocalUserManifest);

parsec_data!("schema/local_manifest/local_user_manifest.json5");

impl_transparent_data_format_conversion!(
    LocalUserManifest,
    LocalUserManifestData,
    base,
    need_sync,
    updated,
    local_workspaces,
    speculative,
);

impl LocalUserManifest {
    pub fn new(
        author: DeviceID,
        timestamp: DateTime,
        id: Option<VlobID>,
        speculative: bool,
    ) -> Self {
        Self {
            base: UserManifest {
                author,
                timestamp,
                id: id.unwrap_or_default(),
                version: 0,
                created: timestamp,
                updated: timestamp,
            },
            need_sync: true,
            updated: timestamp,
            local_workspaces: vec![],
            speculative,
        }
    }

    /// This structure represents mutable data (it gets loaded from disk, updated, then stored back modified)
    /// Hence this `check_data_integrity`'s main goal is during deserialization.
    /// However it is also useful as sanity check:
    /// - Right before serialization
    /// - After any modification (hence the need for this method to be public)
    pub fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn get_local_workspace_entry(
        &self,
        realm_id: VlobID,
    ) -> Option<&LocalUserManifestWorkspaceEntry> {
        self.local_workspaces.iter().find(|w| w.id == realm_id)
    }

    pub fn from_remote(remote: UserManifest) -> Self {
        Self {
            need_sync: false,
            updated: remote.updated,
            local_workspaces: vec![],
            speculative: false,
            base: remote,
        }
    }

    pub fn to_remote(&self, author: DeviceID, timestamp: DateTime) -> UserManifest {
        UserManifest {
            author,
            timestamp,
            id: self.base.id,
            version: self.base.version + 1,
            created: self.base.created,
            updated: self.updated,
        }
    }
}
