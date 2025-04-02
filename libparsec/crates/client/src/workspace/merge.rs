// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use libparsec_types::prelude::*;

const FILENAME_CONFLICT_SUFFIX: &str = "Parsec - name conflict";
const _FILE_CONTENT_CONFLICT_SUFFIX: &str = "Parsec - content conflict";

#[derive(Debug, PartialEq, Eq)]
pub(super) enum MergeLocalFileManifestOutcome {
    NoChange,
    Merged(LocalFileManifest),
    Conflict(FileManifest),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum MergeLocalFolderManifestOutcome {
    NoChange,
    Merged(Arc<LocalFolderManifest>),
}

/// Return `true` if the file manifest has some local changes related to its content.
///
/// File manifest fields can be divided into two parts:
/// - The actual file content (i.e. what is used to read/write the file).
/// - extra fields (currently there is only `update` and `parent` fields)
///
/// The key point here is the extra fields can be merged without conflict, while
/// the file content cannot (as Parsec has no understanding of the file content's
/// internal format !).
fn has_file_content_changed_in_local(
    base_size: u64,
    base_blocksize: Blocksize,
    base_blocks: &[BlockAccess],
    local_size: u64,
    local_blocksize: Blocksize,
    local_blocks: &[Vec<ChunkView>],
) -> bool {
    // Test for obvious changes first
    if base_size != local_size || base_blocksize != local_blocksize {
        return true;
    }

    // Now the tricky part: compare the actual content of the file through the
    // the blocks and chunks.
    //
    // Remember the local and remote manifests represent the content in different ways:
    // - In remote each blocksize area is represented by a block access.
    // - In local each blocksize area is represented by a list of chunk view.
    // - A chunk view in turn may refer to a block access (or not if it correspond to new data).
    // - To have an actual correspondence, a given blocksize area must be represented in
    //   local as a single chunk view that refers to a block access and use it entirely
    //   (not referring to a subset of the block).
    // - A remote manifest can omit some blocksize area if they only contains empty data, on
    //   the contrary a local manifest will represent such area as an empty list of chunk view.
    //
    // For instance considering a blocksize of 1024 bytes and a file of 3072 bytes
    // containing only zero-filled data between bytes 1024 and 2048:
    //
    //          0                1024               2048               3072
    // remote   |<----block #1---->|                  |<----block #2---->|
    // local    |<-non-empty area->|<---empty area--->|<-non-empty area->|
    //
    // Here the two non-empty areas in local are expected to each contains a single
    // chunk view corresponding to the same block than in remote.

    // Compare each blocksize area one by one.
    let mut remote_blocks_iter = base_blocks.iter();
    for local_blocksize_area in local_blocks.iter() {
        match local_blocksize_area.len() {
            // This blocksize area is empty (i.e. it contains only zero-filled data), the
            // remote manifest should have a hole here (hence nothing to compare here).
            0 => (),
            // This blocksize area contains data to compare.
            1 => {
                // The blocksize area is made of a single chunk view in local, from there
                // we can extract the corresponding block access and compare it to the
                // remote one.
                // Note that it's possible to have a chunk view referring a block access
                // but only using part of it (think of truncating a synchronized file),
                // in this case `ChunkView::get_block_access` works as expected and
                // return an error (given the chunk view doesn't *correspond* to the block
                // access but only *uses* it).
                match (local_blocksize_area[0].get_block_access(), remote_blocks_iter.next()) {
                    // The local manifest contains local changes in this blocksize area.
                    (Err(ChunkViewGetBlockAccessError::NotPromotedAsBlock), _)
                    // The local manifest contains more blocksize areas than the remote.
                    | (_, None) => {
                        return true;
                    }
                    (Ok(local_block_access), Some(remote_block_access)) => {
                        // This blocksize area has been modified.
                        if local_block_access != remote_block_access {
                            return true;
                        }
                    }
                }
            }
            // Multiple chunk views in a given blocksize area indicates there is local
            // changes (i.e. reshape hasn't been done).
            _ => {
                return true;
            }
        }
    }

    // No changes \o/
    false
}

/// Merge a local file manifest with a remote file manifest.
pub(super) fn merge_local_file_manifest(
    local_author: DeviceID,
    timestamp: DateTime,
    local: &LocalFileManifest,
    remote: FileManifest,
) -> MergeLocalFileManifestOutcome {
    // 0) Sanity checks, caller is responsible to handle them properly !
    debug_assert_eq!(local.base.id, remote.id);

    // 1) Shortcut in case the remote is outdated
    if remote.version <= local.base.version {
        return MergeLocalFileManifestOutcome::NoChange;
    }

    // 2) Shortcut in case only the remote has changed
    if !local.need_sync {
        return MergeLocalFileManifestOutcome::Merged(LocalFileManifest::from_remote(remote));
    }

    // Both the remote and the local have changed

    // 3) The remote changes are ours (our current local changes occurs while
    // we were uploading previous local changes that became the remote changes),
    // simply acknowledge the remote changes and keep our local changes
    if remote.author == local_author {
        let mut new_local = local.to_owned();
        new_local.base = remote;
        return MergeLocalFileManifestOutcome::Merged(new_local);
    }

    // 4) Merge data and ensure the sync is still needed

    // Destruct local and remote manifests to ensure this code with fail to compile
    // whenever a new field is introduced.
    let LocalFileManifest {
        base:
            FileManifest {
                // `id` has already been checked
                id: _,
                // Ignore `author`: we don't merge data that change on each sync
                author: _,
                // Ignore `timestamp`: we don't merge data that change on each sync
                timestamp: _,
                // Ignore `version`: we don't merge data that change on each sync
                version: _,
                // Ignore `updated`: we don't merge data that change on each sync
                updated: _,
                // `created` should never change, so in theory we should have
                // `local.base.created == remote.base.created`, but there is no strict
                // guarantee (e.g. remote manifest may have been uploaded by a buggy client)
                // so we have no choice but to accept whatever value remote provides.
                created: _,
                parent: local_base_parent,
                size: local_base_size,
                blocksize: local_base_blocksize,
                blocks: local_base_blocks,
            },
        // `need_sync` has already been checked
        need_sync: _,
        // Ignore `updated`: we don't merge data that change on each sync
        updated: _,
        parent: local_parent,
        size: local_size,
        blocksize: local_blocksize,
        blocks: local_blocks,
    } = local;

    let mut local_need_sync = false;

    // 4.1) First focus on the file content (i.e. the actual data of the file) given
    // we cannot merge them in case of conflict.

    let local_content_changed = has_file_content_changed_in_local(
        *local_base_size,
        *local_base_blocksize,
        local_base_blocks,
        *local_size,
        *local_blocksize,
        local_blocks,
    );
    // Once this first step is done, we have only partially done the merge (see
    // next step) and hence why the result is named `merge_in_progress` !
    let mut merge_in_progress = if local_content_changed {
        local_need_sync = true;
        // There is local changes in the content, hence we will have a conflict if there is also remote changes !
        let remote_content_changed = remote.size != *local_base_size
            || remote.blocksize != *local_base_blocksize
            || remote.blocks != *local_base_blocks;
        if remote_content_changed {
            return MergeLocalFileManifestOutcome::Conflict(remote);
        }

        // No conflict, we can keep the local changes
        let mut merge_in_progress = LocalFileManifest::from_remote(remote);
        merge_in_progress.size = *local_size;
        merge_in_progress.blocksize = *local_blocksize;
        local_blocks.clone_into(&mut merge_in_progress.blocks);

        merge_in_progress
    } else {
        // No local changes, so we just apply whatever the remote has done.
        LocalFileManifest::from_remote(remote)
    };
    let remote = &merge_in_progress.base;

    // 4.2) Now we can deal with the extra fields (i.e. not the file actual content) that
    // can be merged without conflict (currently this is only the `parent` field).

    merge_in_progress.parent = merge_parent(*local_base_parent, *local_parent, remote.parent);
    if merge_in_progress.parent != remote.parent {
        local_need_sync = true;
    }

    // 4.3) Finally restore the need sync flag if needed

    if local_need_sync {
        merge_in_progress.updated = timestamp;
        merge_in_progress.need_sync = true;
    }

    MergeLocalFileManifestOutcome::Merged(merge_in_progress)
}

/// Merge a local folder manifest with a remote folder manifest.
/// The local manifest is assumed to be up-to-date with the current prevent sync pattern.
pub(super) fn merge_local_folder_manifest(
    local_author: DeviceID,
    timestamp: DateTime,
    prevent_sync_pattern: &PreventSyncPattern,
    local: &LocalFolderManifest,
    remote: FolderManifest,
) -> MergeLocalFolderManifestOutcome {
    // Destruct local and remote manifests to ensure this code with fail to compile whenever a new field is introduced.
    let LocalFolderManifest {
        base:
            FolderManifest {
                id: _,
                version: _,
                children: _,
                parent: _,
                // Ignored, we don't merge data that change on each sync
                author: _,
                // `created` should never change, so in theory we should have
                // `local.base.created == remote.base.created`, but there is no strict
                // guarantee (e.g. remote manifest may have been uploaded by a buggy client)
                // so we have no choice but to accept whatever value remote provides.
                created: _,
                // Ignored, we don't merge data that change on each sync
                timestamp: _,
                // Ignored, we don't merge data that change on each sync
                updated: _,
            },
        children: _,
        parent: _,
        need_sync: _,
        speculative: local_speculative,
        // Ignored, that field is merged in `from_remote_with_local_context`
        remote_confinement_points: _,
        // Ignored, that field is merged in `from_remote_with_local_context`
        local_confinement_points: _,
        // Ignored, we don't merge data that change on each sync
        updated: _,
    } = local;

    // 0) Sanity checks, caller is responsible to handle them properly !
    debug_assert_eq!(local.base.id, remote.id);

    // 1) Shortcut in case the remote is outdated
    if remote.version <= local.base.version {
        return MergeLocalFolderManifestOutcome::NoChange;
    }

    // 2) Confinement points is a special case: given any entry that match the prevent
    // sync pattern is kept apart, there is no possibility of conflict here (i.e.
    // confined local entries stay in `local.children`, confined remote entries stay in
    // `local.base.children`).
    //
    // So if the current local manifest only changes are confined entries, then the merge
    // operation only consist of converting the remote into a local and adding those
    // confined entries back.
    // This is precisely what we are doing in this step, and why the resulting manifest
    // is called `merge_in_progress`: if the local manifest contains other changes, then
    // we must do a proper children merge and update the manifest accordingly.

    let mut unconfined_local = UnconfinedLocalFolderManifest::remove_confinement(local);
    let mut unconfined_remote = UnconfinedLocalFolderManifest::from_remote(remote);

    // 3) Shortcut in case only the remote has changed
    if !unconfined_local.need_sync {
        let new_manifest =
            unconfined_remote.apply_confinement(local, prevent_sync_pattern, timestamp);
        return MergeLocalFolderManifestOutcome::Merged(Arc::new(new_manifest));
    }

    // Both the remote and the local have changed

    // The following case was present in the original v2 code.
    // ```python
    // # All the local changes have been successfully uploaded
    // if local_manifest.match_remote(remote_manifest):
    //     return local_from_remote
    // ```
    // TODO: Add something similar to avoid returning a `need_sync` manifest
    // when the remote manifest already matches the local one. In practice,
    // this could happen if the connection is lost after the local changes
    // have been uploaded but before the request returns, meaning that the
    // local manifest could not be updated to acknowledge the upload.
    // See https://github.com/Scille/parsec-cloud/issues/8750

    // 4) The remote changes are ours (our current local changes occurs while
    // we were uploading previous local changes that became the remote changes),
    // simply acknowledge the remote changes and keep our local changes
    //
    // However speculative manifest can lead to a funny behavior:
    // 1) alice has access to the workspace
    // 2) alice upload a new remote workspace manifest
    // 3) alice gets it local storage removed
    // So next time alice tries to access this workspace she will
    // creates a speculative workspace manifest.
    // This speculative manifest will eventually be synced against
    // the previous remote manifest which appears to be remote
    // changes we know about (given we are the author of it !).
    // If the speculative flag is not taken into account, we would
    // consider we have willingly removed all entries from the remote,
    // hence uploading a new expunged remote manifest.
    //
    // Of course removing local storage is an unlikely situation, but:
    // - it cannot be ruled out and would produce rare&exotic behavior
    //   that would be considered as bug :/
    // - the fixtures and server data binder system used in the tests
    //   makes it much more likely
    if unconfined_remote.base.author == local_author && !local_speculative {
        // We should end up with a new local manifest that is strictly equivalent
        // to the previous one, except that:
        // - it is now based on the new remote
        // - confinement may have changed if the prevent sync pattern has changed
        unconfined_local.base = unconfined_remote.base;
        let new_manifest =
            unconfined_local.apply_confinement(local, prevent_sync_pattern, timestamp);
        return MergeLocalFolderManifestOutcome::Merged(Arc::new(new_manifest));
    }

    // 5) Merge data and ensure the sync is still needed

    // Solve the folder conflict
    let merged_children = merge_children(
        &unconfined_local.base.children,
        &unconfined_local.children,
        &unconfined_remote.children,
    );
    let merged_parent = merge_parent(
        unconfined_local.base.parent,
        unconfined_local.parent,
        unconfined_remote.parent,
    );

    // Children merge can end up with nothing to sync.
    //
    // This is typically the case when we sync for the first time a workspace
    // shared with us that we didn't modify:
    // - the workspace manifest is a speculative placeholder (with arbitrary update&create dates)
    // - on sync the update date is different than in the remote, so a merge occurs
    // - given we didn't modify the workspace, the children merge is trivial
    // So without this check each each user we share the workspace with would
    // sync a new workspace manifest version with only it updated date changing :/
    //
    // Another case where this happen:
    // - we have local change on our workspace manifest for removing an entry
    // - we rely on a base workspace manifest in version N
    // - remote workspace manifest is in version N+1 and already integrate the removal
    //
    // /!\ Extra attention should be paid here if we want to add new fields
    // /!\ with their own sync logic, as this optimization may shadow them!
    let merged_need_sync =
        merged_children != unconfined_remote.children || merged_parent != unconfined_remote.parent;
    let merged_updated = if merged_need_sync {
        timestamp
    } else {
        unconfined_remote.updated
    };

    // Use merged data to mutate the unconfined remote
    // and re-apply confinement in order to get the final manifest
    unconfined_remote.children = merged_children;
    unconfined_remote.parent = merged_parent;
    unconfined_remote.need_sync = merged_need_sync;
    unconfined_remote.updated = merged_updated;
    let new_manifest = unconfined_remote.apply_confinement(local, prevent_sync_pattern, timestamp);
    MergeLocalFolderManifestOutcome::Merged(Arc::new(new_manifest))
}

fn merge_parent(base: VlobID, local: VlobID, remote: VlobID) -> VlobID {
    let remote_change = remote != base;
    let local_change = local != base;
    match (local_change, remote_change) {
        // No change
        (false, false) => base,
        // Local change
        (true, false) => local,
        // Remote change
        (false, true) => remote,
        // Both remote and local change, conflict !
        //
        // In this case, we simply decide that the remote is right since it means
        // another user managed to upload their change first. Tough luck for the
        // local device!
        //
        // Note the changing the parent is most likely part of a reparenting operation,
        // hence dropping the local change means the destination parent folder will
        // contain an invalid entry (i.e. an entry pointing to our manifest, which
        // itself points to another parent). This is considered okay given parent
        // merge conflict is considered as a rare event and the entry will simply
        // be ignored for all practical purpose.
        (true, true) => remote,
    }
}

fn merge_children(
    base: &HashMap<EntryName, VlobID>,
    local: &HashMap<EntryName, VlobID>,
    remote: &HashMap<EntryName, VlobID>,
) -> HashMap<EntryName, VlobID> {
    // Prepare lookups
    let base_reversed: HashMap<_, _> = base.iter().map(|(k, v)| (*v, k)).collect();
    let local_reversed: HashMap<_, _> = local.iter().map(|(k, v)| (*v, k)).collect();
    let remote_reversed: HashMap<_, _> = remote.iter().map(|(k, v)| (*v, k)).collect();

    // All ids that might remain
    let ids = {
        let mut ids: HashSet<VlobID> = HashSet::new();
        ids.extend(local_reversed.keys());
        ids.extend(remote_reversed.keys());
        ids
    };

    // First map all ids to their rightful name
    let mut solved_local_children: HashMap<EntryName, VlobID> = HashMap::new();
    let mut solved_remote_children: HashMap<EntryName, VlobID> = HashMap::new();
    for id in ids {
        let base_name = base_reversed.get(&id).cloned();
        let local_name = local_reversed.get(&id).cloned();
        let remote_name = remote_reversed.get(&id).cloned();
        match (base_name, local_name, remote_name) {
            // a) Local and remote agree on this entry

            // Removed locally and remotely
            (_, None, None) => {}
            // Preserved remotely and locally with the same naming
            (_, Some(local_name), Some(remote_name)) if local_name == remote_name => {
                solved_remote_children.insert(remote_name.to_owned(), id);
            }

            // b) Conflict between local and remote on this entry

            // Added locally
            (None, Some(local_name), None) => {
                solved_local_children.insert(local_name.to_owned(), id);
            }
            // Added remotely
            (None, None, Some(remote_name)) => {
                solved_remote_children.insert(remote_name.to_owned(), id);
            }

            // Removed locally but renamed remotely
            (Some(base_name), None, Some(remote_name)) if base_name != remote_name => {
                solved_remote_children.insert(remote_name.to_owned(), id);
            }
            // Removed remotely but renamed locally
            (Some(base_name), Some(local_name), None) if base_name != local_name => {
                solved_remote_children.insert(local_name.to_owned(), id);
            }
            // Removed locally
            (Some(_), None, _) => {
                // Note that locally removed children might not be synchronized at this point
            }
            // Removed remotely
            (Some(_), _, None) => {
                // Note that we're blindly removing children just because the remote said so
                // This is OK as long as users have a way to recover their local changes
            }

            // Renamed locally
            (Some(base_name), Some(local_name), Some(remote_name)) if base_name == remote_name => {
                solved_local_children.insert(local_name.to_owned(), id);
            }
            // Renamed remotely
            (Some(base_name), Some(local_name), Some(remote_name)) if base_name == local_name => {
                solved_remote_children.insert(remote_name.to_owned(), id);
            }
            // Renamed both locally and remotely
            (_, Some(_), Some(remote_name)) => {
                // In this case, we simply decide that the remote is right since it means
                // another user managed to upload their change first. Tough luck for the
                // local device!
                solved_remote_children.insert(remote_name.to_owned(), id);
            }
        }
    }

    // Merge mappings and fix conflicting names
    let mut children = solved_remote_children;
    for (entry_name, entry_id) in solved_local_children {
        let entry_name = if children.contains_key(&entry_name) {
            get_conflict_filename(&entry_name, FILENAME_CONFLICT_SUFFIX, |new_entry_name| {
                children.contains_key(new_entry_name)
            })
        } else {
            entry_name
        };
        children.insert(entry_name, entry_id);
    }

    children
}

fn get_conflict_filename(
    filename: &EntryName,
    suffix: &str,
    is_reserved: impl Fn(&EntryName) -> bool,
) -> EntryName {
    let mut count = 1;
    let mut new_filename = rename_with_suffix(filename, suffix);
    while is_reserved(&new_filename) {
        count += 1;
        let suffix_with_count = format!("{} {}", suffix, count);
        new_filename = rename_with_suffix(filename, &suffix_with_count);
    }
    new_filename
}

fn rename_with_suffix(name: &EntryName, suffix: &str) -> EntryName {
    // Leading dot (e.g. `.bashrc`) is a special case given it is not considered
    // as an extension separator.
    // We handle it by stripping it for our algorithm and only re-adding it
    // right before constructing the candidate name.
    let (raw, has_leading_dot) = if name.as_ref().starts_with('.') {
        (&name.as_ref()[1..], true)
    } else {
        (name.as_ref(), false)
    };
    // Separate file name from the extensions (if any)
    let (original_base_name, original_extension) = match raw.split_once('.') {
        None => (raw, None),
        Some((base, ext)) => (base, Some(ext)),
    };
    // Loop over attempts, in case the produced entry name is too long
    let mut base_name = original_base_name;
    let mut extension = original_extension;
    loop {
        // Convert to EntryName
        let raw = match (extension, has_leading_dot) {
            (Some(extension), false) => format!("{} ({}).{}", base_name, suffix, extension),
            (None, false) => format!("{} ({})", base_name, suffix),
            // Leading dot cases
            (Some(extension), true) => format!(".{} ({}).{}", base_name, suffix, extension),
            (None, true) => format!(".{} ({})", base_name, suffix),
        };
        match raw.parse::<EntryName>() {
            Ok(name) => return name,
            Err(EntryNameError::NameTooLong) => {
                if base_name.len() > 10 {
                    // Simply strip 10 characters from the first name then try again
                    base_name = base_name
                        .get(..(base_name.len() - 10))
                        .expect("already checked");
                } else {
                    // Very rare case where the extension part is very long
                    // Pop the left most extension and restore the original base name
                    base_name = original_base_name;
                    extension = match extension.expect("must contain extension").split_once('.') {
                        Some((_, kept)) => Some(kept),
                        None => {
                            // Really ??? What is your use case to have an extension part
                            // composed of a single extension longer than 200bytes o_O'
                            None
                        }
                    };
                }
            }
            Err(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
#[path = "../../tests/unit/workspace/merge_get_conflict_filename.rs"]
#[allow(clippy::unwrap_used)]
mod tests_get_conflict_filename;
#[cfg(test)]
#[path = "../../tests/unit/workspace/merge_has_file_content_changed_in_local.rs"]
#[allow(clippy::unwrap_used)]
mod tests_has_file_content_changed_in_local;
