// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

pub(super) fn merge_workspace_entry(
    base: Option<&WorkspaceEntry>,
    diverged: &WorkspaceEntry,
    target: &WorkspaceEntry,
) -> WorkspaceEntry {
    // Sanity checks, must have been enforced by `merge_workspace_entries`
    assert_eq!(diverged.id, target.id);
    if let Some(base) = base {
        assert_eq!(base.id, target.id);
    }

    // If the name has been modified on both sides, target always wins
    let name = match base {
        Some(base) if base.name != target.name => target.name.to_owned(),
        Some(base) if base.name != diverged.name => diverged.name.to_owned(),
        _ => target.name.to_owned(),
    };

    // Keep last encryption
    let (encryption_revision, encrypted_on, key) =
        if diverged.encryption_revision <= target.encryption_revision {
            // Note `diverged.encryption_revision == target.encryption_revision`
            // should imply `diverged.encrypted_on == target.encrypted_on`, but
            // there is no way to enforce this (e.g. a buggy client may have change
            // this value...). However we'd better keep the remote value to avoid
            // constant sync fight between two client if they end-up with a different
            // value they both consider the "right" one.
            (
                target.encryption_revision,
                target.encrypted_on,
                target.key.to_owned(),
            )
        } else {
            (
                diverged.encryption_revision,
                diverged.encrypted_on,
                diverged.key.to_owned(),
            )
        };

    // Keep most recent cache info on role (legacy stuff only for backward compatibility)
    let (legacy_role_cache_value, legacy_role_cache_timestamp) =
        if target.legacy_role_cache_value == diverged.legacy_role_cache_value {
            let role_cached_on = std::cmp::max(
                target.legacy_role_cache_timestamp,
                diverged.legacy_role_cache_timestamp,
            );
            (target.legacy_role_cache_value, role_cached_on)
        } else if target.legacy_role_cache_timestamp > diverged.legacy_role_cache_timestamp {
            (
                target.legacy_role_cache_value,
                target.legacy_role_cache_timestamp,
            )
        } else {
            (
                diverged.legacy_role_cache_value,
                diverged.legacy_role_cache_timestamp,
            )
        };

    WorkspaceEntry {
        name,
        id: target.id,
        key,
        encryption_revision,
        encrypted_on,
        legacy_role_cache_timestamp,
        legacy_role_cache_value,
    }
}

fn merge_workspace_entries(
    base: &[WorkspaceEntry],
    diverged: &[WorkspaceEntry],
    target: &[WorkspaceEntry],
) -> (Vec<WorkspaceEntry>, bool) {
    // Merging workspace entries is really trivial given:
    // - Workspaces are not required to have distinct names
    // - Workspaces entries are never removed

    // The following code have a quadratic complexity given we don't use hashmap,
    // however this is fine (and most likely faster !) given the number of entries
    // is expected to be very small (typically < 10)

    let mut need_sync = false;
    let mut resolved = target.to_owned();
    for diverged_entry in diverged {
        let item = resolved.iter_mut().find(|x| x.id == diverged_entry.id);
        match item {
            // Diverged have added this entry alone, no conflict then
            None => {
                resolved.push(diverged_entry.to_owned());
                need_sync = true;
            }
            // Target and diverged agree on the entry, nothing more to do
            Some(target_entry) if target_entry == diverged_entry => (),
            // Target and diverged have both modified this entry
            Some(target_entry) => {
                let base_entry = base.iter().find(|x| x.id == target_entry.id);
                // `target_entry` is stored in `resolved`, so do the resolution in place !
                *target_entry = merge_workspace_entry(base_entry, diverged_entry, target_entry);
                need_sync = true;
            }
        }
    }

    // TODO: do we really want to keep this strategy ? what about when we will
    // want to remove a workspace entirely ?
    // Workspace entries should never be removed
    for base_entry in base {
        if !resolved.iter().any(|x| x.id == base_entry.id) {
            // The impossible occured ! This is most likely due to a bug in the client
            // that did the previous workspace synchronization...
            resolved.push(base_entry.to_owned());
        }
    }

    // Sorting by names make things easier for tests
    resolved.sort_by(|a, b| a.name.as_ref().cmp(b.name.as_ref()));

    (resolved, need_sync)
}

pub(super) fn merge_local_user_manifests(
    diverged: &LocalUserManifest,
    target: UserManifest,
) -> LocalUserManifest {
    // Sanity checks, called is responsible to handle them properly !
    assert_eq!(diverged.base.id, target.id);
    assert!(target.version > diverged.base.version);

    // `created` should never change, so in theory we should have
    // `diverged.base.created == target.base.created`, but there is no strict
    // guarantee (e.g. remote manifest may have been uploaded by a buggy client)
    // so we have no choice but to accept whatever value remote provides.

    let (workspaces, need_sync) = merge_workspace_entries(
        &diverged.base.workspaces,
        &diverged.workspaces,
        &target.workspaces,
    );

    let last_processed_message = std::cmp::max(
        diverged.last_processed_message,
        target.last_processed_message,
    );
    let need_sync = need_sync || last_processed_message != target.last_processed_message;

    let updated = if !need_sync {
        target.updated
    } else {
        std::cmp::max(target.updated, diverged.updated)
    };

    LocalUserManifest {
        base: target,
        need_sync,
        updated,
        last_processed_message,
        workspaces,
        speculative: false,
    }
}
