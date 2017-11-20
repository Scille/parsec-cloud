import trio
import attr
from nacl.secret import SecretBox
import nacl.utils

from parsec.core.manifest import *
from parsec.core.manifests_manager import ManifestsManager
from parsec.core.local_storage import LocalStorage
from parsec.core.backend_connection import BackendConnection
from parsec.core.backend_storage import BackendStorage, BackendConcurrencyError
from parsec.core.synchronizer import Synchronizer
from parsec.core.file_manager import _merge_patches, Patch
from parsec.utils import ParsecError


class FSInvalidPath(ParsecError):
    status = 'invalid_path'


class BaseFS:

    def __init__(self, user, manifests_manager, synchronizer):
        self.user = user
        self._objs = {}
        self._manifests_manager = manifests_manager
        self._synchronizer = synchronizer
        self._file_cls = self._file_cls_factory()
        self._folder_cls = self._folder_cls_factory()
        self._manifest = self._manifests_manager.fetch_user_manifest()
        self._root_folder = self._root_folder_factory(self._manifest)

    def _root_folder_factory(self, manifest):
        class RootFolder(BaseFolder):
            _fs = self

        def __eq__(self, other):
            if isinstance(other, BaseFolder):
                return self._entry == other._entry
            else:
                return False

        def __neq__(self, other):
            return not self.__eq__(other)

        root = RootFolder(None, manifest, need_flush=False)
        root.path = '/'
        return root

    def _file_cls_factory(self):
        class File(BaseFile):
            _fs = self

        return File

    def _folder_cls_factory(self):
        class Folder(BaseFolder):
            _fs = self

        return Folder

    async def fetch_entry(self, entry, path='.'):
        obj = self._objs.get(entry.id)
        if not obj:
            manifest = await self._manifests_manager.fetch_manifest(entry)
            if isinstance(manifest, LocalFolderManifest):
                obj = self._folder_cls(entry, manifest, path=path, need_flush=False)
            else:
                obj = self._file_cls(entry, manifest, path=path, need_flush=False)
            self._objs[entry.id] = obj
        return obj

    async def fetch_path(self, path):
        if not path.startswith('/'):
            raise FSInvalidPath('Path must be absolute')
        hops = [n for n in path.split('/') if n]
        obj = self._root_folder
        for hop in hops:
            if not isinstance(obj, BaseFolder):
                raise FSInvalidPath("Path `%s` doesn't exists" % path)
            obj = await obj.fetch_child(hop)
        return obj

    def create_folder(self):
        entry, manifest = self._manifests_manager.create_placeholder_folder()
        return self._folder_cls(entry, manifest, need_flush=True)

    def create_file(self):
        entry, manifest = self._manifests_manager.create_placeholder_file()
        return self._file_cls(entry, manifest, need_flush=True)

    async def sync(self, elem):
        if not elem.need_sync:
            return
        if isinstance(elem, BaseFile):
            await self._sync_file(elem)
        else:
            await self._sync_folder(elem)

    async def _sync_file(self, elem):
        while True:
            try:
                synced_manifest = await self._manifests_manager.sync_manifest(elem._entry, elem._manifest)
                elem._manifest.base_version = synced_manifest.version
                elem._manifest.need_sync = False
            except BackendConcurrencyError:
                # Our manifest is outdated, this means somebody has modified
                # this file in our back... We have no choice but to save our
                # current version under another name
                # TODO: we don't have reference on the parent folder here...
                raise
            else:
                break
            elem.flush()

    async def _sync_folder(self, elem):
        while True:
            try:
                if elem._entry:
                    # TODO: not really nice to fetch the base manifest this way...
                    base_manifest = await self._manifests_manager.fetch_manifest_force_version(
                        elem._entry, version=elem.base_version)
                    synced_manifest = await self._manifests_manager.sync_manifest(elem._entry, elem._manifest)
                else:
                    # TODO: not really nice to fetch the base manifest this way...
                    base_manifest = await self._manifests_manager.fetch_user_manifest_force_version(
                        version=elem.base_version)
                    synced_manifest = await self._manifests_manager.sync_user_manifest(elem._manifest)
                # Given the synchronizing process is asynchronous, it is possible
                # the manifest has been modified in the meantime, so we must merged
                # the syncronized manifest.
                elem._manifest = merge_folder_manifest2(base_manifest, elem._manifest, synced_manifest)
            except BackendConcurrencyError:
                # Our manifest is outdated, we must fetch the last up to date
                # manifest and merge it with our current one before retrying the
                # synchronization
                if elem._entry:
                    target_manifest = await self._manifests_manager.fetch_manifest_force_version(
                        elem._entry)
                else:
                    target_manifest = await self._manifests_manager.fetch_user_manifest_force_version()
                elem._manifest = merge_folder_manifest2(base_manifest, elem._manifest, target_manifest)
            else:
                break
            elem.flush()

    def flush(self, elem):
        if not elem.need_flush:
            return
        if elem._entry:
            self._manifests_manager.flush_manifest(elem._entry, elem._manifest)
        else:
            self._manifests_manager.flush_user_manifest(elem._manifest)
        elem._need_flush = False


class FS(BaseFS):
    def __init__(self, user, backend_addr):
        self.local_storage = LocalStorage(user)
        self.backend_conn = BackendConnection(user, backend_addr)
        self.backend_storage = BackendStorage(self.backend_conn)
        from parsec.core.fs_api import FSApi
        self.api = FSApi(self)
        manifests_manager = ManifestsManager(user, self.local_storage, self.backend_storage)
        # synchronizer = Synchronizer(self.backend_conn)
        synchronizer = None  # TODO...
        super().__init__(user, manifests_manager, synchronizer)
        # self.synchronizer = Synchronizer(self, self.backend_conn)
        # self.local_user_manifest = None
        # self.files_manager = FileManager(self.local_storage)

    async def init(self, nursery):
        await self.backend_conn.init(nursery)
        # await self.synchronizer.init(nursery)
        # await self.backend_storage.init()

    async def teardown(self):
        await self.backend_conn.teardown()


@attr.s(slots=True)
class BaseFile:
    _entry = attr.ib()
    _manifest = attr.ib()
    _need_flush = attr.ib()
    _patches = attr.ib(init=False)
    _need_patches_merge = attr.ib(False, init=False)
    path = attr.ib(default='.')

    @_patches.default
    def _load_patches(self):
        patches = []
        for db in self._manifest.dirty_blocks:
            key = from_jsonb64(db['key'])
            patches.append(Patch.build_from_dirty_block(
                self._fs.local_storage, db['id'], db['offset'], db['size'], key))
        return patches

    def __eq__(self, other):
        if isinstance(other, BaseFile):
            return self._entry.id == other._entry.id
        else:
            return False

    def __neq__(self, other):
        return not self.__eq__(other)

    @property
    def need_sync(self):
        return self._manifest.need_sync

    def _modified(self):
        self._need_flush = True
        self._manifest.need_sync = True

    @property
    def is_placeholder(self):
        return isinstance(self._entry, PlaceHolderEntry)

    @property
    def need_flush(self):
        return self._need_flush

    @property
    def id(self):
        return self._entry.id

    @property
    def created(self):
        return self._manifest.created

    @property
    def updated(self):
        return self._manifest.updated

    @property
    def size(self):
        return self._manifest.size

    @property
    def base_version(self):
        return self._manifest.base_version

    def flush(self):
        # Flush the dirty blocks first, then the manifest
        # Now is time to clean the patches
        if self._need_patches_merge:
            self._patches = _merge_patches(self._patches)
        dirty_blocks = self._manifest.dirty_blocks = []
        for p in self._patches:
            if not p.dirty_block_id:
                p.save_as_dirty_block()
            dirty_blocks.append({
                'id': p.dirty_block_id,
                'key': to_jsonb64(p.dirty_block_key),
                'offset': p.offset,
                'size': p.size
            })
        self._fs.flush(self)

    async def read(self, size=None, offset=0):
        size = size if (size is not None and 0 < size < self.size) else self.size
        if self._need_patches_merge:
            self._patches = _merge_patches(self._patches)

        async def _read_buffers_from_blocks(offset, size):
            buffers = []
            end = offset + size
            curr_pos = offset
            for block in self._manifest.blocks:
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
                    ciphered = self._fs.local_storage.fetch_block(block['id'])
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
        self._need_patches_merge = True
        self._patches.append(Patch(self._fs.local_storage, offset, len(buffer), buffer=buffer))
        if offset + len(buffer) > self.size:
            self._manifest.size = offset + len(buffer)
        self._modified()

    def truncate(self, length):
        self._need_patches_merge = True
        if self.size < length:
            return
        self._manifest.size = length
        self._modified()

    async def sync(self):
        if not self.need_sync:
            return
        self.flush()
        # TODO: instead of nicely creating new blocks, we replace everything
        # by a single big block...
        full_buffer = await self.read()
        block_key = nacl.utils.random(SecretBox.KEY_SIZE)
        box = SecretBox(block_key)
        full_ciphered = box.encrypt(full_buffer)
        block_id = await self._fs.backend_storage.sync_new_block(full_ciphered)
        self._manifest.dirty_blocks = []
        self._manifest.blocks = [
            {
                'id': block_id,
                'key': to_jsonb64(block_key),
                'offset': 0,
                'size': len(full_buffer)
            }
        ]
        # Now synchronize the manifest
        await self._fs.sync(self)
        # Force flush
        self._need_flush = True
        self.flush()


@attr.s(slots=True)
class BaseFolder:
    _entry = attr.ib()
    _manifest = attr.ib()
    _need_flush = attr.ib()
    path = attr.ib(default='./')

    def __eq__(self, other):
        if isinstance(other, BaseFolder):
            return self._entry.id == other._entry.id
        else:
            return False

    def __neq__(self, other):
        return not self.__eq__(other)

    def _build_path(self, sub):
        return '/' + '/'.join([x for x in self.path.split('/') + sub.split('/') if x])

    @property
    def _fs(self):
        raise NotImplementedError()

    @property
    def id(self):
        return self._entry.id

    @property
    def created(self):
        return self._manifest.created

    @property
    def updated(self):
        return self._manifest.updated

    @property
    def base_version(self):
        return self._manifest.base_version

    @property
    def need_sync(self):
        return self._manifest.need_sync

    @property
    def need_flush(self):
        return self._need_flush

    def _modified(self):
        self._need_flush = True
        self._manifest.need_sync = True

    @property
    def is_placeholder(self):
        return isinstance(self._entry, PlaceHolderEntry)

    def keys(self):
        return self._manifest.children.keys()

    def __getitem__(self, name):
        return self._manifest.children[name]

    def __contains__(self, name):
        return name in self._manifest.children

    async def fetch_child(self, name):
        try:
            entry = self._manifest.children[name]
        except KeyError:
            raise FSInvalidPath("Path `%s` doesn't exists" % self._build_path(name))
        return await self._fs.fetch_entry(entry, path=self._build_path(name))

    def delete_child(self, name):
        try:
            del self._manifest.children[name]
        except KeyError:
            raise FSInvalidPath("Path `%s` doesn't exists" % self._build_path(name))
        # TODO: also delete in fs ?
        self._manifest.updated = pendulum.utcnow()
        self._modified()

    def insert_child(self, name, child):
        if not name or name in self._manifest.children:
            raise FSInvalidPath("Path `%s` already exists" % self._build_path(name))
        self._manifest.children[name] = child._entry
        self._manifest.updated = pendulum.utcnow()
        self._modified()

    def create_folder(self, name):
        if name in self._manifest.children:
            raise FSInvalidPath("Path `%s` already exists" % self._build_path(name))
        new_folder = self._fs.create_folder()
        new_folder.path = self._build_path(name)
        self._manifest.children[name] = new_folder._entry
        self._manifest.updated = pendulum.utcnow()
        self._modified()
        return new_folder

    def create_file(self, name):
        if name in self._manifest.children:
            raise FSInvalidPath("Path `%s` already exists" % self._build_path(name))
        new_file = self._fs.create_file()
        new_file.path = self._build_path(name)
        self._manifest.children[name] = new_file._entry
        self._manifest.updated = pendulum.utcnow()
        self._modified()
        return new_file

    def flush(self):
        self._fs.flush(self)

    async def sync(self):
        if self.is_placeholder:
            raise RuntimeError('Cannot sync a placeholder')
        await self._fs.sync(self)
        # Force flush
        self._need_flush = True
        self.flush()
