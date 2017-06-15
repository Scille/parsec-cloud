from datetime import datetime
from uuid import uuid4

from parsec.core.cache import cache
from parsec.exceptions import BlockError, BlockNotFound, UserVlobNotFound, VlobNotFound
from parsec.service import BaseService, service


class BufferedBlock:

    @classmethod
    async def init(cls, content):
        self = BufferedBlock()
        self.id = uuid4().hex
        self.date = datetime.utcnow().isoformat()
        await cache.set(self.id, content)
        return self

    async def read(self):
        content = await cache.get(self.id)
        content = content if content else ''
        return {
            'content': content,
            'creation_date': self.date
        }

    async def stat(self):
        return {
            'creation_date': self.date
        }

    async def discard(self):
        await cache.delete(self.id)


class BufferedUserVlob:

    @classmethod
    async def init(cls, version=1, blob=''):
        self = BufferedUserVlob()
        self.version = version
        await cache.set('USER_VLOB', blob)
        return self

    async def update(self, blob):
        await cache.set('USER_VLOB', blob)

    async def read(self):
        blob = await cache.get('USER_VLOB')
        blob = blob if blob else ''
        return {
            'blob': blob,
            'version': self.version
        }

    async def discard(self):
        await cache.delete('USER_VLOB')


class BufferedVlob:

    @classmethod
    async def init(cls, id=None, version=1, blob='', read_trust_seed=None, write_trust_seed=None):
        self = BufferedVlob()
        self.id = id if id else uuid4().hex
        self.read_trust_seed = read_trust_seed
        self.write_trust_seed = write_trust_seed
        self.version = version
        await cache.set(self.id, blob)
        return self

    async def update(self, blob):
        await cache.set(self.id, blob)

    async def read(self):
        blob = await cache.get(self.id)
        blob = blob if blob else ''
        return {
            'id': self.id,
            'blob': blob,
            'version': self.version
        }

    async def discard(self):
        await cache.delete(self.id)


class SynchronizerService(BaseService):

    name = 'SynchronizerService'

    backend = service('BackendAPIService')
    block = service('BlockService')

    def __init__(self):
        super().__init__()
        self.buffered_blocks = {}
        self.buffered_vlobs = {}
        self.buffered_user_vlob = None

    async def block_create(self, content):
        buffered_block = await BufferedBlock.init(content)
        self.buffered_blocks[buffered_block.id] = buffered_block
        return buffered_block.id

    async def block_read(self, id):
        try:
            return await self.buffered_blocks[id].read()
        except KeyError:
            try:
                return await self.block.read(id)
            except (BlockNotFound, BlockError):
                raise BlockNotFound('Block not found.')

    async def block_stat(self, id):
        try:
            return await self.buffered_blocks[id].stat()
        except KeyError:
            return await self.block.stat(id)

    async def block_delete(self, id):
        try:
            await self.buffered_blocks[id].discard()
            del self.buffered_blocks[id]
        except KeyError:
            raise BlockNotFound('Block not found.')

    async def block_list(self):
        return list(self.buffered_blocks.keys())

    async def block_synchronize(self, id):
        if id in self.buffered_blocks:
            block = await self.buffered_blocks[id].read()
            await self.block.create(block['content'], id)
            await self.block_delete(id)
            return True
        return False

    async def user_vlob_read(self, version=None):
        if self.buffered_user_vlob and (not version or version == self.buffered_user_vlob.version):
            user_vlob = await self.buffered_user_vlob.read()
        else:
            user_vlob = await self.backend.user_vlob_read(version)
            del user_vlob['status']
        return user_vlob

    async def user_vlob_update(self, version, blob=''):
        if self.buffered_user_vlob:
            assert version == self.buffered_user_vlob.version
            await self.buffered_user_vlob.update(blob)
        else:
            self.buffered_user_vlob = await BufferedUserVlob.init(version, blob)

    async def user_vlob(self):
        return self.buffered_user_vlob is not None

    async def user_vlob_delete(self):
        if self.buffered_user_vlob:
            await self.buffered_user_vlob.discard()
            self.buffered_user_vlob = None
        else:
            raise UserVlobNotFound('User vlob not found.')

    async def user_vlob_synchronize(self):
        if self.buffered_user_vlob:
            user_vlob = await self.user_vlob_read()
            await self.backend.user_vlob_update(self.buffered_user_vlob.version, user_vlob['blob'])
            await self.user_vlob_delete()
            return True
        return False

    async def vlob_create(self, blob=''):
        buffered_vlob = await BufferedVlob.init(blob=blob,
                                                read_trust_seed='42',
                                                write_trust_seed='42')
        self.buffered_vlobs[buffered_vlob.id] = buffered_vlob
        return {'id': buffered_vlob.id,
                'read_trust_seed': buffered_vlob.read_trust_seed,
                'write_trust_seed': buffered_vlob.write_trust_seed}

    async def vlob_read(self, id, trust_seed, version=None):
        if (id in self.buffered_vlobs and
                (not version or version == self.buffered_vlobs[id].version)):
            if self.buffered_vlobs[id].read_trust_seed == '42':
                self.buffered_vlobs[id].read_trust_seed = trust_seed
            assert trust_seed == self.buffered_vlobs[id].read_trust_seed
            vlob = await self.buffered_vlobs[id].read()
        else:
            vlob = await self.backend.vlob_read(id, trust_seed, version)
            del vlob['status']
        return vlob

    async def vlob_update(self, id, version, trust_seed, blob=''):
        try:
            assert version == self.buffered_vlobs[id].version
            assert trust_seed == self.buffered_vlobs[id].write_trust_seed
            await self.buffered_vlobs[id].update(blob)
        except KeyError:
            self.buffered_vlobs[id] = await BufferedVlob.init(id, version, blob, '42', trust_seed)

    async def vlob_delete(self, id):
        try:
            await self.buffered_vlobs[id].discard()
            del self.buffered_vlobs[id]
        except KeyError:
            raise VlobNotFound('Vlob not found.')

    async def vlob_list(self):
        return list(self.buffered_vlobs.keys())

    async def vlob_synchronize(self, id):
        if id in self.buffered_vlobs:
            vlob = await self.buffered_vlobs[id].read()
            if self.buffered_vlobs[id].version == 1:
                new_vlob = await self.backend.vlob_create(vlob['blob'])
                del new_vlob['status']
            else:
                await self.backend.vlob_update(id,
                                               self.buffered_vlobs[id].version,
                                               self.buffered_vlobs[id].write_trust_seed,
                                               vlob['blob'])
            await self.vlob_delete(id)
            if vlob['version'] == 1:
                return new_vlob
            return True
        return False

    async def synchronize(self):
        for vlob_id in await self.vlob_list():
            await self.vlob_synchronize(vlob_id)
        for block_id in await self.block_list():
            await self.block_synchronize(block_id)
        await self.user_vlob_synchronize()

    async def periodic_synchronization(self):
        # TODO
        pass
