import os
from pathlib import Path
from shutil import rmtree

from parsec.crypto import encrypt_raw_with_secret_key, decrypt_raw_with_secret_key

# TODO: shouldn't use core.fs.types.Acces here
# from parsec.core.fs.types import Access
Access = None  # TODO: hack to fix recursive import


# TODO: should be in config.py
DEFAULT_MAX_CACHE_SIZE = 128 * 1024 * 1024


class LocalDBError(Exception):
    pass


class LocalDBMissingEntry(LocalDBError):
    def __init__(self, access):
        self.access = access


class LocalDB:
    def __init__(self, path: Path, max_cache_size: int = DEFAULT_MAX_CACHE_SIZE):
        self._path = Path(path)
        self.max_cache_size = max_cache_size
        self._path.mkdir(parents=True, exist_ok=True)
        self._cache = self._path / "cache"
        self._cache.mkdir(parents=True, exist_ok=True)
        self._placeholders = self._path / "placeholders"
        self._placeholders.mkdir(parents=True, exist_ok=True)

    @property
    def path(self):
        return str(self._path)

    def get_cache_size(self):
        cache = str(self._cache)
        return sum(
            os.path.getsize(os.path.join(cache, f))
            for f in os.listdir(cache)
            if os.path.isfile(os.path.join(cache, f))
        )

    def _find(self, access):
        try:
            return next(
                (
                    directory / str(access["id"])
                    for directory in [self._cache, self._placeholders]
                    if (directory / str(access["id"])).exists()
                )
            )
        except StopIteration:
            pass

    def get(self, access: Access):
        file = self._find(access)
        if not file:
            raise LocalDBMissingEntry(access)
        ciphered = file.read_bytes()
        return decrypt_raw_with_secret_key(access["key"], ciphered)

    def set(self, access: Access, raw: bytes, deletable: bool = True):
        assert isinstance(raw, (bytes, bytearray))

        ciphered = encrypt_raw_with_secret_key(access["key"], raw)

        if deletable:
            if self.get_cache_size() + len(ciphered) > self.max_cache_size:
                self.run_garbage_collector()

        try:
            self.clear(access)
        except LocalDBMissingEntry:
            pass
        file = (
            self._cache / str(access["id"]) if deletable else self._placeholders / str(access["id"])
        )
        file.write_bytes(ciphered)

    def clear(self, access: Access):
        file = self._find(access)
        if not file:
            raise LocalDBMissingEntry(access)
        file.unlink()

    def run_garbage_collector(self):
        # TODO: really quick'n dirty GC...
        rmtree(str(self._cache))
        self._cache.mkdir(parents=True, exist_ok=True)
