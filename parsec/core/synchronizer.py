import arrow
import asyncio
import blinker
from uuid import uuid4

import attr
from cachetools import LRUCache
from effect2 import Effect, TypeDispatcher, do

from parsec.core.backend_vlob import EBackendVlobCreate, EBackendVlobUpdate, EBackendVlobRead
from parsec.core.backend_user_vlob import EBackendUserVlobUpdate, EBackendUserVlobRead
from parsec.core.block import EBlockCreate as EBackendBlockCreate, EBlockRead as EBackendBlockRead
from parsec.core import fs
from parsec.exceptions import BlockError, BlockNotFound, UserVlobNotFound, VlobNotFound


@attr.s
class EBlockCreate:
    content = attr.ib()


@attr.s
class EBlockRead:
    id = attr.ib()


@attr.s
class EBlockDelete:
    id = attr.ib()


@attr.s
class EBlockList:
    pass


@attr.s
class EBlockSynchronize:
    id = attr.ib()


@attr.s
class EUserVlobRead:
    version = attr.ib(default=None)


@attr.s
class EUserVlobUpdate:
    version = attr.ib()
    blob = attr.ib(default='')


@attr.s
class EUserVlobExist:
    pass


@attr.s
class EUserVlobDelete:
    version = attr.ib(default=None)


@attr.s
class EUserVlobSynchronize:
    pass


@attr.s
class EVlobCreate:
    blob = attr.ib(default='')


@attr.s
class EVlobRead:
    id = attr.ib()
    trust_seed = attr.ib()
    version = attr.ib(default=None)


@attr.s
class EVlobUpdate:
    id = attr.ib()
    trust_seed = attr.ib()
    version = attr.ib()
    blob = attr.ib()


@attr.s
class EVlobDelete:
    id = attr.ib()
    version = attr.ib(default=None)


@attr.s
class EVlobList:
    pass


@attr.s
class EVlobSynchronize:
    id = attr.ib()


@attr.s
class ESynchronize:
    pass


@attr.s
class ECacheClean:
    pass


class SynchronizerComponent:

    def __init__(self, cache_size):
        self.block_cache = LRUCache(maxsize=cache_size)
        self.user_vlob_cache = LRUCache(maxsize=cache_size)
        self.vlob_cache = LRUCache(maxsize=cache_size)
        self.blocks = {}
        self.vlobs = {}
        self.user_vlob = None
        self.synchronization_idle_interval = 10
        self.last_modified = arrow.utcnow()
        blinker.signal('app_start').connect(self.on_app_start)
        blinker.signal('app_stop').connect(self.on_app_stop)

    def on_app_start(self, app):
        self.synchronization_task = asyncio.ensure_future(self.periodic_synchronization(app))

    def on_app_stop(self, app):
        self.synchronization_task.cancel()
        self.synchronization_task = None

    @do
    def perform_block_create(self, intent):
        self.last_modified = arrow.utcnow()
        block_id = uuid4().hex
        self.blocks[block_id] = {'id': block_id, 'content': intent.content}
        return block_id

    @do
    def perform_block_read(self, intent):
        try:
            return self.blocks[intent.id]
        except KeyError:
            try:
                return self.block_cache[intent.id]
            except KeyError:
                try:
                    block = yield Effect(EBackendBlockRead(intent.id))
                    block = {'id': block.id, 'content': block.content}
                except (BlockNotFound, BlockError):
                    raise BlockNotFound('Block not found.')
                try:
                    self.block_cache[intent.id] = block
                except ValueError:
                    pass  # Value too large if cache is disabled
                return block

    @do
    def perform_block_delete(self, intent):
        self.last_modified = arrow.utcnow()
        try:
            del self.blocks[intent.id]
        except KeyError:
            try:
                del self.block_cache[intent.id]
            except KeyError:
                raise BlockNotFound('Block not found.')

    @do
    def perform_block_list(self, intent):
        return sorted([block_id for block_id in list(self.blocks.keys())])

    @do
    def perform_block_synchronize(self, intent):
        if intent.id in self.blocks:
            block = self.blocks[intent.id]
            yield Effect(EBackendBlockCreate(intent.id, block['content']))
            try:
                self.block_cache[intent.id] = block
            except ValueError:
                pass  # Value too large if cache is disabled
            del self.blocks[intent.id]
            return True
        return False

    @do
    def perform_user_vlob_read(self, intent):
        if self.user_vlob and (not intent.version or intent.version == self.user_vlob['version']):
            return self.user_vlob
        else:
            try:
                return self.user_vlob_cache[intent.version]
            except KeyError:
                user_vlob = yield Effect(EBackendUserVlobRead(intent.version))
                user_vlob = {'blob': user_vlob.blob.decode(), 'version': user_vlob.version}
                try:
                    self.user_vlob_cache[user_vlob['version']] = user_vlob
                except ValueError:
                    pass  # Value too large if cache is disabled
                return user_vlob

    @do
    def perform_user_vlob_update(self, intent):
        self.last_modified = arrow.utcnow()
        self.user_vlob = {'blob': intent.blob, 'version': intent.version}

    @do
    def perform_user_vlob_delete(self, intent):
        self.last_modified = arrow.utcnow()
        if self.user_vlob and (not intent.version or intent.version == self.user_vlob['version']):
            self.user_vlob = None
        else:
            try:
                del self.user_vlob_cache[intent.version]
            except KeyError:
                raise UserVlobNotFound('User vlob not found.')

    @do
    def perform_user_vlob_exist(self, intent):
        return self.user_vlob is not None

    @do
    def perform_user_vlob_synchronize(self, intent):
        if self.user_vlob:
            yield Effect(EBackendUserVlobUpdate(self.user_vlob['version'],
                                                self.user_vlob['blob'].encode()))
            try:
                self.user_vlob_cache[self.user_vlob['version']] = self.user_vlob
            except ValueError:
                pass  # Value too large if cache is disabled
            self.user_vlob = None
            return True
        return False

    @do
    def perform_vlob_create(self, intent):
        self.last_modified = arrow.utcnow()
        vlob_id = uuid4().hex
        self.vlobs[vlob_id] = {'id': vlob_id,
                               'read_trust_seed': '42',
                               'write_trust_seed': '42',
                               'version': 1,
                               'blob': intent.blob}
        return {'id': vlob_id,
                'read_trust_seed': '42',
                'write_trust_seed': '42'}

    @do
    def perform_vlob_read(self, intent):
        if (intent.id in self.vlobs and
                (not intent.version or intent.version == self.vlobs[intent.id]['version'])):
            if self.vlobs[intent.id]['read_trust_seed'] == '42':
                self.vlobs[intent.id]['read_trust_seed'] = intent.trust_seed
            assert intent.trust_seed == self.vlobs[intent.id]['read_trust_seed']
            return {'id': intent.id,
                    'blob': self.vlobs[intent.id]['blob'],
                    'version': self.vlobs[intent.id]['version']}
        else:
            # TODO: if intent.version == None this is buggy !
            try:
                cached_vlob = self.vlob_cache[(intent.id, intent.version)]
                assert intent.trust_seed == cached_vlob['read_trust_seed']
                vlob = {'id': intent.id, 'blob': cached_vlob['blob'], 'version': intent.version}
            except KeyError:
                vlob = yield Effect(EBackendVlobRead(intent.id, intent.trust_seed, intent.version))
                vlob = {'id': vlob.id, 'blob': vlob.blob.decode(), 'version': vlob.version}
                try:
                    cached_vlob = {'id': intent.id,
                                   'read_trust_seed': intent.trust_seed,
                                   'version': intent.version,
                                   'blob': vlob['blob']}
                    self.vlob_cache[(intent.id, intent.version)] = cached_vlob
                except ValueError:
                    pass  # Value too large if cache is disabled
            return vlob

    @do
    def perform_vlob_update(self, intent):
        self.last_modified = arrow.utcnow()
        self.vlobs[intent.id] = {'id': intent.id,
                                 'read_trust_seed': '42',
                                 'write_trust_seed': intent.trust_seed,
                                 'version': intent.version,
                                 'blob': intent.blob}

    @do
    def perform_vlob_delete(self, intent):
        self.last_modified = arrow.utcnow()
        if (intent.id in self.vlobs and
                (not intent.version or intent.version == self.vlobs[intent.id]['version'])):
            del self.vlobs[intent.id]
        else:
            try:
                del self.vlob_cache[(intent.id, intent.version)]
            except KeyError:
                raise VlobNotFound('Vlob not found.')

    @do
    def perform_vlob_list(self, intent):
        return sorted(self.vlobs.keys())

    @do
    def perform_vlob_synchronize(self, intent):
        if intent.id in self.vlobs:
            vlob = self.vlobs[intent.id]
            try:
                self.vlob_cache[(intent.id, vlob['version'])] = vlob
            except ValueError:
                pass  # Value too large if cache is disabled
            new_vlob = None
            if vlob['version'] == 1:
                new_vlob = yield Effect(EBackendVlobCreate(vlob['blob'].encode()))
                new_trust_seed = new_vlob.read_trust_seed
                try:
                    self.vlob_cache[(intent.id, vlob['version'])]['read_trust_seed'] = new_trust_seed
                except KeyError:
                    pass
            else:
                yield Effect(EBackendVlobUpdate(
                    intent.id,
                    self.vlobs[intent.id]['write_trust_seed'],
                    self.vlobs[intent.id]['version'],
                    vlob['blob'].encode()))  # TODO encode is correct?
            del self.vlobs[intent.id]
            if new_vlob:
                return {'id': new_vlob.id,
                        'read_trust_seed': new_vlob.read_trust_seed,
                        'write_trust_seed': new_vlob.write_trust_seed}
            else:
                return True
        return False

    @do
    def perform_synchronize(self, intent):
        # TODO dangerous method: new vlobs are not updated in manifest. Remove it?
        synchronization = False
        block_list = yield self.perform_block_list(EBlockList())
        for block_id in block_list:
            synchronization |= yield self.perform_block_synchronize(EBlockSynchronize(block_id))
        vlob_list = yield self.perform_vlob_list(EVlobList())
        for vlob_id in vlob_list:
            new_vlob = yield self.perform_vlob_synchronize(EVlobSynchronize(vlob_id))
            if new_vlob:
                synchronization |= True
        synchronization |= yield self.perform_user_vlob_synchronize(EUserVlobSynchronize())
        return synchronization

    @do
    def perform_cache_clean(self, intent):
        for item in list(self.block_cache.keys()):
            del self.block_cache[item]
        for item in list(self.user_vlob_cache.keys()):
            del self.user_vlob_cache[item]
        for item in list(self.vlob_cache.keys()):
            del self.vlob_cache[item]

    async def periodic_synchronization(self, app):
        while True:
            await asyncio.sleep(self.synchronization_idle_interval)
            if (arrow.utcnow().timestamp - self.last_modified.timestamp >
                    self.synchronization_idle_interval):
                await app.async_perform(Effect(fs.ESynchronize()))

    def get_dispatcher(self):
        return TypeDispatcher({
            EBlockCreate: self.perform_block_create,
            EBlockRead: self.perform_block_read,
            EBlockDelete: self.perform_block_delete,
            EBlockList: self.perform_block_list,
            EBlockSynchronize: self.perform_block_synchronize,
            EUserVlobRead: self.perform_user_vlob_read,
            EUserVlobUpdate: self.perform_user_vlob_update,
            EUserVlobDelete: self.perform_user_vlob_delete,
            EUserVlobExist: self.perform_user_vlob_exist,
            EUserVlobSynchronize: self.perform_user_vlob_synchronize,
            EVlobCreate: self.perform_vlob_create,
            EVlobRead: self.perform_vlob_read,
            EVlobUpdate: self.perform_vlob_update,
            EVlobDelete: self.perform_vlob_delete,
            EVlobList: self.perform_vlob_list,
            EVlobSynchronize: self.perform_vlob_synchronize,
            ESynchronize: self.perform_synchronize
        })
