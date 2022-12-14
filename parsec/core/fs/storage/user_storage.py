# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import asynccontextmanager
from pathlib import Path
from typing import AsyncIterator

import trio

from parsec._parsec import UserStorage as _SyncUserStorage
from parsec._parsec import (
    user_storage_non_speculative_init as sync_user_storage_non_speculative_init,
)
from parsec.core.types import EntryID, LocalDevice, LocalUserManifest


async def user_storage_non_speculative_init(data_base_dir: Path, device: LocalDevice) -> None:
    return await trio.to_thread.run_sync(
        sync_user_storage_non_speculative_init, data_base_dir, device
    )


class UserStorage:
    """Storage for the user manifest.

    Provides a synchronous interface to the user manifest as it is used very often.
    """

    def __init__(self, device: LocalDevice, user_manifest_id: EntryID, data_base_dir: Path):
        self.sync_instance = _SyncUserStorage(device, user_manifest_id, data_base_dir)

    @property
    def device(self) -> LocalDevice:
        return self.sync_instance.device

    @classmethod
    @asynccontextmanager
    async def run(cls, data_base_dir: Path, device: LocalDevice) -> AsyncIterator["UserStorage"]:
        # Instantiate the user storage
        self = cls(device, device.user_manifest_id, data_base_dir)

        # Populate the cache with the user manifest to be able to
        # access it synchronously at all time
        await trio.to_thread.run_sync(self.sync_instance.load_user_manifest)

        try:
            yield self
        finally:
            self.sync_instance.close_connections()

    # Checkpoint interface

    async def get_realm_checkpoint(self) -> int:
        return await trio.to_thread.run_sync(self.sync_instance.get_realm_checkpoint)

    async def update_realm_checkpoint(
        self, new_checkpoint: int, changed_vlobs: dict[EntryID, int]
    ) -> None:
        """
        Raises: Nothing !
        """
        return await trio.to_thread.run_sync(
            self.sync_instance.update_realm_checkpoint, new_checkpoint, changed_vlobs
        )

    async def get_need_sync_entries(self) -> tuple[set[EntryID], set[EntryID]]:
        return await trio.to_thread.run_sync(self.sync_instance.get_need_sync_entries)

    # User manifest

    def get_user_manifest(self) -> LocalUserManifest:
        """
        Raises nothing, user manifest is guaranteed to be always available
        """
        return self.sync_instance.get_user_manifest()

    async def set_user_manifest(self, user_manifest: LocalUserManifest) -> None:
        return await trio.to_thread.run_sync(self.sync_instance.set_user_manifest, user_manifest)

    # No vacuuming (used in sync monitor)

    async def run_vacuum(self) -> None:
        pass
