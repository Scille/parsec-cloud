// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::{HashMap, HashSet};

use libparsec_types::prelude::*;

const FILENAME_CONFLICT_SUFFIX: &str = "Parsec - name conflict";
const _FILE_CONTENT_CONFLICT_SUFFIX: &str = "Parsec - content conflict";

pub(super) fn merge_local_workspace_manifests(
    local_author: &DeviceID,
    timestamp: DateTime,
    local: &LocalWorkspaceManifest,
    remote: WorkspaceManifest,
) -> Option<LocalWorkspaceManifest> {
    // Sanity checks, caller is responsible to handle them properly !
    assert_eq!(local.base.id, remote.id);

    // `created` should never change, so in theory we should have
    // `local.base.created == remote.base.created`, but there is no strict
    // guarantee (e.g. remote manifest may have been uploaded by a buggy client)
    // so we have no choice but to accept whatever value remote provides.

    // TODO
    // Start by re-applying pattern (idempotent)
    // if force_apply_pattern and isinstance(
    //     local_manifest, (LocalFolderManifest, LocalWorkspaceManifest)
    // ):
    //     local_manifest = local_manifest.apply_prevent_sync_pattern(prevent_sync_pattern, timestamp)

    // The remote hasn't changed
    if remote.version <= local.base.version {
        return None;
    }

    if !local.need_sync {
        // Only the remote has changed
        return Some(LocalWorkspaceManifest::from_remote(remote, None));
    }

    // Both the remote and the local have changed

    // All the local changes have been successfully uploaded
    if local.match_remote(&remote) {
        return Some(LocalWorkspaceManifest::from_remote(remote, None));
    }

    // The remote changes are ours (our current local changes occurs while
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
    // the previous remote remote manifest which appears to be remote
    // changes we know about (given we are the author of it !).
    // If the speculative flag is not taken into account, we would
    // consider we have  willingly removed all entries from the remote,
    // hence uploading a new expunged remote manifest.
    //
    // Of course removing local storage is an unlikely situation, but:
    // - it cannot be ruled out and would produce rare&exotic behavior
    //   that would be considered as bug :/
    // - the fixtures and backend data binder system used in the tests
    //   makes it much more likely
    if remote.author == *local_author && !local.speculative {
        let mut new_local = local.to_owned();
        new_local.base = remote;
        return Some(new_local);
    }

    // The remote has been updated by some other device

    // Solve the folder conflict
    let new_children = merge_children(&local.base.children, &local.children, &remote.children);

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

    if new_children == remote.children {
        Some(LocalWorkspaceManifest::from_remote(remote, None))
    } else {
        let mut local_from_remote = LocalWorkspaceManifest::from_remote(remote, None);
        local_from_remote.children = new_children;
        local_from_remote.updated = timestamp;
        Some(local_from_remote)
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
            // Removed locally
            (Some(_), None, _) => {
                // Note that locally removed children might not be synchronized at this point
            }
            // Removed remotely
            (Some(_), _, None) => {
                // Note that we're blindly removing children just because the remote said so
                // This is OK as long as users have a way to recover their local changes
            }
            // Name changed locally
            (Some(base_name), Some(local_name), Some(remote_name)) if base_name == remote_name => {
                solved_local_children.insert(local_name.to_owned(), id);
            }
            // Name changed remotely
            (Some(base_name), Some(local_name), Some(remote_name)) if base_name == local_name => {
                solved_remote_children.insert(remote_name.to_owned(), id);
            }
            // Name changed both locally and remotely
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
        let suffix_with_count = format!("{} ({})", suffix, count);
        new_filename = rename_with_suffix(filename, &suffix_with_count);
    }
    new_filename
}

fn rename_with_suffix(name: &EntryName, suffix: &str) -> EntryName {
    // Separate file name from the extensions (if any)
    let raw = name.as_ref();
    let (original_base_name, original_extension) = match raw.split_once('.') {
        None => (raw, None),
        Some((base, ext)) => (base, Some(ext)),
    };
    // Loop over attempts, in case the produced entry name is too long
    let mut base_name = original_base_name;
    let mut extension = original_extension;
    loop {
        // Convert to EntryName
        let raw = if let Some(extension) = extension {
            format!("{} ({}).{}", base_name, suffix, extension)
        } else {
            format!("{} ({})", base_name, suffix)
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
