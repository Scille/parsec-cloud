# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import asynccontextmanager
from pathlib import Path
from typing import AsyncIterator

import trio

from parsec._parsec import UserStorage as _RsUserStorage
from parsec._parsec import (
    user_storage_non_speculative_init as _rs_user_storage_non_speculative_init,
)
from parsec.core.types import EntryID, LocalDevice, LocalUserManifest


async def user_storage_non_speculative_init(data_base_dir: Path, device: LocalDevice) -> None:
    # We need to shield the call to the rust function because during the call,
    # It will open a connection to the database and close it at the end.
    # And if we were cancelled we would leak a database connection.
    with trio.CancelScope(shield=True):
        return await _rs_user_storage_non_speculative_init(
            data_base_dir=data_base_dir, device=device
        )


class UserStorage:
    """Storage for the user manifest.

    Provides a synchronous interface to the user manifest as it is used very often.
    """

    def __init__(self, instance: _RsUserStorage):
        self.rs_instance = instance

    @property
    def device(self) -> LocalDevice:
        return self.rs_instance.device

    @classmethod
    @asynccontextmanager
    async def run(cls, data_base_dir: Path, device: LocalDevice) -> AsyncIterator["UserStorage"]:
        # Instantiate the user storage
        # We shield the initialization of the rust instance.
        # During the init phase we open the connections to the database in the tokio runner.
        # If at that moment we were canceled by trio, we would leak those database connections.
        with trio.CancelScope(shield=True):
            rs_instance = await _RsUserStorage.new(device, device.user_manifest_id, data_base_dir)

        try:
            self = cls(rs_instance)

            # Populate the cache with the user manifest to be able to
            # access it synchronously at all time
            await rs_instance.load_user_manifest()

            yield self
        finally:
            with trio.CancelScope(shield=True):
                # We only need to close the connection as the only operation `set_user_manifest` directly flush the manifest to the database.
                await rs_instance.close_connections()

    # Checkpoint interface

    async def get_realm_checkpoint(self) -> int:
        return await self.rs_instance.get_realm_checkpoint()

    async def update_realm_checkpoint(
        self, new_checkpoint: int, changed_vlobs: dict[EntryID, int]
    ) -> None:
        """
        Raises: Nothing !
        """
        return await self.rs_instance.update_realm_checkpoint(new_checkpoint, changed_vlobs)

    async def get_need_sync_entries(self) -> tuple[set[EntryID], set[EntryID]]:
        return await self.rs_instance.get_need_sync_entries()

    # User manifest

    def get_user_manifest(self) -> LocalUserManifest:
        """
        Raises nothing, user manifest is guaranteed to be always available
        """
        return self.rs_instance.get_user_manifest()

    async def set_user_manifest(self, user_manifest: LocalUserManifest) -> None:
        return await self.rs_instance.set_user_manifest(user_manifest)

    # No vacuuming (used in sync monitor)

    async def run_vacuum(self) -> None:
        pass
