import attr
import arrow
from effect2 import TypeDispatcher, do, Effect

from parsec.core.identity import EIdentityGet
from parsec.core.backend_user_vlob import EBackendUserVlobRead, EBackendUserVlobUpdate
# from parsec.core.backend_vlob import EBackendVlobRead
from parsec.exceptions import (
    InvalidPath, FileNotFound, IdentityNotLoadedError, ManifestError, ManifestNotFound)
from parsec.tools import ejson_loads, ejson_dumps


@attr.s(slots=True)
class Folder:
    entries = attr.ib(default=set)


@attr.s(slots=True)
class FileManifest:
    id = attr.ib()
    key = attr.ib()
    read_trust_seed = attr.ib()
    write_trust_seed = attr.ib()


@attr.s(slots=True)
class Manifest(Folder):
    pass


class VlobCache:
    def get(self, id):
        pass

    def add(self, id):
        pass


class BlockCache:
    pass


@attr.s
class EFSInit:
    pass


@attr.s
class EFSReset:
    pass


@attr.s
class EFSFileCreate:
    path = attr.ib()


@attr.s
class EFSFileRead:
    path = attr.ib()
    offset = attr.ib(default=0)
    size = attr.ib(default=0)


@attr.s
class EFSFileWrite:
    path = attr.ib()
    content = attr.ib()
    offset = attr.ib()


@attr.s
class EFSFileTruncate:
    path = attr.ib()
    length = attr.ib()


@attr.s
class EFSFolderCreate:
    path = attr.ib()


@attr.s
class EFSStat:
    path = attr.ib()


@attr.s
class EFSMove:
    src = attr.ib()
    dst = attr.ib()


@attr.s
class EFSDelete:
    path = attr.ib()


class FSComponent:

    def __init__(self):
        self._manifest = None
        self._file_manifest_cache = {}
        self._block_cache = {}

    @do
    def perform_init(self, intent):
        # TODO: check already initialized
        uservlob = yield Effect(EBackendUserVlobRead())
        if uservlob.version == 0:
            self._manifest = self._create_new_manifest()
            self._manifest_version = 0
        else:
            identity = yield Effect(EIdentityGet())
            self._manifest_version = uservlob.version
            self._manifest = ejson_loads(identity.private_key.decrypt(uservlob.blob).decode())

    @do
    def _commit_manifest(self):
        identity = yield Effect(EIdentityGet())
        ciphermanifest = identity.public_key.encrypt(ejson_dumps(self._manifest).encode())
        new_version = self._manifest_version + 1
        yield Effect(EBackendUserVlobUpdate(new_version, ciphermanifest))
        self._manifest_version = new_version

    @do
    def perform_reset(self, intent):
        self._manifest = None
        self._file_manifest_cache = {}
        self._block_cache = {}

    def _create_new_manifest(self):
        now = arrow.get()
        return {
            'type': 'folder',
            'children': {},
            'stat': {'created': now, 'updated': now}
        }

    def _retrieve_file(self, path):
        fileobj = self._retrieve_path(path)
        if fileobj['type'] != 'file':
            raise InvalidPath("Path `%s` is not a file" % path)
        return fileobj

    def _check_path(self, path, should_exists=True, type=None):
        if path == '/':
            if not should_exists or type not in ('folder', None):
                raise InvalidPath('Root `/` folder always exists')
            else:
                return
        dirpath, leafname = path.rsplit('/', 1)
        obj = self._retrieve_path(dirpath)
        if obj['type'] != 'folder':
            raise InvalidPath("Path `%s` is not a folder" % path)
        try:
            leafobj = obj['children'][leafname]
            if not should_exists:
                raise InvalidPath("Path `%s` already exist" % path)
            if type is not None and leafobj['type'] != type:
                raise InvalidPath("Path `%s` is not a %s" % (path, type))
        except KeyError:
            if should_exists:
                raise InvalidPath("Path `%s` doesn't exist" % path)

    def _retrieve_path(self, path):
        if not path:
            return self._manifest
        if not path.startswith('/'):
            raise InvalidPath("Path must start with `/`")
        cur_dir = self._manifest
        reps = path.split('/')
        for rep in reps:
            if not rep or rep == '.':
                continue
            elif rep == '..':
                cur_dir = cur_dir['parent']
            else:
                try:
                    cur_dir = cur_dir['children'][rep]
                except KeyError:
                    raise InvalidPath("Path `%s` doesn't exist" % path)
        return cur_dir

    @do
    def perform_file_create(self, intent):
        yield Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=False)
        dirpath, name = intent.path.rsplit('/', 1)
        dirobj = self._retrieve_path(dirpath)
        now = arrow.get()
        dirobj['children'][name] = {
            'type': 'file', 'data': b'', 'stat': {'created': now, 'updated': now, 'version': 1}
        }

    @do
    def perform_file_write(self, intent):
        yield Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=True, type='file')
        fileobj = self._retrieve_file(intent.path)
        fileobj['data'] = (fileobj['data'][:intent.offset] + intent.content +
                           fileobj['data'][intent.offset + len(intent.content):])
        fileobj['stat']['updated'] = arrow.get()
        fileobj['stat']['version'] += 1

    @do
    def perform_file_truncate(self, intent):
        yield Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=True, type='file')
        fileobj = self._retrieve_file(intent.path)
        fileobj['data'] = fileobj['data'][:intent.length]
        fileobj['stat']['updated'] = arrow.get()

    @do
    def perform_file_read(self, intent):
        yield Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=True, type='file')
        fileobj = self._retrieve_file(intent.path)
        if intent.size is None:
            return fileobj['data'][intent.offset:]
        else:
            return fileobj['data'][intent.offset:intent.offset + intent.size]

    @do
    def perform_stat(self, intent):
        yield Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=True)
        obj = self._retrieve_path(intent.path)
        if obj['type'] == 'folder':
            # return {**obj['stat'], 'type': obj['type'], 'children': list(obj['children'].keys())}
            return {'type': obj['type'], 'children': list(obj['children'].keys())}
        else:
            return {**obj['stat'], 'type': obj['type'], 'size': len(obj['data'])}

    @do
    def perform_folder_create(self, intent):
        yield Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=False)
        dirpath, name = intent.path.rsplit('/', 1)
        dirobj = self._retrieve_path(dirpath)
        now = arrow.get()
        dirobj['children'][name] = {
            'type': 'folder', 'children': {}, 'stat': {'created': now, 'updated': now}}
        yield self._commit_manifest()

    @do
    def perform_move(self, intent):
        yield Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.src, should_exists=True)
        self._check_path(intent.dst, should_exists=False)

        srcdirpath, scrfilename = intent.src.rsplit('/', 1)
        dstdirpath, dstfilename = intent.dst.rsplit('/', 1)

        srcobj = self._retrieve_path(srcdirpath)
        dstobj = self._retrieve_path(dstdirpath)
        dstobj['children'][dstfilename] = srcobj['children'][scrfilename]
        del srcobj['children'][scrfilename]
        yield self._commit_manifest()

    @do
    def perform_delete(self, intent):
        yield Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=True)
        dirpath, leafname = intent.path.rsplit('/', 1)
        obj = self._retrieve_path(dirpath)
        del obj['children'][leafname]
        yield self._commit_manifest()

    def get_dispatcher(self):
        return TypeDispatcher({
            EFSFileCreate: self.perform_file_create,
            EFSFileRead: self.perform_file_read,
            EFSFileWrite: self.perform_file_write,
            EFSFileTruncate: self.perform_file_truncate,
            EFSFolderCreate: self.perform_folder_create,
            EFSStat: self.perform_stat,
            EFSMove: self.perform_move,
            EFSDelete: self.perform_delete,

            EFSReset: self.perform_reset,
            EFSInit: self.perform_init,
        })
