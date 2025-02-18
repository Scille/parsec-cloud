// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::{HashMap, HashSet};

use libparsec_serialization_format::parsec_data;
use serde::{Deserialize, Serialize};

use crate::{
    self as libparsec_types, impl_transparent_data_format_conversion, DataError, DataResult,
    DateTime, DeviceID, EntryName, FolderManifest, PreventSyncPattern, VlobID,
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
        prevent_sync_pattern: &PreventSyncPattern,
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
            HashSet::<_>::from_iter(data.values().filter_map(|v| v.as_ref()))
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
        prevent_sync_pattern: &PreventSyncPattern,
        timestamp: DateTime,
    ) -> Self {
        UnconfinedLocalFolderManifest::remove_confinement(self).apply_confinement(
            self,
            prevent_sync_pattern,
            timestamp,
        )
    }

    pub fn from_remote(remote: FolderManifest, prevent_sync_pattern: &PreventSyncPattern) -> Self {
        UnconfinedLocalFolderManifest::apply_confinement_from_remote(remote, prevent_sync_pattern)
    }

    /// Create a `LocalFolderManifest` from the provided `FolderManifest`, then
    /// apply to the new local folder manifest the entry that were locally confined
    /// in the provided `local_manifest`.
    ///
    /// The result shouldn't be considered a merge of remote and local, but an intermediate
    /// representation required to do a proper merge.
    ///
    /// The produced `LocalFolderManifest` does not have `need_sync` set to true in most cases.
    /// However, it might happen that the remote manifest references a non-confined id that
    /// corresponds to an existing locally confined entry. In this specific case, the entry
    /// keeps its local confined name, which is analogous to the remote entry being deleted.
    /// This is why the `need_sync` field is set to true in this case, with the `updated` field
    /// set to the provided `timestamp`.
    pub fn from_remote_with_restored_local_confinement_points(
        remote: FolderManifest,
        prevent_sync_pattern: &PreventSyncPattern,
        local_manifest: &Self,
        timestamp: DateTime,
    ) -> Self {
        UnconfinedLocalFolderManifest::from_remote(remote).apply_confinement(
            local_manifest,
            prevent_sync_pattern,
            timestamp,
        )
    }

    pub fn to_remote(&self, author: DeviceID, timestamp: DateTime) -> FolderManifest {
        UnconfinedLocalFolderManifest::remove_confinement(self).into_remote(author, timestamp)
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

    pub fn into_remote(self, author: DeviceID, timestamp: DateTime) -> FolderManifest {
        FolderManifest {
            author,
            timestamp,
            id: self.base.id,
            version: self.base.version + 1,
            created: self.base.created,
            parent: self.parent,
            updated: self.updated,
            children: self.children,
        }
    }

    /// Convert a `FolderManifest` to a `LocalFolderManifest`
    /// while applying the prevent sync pattern when there are no existing
    /// local manifest to merge with. Otherwise, use `apply_confinement`.
    pub fn apply_confinement_from_remote(
        remote: FolderManifest,
        prevent_sync_pattern: &PreventSyncPattern,
    ) -> LocalFolderManifest {
        // Filter out the base entries that matches the prevent sync pattern
        let mut new_children = remote.children.clone();
        let remote_confinement_points: HashSet<_> = remote
            .children
            .iter()
            .filter_map(|(name, entry_id)| {
                if prevent_sync_pattern.is_match(name.as_ref()) {
                    new_children.remove(name);
                    Some(*entry_id)
                } else {
                    None
                }
            })
            .collect();

        LocalFolderManifest {
            parent: remote.parent,
            need_sync: false,
            updated: remote.updated,
            children: new_children,
            local_confinement_points: HashSet::new(),
            remote_confinement_points,
            speculative: false,
            base: remote,
        }
    }

    /// Apply the prevent sync pattern to the current unconfined local manifest
    /// and restore the existing locally confined entries from the provided manifest.
    /// This returns a new properly confined local manifest that can be stored to the
    /// local storage.
    ///
    /// The provided timestamp is used to update the `updated` field of the new manifest,
    /// if a change requiring synchronization is made. That might happen if the provided
    /// local manifest used a different prevent sync pattern and that some entries ends up
    /// being longer confined. It might also happen when the provided local manifest
    /// contains a locally confined entry with the same id as a non-confined entry in the
    /// current unconfined manifest. In this case, the entry is kept confined, which is
    /// analogous to the remote entry being deleted.
    pub fn apply_confinement(
        self,
        existing_local_manifest: &LocalFolderManifest,
        prevent_sync_pattern: &PreventSyncPattern,
        timestamp: DateTime,
    ) -> LocalFolderManifest {
        // Filter out the base entries that matches the prevent sync pattern
        let mut new_children = self.children;
        let remote_confinement_points: HashSet<_> = self
            .base
            .children
            .iter()
            .filter_map(|(name, entry_id)| {
                if prevent_sync_pattern.is_match(name.as_ref()) {
                    if new_children.get(name).is_some_and(|x| x == entry_id) {
                        new_children.remove(name);
                    }
                    Some(*entry_id)
                } else {
                    None
                }
            })
            .collect();

        // List the local-only entries that matches the prevent sync pattern
        let local_confinement_points: HashSet<_> = new_children
            .iter()
            .filter_map(|(name, entry_id)| {
                if prevent_sync_pattern.is_match(name.as_ref()) {
                    Some(*entry_id)
                } else {
                    None
                }
            })
            .collect();

        // Create the new manifest
        let mut new_manifest = LocalFolderManifest {
            base: self.base,
            parent: self.parent,
            need_sync: self.need_sync,
            updated: self.updated,
            children: new_children,
            local_confinement_points,
            remote_confinement_points,
            speculative: self.speculative,
        };

        // Check whether there are existing local entries to restore, either because:
        // - 1: They were confined entries, in which case they appear in the local confinement points
        //      of the existing local manifest. Note that may or maybe not be confined in the new
        //      manifest depending on the prevent sync pattern, but we have to restore them anyway.
        //      We should also be careful to not duplicate the entry ID and perform a rename
        //      if the entry ID is already present in the new manifest.
        // - 2: They weren't confined entries, but have been filtered out due to them being present
        //      in the remote confinement points of the new manifest. Note that we don't want to restore
        //      them if they don't match the prevent sync pattern, in order to avoid having a local
        //      non-confined entry with the same id as a remote confined entry.
        if !existing_local_manifest.local_confinement_points.is_empty()
            || !new_manifest.remote_confinement_points.is_empty()
        {
            // Reverse lookup for new entry ids
            let mut new_entry_ids = HashMap::<_, _>::from_iter(
                new_manifest
                    .children
                    .iter()
                    .map(|(name, entry_id)| (entry_id, name)),
            );

            // Build a map of changes to apply
            let mut existing_local_confined_entries: HashMap<_, _> = HashMap::new();
            for (name, entry_id) in existing_local_manifest.children.iter() {
                // Case 1
                if existing_local_manifest
                    .local_confinement_points
                    .contains(entry_id)
                    && new_entry_ids.get(entry_id) != Some(&name)
                {
                    // Perform a rename if the entry id is already present in the new manifest
                    if let Some(&previous_name) = new_entry_ids.get(entry_id) {
                        existing_local_confined_entries.insert(previous_name.clone(), None);
                    }

                    // Insert the locally confined entry in the new manifest
                    existing_local_confined_entries.insert(name.clone(), Some(*entry_id));
                    new_entry_ids.insert(entry_id, name);
                }

                // Case 2
                if !new_entry_ids.contains_key(entry_id)
                    && new_manifest.remote_confinement_points.contains(entry_id)
                    && prevent_sync_pattern.is_match(name.as_ref())
                {
                    existing_local_confined_entries.insert(name.clone(), Some(*entry_id));
                    new_entry_ids.insert(entry_id, name);
                }
            }

            // Restore existing local confinement entries
            new_manifest.evolve_children_and_mark_updated(
                existing_local_confined_entries,
                prevent_sync_pattern,
                timestamp,
            );
        }

        new_manifest
    }

    /// Remove the local context and re-apply the remote context from the given
    /// local manifest in order to create an unconfined local manifest that is
    /// ready to be converted to a remote manifest and uploaded.
    pub fn remove_confinement(local_manifest: &LocalFolderManifest) -> Self {
        // Filter out the entries that are present in the local confinement points

        let mut new_children = local_manifest.children.clone();

        if !local_manifest.local_confinement_points.is_empty() {
            new_children
                .retain(|_, entry_id| !local_manifest.local_confinement_points.contains(entry_id));
        }

        // Restore from base children the entries that are present in the remote confinement points
        if !local_manifest.remote_confinement_points.is_empty() {
            let existing_entries: HashSet<_> = new_children.values().copied().collect();
            for (name, entry_id) in local_manifest.base.children.iter() {
                if local_manifest.remote_confinement_points.contains(entry_id)
                    && !existing_entries.contains(entry_id)
                {
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
