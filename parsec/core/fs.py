import attr
import arrow
from uuid import uuid4
from effect2 import TypeDispatcher, Effect

from parsec.base import ERegisterEvent
from parsec.core.identity import EIdentityGet, EIdentityUnload
from parsec.core.backend_user_vlob import EBackendUserVlobRead, EBackendUserVlobUpdate
from parsec.core.backend_vlob import EBackendVlobCreate, EBackendVlobUpdate, EBackendVlobRead, VlobAtom
from parsec.core.block import EBlockRead, EBlockCreate
from parsec.core.synchronizer import ESynchronizerPutJob, ESynchronizerFlush
from parsec.exceptions import (InvalidPath, IdentityNotLoadedError, ManifestError)
from parsec.tools import ejson_loads, ejson_dumps, to_jsonb64, from_jsonb64
from parsec.crypto import generate_sym_key, load_sym_key


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
        self._vlob_cache = {}
        self._block_cache = {}

    def get_manifest(self):
        # _manifest field can be set to None at any time by another coroutine
        # doing a EFSReset, hence we must check it is valid
        if not self._manifest:
            raise ManifestError('Identity must be loaded to have a manifest')
        return self._manifest

    async def _get_block(self, id):
        try:
            return self._block_cache[id]
        except KeyError:
            content = (await Effect(EBlockRead(id))).content
            self._block_cache[id] = content
        return content

    async def _create_block(self, id, content):
        self._block_cache[id] = content
        intent = EBlockCreate(id, content)
        await Effect(ESynchronizerPutJob(intent))

    async def _get_vlob(self, id, trust_seed):
        try:
            return self._vlob_cache[id]
        except KeyError:
            vlob = await Effect(EBackendVlobRead(id, trust_seed))
            self._vlob_cache[id] = vlob
        return vlob

    async def _create_vlob(self, content):
        vlob = await Effect(EBackendVlobCreate(blob=content))
        self._block_cache[id] = vlob
        return vlob

    async def _update_vlob(self, id, trust_seed, version, content):
        intent = EBackendVlobUpdate(id, trust_seed, version, content)
        await Effect(ESynchronizerPutJob(intent))
        self._vlob_cache[id] = VlobAtom(id, version, content)

    async def _create_file(self):
        now = arrow.get()
        raw_file_manifest = ejson_dumps({
            'created': now,
            'updated': now,
            'blocks': [],
            'size': 0
        }).encode()
        vlob_key = generate_sym_key()
        vlob_content = vlob_key.encrypt(raw_file_manifest)
        vlob_access = await self._create_vlob(vlob_content)
        return {
            'type': 'file',
            'id': vlob_access.id,
            'read_trust_seed': vlob_access.read_trust_seed,
            'write_trust_seed': vlob_access.write_trust_seed,
            'key': to_jsonb64(vlob_key.key)
        }

    async def _stat_file(self, fileobj):
        # Retrieve file manifest
        vlob = await self._get_vlob(fileobj['id'], fileobj['read_trust_seed'])
        vlob_key = load_sym_key(from_jsonb64(fileobj['key']))
        file_manifest = ejson_loads(vlob_key.decrypt(vlob.blob).decode())
        return {
            'type': fileobj['type'],
            'updated': file_manifest['updated'],
            'created': file_manifest['created'],
            'size': file_manifest['size'],
            'version': vlob.version
        }

    async def _read_file(self, fileobj, offset, size):
        # Retrieve file manifest
        vlob = await self._get_vlob(fileobj['id'], fileobj['read_trust_seed'])
        vlob_key = load_sym_key(from_jsonb64(fileobj['key']))
        file_manifest = ejson_loads(vlob_key.decrypt(vlob.blob).decode())

        # Retrieve data
        blocks = []
        for block_access in file_manifest['blocks']:
            cipherblock = await self._get_block(block_access['id'])
            block_key = load_sym_key(from_jsonb64(block_access['key']))
            blocks.append(block_key.decrypt(cipherblock))
        data = b''.join(blocks)

        if size is None:
            return data[offset:]
        else:
            return data[offset:offset + size]

    async def _write_file(self, fileobj, offset, content):
        # Retrieve file manifest
        vlob = await self._get_vlob(fileobj['id'], fileobj['read_trust_seed'])
        vlob_key = load_sym_key(from_jsonb64(fileobj['key']))
        file_manifest = ejson_loads(vlob_key.decrypt(vlob.blob).decode())

        # Retrieve data
        blocks = []
        for block_access in file_manifest['blocks']:
            cipherblock = await self._get_block(block_access['id'])
            block_key = load_sym_key(from_jsonb64(block_access['key']))
            blocks.append(block_key.decrypt(cipherblock))
        data = b''.join(blocks)

        # Modify data
        data = (data[:offset] + content +
                data[offset + len(content):])
        file_manifest['updated'] = arrow.get()
        file_manifest['size'] = len(data)

        # Reupload data
        file_manifest['blocks'].clear()
        chunk_size = 1024 * 1024  # 1mo chunks
        chunk_offset = 0
        chunk = data[:chunk_size]
        while chunk:
            block_id = uuid4().hex
            block_key = generate_sym_key()
            cipherblock = block_key.encrypt(chunk)
            await self._create_block(block_id, cipherblock)
            file_manifest['blocks'].append({'id': block_id, 'key': to_jsonb64(block_key.key)})

            chunk_offset += chunk_size
            chunk = data[chunk_offset: chunk_offset + chunk_size]

        # Save file manifest and we're done ;-)
        ciphervlob = vlob_key.encrypt(ejson_dumps(file_manifest).encode())
        await self._update_vlob(
            fileobj['id'], fileobj['write_trust_seed'],
            vlob.version + 1, ciphervlob
        )

    async def _truncate_file(self, fileobj, length):
        # Retrieve file manifest
        vlob = await self._get_vlob(fileobj['id'], fileobj['read_trust_seed'])
        vlob_key = load_sym_key(from_jsonb64(fileobj['key']))
        file_manifest = ejson_loads(vlob_key.decrypt(vlob.blob).decode())

        # Retrieve data
        blocks = []
        for block_access in file_manifest['blocks']:
            cipherblock = await self._get_block(block_access['id'])
            block_key = load_sym_key(from_jsonb64(block_access['key']))
            blocks.append(block_key.decrypt(cipherblock))
        data = b''.join(blocks)

        # Modify data
        data = data[:length]
        file_manifest['updated'] = arrow.get()
        file_manifest['size'] = len(data)

        # Reupload data
        file_manifest['blocks'].clear()
        chunk_size = 4096
        chunk_offset = 0
        chunk = data[:chunk_size]
        while chunk:
            block_id = uuid4().hex
            block_key = generate_sym_key()
            cipherblock = block_key.encrypt(chunk)
            await self._create_block(block_id, cipherblock)
            file_manifest['blocks'].append({'id': block_id, 'key': to_jsonb64(block_key.key)})

            chunk_offset += chunk_size
            chunk = data[chunk_offset: chunk_offset + chunk_size]

        # Save file manifest and we're done ;-)
        ciphervlob = vlob_key.encrypt(ejson_dumps(file_manifest).encode())
        await self._update_vlob(
            fileobj['id'], fileobj['write_trust_seed'],
            vlob.version + 1, ciphervlob
        )

    async def startup(self, app):
        await Effect(ERegisterEvent(lambda e, s: EFSInit(), 'identity_loaded', None))
        await Effect(ERegisterEvent(lambda e, s: EFSReset(), 'identity_unloaded', None))

    async def perform_init(self, intent):
        if self._manifest:
            raise ManifestError('Manifest already loaded')
        uservlob = await Effect(EBackendUserVlobRead())
        if uservlob.version == 0:
            self._manifest = self._create_new_manifest()
            self._manifest_version = 0
        else:
            identity = await Effect(EIdentityGet())
            self._manifest_version = uservlob.version
            try:
                self._manifest = ejson_loads(identity.private_key.decrypt(uservlob.blob).decode())
            except Exception as exc:
                # Avoid being in an inconsistent state
                await Effect(EIdentityUnload())
                raise ManifestError('Impossible to load manifest: %s' % exc)

    async def _commit_manifest(self):
        identity = await Effect(EIdentityGet())
        ciphermanifest = identity.public_key.encrypt(ejson_dumps(self._manifest).encode())
        new_version = self._manifest_version + 1
        await Effect(EBackendUserVlobUpdate(new_version, ciphermanifest))
        self._manifest_version = new_version

    async def perform_reset(self, intent):
        self._manifest = None
        self._file_manifest_cache = {}
        self._block_cache = {}
        await Effect(ESynchronizerFlush())

    def _create_new_manifest(self):
        now = arrow.get()
        return {
            'type': 'folder',
            'children': {},
            'stat': {'created': now, 'updated': now}
        }

    def _create_file_new_manifest(self):
        now = arrow.get()
        return {
            'type': 'file',
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
            return self.get_manifest()
        if not path.startswith('/'):
            raise InvalidPath("Path must start with `/`")
        cur_dir = self.get_manifest()
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

    async def perform_file_create(self, intent):
        await Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=False)
        dirpath, name = intent.path.rsplit('/', 1)
        dirobj = self._retrieve_path(dirpath)
        fileobj = await self._create_file()
        dirobj['children'][name] = fileobj
        # TODO: fill journal

    async def perform_file_write(self, intent):
        await Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=True, type='file')
        fileobj = self._retrieve_file(intent.path)
        await self._write_file(fileobj, intent.offset, intent.content)

    async def perform_file_truncate(self, intent):
        await Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=True, type='file')
        fileobj = self._retrieve_file(intent.path)
        await self._truncate_file(fileobj, intent.length)

    async def perform_file_read(self, intent):
        await Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=True, type='file')
        fileobj = self._retrieve_file(intent.path)
        return await self._read_file(fileobj, intent.offset, intent.size)

    async def perform_stat(self, intent):
        await Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=True)
        obj = self._retrieve_path(intent.path)
        if obj['type'] == 'folder':
            return {'type': obj['type'], 'children': list(sorted(obj['children'].keys()))}
        else:
            return await self._stat_file(obj)

    async def perform_folder_create(self, intent):
        await Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=False)
        dirpath, name = intent.path.rsplit('/', 1)
        dirobj = self._retrieve_path(dirpath)
        now = arrow.get()
        dirobj['children'][name] = {
            'type': 'folder', 'children': {}, 'stat': {'created': now, 'updated': now}}
        # TODO: fill journal

    async def perform_move(self, intent):
        await Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.src, should_exists=True)
        self._check_path(intent.dst, should_exists=False)

        srcdirpath, scrfilename = intent.src.rsplit('/', 1)
        dstdirpath, dstfilename = intent.dst.rsplit('/', 1)

        srcobj = self._retrieve_path(srcdirpath)
        dstobj = self._retrieve_path(dstdirpath)
        dstobj['children'][dstfilename] = srcobj['children'][scrfilename]
        del srcobj['children'][scrfilename]
        # TODO: fill journal

    async def perform_delete(self, intent):
        await Effect(EIdentityGet())  # Trigger exception if identity is not loaded
        self._check_path(intent.path, should_exists=True)
        dirpath, leafname = intent.path.rsplit('/', 1)
        obj = self._retrieve_path(dirpath)
        del obj['children'][leafname]
        # TODO: fill journal

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
