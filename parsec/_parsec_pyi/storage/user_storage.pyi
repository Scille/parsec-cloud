from __future__ import annotations

from pathlib import Path

from parsec._parsec_pyi.ids import EntryID
from parsec._parsec_pyi.local_device import LocalDevice
from parsec._parsec_pyi.local_manifest import LocalUserManifest

class UserStorage:
    @staticmethod
    async def new(
        device: LocalDevice, user_manifest_id: EntryID, data_base_dir: Path
    ) -> UserStorage: ...
    async def close_connections(self) -> None: ...
    async def get_realm_checkpoint(self) -> int: ...
    async def update_realm_checkpoint(
        self, new_checkpoint: int, changed_vlobs: dict[EntryID, int]
    ) -> None: ...
    async def get_need_sync_entries(self) -> tuple[set[EntryID], set[EntryID]]: ...
    def get_user_manifest(self) -> LocalUserManifest: ...
    async def load_user_manifest(self) -> None: ...
    async def set_user_manifest(self, user_manifest: LocalUserManifest) -> None: ...
    @property
    def device(self) -> LocalDevice: ...

async def user_storage_non_speculative_init(data_base_dir: Path, device: LocalDevice) -> None: ...
