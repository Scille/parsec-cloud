# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from typing import List, NamedTuple
from pendulum import Pendulum

from parsec.api.protocol import DeviceID
from parsec.core.types import FsPath, EntryID
from parsec.core.fs.utils import is_file_manifest, is_folder_manifest, is_workspace_manifest
from parsec.core.fs.exceptions import FSRemoteManifestNotFoundBadVersion, FSLocalMissError


class EntryNotFound(Exception):
    pass


SYNC_GUESSED_TIME_FRAME = 30


class TimestampBoundedData(NamedTuple):
    id: EntryID
    version: int
    early: Pendulum
    late: Pendulum
    creator: DeviceID
    updated: Pendulum
    is_folder: bool
    size: int
    source: FsPath
    destination: FsPath


class TimestampBoundedEntry(NamedTuple):
    id: EntryID
    version: int
    early: Pendulum
    late: Pendulum


class ManifestData(NamedTuple):
    creator: DeviceID
    updated: Pendulum
    is_folder: bool
    size: int


class ManifestDataAndPaths(NamedTuple):
    data: ManifestData
    source: FsPath
    destination: FsPath


async def list_versions(
    workspacefs, path: FsPath, skip_minimal_sync: bool = True
) -> List[TimestampBoundedData]:
    """
    Raises:
        FSError
        FSBackendOfflineError
        FSWorkspaceInMaintenance
        FSRemoteManifestNotFound
    """
    # Could be optimized if we could use manifest.updated
    manifest_cache = {}
    versions_list_cache = {}

    async def _load_manifest_or_cached(entry_id: EntryID, version=None, timestamp=None):
        try:
            if version:
                return manifest_cache[entry_id][version][2]
            if timestamp:
                return next(
                    (
                        t[2]
                        for t in manifest_cache[entry_id].values()
                        if t[0] and t[1] and t[0] <= timestamp <= t[1]
                    )
                )
        except (KeyError, StopIteration):
            pass
        manifest = await workspacefs.remote_loader.load_manifest(
            entry_id, version=version, timestamp=timestamp
        )
        if manifest.id not in manifest_cache:
            manifest_cache[manifest.id] = {}
        if manifest.version not in manifest_cache[manifest.id]:
            manifest_cache[manifest.id][manifest.version] = (timestamp, timestamp, manifest)
        elif timestamp:
            if (
                manifest_cache[manifest.id][manifest.version][1] is None
                or timestamp > manifest_cache[manifest.id][manifest.version][1]
            ):
                manifest_cache[manifest.id][manifest.version] = (
                    manifest_cache[manifest.id][manifest.version][0],
                    timestamp,
                    manifest,
                )
            if (
                manifest_cache[manifest.id][manifest.version][0] is None
                or timestamp < manifest_cache[manifest.id][manifest.version][0]
            ):
                manifest_cache[manifest.id][manifest.version] = (
                    timestamp,
                    manifest_cache[manifest.id][manifest.version][1],
                    manifest,
                )
        return manifest

    async def _list_versions(entry_id: EntryID):
        if entry_id in versions_list_cache:
            return versions_list_cache[entry_id]
        versions_list_cache[entry_id] = await workspacefs.remote_loader.list_versions(entry_id)
        return versions_list_cache[entry_id]

    async def _get_past_path(entry_id: EntryID, version=None, timestamp=None) -> FsPath:

        # Get first manifest
        try:
            current_id = entry_id
            current_manifest = await _load_manifest_or_cached(
                current_id, version=version, timestamp=timestamp
            )
        except FSLocalMissError:
            raise EntryNotFound(entry_id)

        # Loop over parts
        parts = []
        while not is_workspace_manifest(current_manifest):

            # Get the manifest
            try:
                parent_manifest = await _load_manifest_or_cached(
                    current_manifest.parent, version=version, timestamp=timestamp
                )
            except FSLocalMissError:
                raise EntryNotFound(entry_id)

            # Find the child name
            try:
                name = next(
                    name
                    for name, child_id in parent_manifest.children.items()
                    if child_id == current_id
                )
            except StopIteration:
                raise EntryNotFound(entry_id)
            else:
                parts.append(name)

            # Continue until root is found
            current_id = current_manifest.parent
            current_manifest = parent_manifest

        # Return the path
        return FsPath("/" + "/".join(reversed(parts)))

    async def _populate_tree_load(
        nursery,
        target: FsPath,
        path_level: int,
        tree: dict,
        entry_id: EntryID,
        early: Pendulum,
        late: Pendulum,
        version_number: int,
        next_version_number: int,
    ):
        if early > late:
            return
        manifest = await _load_manifest_or_cached(entry_id, version=version_number)
        data = [
            ManifestData(
                manifest.author,
                manifest.updated,
                is_folder_manifest(manifest),
                None if not is_file_manifest(manifest) else manifest.size,
            ),
            None,  # Source path
            None,  # Destination path
            None,  # Current path
        ]

        if len(target.parts) == path_level + 1:

            async def _populate_path_w_index(data, index, entry_id, timestamp):
                try:
                    data[index] = await _get_past_path(entry_id, timestamp=timestamp)
                except (FSRemoteManifestNotFoundBadVersion, EntryNotFound):
                    pass

            # TODO : Use future manifest source field to follow files and directories
            async with trio.open_nursery() as child_nursery:
                child_nursery.start_soon(
                    _populate_path_w_index, data, 1, entry_id, early.add(microseconds=-1)
                )
                child_nursery.start_soon(_populate_path_w_index, data, 2, entry_id, late)
                child_nursery.start_soon(_populate_path_w_index, data, 3, entry_id, early)
            tree[
                TimestampBoundedEntry(manifest.id, manifest.version, early, late)
            ] = ManifestDataAndPaths(
                data=data[0],
                source=data[1] if data[1] != data[3] else None,
                destination=data[2] if data[2] != data[3] else None,
            )
        else:
            if not is_file_manifest(manifest):
                for child_name, child_id in manifest.children.items():
                    if child_name == target.parts[path_level + 1]:
                        return await _populate_tree_list_versions(
                            nursery,
                            target,
                            path_level + 1,
                            tree,
                            child_id,
                            early if early > manifest.updated else manifest.updated,
                            late,
                        )
            else:
                pass  # TODO : Broken path. What to do?

    async def _populate_tree_list_versions(
        nursery,
        target: FsPath,
        path_level: int,
        tree: dict,
        entry_id: EntryID,
        early: Pendulum,
        late: Pendulum,
    ):
        # TODO : Check if directory, melt the same entries through different parent
        versions = await _list_versions(entry_id)
        for version, (timestamp, creator) in versions.items():
            next_version = min((v for v in versions if v > version), default=None)
            nursery.start_soon(
                _populate_tree_load,
                nursery,
                target,
                path_level,
                tree,
                entry_id,
                max(early, timestamp),
                late if next_version not in versions else min(late, versions[next_version][0]),
                version,
                next_version,
            )

    return_tree = {}
    root_manifest = await workspacefs.transactions._get_manifest(workspacefs.workspace_id)
    async with trio.open_nursery() as nursery:
        nursery.start_soon(
            _populate_tree_list_versions,
            nursery,
            path,
            0,
            return_tree,
            root_manifest.id,
            root_manifest.created,
            Pendulum.now(),
        )
    versions_list = [
        TimestampBoundedData(*item[0], *item[1].data, item[1].source, item[1].destination)
        for item in sorted(
            list(return_tree.items()), key=lambda item: (item[0].late, item[0].id, item[0].version)
        )
    ]
    previous = None
    new_list = []
    # Merge duplicates with overlapping time frames as it can be caused by an update of a parent
    # dir, also, if option is set, remove what can be guessed as empty manifests set before parent
    # during sync
    for item in versions_list:
        if previous is not None:
            # If same entry_id and version
            if previous.id == item.id and previous.version == item.version:
                if previous.late == item.early:  # Same timestamp, only parent directory updated
                    # Update source FsPath for current entry
                    previous = TimestampBoundedData(
                        item.id,
                        item.version,
                        previous.early,
                        item.late,
                        item.creator,
                        item.updated,
                        item.is_folder,
                        item.size,
                        previous.source,
                        item.destination,
                    )
                    continue
            # If option is set, same entry_id, previous version is 0 bytes
            if (
                skip_minimal_sync
                and previous.id == item.id
                and previous.version == item.version - 1
                and previous.version == 1
                and previous.size == 0  # Empty file (would be None for dir)
                and previous.is_folder == item.is_folder
                and previous.late == item.early
                and previous.late < previous.early.add(seconds=SYNC_GUESSED_TIME_FRAME)
                and not previous.source
            ):
                previous = item
                continue
            new_list.append(
                TimestampBoundedData(*previous[:8], previous.source, previous.destination)
            )
        previous = item
    if previous:
        new_list.append(TimestampBoundedData(*previous[:8], previous.source, previous.destination))
    return new_list
