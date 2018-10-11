from pathlib import Path

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

        # TODO: fix recursive import
        from parsec.core.encryption_manager import encrypt_with_symkey, decrypt_with_symkey

        self._encrypt_with_symkey = encrypt_with_symkey
        self._decrypt_with_symkey = decrypt_with_symkey

    @property
    def path(self):
        return str(self._path)

    def get(self, access: Access) -> None:
        file = self._path / str(access["id"])
        try:
            ciphered = file.read_bytes()
        except FileNotFoundError:
            raise LocalDBMissingEntry(access)
        return self._decrypt_with_symkey(access["key"], ciphered)

    def set(self, access: Access, raw: bytes) -> None:
        assert isinstance(raw, (bytes, bytearray))
        ciphered = self._encrypt_with_symkey(access["key"], raw)
        file = self._path / str(access["id"])
        file.write_bytes(ciphered)

    def clear(self, access: Access) -> None:
        file = self._path / str(access["id"])
        try:
            file.unlink()
        except FileNotFoundError:
            pass
