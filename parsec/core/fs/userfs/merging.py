# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Optional, Tuple

from parsec.core.types import UserManifest, LocalUserManifest, WorkspaceEntry


# TODO: replace sanity asserts by cleaner exceptions given they could be
# triggered by a malicious client trying to make us crash


def merge_workspace_entry(
    base: WorkspaceEntry, diverged: WorkspaceEntry, target: WorkspaceEntry
) -> WorkspaceEntry:
    assert diverged.id == target.id
    assert not base or base.id == target.id

    # If the name has been modified on both sides, target always wins
    if base and base.name != target.name:
        name = target.name
    elif base and base.name != diverged.name:
        name = diverged.name
    else:
        name = target.name

    # Keep last encryption
    if diverged.encryption_revision < target.encryption_revision:
        encryption_revision = target.encryption_revision
        encrypted_on = target.encrypted_on
        key = target.key
    else:
        encryption_revision = diverged.encryption_revision
        encrypted_on = diverged.encrypted_on
        key = diverged.key

    # Keep last role
    if target.role == diverged.role:
        role = target.role
        role_cached_on = min(target.role_cached_on, diverged.role_cached_on)

    elif target.role_cached_on > diverged.role_cached_on:
        role = target.role
        role_cached_on = target.role_cached_on

    else:
        role = diverged.role
        role_cached_on = diverged.role_cached_on

    return WorkspaceEntry(
        name=name,
        id=target.id,
        key=key,
        encryption_revision=encryption_revision,
        encrypted_on=encrypted_on,
        role_cached_on=role_cached_on,
        role=role,
    )


def merge_workspace_entries(
    base: Tuple[WorkspaceEntry], diverged: Tuple[WorkspaceEntry], target: Tuple[WorkspaceEntry]
) -> Tuple[Tuple[WorkspaceEntry], bool]:
    # Merging workspace entries is really trivial given:
    # - Workspaces are not required to have distinct names
    # - Workspaces entries are never removed

    # Workspace entries should never be removed
    base_entries = {entry.id for entry in base or ()}
    diverged_entries = {entry.id for entry in diverged}
    target_entries = {entry.id for entry in target}
    assert not base_entries - diverged_entries
    assert not base_entries - target_entries

    resolved = set(target)
    for d_entry in diverged:
        t_entry = next((we for we in resolved if we.id == d_entry.id), None)

        if t_entry == d_entry:
            # Target and diverged agree on the entry, nothing more to do
            continue

        elif not t_entry:
            # Diverged have added this entry alone, no conflict then
            resolved.add(d_entry)

        else:
            # Target and diverged have both modified this entry
            b_entry = next((we for we in base or () if we.id == d_entry.id), None)
            merged_entry = merge_workspace_entry(b_entry, d_entry, t_entry)
            resolved.remove(t_entry)
            resolved.add(merged_entry)

    need_sync = resolved != set(target)

    # Sorting by names make things easier for tests
    resolved_sorted = sorted(resolved, key=lambda w: w.name)

    return tuple(resolved_sorted), need_sync


def merge_remote_user_manifests(
    base: Optional[UserManifest], diverged: UserManifest, target: UserManifest
) -> Tuple[UserManifest, bool]:
    if base:
        assert isinstance(base, UserManifest)
    assert isinstance(diverged, UserManifest)
    assert isinstance(target, UserManifest)

    if base is None:
        base_version = 0
        base_workspaces = {}
    else:
        base_version = base.version
        base_workspaces = base.workspaces
    # TODO: assert means we will get a crash in case of malicious target
    assert base_version + 1 == diverged.version
    assert target.version >= diverged.version
    # Not true when merging user manifest v1 given v0 is lazily generated
    assert base_version == 0 or diverged.created == target.created

    workspaces, need_sync = merge_workspace_entries(
        base_workspaces, diverged.workspaces, target.workspaces
    )

    last_processed_message = max(diverged.last_processed_message, target.last_processed_message)
    need_sync = need_sync or last_processed_message != target.last_processed_message

    if not need_sync:
        updated = target.updated
    else:
        if target.updated > diverged.updated:
            updated = target.updated
        else:
            updated = diverged.updated

    merged = target.evolve(
        updated=updated,
        workspaces=workspaces,
        last_processed_message=last_processed_message,
        version=target.version + 1,
    )

    return merged, need_sync


def merge_local_user_manifests(
    base: Optional[LocalUserManifest], diverged: LocalUserManifest, target: LocalUserManifest
) -> LocalUserManifest:
    if base:
        assert isinstance(base, LocalUserManifest)
    assert isinstance(diverged, LocalUserManifest)
    assert isinstance(target, LocalUserManifest)
    assert not target.need_sync
    assert not target.is_placeholder

    if base is None:
        base_version = 0
        base_workspaces = None
    else:
        assert base.base_version == 0 or not base.is_placeholder
        base_version = base.base_version
        base_workspaces = base.workspaces

    assert base_version == diverged.base_version
    assert target.base_version > diverged.base_version
    # Not true when merging user manifest v1 given v0 is lazily generated
    assert base_version == 0 or diverged.created == target.created

    workspaces, need_sync = merge_workspace_entries(
        base_workspaces, diverged.workspaces, target.workspaces
    )

    last_processed_message = max(diverged.last_processed_message, target.last_processed_message)
    need_sync = need_sync or last_processed_message != target.last_processed_message

    if not need_sync:
        updated = target.updated
    else:
        if target.updated > diverged.updated:
            updated = target.updated
        else:
            updated = diverged.updated

    merged = target.evolve(
        need_sync=need_sync,
        updated=updated,
        workspaces=workspaces,
        last_processed_message=last_processed_message,
    )

    return merged
