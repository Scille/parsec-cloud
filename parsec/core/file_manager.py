import attr
import json
import trio
from uuid import uuid4
import pendulum
from nacl.public import PrivateKey
from nacl.secret import SecretBox
import nacl.utils

from parsec.utils import from_jsonb64, to_jsonb64


def _generate_sym_key():
    return nacl.utils.random(SecretBox.KEY_SIZE)


class FileManager:

    def __init__(self, local_storage):
        self.local_storage = local_storage
        self._files = {}

    async def get_file(self, id, rts, wts, key):
        file = self._files.get(id)
        if not file:
            # First check for dirty file manifest
            ciphered_data = self.local_storage.get_dirty_file_manifest(id)
            if not ciphered_data:
                # then for regular file manifest in cache
                ciphered_data = self.local_storage.get_file_manifest(id)
                if not ciphered_data:
                    # TODO: handle cache miss with async request to backend
                    # (This is where `file.is_ready` is useful)
                    return None
            file = LocalFile.load(self, id, rts, wts, key, ciphered_data)
            file.is_ready.set()
            self._files[id] = file
        await file.is_ready.wait()
        return file

    def get_placeholder_file(self, id, key):
        file = self._files.get(id)
        if not file:
            ciphered_data = self.local_storage.get_placeholder_file_manifest(id)
            if ciphered_data:
                file = PlaceHolderFile.load(self, id, key, ciphered_data)
            else:
                # TODO: better exception ?
                raise RuntimeError('Unknown placeholder file `%s`' % id)
            self._files[id] = file
        return file

    def create_placeholder_file(self):
        file, key = PlaceHolderFile.create(self)
        self._files[file.id] = file
        return file, key

    def destroy_placeholder_file(self, id):
        del self._files[id]
        # TODO: also destroy in local storage ?


class BaseLocalFile:
    async def read(self, size, offset=0):
        raise NotImplementedError()

    def write(self, buffer, offset=0):
        raise NotImplementedError()

    def truncate(self, length):
        raise NotImplementedError()

    def sync(self):
        raise NotImplementedError()


@attr.s(init=False)
class PatchedLocalFileFixture(BaseLocalFile):
    _patches = attr.ib()
    _need_merge = attr.ib(False, init=False)
    _need_sync = attr.ib(False, init=False)

    def _load_patches(self):
        patches = []
        for db in self.data.get('dirty_blocks', []):
            key = from_jsonb64(db['key'])
            patches.append(Patch.build_from_dirty_block(
                self.file_manager, db['id'], db['offset'], db['size'], key))
        return patches

    async def read(self, size=None, offset=0):
        size = size if (size is not None and 0 < size < self.size) else self.size
        if self._need_merge:
            self._patches = _merge_patches(self._patches)

        async def _read_buffers_from_blocks(offset, size):
            buffers = []
            end = offset + size
            curr_pos = offset
            for block in self.data['blocks']:
                block_size = block['size']
                block_offset = block['offset']
                block_end = block_offset + block_size
                # Blocks should be contiguous, so no holes in our read is possible
                # TODO: detect bad contiguous blocks and raise error ?
                if block_end <= curr_pos:
                    continue
                elif block_offset >= end:
                    break
                else:
                    # TODO: have a hot cache in RAM with unciphered data ?
                    ciphered = self.file_manager.local_storage.get_block(block['id'])
                    if not ciphered:
                        # TODO: fetch block on backend
                        raise NotImplementedError()
                    key = from_jsonb64(block['key'])
                    block_data = SecretBox(key).decrypt(ciphered)
                    buffer = block_data[curr_pos - block_offset:end - block_offset]
                    buffers.append(buffer)
                    curr_pos += len(buffer)
            assert curr_pos == end
            return buffers

        buffers = []
        curr_pos = offset
        end = offset + size
        # Remember patches are ordered once merged
        for p in self._patches:
            if p.end <= curr_pos:
                continue
            elif p.offset >= end:
                # We're done here
                break
            else:
                if p.offset > curr_pos:
                    missing_size = p.offset - curr_pos
                    buffers += await _read_buffers_from_blocks(curr_pos, missing_size)
                    curr_pos += missing_size
                buffer = p.get_buffer()[curr_pos - p.offset:end - p.offset]
                buffers.append(buffer)
                curr_pos += len(buffer)
                if curr_pos == end:
                    break
        if curr_pos < end:
            buffers += await _read_buffers_from_blocks(curr_pos, end - curr_pos)

        return b''.join(buffers)

    def write(self, buffer, offset=0):
        self._need_merge = True
        self._need_sync = True
        self._patches.append(Patch(self.file_manager, offset, len(buffer), buffer=buffer))
        if offset + len(buffer) > self.size:
            self.data['size'] = offset + len(buffer)

    def truncate(self, length):
        self._need_merge = True
        self._need_sync = True
        if self.size < length:
            return
        self.data['size'] = length

    def sync(self):
        if not self._need_sync:
            return
        # Now is time to clean the patches
        if self._need_merge:
            self._patches = _merge_patches(self._patches)
        dirty_blocks = self.data['dirty_blocks'] = []
        for p in self._patches:
            if not p.dirty_block_id:
                p.save_as_dirty_block()
            dirty_blocks.append({
                'id': p.dirty_block_id,
                'key': to_jsonb64(p.dirty_block_key),
                'offset': p.offset,
                'size': p.size
            })
        # Child should take care of saving `self.data`


@attr.s
class LocalFile(PatchedLocalFileFixture, BaseLocalFile):
    file_manager = attr.ib()
    id = attr.ib()
    rts = attr.ib()
    wts = attr.ib()
    box = attr.ib()
    data = attr.ib()
    is_ready = attr.ib(default=attr.Factory(trio.Event))
    _patches = attr.ib()

    @_patches.default
    def _load_patches(self):
        return super()._load_patches()

    @property
    def created(self):
        return self.data['created']

    @property
    def updated(self):
        return self.data['updated']

    @property
    def version(self):
        return self.data['version']

    @property
    def size(self):
        return self.data['size']

    @property
    def is_dirty(self):
        return bool(self.data.get('dirty_blocks'))

    @classmethod
    def load(cls, file_manager, id, rts, wts, key, ciphered_data):
        box = SecretBox(key)
        data = json.loads(box.decrypt(ciphered_data).decode())
        data.setdefault('dirty_blocks', [])
        return cls(file_manager, id, rts, wts, box, data)

    def dump(self):
        return self.box.encrypt(json.dumps(self.data).encode())

    def sync(self):
        if not self._need_sync:
            return
        super().sync()
        self.file_manager.local_storage.save_dirty_file_manifest(self.id, self.dump())


@attr.s
class PlaceHolderFile(PatchedLocalFileFixture, BaseLocalFile):
    file_manager = attr.ib()
    id = attr.ib()
    box = attr.ib()
    data = attr.ib()
    _patches = attr.ib()

    @_patches.default
    def _load_patches(self):
        return super()._load_patches()

    @data.default
    def _default_data(field):
        now = pendulum.utcnow().isoformat()
        return {
            'created': now,
            'updated': now,
            'version': 0,
            'size': 0,
            'blocks': [],
            'dirty_blocks': []
        }

    @property
    def created(self):
        return self.data['created']

    @property
    def updated(self):
        return self.data['updated']

    @property
    def version(self):
        return self.data['version']

    @property
    def size(self):
        return self.data['size']

    @property
    def is_dirty(self):
        return True

    @classmethod
    def load(cls, file_manager, id, key, ciphered_data):
        box = SecretBox(key)
        data = json.loads(box.decrypt(ciphered_data).decode())
        assert data.pop('format') == 1
        return cls(file_manager, id, box, data)

    def dump(self):
        data = {**self.data, 'format': 1}
        return self.box.encrypt(json.dumps(data).encode())

    def sync(self):
        if not self._need_sync:
            return
        super().sync()
        # TODO: only save a new placeholder version if it has changed
        self.file_manager.local_storage.save_placeholder_file_manifest(self.id, self.dump())

    @classmethod
    def create(cls, file_manager):
        id = uuid4().hex
        key = _generate_sym_key()
        box = SecretBox(key)
        file = cls(file_manager, id, box)
        file._need_sync = True
        return file, key


def _try_merge_two_patches(p1, p2):
    if ((p1.offset < p2.offset and p1.offset + p1.size < p2.offset) or
            (p2.offset < p1.offset and p2.offset + p2.size < p1.offset)):
        return None
    p1buffer = p1.get_buffer()
    p2buffer = p2.get_buffer()
    # Remember p2 has priority over p1
    if p1.offset < p2.offset:
        newbuffer = p1buffer[:p2.offset - p1.offset] + p2buffer + p1buffer[p2.offset + p2.size - p1.offset:]
        newsize = len(newbuffer)
        newoffset = p1.offset
    else:
        newbuffer = p2buffer + p1buffer[p2.offset + p2.size - p1.offset:]
        newsize = len(newbuffer)
        newoffset = p2.offset
    return Patch(p1.file_manager, newoffset, newsize, buffer=newbuffer)


def _merge_patches(patches):
    merged = []
    for p2 in patches:
        new_merged = []
        for p1 in merged:
            res = _try_merge_two_patches(p1, p2)
            if res:
                p2 = res
            else:
                new_merged.append(p1)
        new_merged.append(p2)
        merged = new_merged
    return sorted(merged, key=lambda x: x.offset)


@attr.s(slots=True)
class Patch:
    file_manager = attr.ib()
    offset = attr.ib()
    size = attr.ib()
    dirty_block_id = attr.ib(default=None)
    dirty_block_key = attr.ib(default=None)
    _buffer = attr.ib(default=None)

    @property
    def end(self):
        return self.offset + self.size

    def get_buffer(self):
        if self._buffer is None:
            if not self.dirty_block_id:
                raise RuntimeError('This patch has no buffer...')
            ciphered = self.file_manager.local_storage.get_dirty_block(self.dirty_block_id)
            self._buffer = SecretBox(self.dirty_block_key).decrypt(ciphered)
        return self._buffer

    def save_as_dirty_block(self):
        if self.dirty_block_id:
            raise RuntimeError('Cannot modify already existing `%s` dirty block' % self.dirty_block_id)
        self.dirty_block_id = uuid4().hex
        self.dirty_block_key = _generate_sym_key()
        ciphered = SecretBox(self.dirty_block_key).encrypt(self._buffer)
        self.file_manager.local_storage.save_dirty_block(self.dirty_block_id, ciphered)

    @classmethod
    def build_from_dirty_block(cls, file_manager, id, offset, size, key):
        return cls(file_manager, offset, size, id, key)
