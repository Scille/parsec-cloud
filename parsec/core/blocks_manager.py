from nacl.secret import SecretBox

from parsec.core.base import Interface, IAsyncComponent, implements


class IBlocksManager(Interface):

    async def fetch_from_local(self, id, key):
        pass

    async def fetch_from_backend(self, id, key):
        pass

    async def flush_on_local(self, id, key, block):
        pass

    async def sync_new_block_with_backend(self, key, block):
        pass


class BlocksManager(implements(IAsyncComponent, IBlocksManager)):

    def __init__(self, local_storage, backend_storage):
        self.local_storage = local_storage
        self.backend_storage = backend_storage

    async def init(self, nursery):
        pass

    async def teardown(self):
        pass

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
        crypted = await self.backend_storage.fetch_block(id)
        if crypted:
            return self._decrypt_block(key, crypted)

    async def flush_on_local(self, id, key, block):
        crypted = self._encrypt_block(key, block)
        self.local_storage.flush_block(id, crypted)

    async def sync_new_block_with_backend(self, key, block):
        crypted = self._encrypt_block(key, block)
        return await self.backend_storage.sync_new_block(crypted)
