from nacl.secret import SecretBox

from parsec.core.base import BaseAsyncComponent


class BlocksManager(BaseAsyncComponent):
    def __init__(self, local_storage, backend_storage):
        super().__init__()
        self.local_storage = local_storage
        self.backend_storage = backend_storage

    async def _init(self, nursery):
        pass

    async def _teardown(self):
        pass

    def _encrypt_block(self, key, block):
        # TODO: handle block size padding here
        box = SecretBox(key)
        # signed = self.user.signkey.sign(raw)
        # return box.encrypt(signed)
        return box.encrypt(block)

    def _decrypt_block(self, key, blob):
        box = SecretBox(key)
        return box.decrypt(blob)

    # signed = box.decrypt(blob)
    # raw = self.user.verifykey.verify(signed)

    def fetch_from_local(self, id, key):
        crypted = self.local_storage.fetch_dirty_block(id)
        if not crypted:
            crypted = self.local_storage.fetch_block(id)
        if crypted:
            return self._decrypt_block(key, crypted)

    async def fetch_from_backend(self, id, key):
        # TODO: add local cache
        crypted = await self.backend_storage.fetch_block(id)
        if crypted:
            return self._decrypt_block(key, crypted)

    def flush_on_local(self, id, key, block):
        crypted = self._encrypt_block(key, block)
        self.local_storage.flush_block(id, crypted)

    async def sync_new_block_with_backend(self, key, block):
        # TODO: add local cache
        crypted = self._encrypt_block(key, block)
        return await self.backend_storage.sync_new_block(crypted)
