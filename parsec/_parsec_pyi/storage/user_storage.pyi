from pathlib import Path

from parsec._parsec_pyi.ids import EntryID
from parsec._parsec_pyi.local_device import LocalDevice
from parsec._parsec_pyi.local_manifest import LocalUserManifest

class UserStorage:
    def __init__(
        self, device: LocalDevice, user_manifest_id: EntryID, data_base_dir: Path
    ) -> None: ...
    def close_connections(self) -> None: ...
    def get_realm_checkpoint(self) -> int: ...
    def update_realm_checkpoint(
        self, new_checkpoint: int, changed_vlobs: dict[EntryID, int]
    ) -> None: ...
    def get_need_sync_entries(self) -> tuple[set[EntryID], set[EntryID]]: ...
    def get_user_manifest(self) -> LocalUserManifest: ...
    def load_user_manifest(self) -> None: ...
    def set_user_manifest(self, user_manifest: LocalUserManifest) -> None: ...
    @property
    def device(self) -> LocalDevice: ...

def user_storage_non_speculative_init(data_base_dir: Path, device: LocalDevice) -> None: ...
