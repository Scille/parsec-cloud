import trio
import attr

from parsec.core.manifest import *
from parsec.core.manifests_manager import ManifestsManager
from parsec.core.local_storage import LocalStorage
from parsec.core.backend_connection import BackendConnection
from parsec.core.backend_storage import BackendStorage, BackendConcurrencyError
from parsec.core.synchronizer import Synchronizer
from parsec.core.file_manager import *
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
    path = attr.ib(default='.')

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
        self._fs.flush(self)

    async def sync(self):
        await self._fs.sync(self)

    # TODO: implement file read/write etc.

    async def read(self, length, offset=0):
        # TODO: really implement this !
        return b'teube'

    def write(self, content, offset=0):
        self._modified()
        # TODO: really implement this !

    def truncate(self, length):
        self._modified()
        # TODO: really implement this !


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
