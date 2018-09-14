import os
from pathlib import Path
from shutil import rmtree

# TODO: shouldn't use core.fs.types.Acces here
from parsec.core.fs.types import Access


class LocalDBError(Exception):
    pass


class LocalDBMissingEntry(LocalDBError):
    def __init__(self, access):
        self.access = access


class LocalDB:
    def __init__(self, path):
        self._path = Path(path)
        self._path.mkdir(parents=True, exist_ok=True)
        self._cache = self._path / 'cache'
        self._cache.mkdir(parents=True, exist_ok=True)
        self._placeholders = self._path / 'placeholders'
        self._placeholders.mkdir(parents=True, exist_ok=True)

        # TODO: fix recursive import
        from parsec.core.encryption_manager import encrypt_with_symkey, decrypt_with_symkey

        self._encrypt_with_symkey = encrypt_with_symkey
        self._decrypt_with_symkey = decrypt_with_symkey

    @property
    def path(self):
        return str(self._path)

    def find(self, access):
        file = self._cache / access["id"]
        try:
            return file
        except FileNotFoundError:
            pass
        file = self._placeholders / access["id"]
        try:
            return file
        except FileNotFoundError:
            pass

    def get(self, access):
        file = self.find(access)
        try:
            ciphered = file.read_bytes()
        except FileNotFoundError:
            raise LocalDBMissingEntry(access)
        return self._decrypt_with_symkey(access["key"], ciphered)

    def set(self, access, raw: bytes, deletable=True):
        cache = str(self._cache)
        cache_size = sum(os.path.getsize(os.path.join(cache, f)) for f in os.listdir(cache) if os.path.isfile(os.path.join(cache, f)))
        if cache_size > 2097152:
            self.clean()
        assert isinstance(raw, (bytes, bytearray))
        ciphered = self._encrypt_with_symkey(access["key"], raw)
        self.clear(access)
        file = self._cache / access["id"] if deletable else self._placeholders / access["id"]
        file.write_bytes(ciphered)

    def clear(self, access):
        file = self.find(access)
        try:
            file.unlink()
        except FileNotFoundError:
            pass

    def clean(self):
        rmtree(str(self._cache))
        self._cache.mkdir(parents=True, exist_ok=True)
