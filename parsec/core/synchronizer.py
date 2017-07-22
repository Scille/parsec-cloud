import arrow
import asyncio
import blinker
from collections import defaultdict
import sys
from uuid import uuid4

import attr
from cachetools import LRUCache
from effect2 import Effect, TypeDispatcher, do

from parsec.core.backend_vlob import EBackendVlobCreate, EBackendVlobUpdate, EBackendVlobRead
from parsec.core.backend_user_vlob import EBackendUserVlobUpdate, EBackendUserVlobRead
from parsec.core.block import EBlockCreate as EBackendBlockCreate, EBlockRead as EBackendBlockRead
from parsec.core import fs
from parsec.exceptions import BlockError, BlockNotFound, UserVlobNotFound, VlobNotFound


cache = LRUCache(maxsize=sys.maxsize)


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


class BufferedBlock:

    @classmethod
    def init(cls, content, id=None, synchronized=False):
        self = BufferedBlock()
        self.id = uuid4().hex if not id else id
        self.synchronized = synchronized
        cache[self.id] = content
        return self

    def read(self):
        content = cache[self.id]
        content = content if content else ''
        return {'id': self.id, 'content': content}

    def discard(self):
        del cache[self.id]


class BufferedUserVlob:

    @classmethod
    def init(cls, version=1, blob='', synchronized=False):
        self = BufferedUserVlob()
        self.version = version
        self.synchronized = synchronized
        cache['USER_VLOB_' + str(self.version)] = blob
        return self

    def update(self, blob):
        cache['USER_VLOB_' + str(self.version)] = blob

    def read(self):
        blob = cache['USER_VLOB_' + str(self.version)]
        blob = blob if blob else ''
        return {'blob': blob, 'version': self.version}

    def discard(self):
        del cache['USER_VLOB_' + str(self.version)]


class BufferedVlob:

    @classmethod
    def init(cls, id=None, version=1, blob='', read_trust_seed=None, write_trust_seed=None,
             synchronized=False):
        self = BufferedVlob()
        self.id = id if id else uuid4().hex
        self.read_trust_seed = read_trust_seed
        self.write_trust_seed = write_trust_seed
        self.version = version
        self.synchronized = synchronized
        cache[self.id + '_' + str(self.version)] = blob
        return self

    def update(self, blob):
        cache[self.id + '_' + str(self.version)] = blob

    def read(self):
        blob = cache[self.id + '_' + str(self.version)]
        blob = blob if blob else ''
        return {'id': self.id, 'blob': blob, 'version': self.version}

    def discard(self):
        del cache[self.id + '_' + str(self.version)]


class SynchronizerComponent:

    def __init__(self):
        self.buffered_blocks = {}
        self.buffered_vlobs = defaultdict(dict)
        self.buffered_user_vlob = {}
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
        buffered_block = BufferedBlock.init(intent.content)
        self.buffered_blocks[buffered_block.id] = buffered_block
        return buffered_block.id

    @do
    def perform_block_read(self, intent):
        try:
            return self.buffered_blocks[intent.id].read()
        except KeyError:
            try:
                block = yield Effect(EBackendBlockRead(intent.id))
                buffered_block = BufferedBlock.init(block.content, block.id, synchronized=True)
                self.buffered_blocks[buffered_block.id] = buffered_block
                return {'id': block.id, 'content': block.content}
            except (BlockNotFound, BlockError):
                raise BlockNotFound('Block not found.')

    @do
    def perform_block_delete(self, intent):
        self.last_modified = arrow.utcnow()
        try:
            self.buffered_blocks[intent.id].discard()
            del self.buffered_blocks[intent.id]
        except KeyError:
            raise BlockNotFound('Block not found.')

    @do
    def perform_block_list(self, intent):
        return sorted([block_id for block_id in list(self.buffered_blocks.keys())
                       if not self.buffered_blocks[block_id].synchronized])

    @do
    def perform_block_synchronize(self, intent):
        if intent.id in self.buffered_blocks and not self.buffered_blocks[intent.id].synchronized:
            buffered_block = self.buffered_blocks[intent.id]
            buffered_block.synchronized = True
            block_read = buffered_block.read()
            yield Effect(EBackendBlockCreate(intent.id, block_read['content']))
            return True
        return False

    @do
    def perform_user_vlob_read(self, intent):
        if intent.version:
            version = intent.version
        else:
            version = self.get_current_user_vlob_version()
        if version in self.buffered_user_vlob:
            return self.buffered_user_vlob[version].read()
        else:
            user_vlob = yield Effect(EBackendUserVlobRead(intent.version))
            buffered_user_vlob = BufferedUserVlob.init(user_vlob.version,
                                                       user_vlob.blob.decode(),
                                                       synchronized=True)
            self.buffered_user_vlob[user_vlob.version] = buffered_user_vlob
            return {'blob': user_vlob.blob.decode(), 'version': user_vlob.version}

    @do
    def perform_user_vlob_update(self, intent):
        self.last_modified = arrow.utcnow()
        if intent.version in self.buffered_user_vlob:
            assert not self.buffered_user_vlob[intent.version].synchronized
            self.buffered_user_vlob[intent.version].update(intent.blob)
        else:
            self.buffered_user_vlob[intent.version] = BufferedUserVlob.init(intent.version,
                                                                            intent.blob)

    @do
    def perform_user_vlob_delete(self, intent):
        self.last_modified = arrow.utcnow()
        if intent.version:
            version = intent.version
        else:
            version = self.get_current_user_vlob_version()
        if version in self.buffered_user_vlob:
            self.buffered_user_vlob[version].discard()
            del self.buffered_user_vlob[version]
        else:
            raise UserVlobNotFound('User vlob not found.')

    @do
    def perform_user_vlob_exist(self, intent):
        return self.get_current_user_vlob_version() is not None

    def get_current_user_vlob_version(self):
        versions = sorted(self.buffered_user_vlob.keys())
        for version in reversed(versions):
            if not self.buffered_user_vlob[version].synchronized:
                return version

    @do
    def perform_user_vlob_synchronize(self, intent):
        current_version = self.get_current_user_vlob_version()
        if current_version:
            user_vlob = yield self.perform_user_vlob_read(EUserVlobRead())
            self.buffered_user_vlob[current_version].synchronized = True
            yield Effect(EBackendUserVlobUpdate(current_version,
                                                user_vlob['blob'].encode()))
            return True
        return False

    @do
    def perform_vlob_create(self, intent):
        self.last_modified = arrow.utcnow()
        buffered_vlob = BufferedVlob.init(blob=intent.blob,
                                          read_trust_seed='42',
                                          write_trust_seed='42')
        self.buffered_vlobs[buffered_vlob.id][1] = buffered_vlob
        return {'id': buffered_vlob.id,
                'read_trust_seed': buffered_vlob.read_trust_seed,
                'write_trust_seed': buffered_vlob.write_trust_seed}

    @do
    def perform_vlob_read(self, intent):
        if intent.version:
            version = intent.version
        else:
            version = self.get_current_vlob_version(intent.id)
        if version in self.buffered_vlobs[intent.id]:
            if self.buffered_vlobs[intent.id][version].read_trust_seed == '42':
                self.buffered_vlobs[intent.id][version].read_trust_seed = intent.trust_seed
            assert (intent.trust_seed ==
                    self.buffered_vlobs[intent.id][version].read_trust_seed)
            return self.buffered_vlobs[intent.id][version].read()
        else:
            vlob = yield Effect(EBackendVlobRead(intent.id, intent.trust_seed, version))
            buffered_vlob = BufferedVlob.init(vlob.id,
                                              vlob.version,
                                              vlob.blob.decode(),
                                              intent.trust_seed,
                                              '42',
                                              synchronized=True)
            self.buffered_vlobs[vlob.id][vlob.version] = buffered_vlob
            return {'id': vlob.id, 'blob': vlob.blob.decode(), 'version': vlob.version}

    @do
    def perform_vlob_update(self, intent):
        self.last_modified = arrow.utcnow()
        try:
            assert intent.version == self.buffered_vlobs[intent.id][intent.version].version
            self.buffered_vlobs[intent.id][intent.version].write_trust_seed = intent.trust_seed
            assert (intent.trust_seed ==
                    self.buffered_vlobs[intent.id][intent.version].write_trust_seed)
            self.buffered_vlobs[intent.id][intent.version].update(intent.blob)
        except KeyError:
            self.buffered_vlobs[intent.id][intent.version] = BufferedVlob.init(intent.id,
                                                                               intent.version,
                                                                               intent.blob,
                                                                               '42',
                                                                               intent.trust_seed)

    @do
    def perform_vlob_delete(self, intent):
        self.last_modified = arrow.utcnow()
        if intent.version:
            version = intent.version
        else:
            version = self.get_current_vlob_version(intent.id)
        try:
            self.buffered_vlobs[intent.id][version].discard()
            del self.buffered_vlobs[intent.id][version]
            if not self.buffered_vlobs[intent.id]:
                del self.buffered_vlobs[intent.id]
        except KeyError:
            raise VlobNotFound('Vlob not found.')

    @do
    def perform_vlob_list(self, intent):
        vlob_list = []
        for vlob_id in self.buffered_vlobs:
            current_version = self.get_current_vlob_version(vlob_id)
            if current_version:
                vlob_list.append(vlob_id)
        return sorted(vlob_list)

    def get_current_vlob_version(self, vlob_id):
        if vlob_id not in self.buffered_vlobs:
            raise VlobNotFound('Vlob not found.')
        versions = sorted(self.buffered_vlobs[vlob_id].keys())
        for version in reversed(versions):
            if not self.buffered_vlobs[vlob_id][version].synchronized:
                return version

    @do
    def perform_vlob_synchronize(self, intent):
        if self.buffered_vlobs[intent.id]:
            current_version = self.get_current_vlob_version(intent.id)
            if not current_version:
                return False
            buffered_vlob = self.buffered_vlobs[intent.id][current_version]
            buffered_vlob.synchronized = True
            vlob_read = buffered_vlob.read()
            new_vlob = None
            if current_version == 1:
                new_vlob = yield Effect(EBackendVlobCreate(vlob_read['blob'].encode()))
                buffered_vlob.id = new_vlob.id
                buffered_vlob.read_trust_seed = new_vlob.read_trust_seed
                buffered_vlob.write_trust_seed = new_vlob.write_trust_seed
            else:
                yield Effect(EBackendVlobUpdate(
                    intent.id,
                    self.buffered_vlobs[intent.id][current_version].write_trust_seed,
                    self.buffered_vlobs[intent.id][current_version].version,
                    vlob_read['blob'].encode()))  # TODO encode is correct?
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
        deleted_blocks = []
        deleted_vlobs = []
        deleted_user_vlobs = []
        for block in self.buffered_blocks:
            if self.buffered_blocks[block].synchronized:
                deleted_blocks.append(block)
        for vlob in self.buffered_vlobs:
            current_version = self.get_current_vlob_version(vlob)
            for version in self.buffered_vlobs[vlob]:
                if version != current_version:
                    deleted_vlobs.append((vlob, version))
        current_version = self.get_current_user_vlob_version()
        for version in self.buffered_user_vlob:
            if version != current_version:
                deleted_user_vlobs.append(version)
        for block in deleted_blocks:
            yield self.perform_block_delete(EBlockDelete(block))
        for vlob, version in deleted_vlobs:
            yield self.perform_vlob_delete(EVlobDelete(vlob, version))
        for version in deleted_user_vlobs:
            yield self.perform_user_vlob_delete(EUserVlobDelete(version))

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
