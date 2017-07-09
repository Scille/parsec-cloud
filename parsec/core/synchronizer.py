from uuid import uuid4

import attr
from effect import TypeDispatcher
from effect2 import Effect, do

from parsec.core.cache import cache
from parsec.core.backend_vlob import EBackendVlobCreate, EBackendVlobUpdate, EBackendVlobRead
from parsec.core.backend_user_vlob import EBackendUserVlobUpdate, EBackendUserVlobRead
from parsec.core.block import EBlockCreate as EBackendBlockCreate, EBlockRead as EBackendBlockRead
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
    pass


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


@attr.s
class EVlobList:
    pass


@attr.s
class EVlobSynchronize:
    id = attr.ib()


@attr.s
class ESynchronize:
    pass


class BufferedBlock:

    @classmethod
    def init(cls, content):
        self = BufferedBlock()
        self.id = uuid4().hex
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
    def init(cls, version=1, blob=''):
        self = BufferedUserVlob()
        self.version = version
        cache['USER_VLOB'] = blob
        return self

    def update(self, blob):
        cache['USER_VLOB'] = blob

    def read(self):
        blob = cache['USER_VLOB']
        blob = blob if blob else ''
        return {'blob': blob, 'version': self.version}

    def discard(self):
        del cache['USER_VLOB']


class BufferedVlob:

    @classmethod
    def init(cls, id=None, version=1, blob='', read_trust_seed=None, write_trust_seed=None):
        self = BufferedVlob()
        self.id = id if id else uuid4().hex
        self.read_trust_seed = read_trust_seed
        self.write_trust_seed = write_trust_seed
        self.version = version
        cache[self.id] = blob
        return self

    def update(self, blob):
        cache[self.id] = blob

    def read(self):
        blob = cache[self.id]
        blob = blob if blob else ''
        return {'id': self.id, 'blob': blob, 'version': self.version}

    def discard(self):
        del cache[self.id]


class SynchronizerComponent:

    def __init__(self):
        self.buffered_blocks = {}
        self.buffered_vlobs = {}
        self.buffered_user_vlob = None

    @do
    def perform_block_create(self, intent):
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
                return {'id': block.id, 'content': block.content}
            except (BlockNotFound, BlockError):
                raise BlockNotFound('Block not found.')

    @do
    def perform_block_delete(self, intent):
        try:
            self.buffered_blocks[intent.id].discard()
            del self.buffered_blocks[intent.id]
        except KeyError:
            raise BlockNotFound('Block not found.')

    @do
    def perform_block_list(self, intent):
        return sorted(list(self.buffered_blocks.keys()))

    @do
    def perform_block_synchronize(self, intent):
        if intent.id in self.buffered_blocks:
            block = self.buffered_blocks[intent.id].read()
            yield Effect(EBackendBlockCreate(intent.id, block['content']))
            yield self.perform_block_delete(EBlockDelete(intent.id))
            return True
        return False

    @do
    def perform_user_vlob_read(self, intent):
        if (self.buffered_user_vlob and
                (not intent.version or intent.version == self.buffered_user_vlob.version)):
            return self.buffered_user_vlob.read()
        else:
            user_vlob = yield Effect(EBackendUserVlobRead(intent.version))
            return {'blob': user_vlob.blob.decode(), 'version': user_vlob.version}

    @do
    def perform_user_vlob_update(self, intent):
        if self.buffered_user_vlob:
            assert intent.version == self.buffered_user_vlob.version
            self.buffered_user_vlob.update(intent.blob)
        else:
            self.buffered_user_vlob = BufferedUserVlob.init(intent.version, intent.blob)

    @do
    def perform_user_vlob_delete(self, intent):
        if self.buffered_user_vlob:
            self.buffered_user_vlob.discard()
            self.buffered_user_vlob = None
        else:
            raise UserVlobNotFound('User vlob not found.')

    @do
    def perform_user_vlob_exist(self, intent):
        return self.buffered_user_vlob is not None

    @do
    def perform_user_vlob_synchronize(self, intent):
        if self.buffered_user_vlob:
            user_vlob = yield self.perform_user_vlob_read(EUserVlobRead())
            yield Effect(EBackendUserVlobUpdate(self.buffered_user_vlob.version,
                                                user_vlob['blob'].encode()))
            yield self.perform_user_vlob_delete(EUserVlobDelete())
            return True
        return False

    @do
    def perform_vlob_create(self, intent):
        buffered_vlob = BufferedVlob.init(blob=intent.blob,
                                          read_trust_seed='42',
                                          write_trust_seed='42')
        self.buffered_vlobs[buffered_vlob.id] = buffered_vlob
        return {'id': buffered_vlob.id,
                'read_trust_seed': buffered_vlob.read_trust_seed,
                'write_trust_seed': buffered_vlob.write_trust_seed}

    @do
    def perform_vlob_read(self, intent):
        if (intent.id in self.buffered_vlobs and
                (not intent.version or intent.version == self.buffered_vlobs[intent.id].version)):
            if self.buffered_vlobs[intent.id].read_trust_seed == '42':
                self.buffered_vlobs[intent.id].read_trust_seed = intent.trust_seed
            assert intent.trust_seed == self.buffered_vlobs[intent.id].read_trust_seed
            return self.buffered_vlobs[intent.id].read()
        else:
            vlob = yield Effect(EBackendVlobRead(intent.id, intent.trust_seed, intent.version))
            return {'id': vlob.id, 'blob': vlob.blob.decode(), 'version': vlob.version}

    @do
    def perform_vlob_update(self, intent):
        try:
            assert intent.version == self.buffered_vlobs[intent.id].version
            assert intent.trust_seed == self.buffered_vlobs[intent.id].write_trust_seed
            self.buffered_vlobs[intent.id].update(intent.blob)
        except KeyError:
            self.buffered_vlobs[intent.id] = BufferedVlob.init(intent.id,
                                                               intent.version,
                                                               intent.blob,
                                                               '42',
                                                               intent.trust_seed)

    @do
    def perform_vlob_delete(self, intent):
        try:
            self.buffered_vlobs[intent.id].discard()
            del self.buffered_vlobs[intent.id]
        except KeyError:
            raise VlobNotFound('Vlob not found.')

    @do
    def perform_vlob_list(self, intent):
        return sorted(list(self.buffered_vlobs.keys()))

    @do
    def perform_vlob_synchronize(self, intent):
        if intent.id in self.buffered_vlobs:
            vlob = self.buffered_vlobs[intent.id].read()
            new_vlob = None
            if self.buffered_vlobs[intent.id].version == 1:
                new_vlob = yield Effect(EBackendVlobCreate(vlob['blob'].encode()))
            else:
                yield Effect(EBackendVlobUpdate(intent.id,
                                                self.buffered_vlobs[intent.id].write_trust_seed,
                                                self.buffered_vlobs[intent.id].version,
                                                vlob['blob'].encode()))  # TODO encode is correct?
            yield self.perform_vlob_delete(EVlobDelete(intent.id))
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

    def periodic_synchronization(self):
        raise NotImplementedError()

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
