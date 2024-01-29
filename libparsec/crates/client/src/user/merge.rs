// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

pub(super) fn merge_local_user_manifests(
    local: &LocalUserManifest,
    remote: UserManifest,
) -> Option<LocalUserManifest> {
    // Sanity check (caller is responsible for this !)
    assert_eq!(local.base.id, remote.id);

    if remote.version <= local.base.version {
        return None;
    }

    let need_sync = false;

    // `created` should never change, so in theory we should have
    // `diverged.base.created == target.base.created`, but there is no strict
    // guarantee (e.g. remote manifest may have been uploaded by a buggy client)
    // so we have no choice but to accept whatever value remote provides.

    // `LocalUserManifest::local_workspaces` field is not synced.

    // That's all ? Yes. Currently there is nothing to sync in the user manifest
    // (i.e. the local user manifest is only used to store newly created workspaces
    // that doesn't have certificates yet, and to keep a cache on decrypted
    // workspaces name to allow offline access).
    // However we keep this code here in prevision of a future need of synchronization
    // (typically for user configuration).

    let updated = if !need_sync {
        remote.updated
    } else {
        std::cmp::max(remote.updated, local.updated)
    };

    Some(LocalUserManifest {
        base: remote,
        need_sync,
        updated,
        local_workspaces: local.local_workspaces.clone(),
        speculative: false,
    })
}
