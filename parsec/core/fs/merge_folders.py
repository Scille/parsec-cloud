# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
from itertools import count
from typing import Optional, List, Tuple

from parsec.core.types import RemoteManifest, LocalManifest, UserManifest, LocalUserManifest


def find_conflicting_name_for_child_entry(original_name: str, check_candidate_name) -> str:
    try:
        base_name, extension = original_name.rsplit(".")
    except ValueError:
        extension = None
        base_name = original_name

    now = pendulum.now()
    for tentative in count(1):
        tentative_str = "" if tentative == 1 else f" n°{tentative}"
        # TODO: Also add device id in the naming ?
        diverged_name = f"{base_name} (conflict{tentative_str} {now.to_datetime_string()})"
        if extension:
            diverged_name += f".{extension}"

        if check_candidate_name(diverged_name):
            return diverged_name

    assert False  # Should never be here


def merge_children(base, diverged, target):
    # If entry is in base but not in diverged and target, it is then already
    # resolved.
    all_entries = diverged.keys() | target.keys()
    conflicts = []
    resolved = {}
    need_sync = False

    for entry_name in all_entries:
        base_entry = base.get(entry_name)
        target_entry = target.get(entry_name)
        diverged_entry = diverged.get(entry_name)

        if diverged_entry == target_entry:
            # No modifications or same modification on both sides, either case
            # just keep things like this
            if target_entry:
                resolved[entry_name] = target_entry

        elif target_entry == base_entry:
            # Entry has been modified on diverged side only
            need_sync = True
            if diverged_entry:
                resolved[entry_name] = diverged_entry

        elif diverged_entry == base_entry:
            # Entry has been modified en target side only
            if target_entry:
                resolved[entry_name] = target_entry

        else:
            # Entry modified on both side...
            if not target_entry:
                need_sync = True
                # Entry removed on target side, apply diverged modifications
                # not to loose them
                # TODO: rename entry to `<name>.deleted` ?
                resolved[entry_name] = diverged_entry

            elif not diverged_entry:
                need_sync = True
                # Entry removed on diverged side and modified (no remove) on
                # target side, just apply them
                # TODO: rename entry to `<name>.deleted` ?
                resolved[entry_name] = target_entry

            else:
                need_sync = True
                # Entry modified on both side (no remove), conflict !

                def check_candidate_name(name):
                    return name not in target and name not in diverged and name not in resolved

                conflict_entry_name = find_conflicting_name_for_child_entry(
                    entry_name, check_candidate_name
                )
                resolved[entry_name] = target_entry
                conflict_entry_entry = resolved[conflict_entry_name] = diverged[entry_name]
                conflicts.append(
                    (entry_name, target_entry.id, conflict_entry_name, conflict_entry_entry.id)
                )

    return resolved, need_sync, conflicts


def merge_workspaces(base, diverged, target):
    resolved = set()
    conflicts = []

    def _check_candidate_name(name):
        return all(we.name != name for we in resolved)

    def _insert_entry_with_unique_name(new_entry):
        conflicts = []
        if not _check_candidate_name(new_entry.name):
            conflict_entry_name = find_conflicting_name_for_child_entry(
                new_entry.name, _check_candidate_name
            )
            orginal_id = next(we.access.id for we in resolved if we.name == new_entry.name)
            conflicts.append((new_entry.name, orginal_id, conflict_entry_name, new_entry.access.id))
            new_entry = new_entry.evolve(name=conflict_entry_name)

        resolved.add(new_entry)
        return conflicts

    # Sanity pass to make sure the workspaces have unique names
    # (should have been enforced when target has been synchroned)
    for t_entry in target:
        conflicts += _insert_entry_with_unique_name(t_entry)
    assert resolved == set(target)

    for d_entry in diverged:
        t_entry = next((we for we in resolved if we.access.id == d_entry.access.id), None)
        if t_entry == d_entry:
            # Target and diverged agree on the entry, nothing more to do
            continue

        elif not t_entry:
            # Diverged have added this entry alone, no conflict then
            conflicts += _insert_entry_with_unique_name(d_entry)

        else:
            # Target and diverged have both modified this entry
            b_entry = next((we for we in base or () if we.access.id == d_entry.access.id), None)

            # If the name has been modified on both sides, target always wins
            if b_entry and b_entry.name != t_entry.name:
                name = t_entry.name
            elif b_entry and b_entry.name != d_entry.name:
                name = d_entry.name
            else:
                name = t_entry.name

            # Keep last right informations
            if t_entry.granted_on >= d_entry.granted_on:
                merged_entry = t_entry.evolve(name=name)

            else:
                merged_entry = d_entry.evolve(name=name)

            resolved.remove(t_entry)
            conflicts += _insert_entry_with_unique_name(merged_entry)

    need_sync = resolved != set(target)
    return tuple(resolved), need_sync, conflicts


def merge_remote_user_manifests(
    base: Optional[UserManifest], diverged: UserManifest, target: UserManifest
) -> Tuple[UserManifest, bool, List]:
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
    assert base_version + 1 == diverged.version
    assert target.version >= diverged.version
    # Not true when merging user manifest v1 given v0 is lazily generated
    # assert diverged.created == target.created

    workspaces, need_sync, conflicts = merge_workspaces(
        base_workspaces, diverged.workspaces, target.workspaces
    )

    if not need_sync:
        updated = target.updated
    else:
        if target.updated > diverged.updated:
            updated = target.updated
        else:
            updated = diverged.updated

    evolves = {
        "updated": updated,
        "workspaces": workspaces,
        "last_processed_message": max(
            diverged.last_processed_message, target.last_processed_message
        ),
    }

    merged = target.evolve(**evolves)
    return merged, need_sync, conflicts


def merge_remote_folder_manifests(
    base: Optional[RemoteManifest], diverged: RemoteManifest, target: RemoteManifest
) -> Tuple[RemoteManifest, bool, List]:
    if base:
        assert not isinstance(base, UserManifest)
    assert not isinstance(diverged, UserManifest)
    assert not isinstance(target, UserManifest)

    if base is None:
        base_version = 0
        base_children = {}
    else:
        base_version = base.version
        base_children = base.children
    assert base_version + 1 == diverged.version
    assert target.version >= diverged.version
    # Not true when merging user manifest v1 given v0 is lazily generated
    # assert diverged.created == target.created

    children, need_sync, conflicts = merge_children(
        base_children, diverged.children, target.children
    )

    if not need_sync:
        updated = target.updated
    else:
        if target.updated > diverged.updated:
            updated = target.updated
        else:
            updated = diverged.updated

    evolves = {"updated": updated, "children": children}

    merged = target.evolve(**evolves)
    return merged, need_sync, conflicts


def merge_remote_manifests(
    base: Optional[RemoteManifest], diverged: RemoteManifest, target: RemoteManifest
) -> Tuple[RemoteManifest, bool, List]:
    if isinstance(target, UserManifest):
        return merge_remote_user_manifests(base, diverged, target)
    else:
        return merge_remote_folder_manifests(base, diverged, target)


def merge_local_folder_manifests(
    base: Optional[LocalManifest], diverged: LocalManifest, target: LocalManifest
) -> Tuple[LocalManifest, bool, List]:
    if base:
        assert not isinstance(base, LocalUserManifest)
    assert not isinstance(diverged, LocalUserManifest)
    assert not isinstance(target, LocalUserManifest)

    if base is None:
        base_version = 0
        base_children = {}
    else:
        base_version = base.base_version
        base_children = base.children
    assert base_version == diverged.base_version
    assert target.base_version > diverged.base_version
    # Not true when merging user manifest v1 given v0 is lazily generated
    # assert diverged.created == target.created

    children, need_sync, conflicts = merge_children(
        base_children, diverged.children, target.children
    )

    if not need_sync:
        updated = target.updated
    else:
        # TODO: potentially unsafe if two modifications are done within the same millisecond
        if target.updated > diverged.updated:
            updated = target.updated
        else:
            updated = diverged.updated

    evolves = {"need_sync": need_sync, "updated": updated, "children": children}

    merged = target.evolve(**evolves)
    return merged, need_sync, conflicts


def merge_local_user_manifests(
    base: Optional[LocalUserManifest], diverged: LocalUserManifest, target: LocalUserManifest
) -> Tuple[LocalUserManifest, bool, List]:
    if base:
        assert isinstance(base, LocalUserManifest)
    assert isinstance(diverged, LocalUserManifest)
    assert isinstance(target, LocalUserManifest)

    if base is None:
        base_version = 0
        base_workspaces = None
    else:
        base_version = base.base_version
        base_workspaces = base.workspaces

    assert base_version == diverged.base_version
    assert target.base_version > diverged.base_version
    # Not true when merging user manifest v1 given v0 is lazily generated
    # assert diverged.created == target.created

    workspaces, need_sync, conflicts = merge_workspaces(
        base_workspaces, diverged.workspaces, target.workspaces
    )

    if not need_sync:
        updated = target.updated
    else:
        # TODO: potentially unsafe if two modifications are done within the same millisecond
        if target.updated > diverged.updated:
            updated = target.updated
        else:
            updated = diverged.updated

    evolves = {
        "need_sync": need_sync,
        "updated": updated,
        "workspaces": workspaces,
        "last_processed_message": max(
            diverged.last_processed_message, target.last_processed_message
        ),
    }

    merged = target.evolve(**evolves)
    return merged, need_sync, conflicts


def merge_local_manifests(
    base: Optional[LocalManifest], diverged: LocalManifest, target: LocalManifest
) -> Tuple[LocalManifest, bool, List]:
    if isinstance(target, LocalUserManifest):
        return merge_local_user_manifests(base, diverged, target)

    else:
        return merge_local_folder_manifests(base, diverged, target)
