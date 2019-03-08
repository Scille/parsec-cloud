import hashlib

from structlog import get_logger

from parsec.types import UserID
from parsec.core.types import (
    Access,
    LocalManifest,
    local_manifest_serializer,
    remote_user_serializer,
    BlockAccess,
    ManifestAccess,
)
from .persistent_storage import PersistentStorage, LocalStorageMissingEntry

logger = get_logger()


class LocalStorage:
    """Manage the access to the local storage.

    That includes:
    - a cache in memory for fast access to deserialized data
    - the persistent storage to keep serialized data on the disk
    """

    def __init__(self, path, **kwargs):
        self.local_symkey = None
        self.manifest_cache = {}
        self.persistent_storage = PersistentStorage(path, **kwargs)

    def __enter__(self):
        self.persistent_storage.__enter__()
        return self

    def __exit__(self, *args):
        self.persistent_storage.__exit__(*args)

    # User interface

    def _build_remote_user_local_access(self, user_id: UserID) -> ManifestAccess:
        return ManifestAccess(
            id=hashlib.sha256(user_id.encode("utf8")).hexdigest(), key=self.local_symkey
        )

    def get_user(self, user_id):
        access = self._build_remote_user_local_access(user_id)
        raw_user_data = self.persistent_storage.get_user(access)
        return remote_user_serializer.loads(raw_user_data)

    def set_user(self, user_id, user):
        access = self._build_remote_user_local_access(user_id)
        raw = remote_user_serializer.dumps(user)
        self.persistent_storage.set_user(access, raw)
