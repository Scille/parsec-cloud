# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

"""Versioning helpers

This file implements the algorithm needed to reconstruct the history of a path. For that, we
recursively list versions and download manifests that matches the path for a given timeframe.

As a path can be valid or not during a timeframe depending on the fact that all the manifests
composing it can change, and as a manifest can also represent different files during it's life,
this implementation is quite complex.

This recursive implementation using tasks which are attributed a timestamp facilitates the
development of different loading strategies, concerning whether the possibility to prioritize
the download of the soonest needed manifests (which is already implemented), or the number of
concurrent downloads (which will be implemented in a next version).
"""

import math
import typing
from collections import defaultdict
from functools import partial
from heapq import heappop, heappush
from typing import TYPE_CHECKING, Awaitable, Dict, List, NamedTuple, Tuple, Union, cast

import attr

from parsec._parsec import DateTime
from parsec.api.data import FileManifest, FolderManifest, UserManifest, WorkspaceManifest
from parsec.api.protocol import DeviceID
from parsec.core.fs.exceptions import FSRemoteManifestNotFound
from parsec.core.fs.path import FsPath
from parsec.core.types import EntryID
from parsec.utils import open_service_nursery

if TYPE_CHECKING:
    from parsec.core.fs import WorkspaceFS
    from parsec.core.fs.remote_loader import RemoteLoader


RemoteManifest = Union[FileManifest, FolderManifest, UserManifest, WorkspaceManifest]


class EntryNotFound(Exception):
    pass


class ManifestCacheNotFound(Exception):
    pass


class ManifestCacheDownloadLimitReached(Exception):
    pass


SYNC_GUESSED_TIME_FRAME = 30


class TimestampBoundedData(NamedTuple):
    id: EntryID
    version: int
    early: DateTime
    late: DateTime
    creator: DeviceID
    updated: DateTime
    is_folder: bool
    size: int | None
    source: FsPath | None
    destination: FsPath | None


class TimestampBoundedEntry(NamedTuple):
    id: EntryID
    version: int
    early: DateTime
    late: DateTime


class ManifestData(NamedTuple):
    creator: DeviceID
    updated: DateTime
    is_folder: bool
    size: int | None


class ManifestDataAndPaths(NamedTuple):
    data: ManifestData
    source: FsPath | None
    destination: FsPath | None


@attr.s
class ManifestDataAndMutablePaths:
    """
    Contains a manifest and his known current, previous and next paths.
    Unlike ManifestDataAndPaths, attributes are mutable.
    """

    manifest: ManifestData = attr.ib()
    source_path: FsPath | None = attr.ib(default=None)
    destination_path: FsPath | None = attr.ib(default=None)
    current_path: FsPath | None = attr.ib(default=None)

    async def try_get_path_at_timestamp(
        self, manifest_cache: ManifestCacheCounter, entry_id: EntryID, timestamp: DateTime
    ) -> FsPath | None:
        try:
            return await manifest_cache.get_path_at_timestamp(entry_id, timestamp)
        except EntryNotFound:
            return None  # Just ignore when path is inconsistent

    async def populate_source_path(
        self, manifest_cache: ManifestCacheCounter, entry_id: EntryID, timestamp: DateTime
    ) -> None:
        self.source_path = await self.try_get_path_at_timestamp(manifest_cache, entry_id, timestamp)

    async def populate_destination_path(
        self, manifest_cache: ManifestCacheCounter, entry_id: EntryID, timestamp: DateTime
    ) -> None:
        self.destination_path = await self.try_get_path_at_timestamp(
            manifest_cache, entry_id, timestamp
        )

    async def populate_current_path(
        self, manifest_cache: ManifestCacheCounter, entry_id: EntryID, timestamp: DateTime
    ) -> None:
        self.current_path = await self.try_get_path_at_timestamp(
            manifest_cache, entry_id, timestamp
        )

    async def populate_paths(
        self,
        manifest_cache: ManifestCacheCounter,
        entry_id: EntryID,
        early: DateTime,
        late: DateTime,
    ) -> None:
        # TODO : Use future manifest source field to follow files and directories
        async with open_service_nursery() as child_nursery:
            child_nursery.start_soon(
                self.populate_source_path, manifest_cache, entry_id, early.add(microseconds=-1)
            )
            child_nursery.start_soon(self.populate_destination_path, manifest_cache, entry_id, late)
            child_nursery.start_soon(self.populate_current_path, manifest_cache, entry_id, early)


class CacheEntry(NamedTuple):
    """
    Contains a manifest and the earliest and last timestamp for which its version has been returned
    """

    early: DateTime | None
    late: DateTime | None
    manifest: RemoteManifest


class ManifestCache:
    """
    Caches manifest through their version number, and the timeframe for which they could be
    obtained.
    """

    def __init__(self, remote_loader: RemoteLoader):
        self._manifest_cache: Dict[EntryID, Dict[int, CacheEntry]] = {}
        self._remote_loader: RemoteLoader = remote_loader

    def get(
        self,
        entry_id: EntryID,
        version: int | None,
        timestamp: DateTime | None,
        expected_backend_timestamp: DateTime | None = None,
    ) -> RemoteManifest:
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
        elif timestamp:
            try:
                return next(
                    t.manifest
                    for t in self._manifest_cache[entry_id].values()
                    if t.early and t.late and t.early <= timestamp <= t.late
                )
            except (KeyError, StopIteration):
                raise ManifestCacheNotFound
        else:
            raise ManifestCacheNotFound

    def update(
        self,
        manifest: RemoteManifest,
        entry_id: EntryID,
        version: int | None,
        timestamp: DateTime | None,
    ) -> None:
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
            late = self._manifest_cache[manifest.id][manifest.version].late
            early = self._manifest_cache[manifest.id][manifest.version].early
            if late is None or timestamp > late:
                self._manifest_cache[manifest.id][manifest.version] = CacheEntry(
                    early, timestamp, manifest
                )
            if early is None or timestamp < early:
                self._manifest_cache[manifest.id][manifest.version] = CacheEntry(
                    timestamp, late, manifest
                )

    async def load(
        self,
        entry_id: EntryID,
        version: int | None,
        timestamp: DateTime | None,
        expected_backend_timestamp: DateTime | None = None,
    ) -> Tuple[RemoteManifest, bool]:
        """
        Tries to find specified manifest in cache, tries to download it otherwise and updates cache

        Returns:
            A tuple containing the manifest that has been downloaded or retrieved from then cache,
            and a boolean indicating if a download was required

        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSRemoteManifestNotFound
            FSBadEncryptionRevision
            FSWorkspaceNoAccess
        """
        try:
            return (
                self.get(
                    entry_id,
                    version=version,
                    timestamp=timestamp,
                    expected_backend_timestamp=expected_backend_timestamp,
                ),
                False,
            )
        except ManifestCacheNotFound:
            pass
        manifest = await self._remote_loader.load_manifest(
            entry_id,
            version=version,
            timestamp=timestamp,
            expected_backend_timestamp=expected_backend_timestamp,
        )
        self.update(manifest, entry_id, version=version, timestamp=timestamp)
        return (manifest, True)

    async def get_path_at_timestamp(self, entry_id: EntryID, timestamp: DateTime) -> FsPath:
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
            current_manifest, _ = await self.load(current_id, version=None, timestamp=timestamp)
        except FSRemoteManifestNotFound:
            raise EntryNotFound(entry_id)

        # Loop over parts
        parts = []
        while not isinstance(current_manifest, WorkspaceManifest):
            # Get the manifest
            try:
                current_manifest = cast(Union[FolderManifest, FileManifest], current_manifest)
                parent_manifest, _ = await self.load(
                    current_manifest.parent, version=None, timestamp=timestamp
                )
                parent_manifest = cast(Union[FolderManifest, WorkspaceManifest], parent_manifest)
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
        return FsPath("/" + "/".join(reversed([part.str for part in parts])))


class ManifestCacheCounter:
    """
    Exposes a ManifestCache but simplifies the count of backend calls

    The number of downloads allowed is unlimited if limit is None or 0

    Raises:
        ManifestCacheDownloadLimitReached
    """

    def __init__(self, manifest_cache: ManifestCache, limit: int | None):
        self._manifest_cache = manifest_cache
        self.counter = 0
        self.limit = limit or math.inf

    async def load(
        self,
        entry_id: EntryID,
        version: int | None,
        timestamp: DateTime | None,
        expected_backend_timestamp: DateTime | None = None,
    ) -> RemoteManifest:
        if self.limit == self.counter:
            raise ManifestCacheDownloadLimitReached
        manifest, was_downloaded = await self._manifest_cache.load(
            entry_id, version, timestamp, expected_backend_timestamp
        )
        if was_downloaded:
            self.counter += 1
        return manifest

    async def get_path_at_timestamp(self, entry_id: EntryID, timestamp: DateTime) -> FsPath:
        """
        Simpler not to count manifest used for pathfinding as they are probably already cached
        """
        return await self._manifest_cache.get_path_at_timestamp(entry_id, timestamp)


class VersionsListCache:
    """
    Caches results of the remote_loader.list_versions calls (working on EntryIDs).

    Caches thoses in the instance of this class. No garbage collection is done for now, as this
    class is only instanciated during the list_versions execution.
    """

    def __init__(self, remote_loader: RemoteLoader):
        self._versions_list_cache: dict[EntryID, dict[int, tuple[DateTime, DeviceID]]] = {}
        self._remote_loader: RemoteLoader = remote_loader

    async def load(self, entry_id: EntryID) -> dict[int, tuple[DateTime, DeviceID]]:
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


Task = typing.Callable[[], Awaitable[None]]


class VersionListerTaskList:
    """
    Enables prioritization of tasks, as it is better to be able to configure it that way

    This class uses both an heapq to enables us to always access the latest timestamp of the tasks
    in linear time, and a dict containing lists of tasks with timestamp as keys
    """

    # TODO : use nursery for coroutines
    def __init__(
        self, manifest_cache: ManifestCacheCounter, versions_list_cache: VersionsListCache
    ):
        self.tasks: defaultdict[DateTime, list[Task]] = defaultdict(list)
        self.heapq_tasks: list[DateTime] = []
        self.manifest_cache = manifest_cache
        self.versions_list_cache = versions_list_cache

    def add(self, timestamp: DateTime, task: Task) -> None:
        if timestamp not in self.tasks:
            heappush(self.heapq_tasks, timestamp)
        self.tasks[timestamp].append(task)

    def is_empty(self) -> bool:
        return not bool(self.tasks)

    async def execute_one(self) -> None:
        min = heappop(self.heapq_tasks)
        task = self.tasks[min].pop()
        if len(self.tasks[min]) == 0:
            del self.tasks[min]
        else:
            heappush(self.heapq_tasks, min)
        await task()

    async def execute(self, number: int = 1) -> None:
        for i in range(number):
            await self.execute_one()


class VersionLister:
    """
    This class builds a version tree of a path to allow obtention of a version list.

    As we both have multiple versions of manifests for the same entry_id, and in some conditions
    entry_ids that change paths, we need a way to keep track of all those changes.
    So, we need to list versions of the different manifests composing the path, and the children of
    the dir matching pathname, recursively. By that we must also keep track of timeframes at which
    a manifest at a specific version corresponds to a part of the path.
    We also prioritize obtention of the latest used manifests at specific versions so that if a
    download cache limit is set, as many version entries as possible will be available. It is then
    also possible to re-launch the algorithm another time, using the same cache, which enables it
    to continue the download of the manifests where it stopped.
    """

    def __init__(
        self,
        workspace_fs: WorkspaceFS,
        manifest_cache: ManifestCache | None = None,
        versions_list_cache: VersionsListCache | None = None,
    ):
        self.manifest_cache = manifest_cache or ManifestCache(workspace_fs.remote_loader)
        self.versions_list_cache = versions_list_cache or VersionsListCache(
            workspace_fs.remote_loader
        )
        self.workspace_fs = workspace_fs

    async def list(
        self,
        path: FsPath,
        skip_minimal_sync: bool = True,
        starting_timestamp: DateTime | None = None,
        ending_timestamp: DateTime | None = None,
        max_manifest_queries: int | None = None,
    ) -> Tuple[List[TimestampBoundedData], bool]:
        """
        Returns:
            A tuple containing a list of TimestampBoundedData and a bool indicating wether the
            download limit has been reached
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSRemoteManifestNotFound
        """
        version_lister_one_shot = VersionListerOneShot(
            self.workspace_fs, path, self.manifest_cache, self.versions_list_cache
        )
        return await version_lister_one_shot.list(
            path,
            skip_minimal_sync=skip_minimal_sync,
            starting_timestamp=starting_timestamp,
            ending_timestamp=ending_timestamp,
            max_manifest_queries=max_manifest_queries,
        )


class VersionListerOneShot:
    def __init__(
        self,
        workspace_fs: WorkspaceFS,
        path: FsPath,
        manifest_cache: ManifestCache | None = None,
        versions_list_cache: VersionsListCache | None = None,
    ):
        self.manifest_cache = manifest_cache or ManifestCache(workspace_fs.remote_loader)
        self.versions_list_cache = versions_list_cache or VersionsListCache(
            workspace_fs.remote_loader
        )
        self.workspace_fs = workspace_fs
        self.target = path
        self.return_dict: Dict[TimestampBoundedEntry, ManifestDataAndPaths] = {}

    async def list(
        self,
        path: FsPath,
        skip_minimal_sync: bool = True,
        starting_timestamp: DateTime | None = None,
        ending_timestamp: DateTime | None = None,
        max_manifest_queries: int | None = None,
    ) -> Tuple[List[TimestampBoundedData], bool]:
        """
        Returns:
            A tuple containing a list of TimestampBoundedData and a bool indicating wether the
            download limit has been reached
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSRemoteManifestNotFound
        """
        root_manifest = await self.workspace_fs.transactions._get_manifest(
            self.workspace_fs.workspace_id
        )
        download_limit_reached = True
        try:
            self.task_list = VersionListerTaskList(
                ManifestCacheCounter(self.manifest_cache, max_manifest_queries),
                self.versions_list_cache,
            )
            self.task_list.add(
                starting_timestamp or root_manifest.created,
                partial(
                    self._populate_tree_list_versions,
                    0,
                    root_manifest.id,
                    starting_timestamp or root_manifest.created,
                    ending_timestamp or DateTime.now(),
                ),
            )
            while not self.task_list.is_empty():
                await self.task_list.execute_one()
        except ManifestCacheDownloadLimitReached:
            # TODO : expose last timestamp for which we don't miss data
            download_limit_reached = False
        versions_list = [
            TimestampBoundedData(
                id=id,
                version=version,
                early=early,
                late=late,
                creator=value.data.creator,
                updated=value.data.updated,
                is_folder=value.data.is_folder,
                size=value.data.size,
                source=value.source,
                destination=value.destination,
            )
            for (id, version, early, late), value in sorted(
                self.return_dict.items(),
                key=lambda item: (item[0].late, item[0].id, item[0].version),
            )
        ]
        return (self._sanitize_list(versions_list, skip_minimal_sync), download_limit_reached)

    def _sanitize_list(
        self, versions_list: List[TimestampBoundedData], skip_minimal_sync: bool
    ) -> List[TimestampBoundedData]:
        previous = None
        new_list = []
        # Merge duplicates with overlapping time frames as it can be caused by an update of a
        # parent dir, also, if option is set, remove what can be guessed as empty manifests set
        # before parent during sync
        for item in versions_list:
            if previous is not None:
                # If same entry_id and version
                if previous.id == item.id and previous.version == item.version:
                    # If same timestamp, only parent directory updated
                    if previous.late == item.early:
                        # Update source FsPath for current entry
                        previous = TimestampBoundedData(
                            id=item.id,
                            version=item.version,
                            early=previous.early,
                            late=item.late,
                            creator=item.creator,
                            updated=item.updated,
                            is_folder=item.is_folder,
                            size=item.size,
                            source=previous.source,
                            destination=item.destination,
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

    async def _populate_tree_load(
        self,
        path_level: int,
        entry_id: EntryID,
        early: DateTime,
        late: DateTime,
        version_number: int,
        expected_timestamp: DateTime,
        next_version_number: int,
    ) -> None:
        if early > late:
            return
        manifest = await self.task_list.manifest_cache.load(
            entry_id,
            version=version_number,
            timestamp=None,
            expected_backend_timestamp=expected_timestamp,
        )
        data = ManifestDataAndMutablePaths(
            ManifestData(
                manifest.author,
                manifest.updated,
                isinstance(manifest, FolderManifest),
                None if not isinstance(manifest, FileManifest) else manifest.size,
            )
        )
        if len(self.target.parts) == path_level:
            await data.populate_paths(self.task_list.manifest_cache, entry_id, early, late)
            self.return_dict[
                TimestampBoundedEntry(manifest.id, manifest.version, early, late)
            ] = ManifestDataAndPaths(
                data=data.manifest,
                source=data.source_path if data.source_path != data.current_path else None,
                destination=data.destination_path
                if data.destination_path != data.current_path
                else None,
            )
        else:
            if isinstance(
                manifest, (FolderManifest, WorkspaceManifest)
            ):  # If it is a file, just ignores current path
                for child_name, child_id in manifest.children.items():
                    if child_name == self.target.parts[path_level]:
                        return await self._populate_tree_list_versions(
                            path_level + 1, child_id, early, late
                        )

    async def _populate_tree_list_versions(
        self, path_level: int, entry_id: EntryID, early: DateTime, late: DateTime
    ) -> None:
        # TODO : Check if directory, melt the same entries through different parent
        versions = await self.task_list.versions_list_cache.load(entry_id)
        for version, (timestamp, creator) in versions.items():
            next_version = min(
                (v for v in versions if v > version), default=None
            )  # TODO : consistency
            self.task_list.add(
                max(early, timestamp),
                partial(
                    self._populate_tree_load,
                    path_level=path_level,
                    entry_id=entry_id,
                    early=max(early, timestamp),
                    late=late
                    if next_version not in versions
                    else min(late, versions[next_version][0]),
                    version_number=version,
                    expected_timestamp=timestamp,
                    next_version_number=next_version,
                ),
            )
