# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Tuple, Optional, Dict

from parsec.api.data import UserManifest, WorkspaceEntry
from parsec.core.types import LocalUserManifest
from parsec.api.data.entry import EntryID


# TODO: replace sanity asserts by cleaner exceptions given they could be
# triggered by a malicious client trying to make us crash


def merge_workspace_entry(
    base: Optional[WorkspaceEntry], diverged: WorkspaceEntry, target: WorkspaceEntry
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
    if diverged.encryption_revision <= target.encryption_revision:
        # Note `diverged.encryption_revision == target.encryption_revision`
        # should imply `diverged.encrypted_on == target.encrypted_on`, but
        # there is no way to enforce this (e.g. a buggy client may have change
        # this value...). However we'd better keep the remote value to avoid
        # constant sync fight between two client if they end-up with a different
        # value they both consider the "right" one.
        encryption_revision = target.encryption_revision
        encrypted_on = target.encrypted_on
        key = target.key
    else:
        encryption_revision = diverged.encryption_revision
        encrypted_on = diverged.encrypted_on
        key = diverged.key

    # Keep most recent cache info on role
    if target.role == diverged.role:
        role = target.role
        role_cached_on = max(target.role_cached_on, diverged.role_cached_on)

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
    base: Optional[Tuple[WorkspaceEntry, ...]],
    diverged: Tuple[WorkspaceEntry, ...],
    target: Tuple[WorkspaceEntry, ...],
) -> Tuple[Tuple[WorkspaceEntry, ...], bool]:
    # Merging workspace entries is really trivial given:
    # - Workspaces are not required to have distinct names
    # - Workspaces entries are never removed

    # Workspace entries should never be removed
    base_entries = {entry.id for entry in base or ()}
    diverged_entries = {entry.id for entry in diverged}
    target_entries = {entry.id for entry in target}
    assert not base_entries - diverged_entries
    assert not base_entries - target_entries

    resolved: Dict[EntryID, WorkspaceEntry] = {we.id: we for we in target}
    for d_entry in diverged:
        t_entry: Optional[WorkspaceEntry] = resolved.get(d_entry.id)

        if t_entry == d_entry:
            # Target and diverged agree on the entry, nothing more to do
            continue

        elif not t_entry:
            # Diverged have added this entry alone, no conflict then
            resolved[d_entry.id] = d_entry

        else:
            # Target and diverged have both modified this entry
            b_entry = next((we for we in base or () if we.id == d_entry.id), None)
            merged_entry = merge_workspace_entry(b_entry, d_entry, t_entry)
            resolved[d_entry.id] = merged_entry

    need_sync = resolved.keys() != target_entries

    # Sorting by names make things easier for tests
    resolved_sorted = sorted(resolved.values(), key=lambda w: w.name)

    return tuple(resolved_sorted), need_sync


def merge_local_user_manifests(
    diverged: LocalUserManifest, target: UserManifest
) -> LocalUserManifest:
    assert isinstance(diverged, LocalUserManifest)
    assert isinstance(target, UserManifest)
    assert diverged.id == target.id

    base_version = diverged.base_version
    base_workspaces = diverged.base.workspaces if diverged.base is not None else None

    assert target.version > base_version

    # `created` should never change, so in theory we should have
    # `diverged.created == target.created`, but there is no strict guarantee
    # (e.g. remote manifest may have been uploaded by a buggy client) so
    # we have no choice but to accept whatever value remote provides.

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

    return LocalUserManifest(
        base=target,
        need_sync=need_sync,
        updated=updated,
        last_processed_message=last_processed_message,
        workspaces=workspaces,
        speculative=False,
    )
