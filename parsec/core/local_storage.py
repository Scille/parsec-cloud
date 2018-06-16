from pathlib import Path

from parsec.core.base import BaseAsyncComponent


class LocalStorage(BaseAsyncComponent):
    def __init__(self, path):
        super().__init__()
        self.path = Path(path)
        self._manifests_dir = self.path / "manifests"
        self._blocks_dir = self.path / "blocks"
        self._users_dir = self.path / "users"

    async def _init(self, nursery):
        self.path.mkdir(parents=True, exist_ok=True)
        self._manifests_dir.mkdir(exist_ok=True)
        self._blocks_dir.mkdir(exist_ok=True)
        self._users_dir.mkdir(exist_ok=True)

    async def _teardown(self):
        pass

    def fetch_user_manifest(self):
        return self.fetch_manifest("0")

    def flush_user_manifest(self, blob):
        self.flush_manifest("0", blob)

    def fetch_manifest(self, id):
        manifest = self._manifests_dir / id
        try:
            return manifest.read_bytes()
        except FileNotFoundError:
            return None

    def flush_manifest(self, id, blob):
        manifest = self._manifests_dir / id
        manifest.write_bytes(blob)

    def move_manifest(self, id, new_id):
        manifest = self._manifests_dir / id
        destination = self._manifests_dir / new_id
        try:
            return manifest.rename(destination)
        except FileNotFoundError:
            return None

    def remove_manifest_local_data(self, id):
        manifest = self._manifests_dir / id
        try:
            return manifest.unlink()
        except FileNotFoundError:
            return None

    def fetch_block(self, id):
        block = self._blocks_dir / id
        try:
            return block.read_bytes()
        except FileNotFoundError:
            return None

    def flush_block(self, id, blob):
        block = self._blocks_dir / id
        block.write_bytes(blob)

    def fetch_dirty_block(self, id):
        return self.fetch_block(id)

    def flush_dirty_block(self, id, blob):
        self.flush_block(id, blob)

    def fetch_user_pubkey(self, user_id):
        user = self._users_dir / user_id
        try:
            return user.read_bytes()
        except FileNotFoundError:
            return None

    def flush_user_pubkey(self, user_id, pubkey):
        user = self._users_dir / user_id
        user.write_bytes(pubkey)

    def fetch_device_verifykey(self, user_id, device_name):
        device = self._users_dir / ("%s@%s" % (user_id, device_name))
        try:
            return device.read_bytes()
        except FileNotFoundError:
            return None

    def flush_device_verifykey(self, user_id, device_name, verifykey):
        device = self._users_dir / ("%s@%s" % (user_id, device_name))
        device.write_bytes(verifykey)
