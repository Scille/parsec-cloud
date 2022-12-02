# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import asynccontextmanager
from pathlib import Path
from typing import AsyncIterator, Dict, Set, Tuple, cast

from parsec.core.fs.exceptions import FSLocalMissError
from parsec.core.fs.storage.local_database import LocalDatabase
from parsec.core.fs.storage.manifest_storage import ManifestStorage
from parsec.core.fs.storage.version import get_user_data_storage_db_path
from parsec.core.types import EntryID, LocalDevice, LocalUserManifest


async def user_storage_non_speculative_init(data_base_dir: Path, device: LocalDevice) -> None:
    data_path = get_user_data_storage_db_path(data_base_dir, device)

    # Local data storage service
    async with LocalDatabase.run(data_path) as localdb:

        # Manifest storage service
        async with ManifestStorage.run(
            device, localdb, device.user_manifest_id
        ) as manifest_storage:

            timestamp = device.timestamp()
            manifest = LocalUserManifest.new_placeholder(
                author=device.device_id,
                id=device.user_manifest_id,
                timestamp=timestamp,
                speculative=False,
            )
            await manifest_storage.set_manifest(device.user_manifest_id, manifest)


class UserStorage:
    """Storage for the user manifest.

    Provides a synchronous interface to the user manifest as it is used very often.
    """

    def __init__(
        self, device: LocalDevice, user_manifest_id: EntryID, manifest_storage: ManifestStorage
    ):
        self.device = device
        self.user_manifest_id = user_manifest_id
        self.manifest_storage = manifest_storage

    @classmethod
    @asynccontextmanager
    async def run(cls, data_base_dir: Path, device: LocalDevice) -> AsyncIterator["UserStorage"]:
        data_path = get_user_data_storage_db_path(data_base_dir, device)

        # Local database service
        async with LocalDatabase.run(data_path) as localdb:

            # Manifest storage service
            async with ManifestStorage.run(
                device, localdb, device.user_manifest_id
            ) as manifest_storage:

                # Instantiate the user storage
                self = cls(device, device.user_manifest_id, manifest_storage)

                # Populate the cache with the user manifest to be able to
                # access it synchronously at all time
                await self._load_user_manifest()
                assert self.user_manifest_id in self.manifest_storage._cache

                yield self

    # Checkpoint interface

    async def get_realm_checkpoint(self) -> int:
        return await self.manifest_storage.get_realm_checkpoint()

    async def update_realm_checkpoint(
        self, new_checkpoint: int, changed_vlobs: Dict[EntryID, int]
    ) -> None:
        """
        Raises: Nothing !
        """
        await self.manifest_storage.update_realm_checkpoint(new_checkpoint, changed_vlobs)

    async def get_need_sync_entries(self) -> Tuple[Set[EntryID], Set[EntryID]]:
        return await self.manifest_storage.get_need_sync_entries()

    # User manifest

    def get_user_manifest(self) -> LocalUserManifest:
        """
        Raises nothing, user manifest is guaranteed to be always available
        """
        return cast(LocalUserManifest, self.manifest_storage._cache[self.user_manifest_id])

    async def _load_user_manifest(self) -> None:
        try:
            await self.manifest_storage.get_manifest(self.user_manifest_id)
        except FSLocalMissError:
            # It is possible to lack the user manifest in local if our
            # device hasn't tried to access it yet (and we are not the
            # initial device of our user, in which case the user local db is
            # initialized with a non-speculative local manifest placeholder).
            # In such case it is easy to fall back on an empty manifest
            # which is a good enough approximation of the very first version
            # of the manifest (field `created` is invalid, but it will be
            # correction by the merge during sync).
            timestamp = self.device.timestamp()
            manifest = LocalUserManifest.new_placeholder(
                self.device.device_id,
                id=self.device.user_manifest_id,
                timestamp=timestamp,
                speculative=True,
            )
            await self.manifest_storage.set_manifest(self.user_manifest_id, manifest)

    async def set_user_manifest(self, user_manifest: LocalUserManifest) -> None:
        assert self.user_manifest_id == user_manifest.id
        await self.manifest_storage.set_manifest(self.user_manifest_id, user_manifest)

    # No vacuuming (used in sync monitor)

    async def run_vacuum(self) -> None:
        pass
