// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::{hash_map::RandomState, HashMap, HashSet};

use libparsec_serialization_format::parsec_data;
use serde::{Deserialize, Serialize};

use crate::{
    self as libparsec_types, impl_transparent_data_format_conversion, DataError, DataResult,
    DateTime, DeviceID, EntryName, FolderManifest, Regex, VlobID,
};

use super::{impl_local_manifest_dump, impl_local_manifest_load};

/// The `LocalFolderManifest` represents a folder in the client.
///
/// Unlike `FolderManifest`, it is designed to be modified as changes
/// occur locally. It is also stored serialized on the local storage.
///
/// It can always be merged with a `FolderManifest` without conflict (CRDT ftw!).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalFolderManifestData", from = "LocalFolderManifestData")]
pub struct LocalFolderManifest {
    pub base: FolderManifest,
    pub parent: VlobID,
    pub need_sync: bool,
    pub updated: DateTime,
    pub children: HashMap<EntryName, VlobID>,
    // Confined entries are entries that are meant to stay locally and not be added
    // to the uploaded remote manifest when synchronizing. The criteria for being
    // confined is to have a filename that matched the "prevent sync" pattern at the time of
    // the last change (or when a new filter was successfully applied)
    pub local_confinement_points: HashSet<VlobID>,
    // Filtered entries are entries present in the base manifest that are not exposed
    // locally. We keep track of them to remember that those entries have not been
    // deleted locally and hence should be restored when crafting the remote manifest
    // to upload.
    pub remote_confinement_points: HashSet<VlobID>,
    // Speculative placeholders are created when we want to access a workspace
    // but didn't retrieve manifest data from server yet. This implies:
    // - only the root folder can be speculative
    // - non-placeholders cannot be speculative
    // - the only non-speculative placeholder is the placeholder initialized
    //   during the initial workspace creation
    // This speculative information is useful during merge to understand if
    // a data is not present in the placeholder compared with a remote because:
    // a) the data is not locally known (speculative is True)
    // b) the data is known, but has been locally removed (speculative is False)
    pub speculative: bool,
}

parsec_data!("schema/local_manifest/local_folder_manifest.json5");

impl_transparent_data_format_conversion!(
    LocalFolderManifest,
    LocalFolderManifestData,
    base,
    parent,
    need_sync,
    updated,
    children,
    local_confinement_points,
    remote_confinement_points,
    speculative,
);

impl_local_manifest_dump!(LocalFolderManifest);
impl_local_manifest_load!(LocalFolderManifest);

impl LocalFolderManifest {
    pub fn new(author: DeviceID, parent: VlobID, timestamp: DateTime) -> Self {
        Self {
            base: FolderManifest {
                author,
                timestamp,
                id: VlobID::default(),
                parent,
                version: 0,
                created: timestamp,
                updated: timestamp,
                children: HashMap::new(),
            },
            parent,
            need_sync: true,
            updated: timestamp,
            children: HashMap::new(),
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
            speculative: false,
        }
    }

    /// A note about this method being public:
    /// This structure represents mutable data (it gets loaded from disk, updated, then stored back modified)
    /// Hence this `check_data_integrity`'s main goal is during deserialization.
    /// However it is also useful as sanity check:
    /// - Right before serialization
    /// - After any modification (hence the need for this method to be public)
    ///
    /// Note that this method does not perform data integrity check related to the manifest being a
    /// child or root manifest.
    pub fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub(super) fn check_data_integrity_as_child(&self) -> DataResult<()> {
        self.check_data_integrity()?;

        // Check that id and parent are different
        if self.base.id == self.parent {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "id and parent are different for child manifest",
            });
        }
        Ok(())
    }

    pub(super) fn check_data_integrity_as_root(&self) -> DataResult<()> {
        self.check_data_integrity()?;

        // Check that id and parent are the same
        if self.base.id != self.parent {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "id and parent are the same for root manifest",
            });
        }
        Ok(())
    }

    /// Root folder manifest (aka "workspace manifest" for historical reasons) is a special
    /// folder manifest which ID the same as the realm ID.
    /// It is the only folder manifest that can be speculative.
    pub fn new_root(
        author: DeviceID,
        realm: VlobID,
        timestamp: DateTime,
        speculative: bool,
    ) -> Self {
        Self {
            base: FolderManifest {
                author,
                timestamp,
                id: realm,
                parent: realm,
                version: 0,
                created: timestamp,
                updated: timestamp,
                children: HashMap::new(),
            },
            parent: realm,
            need_sync: true,
            updated: timestamp,
            children: HashMap::new(),
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
            speculative,
        }
    }

    /// Note the manifest will be marked as updated (i.e. `need_sync` set to `true`
    /// and `updated` field set to `timestamp`) only if the changes concern non
    /// confined entries.
    pub fn evolve_children_and_mark_updated(
        &mut self,
        data: HashMap<EntryName, Option<VlobID>>,
        prevent_sync_pattern: &Regex,
        timestamp: DateTime,
    ) {
        let mut actually_updated = false;
        // Deal with removal first
        for (name, entry_id) in data.iter() {
            // Here `entry_id` can be either:
            // - a new entry id that might overwrite the previous one with the same name if it exists
            // - `None` which means the entry for the corresponding name should be removed
            if !self.children.contains_key(name) {
                // Make sure we don't remove a name that does not exist
                assert!(entry_id.is_some());
                continue;
            }
            // Remove old entry
            if let Some(old_entry_id) = self.children.remove(name) {
                if !self.local_confinement_points.remove(&old_entry_id) {
                    actually_updated = true;
                }
            }
        }
        // Make sure no entry_id is duplicated
        assert_eq!(
            HashSet::<_, RandomState>::from_iter(data.values().filter_map(|v| v.as_ref()))
                .intersection(&HashSet::from_iter(self.children.values()))
                .count(),
            0
        );

        // Deal with additions second
        for (name, entry_id) in data.into_iter() {
            if let Some(entry_id) = entry_id {
                if prevent_sync_pattern.is_match(name.as_ref()) {
                    self.local_confinement_points.insert(entry_id);
                } else {
                    actually_updated = true;
                }
                // Add new entry
                self.children.insert(name, entry_id);
            }
        }

        if !actually_updated {
            return;
        }

        self.need_sync = true;
        self.updated = timestamp;
    }

    pub fn apply_prevent_sync_pattern(
        &self,
        prevent_sync_pattern: &Regex,
        timestamp: DateTime,
    ) -> Self {
        UnconfinedLocalFolderManifest::remove_confinement(self)
            .apply_confinement(Some((self, timestamp)), prevent_sync_pattern)
    }

    pub fn from_remote(remote: FolderManifest, prevent_sync_pattern: &Regex) -> Self {
        UnconfinedLocalFolderManifest::from_remote(remote)
            .apply_confinement(None, prevent_sync_pattern)
    }

    /// Create a `LocalFolderManifest` from the provided `FolderManifest`, then
    /// apply to the new local folder manifest the entry that were locally confined
    /// in the provided `local_manifest`.
    ///
    /// The result shouldn't be considered a merge of remote and local, but an intermediate
    /// representation required to do a proper merge.
    ///
    /// Also note `local_manifest` doesn't need to have it confinement points up-to-date
    /// with the provided `prevent_sync_pattern`. As a matter of fact, the `timestamp`
    /// parameter is only used to update
    pub fn from_remote_with_restored_local_confinement_points(
        remote: FolderManifest,
        prevent_sync_pattern: &Regex,
        local_manifest: &Self,
        timestamp: DateTime,
    ) -> Self {
        UnconfinedLocalFolderManifest::from_remote(remote)
            .apply_confinement(Some((local_manifest, timestamp)), prevent_sync_pattern)
    }

    pub fn to_remote(&self, author: DeviceID, timestamp: DateTime) -> FolderManifest {
        UnconfinedLocalFolderManifest::remove_confinement(self).to_remote(author, timestamp)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnconfinedLocalFolderManifest {
    pub base: FolderManifest,
    pub parent: VlobID,
    pub need_sync: bool,
    pub updated: DateTime,
    pub children: HashMap<EntryName, VlobID>,
    pub speculative: bool,
}

impl UnconfinedLocalFolderManifest {
    pub fn from_remote(remote: FolderManifest) -> Self {
        Self {
            parent: remote.parent,
            need_sync: false,
            updated: remote.updated,
            children: remote.children.clone(),
            speculative: false,
            base: remote,
        }
    }

    pub fn to_remote(&self, author: DeviceID, timestamp: DateTime) -> FolderManifest {
        FolderManifest {
            author,
            timestamp,
            id: self.base.id,
            version: self.base.version + 1,
            created: self.base.created,
            parent: self.parent,
            updated: self.updated,
            children: self.children.clone(),
        }
    }

    pub fn apply_confinement(
        &self,
        existing_local_manifest: Option<(&LocalFolderManifest, DateTime)>,
        prevent_sync_pattern: &Regex,
    ) -> LocalFolderManifest {
        // Previous `filter_remote_entries` method

        let mut new_children = self.children.clone();
        let remote_confinement_points: HashSet<_> = self
            .children
            .iter()
            .filter_map(|(name, entry_id)| {
                if prevent_sync_pattern.is_match(name.as_ref()) {
                    Some(*entry_id)
                } else {
                    None
                }
            })
            .collect();

        if !remote_confinement_points.is_empty() {
            new_children.retain(|_, entry_id| !remote_confinement_points.contains(entry_id));
        }

        // Create the new manifest
        let mut new_manifest = LocalFolderManifest {
            base: self.base.clone(),
            parent: self.parent,
            need_sync: self.need_sync,
            updated: self.updated,
            children: new_children,
            local_confinement_points: HashSet::new(),
            remote_confinement_points,
            speculative: self.speculative,
        };

        // No existing local manifest to extract as local context
        let (other, timestamp) = match existing_local_manifest {
            None => return new_manifest,
            Some(x) => x,
        };

        // Previous `restore_local_confinement_points` method

        // Using `self.remote_confinement_points` is useful to restore entries that were present locally
        // before applying a new filter that filtered those entries from the remote manifest
        if other.local_confinement_points.is_empty()
            && new_manifest.remote_confinement_points.is_empty()
        {
            return new_manifest;
        }
        // Create a set for fast lookup in order to make sure no entry gets duplicated.
        // This might happen when a synchronized entry is renamed to a confined name locally.
        let new_entry_ids = HashSet::<_, RandomState>::from_iter(new_manifest.children.values());
        let previously_local_confinement_points = other
            .children
            .iter()
            .filter_map(|(name, entry_id)| {
                if !new_entry_ids.contains(entry_id)
                    && (other.local_confinement_points.contains(entry_id)
                        || new_manifest.remote_confinement_points.contains(entry_id))
                {
                    Some((name.clone(), Some(*entry_id)))
                } else {
                    None
                }
            })
            .collect();

        new_manifest.evolve_children_and_mark_updated(
            previously_local_confinement_points,
            prevent_sync_pattern,
            timestamp,
        );
        new_manifest
    }

    pub fn remove_confinement(local_manifest: &LocalFolderManifest) -> Self {
        // Previous `filter_local_confinement_points method`

        let mut new_children = local_manifest.children.clone();

        if !local_manifest.local_confinement_points.is_empty() {
            new_children
                .retain(|_, entry_id| !local_manifest.local_confinement_points.contains(entry_id));
        }

        // Previous `restore_remote_confinement_points` method

        if !local_manifest.remote_confinement_points.is_empty() {
            for (name, entry_id) in local_manifest.base.children.iter() {
                if local_manifest.remote_confinement_points.contains(entry_id) {
                    new_children.insert(name.clone(), *entry_id);
                }
            }
        }

        // Create the new manifest
        UnconfinedLocalFolderManifest {
            base: local_manifest.base.clone(),
            parent: local_manifest.parent,
            need_sync: local_manifest.need_sync,
            updated: local_manifest.updated,
            children: new_children,
            speculative: local_manifest.speculative,
        }
    }
}
