from uuid import uuid4
from nacl.public import Box
from nacl.secret import SecretBox
from nacl.signing import SigningKey
from nacl.exceptions import BadSignatureError, CryptoError


class BaseBlocksManager:
    async def fetch_from_local(self, id, key):
        raise NotImplementedError()

    async def fetch_from_backend(self, id, key):
        raise NotImplementedError()

    async def flush_on_local(self, id, key, block):
        raise NotImplementedError()

    async def sync_new_block_with_backend(self, key, block):
        raise NotImplementedError()


class BlocksManager:
    def __init__(self, local_storage, backend_storage):
        self.local_storage = local_storage
        self.backend_storage = backend_storage

    def _encrypt_block(self, key, block):
        box = SecretBox(key)
        # signed = self.user.signkey.sign(raw)
        # return box.encrypt(signed)
        return box.encrypt(block)

    def _decrypt_block(self, key, blob):
        box = SecretBox(key)
        return box.decrypt(blob)
        # signed = box.decrypt(blob)
        # raw = self.user.verifykey.verify(signed)

    async def fetch_from_local(self, id, key):
        crypted = self.local_storage.fetch_dirty_block(id)
        if not crypted:
            crypted = self.local_storage.fetch_block(id)
        if crypted:
            return self._decrypt_block(key, crypted)

    async def fetch_from_backend(self, id, key):
        crypted = self.backend_storage.fetch_block(id)
        if crypted:
            return self._decrypt_block(key, crypted)

    async def flush_on_local(self, id, key, block):
        crypted = self._encrypt_block(key, block)
        self.local_storage.flush_block(id, crypted)

    async def sync_new_block_with_backend(self, key, block):
        crypted = self._encrypt_block(key, block)
        return self.backend_storage.sync_new_block(crypted)
