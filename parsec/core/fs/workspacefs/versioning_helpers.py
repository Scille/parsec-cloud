# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import trio
from typing import List, NamedTuple
from pendulum import Pendulum

from parsec.api.data import Manifest as RemoteManifest
from parsec.api.protocol import DeviceID
from parsec.core.types import FsPath, EntryID
from parsec.core.fs.utils import is_file_manifest, is_folder_manifest, is_workspace_manifest
from parsec.core.fs.exceptions import FSRemoteManifestNotFound


class EntryNotFound(Exception):
    pass


class ManifestCacheNotFound(Exception):
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


@attr.s
class ManifestDataAndMutablePaths:
    """
    Contains a manifest and his known current, previous and next paths.
    Unlike ManifestDataAndPaths, attributes are mutable.
    """

    manifest: ManifestData = attr.ib()
    source_path: FsPath = attr.ib(default=None)
    destination_path: FsPath = attr.ib(default=None)
    current_path: FsPath = attr.ib(default=None)


class CacheEntry(NamedTuple):
    """
    Contains a manifest and the earliest and last timestamp for which its version has been returned
    """

    early: Pendulum
    late: Pendulum
    manifest: RemoteManifest


class ManifestCache:
    """
    Caches manifest through their version number, and the timeframe for which they could be
    obtained.
    """

    def __init__(self, remote_loader):
        self._manifest_cache = {}
        self._remote_loader = remote_loader

    def get(self, entry_id: EntryID, version=None, timestamp=None) -> RemoteManifest:
        """
        Tries to find specified manifest in cache, raises ManifestCacheNotFound otherwise

        Raises:
            ManifestCacheNotFound
        """
        if version:
            try:
                return self._manifest_cache[entry_id][version].manifest
            except KeyError:
                raise ManifestCacheNotFound
        if timestamp:
            try:
                return next(
                    t.manifest
                    for t in self._manifest_cache[entry_id].values()
                    if t.early and t.late and t.early <= timestamp <= t.late
                )
            except (KeyError, StopIteration):
                raise ManifestCacheNotFound

    def update(self, manifest: RemoteManifest, entry_id: EntryID, version=None, timestamp=None):
        """
        Updates manifest cache, increasing timeframe for a specified version if possible

        Raises:
            ManifestCacheNotFound
        """
        if manifest.id not in self._manifest_cache:
            self._manifest_cache[manifest.id] = {}
        if manifest.version not in self._manifest_cache[manifest.id]:
            self._manifest_cache[manifest.id][manifest.version] = CacheEntry(
                timestamp, timestamp, manifest
            )
        elif timestamp:
            if (
                self._manifest_cache[manifest.id][manifest.version].late is None
                or timestamp > self._manifest_cache[manifest.id][manifest.version].late
            ):
                self._manifest_cache[manifest.id][manifest.version] = CacheEntry(
                    self._manifest_cache[manifest.id][manifest.version].early, timestamp, manifest
                )
            if (
                self._manifest_cache[manifest.id][manifest.version].early is None
                or timestamp < self._manifest_cache[manifest.id][manifest.version].early
            ):
                self._manifest_cache[manifest.id][manifest.version] = CacheEntry(
                    timestamp, self._manifest_cache[manifest.id][manifest.version].late, manifest
                )

    async def load(self, entry_id: EntryID, version=None, timestamp=None):
        """
        Tries to find specified manifest in cache, tries to download it otherwise and updates cache

        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSRemoteManifestNotFound
            FSBadEncryptionRevision
            FSWorkspaceNoAccess
        """
        try:
            return self.get(entry_id, version=version, timestamp=timestamp)
        except ManifestCacheNotFound:
            pass
        manifest = await self._remote_loader.load_manifest(
            entry_id, version=version, timestamp=timestamp
        )
        self.update(manifest, entry_id, version=version, timestamp=timestamp)
        return manifest


class VersionsListCache:
    """
    Caches results of the remote_loader.list_versions calls (working on EntryIDs).

    Caches thoses in the instance of this class. No garbage collection is done for now, as this
    class is only instanciated during _get_path_at_timestamp execution.
    """

    def __init__(self, remote_loader):
        self._versions_list_cache = {}
        self._remote_loader = remote_loader

    async def load(self, entry_id: EntryID):
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSRemoteManifestNotFound
        """
        if entry_id in self._versions_list_cache:
            return self._versions_list_cache[entry_id]
        self._versions_list_cache[entry_id] = await self._remote_loader.list_versions(entry_id)
        return self._versions_list_cache[entry_id]


async def _get_path_at_timestamp(
    manifest_cache: ManifestCache, entry_id: EntryID, timestamp: Pendulum
) -> FsPath:
    """
    Find a path for an entry_id at a specific timestamp.

    If the path is broken, will raise an EntryNotFound exception. All the other exceptions are
    thrown by the ManifestCache.

    Raises:
        FSError
        FSBackendOfflineError
        FSWorkspaceInMaintenance
        FSBadEncryptionRevision
        FSWorkspaceNoAccess
        EntryNotFound
    """
    # Get first manifest
    try:
        current_id = entry_id
        current_manifest = await manifest_cache.load(current_id, timestamp=timestamp)
    except FSRemoteManifestNotFound:
        raise EntryNotFound(entry_id)

    # Loop over parts
    parts = []
    while not is_workspace_manifest(current_manifest):

        # Get the manifest
        try:
            parent_manifest = await manifest_cache.load(
                current_manifest.parent, timestamp=timestamp
            )
        except FSRemoteManifestNotFound:
            raise EntryNotFound(entry_id)

        # Find the child name
        for name, child_id in parent_manifest.children.items():
            if child_id == current_id:
                parts.append(name)
                break
        else:
            raise EntryNotFound(entry_id)

        # Continue until root is found
        current_id = current_manifest.parent
        current_manifest = parent_manifest

    # Return the path
    return FsPath("/" + "/".join(reversed(parts)))


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
    manifest_cache = ManifestCache(workspacefs.remote_loader)
    versions_list_cache = VersionsListCache(workspacefs.remote_loader)

    async def _try_get_path_at_timestamp(entry_id: EntryID, timestamp: Pendulum) -> FsPath:
        try:
            return await _get_path_at_timestamp(manifest_cache, entry_id, timestamp)
        except EntryNotFound:
            return None

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
        manifest = await manifest_cache.load(entry_id, version=version_number)
        data = ManifestDataAndMutablePaths(
            ManifestData(
                manifest.author,
                manifest.updated,
                is_folder_manifest(manifest),
                None if not is_file_manifest(manifest) else manifest.size,
            )
        )
        if len(target.parts) == path_level:

            async def _populate_source_path(data, entry_id, timestamp):
                data.source_path = await _try_get_path_at_timestamp(entry_id, timestamp)

            async def _populate_destination_path(data, entry_id, timestamp):
                data.destination_path = await _try_get_path_at_timestamp(entry_id, timestamp)

            async def _populate_current_path(data, entry_id, timestamp):
                data.current_path = await _try_get_path_at_timestamp(entry_id, timestamp)

            # TODO : Use future manifest source field to follow files and directories
            async with trio.open_service_nursery() as child_nursery:
                child_nursery.start_soon(
                    _populate_source_path, data, entry_id, early.add(microseconds=-1)
                )
                child_nursery.start_soon(_populate_destination_path, data, entry_id, late)
                child_nursery.start_soon(_populate_current_path, data, entry_id, early)
            tree[
                TimestampBoundedEntry(manifest.id, manifest.version, early, late)
            ] = ManifestDataAndPaths(
                data=data.manifest,
                source=data.source_path if data.source_path != data.current_path else None,
                destination=data.destination_path
                if data.destination_path != data.current_path
                else None,
            )
        else:
            if not is_file_manifest(manifest):
                for child_name, child_id in manifest.children.items():
                    if child_name == target.parts[path_level]:
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
        versions = await versions_list_cache.load(entry_id)
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
    async with trio.open_service_nursery() as nursery:
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
            new_list.append(previous)
        previous = item
    if previous:
        new_list.append(previous)
    return new_list
