import attr
import arrow
import struct

from parsec.core2.manifest import (
    FileManifestAccess, FileManifest, FolderEntry, BlockAccess,
    load_user_manifest, load_file_manifest, dump_file_manifest, dump_user_manifest,
)
from parsec.exceptions import InvalidPath
from parsec.crypto import generate_sym_key
from parsec.tools import from_jsonb64
from parsec.core2.bbbackend_api_service import (
    VlobCreate, VlobRead, VlobUpdate, UserVlobRead, UserVlobUpdate)
from parsec.core2.block_service import BlockRead, BlockCreate


class FS:

    def __init__(self, identity):
        self.identity = identity
        self._wksp_reader = WorkspaceReader(identity)
        self._wksp_writer = WorkspaceWriter(identity)
        self._file_reader = FileReader(identity)
        self._file_writer = FileWriter(identity)

    @do
    def file_create(self, path: str):
        file_manifest_access = yield from self._file_writer.new()
        yield from self._wksp_writer.insert(path, file_manifest_access)

    @do
    def file_write(self, path: str, content: bytes, offset: int=0):
        obj = yield from self._wksp_reader.get(path)
        if not isinstance(obj, FileManifestAccess):
            raise InvalidPath("Path `%s` is not a file" % path)
        yield from self._file_writer.write(obj, content, offset)

    @do
    def file_read(self, path: str, offset: int=0, size: int=-1):
        obj = yield from self._wksp_reader.get(path)
        if not isinstance(obj, FileManifestAccess):
            raise InvalidPath("Path `%s` is not a file" % path)
        return (yield from self._file_reader.read(obj, offset, size))

    @do
    def file_truncate(self, path: str, length: int):
        obj = yield from self._wksp_reader.get(path)
        if not isinstance(obj, FileManifestAccess):
            raise InvalidPath("Path `%s` is not a file" % path)
        yield from self._file_writer.truncate(obj, length)

    @do
    def stat(self, path: str):
        obj = yield from self._wksp_reader.get(path)
        if isinstance(obj, FileManifestAccess):
            return yield from self._file_reader.stat(obj)
        else:
            return {'created': obj.created, 'updated': obj.updated,
                    'type': 'folder', 'children': list(obj.children.keys())}

    @do
    def folder_create(self, path: str):
        yield from self._wksp_writer.insert(path, FolderEntry())

    @do
    def move(self, src: str, dst: str):
        yield from self._wksp_writer.move(src, dst)

    @do
    def delete(self, path: str):
        yield from self._wksp_writer.delete(path)

    # TODO ?
    # async def copy(self, path: str):
    #     obj = await self._writer.get(path)
    #     if isinstance(obj, FileManifestAccess):
    #         new_obj = await self._file_reader.copy(obj)
    #     else:
    #         new_obj = FolderEntry()
    #     self._wksp_writer.insert(path, new_obj)


BLOCK_SIZE = 2**16


@attr.s
class FileReader:
    _backend = attr.ib()
    _block = attr.ib()
    _identity = attr.ib()

    @do
    def _load_file_manifest(self, access):
        # Get back FileManifest
        vlob_atom = yield Effect(VlobRead(access.id, access.read_trust_seed))
        raw = access.key.decrypt(vlob_atom.blob)
        signature_len, = struct.unpack('>I', raw[:4])
        signature = raw[4:signature_len + 4]
        raw_manifest = raw[signature_len + 4:]
        print('=============> load', access, signature, raw)
        # TODO: use pubkey service to get the key to verify
        self.identity.public_key.verify(signature, raw_manifest)
        return load_file_manifest(raw_manifest)

    @do
    def read(self, access: FileManifestAccess, offset: int=0, size: int=-1):
        file_manifest = yield from self._load_file_manifest(access)
        # Retrieve blocks to read
        start_block = offset // BLOCK_SIZE
        # TODO: really not optimized !
        data = []
        for block in file_manifest.blocks[start_block:]:
            block_blob = (yield Effect(BlockRead(block.id))).content
            block_data = block.key.decrypt(block_blob)
            self.identity.public_key.verify(block.signature, block_data)
            data.append(block_data)
        data = b''.join(data)
        return data[:size] if size != -1 else data

    @do
    def stat(self, access: FileManifestAccess):
        file_manifest = yield from self._load_file_manifest(access)
        return {'created': file_manifest.created, 'updated': file_manifest.updated,
                'type': 'file', 'size': sum([x.size for x in file_manifest.blocks])}


@attr.s
class FileWriter:
    _backend = attr.ib()
    _block = attr.ib()
    _identity = attr.ib()

    async def _load_file_manifest(self, access):
        # Get back FileManifest
        vlob_atom = yield Effect(VlobRead(access.id, access.read_trust_seed))
        raw = access.key.decrypt(vlob_atom.blob)
        signature_len, = struct.unpack('>I', raw[:4])
        signature = raw[4:signature_len + 4]
        raw_manifest = raw[signature_len + 4:]
        print('=============> load', access, signature, raw)
        # TODO: use pubkey service to get the key to verify
        self.identity.public_key.verify(signature, raw_manifest)
        return load_file_manifest(raw_manifest), vlob_atom.version

    async def new(self):
        file_manifest = FileManifest()

        raw = dump_file_manifest(file_manifest)
        signature = self.identity.private_key.sign(raw)
        key = generate_sym_key()
        ciphered = key.encrypt(struct.pack('>I', len(signature)) + signature + raw)
        vlob_access = yield Effect(BlockCreate(ciphered))
        access = FileManifestAccess(vlob_access.id, key, signature,
            vlob_access.read_trust_seed, vlob_access.write_trust_seed)
        print('=============> new', access, signature, raw)
        return access

    async def _slice_and_create_blocks(self, data):
        blocks = []
        curr = 0
        while True:
            block_data = data[curr:curr + BLOCK_SIZE]
            if not block_data:
                break
            key = generate_sym_key()
            signature = self.identity.private_key.sign(block_data)
            block_blob = key.encrypt(block_data)
            block = yield Effect(BlockCreate(block_blob))
            block_size = len(block_data)
            blocks.append(BlockAccess(block.id, key, signature, block_size))
            if block_size < BLOCK_SIZE:
                break
            curr += block_size
        return blocks

    async def _fetch_block(self, access):
        ciphered = (yield Effect(BlockRead(access.id))).content
        raw = access.key.decrypt(ciphered)
        self.identity.public_key.verify(access.signature, raw)
        return raw

    async def _commit_file_manifest(self, access, file_manifest, version):
        raw = dump_file_manifest(file_manifest)
        signature = self.identity.private_key.sign(raw)
        blob = access.key.encrypt(struct.pack('>I', len(signature)) + signature + raw)
        print('=============> commit', access, signature, raw)
        yield Effect(VlobUpdate(access.id, version + 1, access.write_trust_seed, blob))

    async def write(self, access: FileManifestAccess, content: bytes, offset: int=None):
        file_manifest, version = yield from self._load_file_manifest(access)
        if offset is None or offset > sum(x.size for x in file_manifest.blocks):
            if not content:
                # Nothing to do
                return
            # Append to the end of the file
            last_block = file_manifest.blocks[-1]
            if last_block.size != BLOCK_SIZE:
                block_raw = yield from self._fetch_block(last_block)
                content = block_raw + content
            new_blocks = yield from self._slice_and_create_blocks(content)
            file_manifest.blocks += new_blocks
        else:
            kept_blocks = file_manifest.blocks[:offset // BLOCK_SIZE]
            # It's possible we only keep a subpart of the last block
            last_block_sub_part = offset % BLOCK_SIZE
            # TODO: buggy implementation for sure !
            if last_block_sub_part:
                last_block = file_manifest.blocks[offset // BLOCK_SIZE]
                block_raw = yield self._fetch_block(last_block)
                content = (block_raw[:last_block_sub_part] + content +
                           block_raw[last_block_sub_part + len(content):])
            new_blocks = yield from self._slice_and_create_blocks(content)
            final_kept_blocks = file_manifest.blocks[len(kept_blocks) + len(new_blocks):]
            file_manifest.blocks = kept_blocks + new_blocks + final_kept_blocks
        file_manifest.updated = arrow.get()
        yield from self._commit_file_manifest(access, file_manifest, version)

    async def truncate(self, access: FileManifestAccess, length: int=None):
        file_manifest, version = yield from self._load_file_manifest(access)
        file_size = sum(x.size for x in file_manifest.blocks)
        if length >= file_size:
            # Nothing to do
            return
        kept_blocks = file_manifest.blocks[:length // BLOCK_SIZE]
        # It's possible we only keep a subpart of the last block
        last_block_sub_part = length % BLOCK_SIZE
        if last_block_sub_part:
            last_block = file_manifest.blocks[length // BLOCK_SIZE]
            block_raw = yield from self._fetch_block(last_block)
            content = block_raw[:last_block_sub_part]
            kept_blocks = kept_blocks + (yield from self._slice_and_create_blocks(content))
        file_manifest.blocks = kept_blocks
        file_manifest.updated = arrow.get()
        yield from self._commit_file_manifest(access, file_manifest, version)


@attr.s
class WorkspaceReader:
    _backend = attr.ib()
    _identity = attr.ib()

    async def _fetch_user_manifest(self):
        vlob_atom = yield Effect(UserManifestRead())
        if not vlob_atom.blob:
            user_raw = vlob_atom.blob
        else:
            user_raw = self.identity.private_key.decrypt(vlob_atom.blob)
        return load_user_manifest(user_raw)

    async def get(self, path):
        user_manifest = yield from self._fetch_user_manifest()
        return _retrieve_path(user_manifest, path)


@attr.s
class WorkspaceWriter:
    _backend = attr.ib()
    _identity = attr.ib()

    async def _fetch_user_manifest(self):
        vlob_atom = yield Effect(UserManifestRead())
        if not vlob_atom.blob:
            user_raw = vlob_atom.blob
        else:
            user_raw = identity.private_key.decrypt(vlob_atom.blob)
        return load_user_manifest(user_raw), vlob_atom.version

    async def _commit_user_manifest(self, user_manifest, version):
        raw = dump_user_manifest(user_manifest)
        blob = self.identity.public_key.encrypt(raw)
        yield Effect(UserVlobUpdate(version + 1, blob))

    async def insert(self, path, entry):
        user_manifest, version = await self._fetch_user_manifest()
        _check_path(user_manifest, path, should_exists=False)
        dirpath, name = path.rsplit('/', 1)
        dirobj = _retrieve_path(user_manifest, dirpath)
        dirobj.children[name] = entry
        await self._commit_user_manifest(user_manifest, version)

    async def move(self, src, dst):
        user_manifest, version = await self._fetch_user_manifest()

        _check_path(user_manifest, src, should_exists=True)
        _check_path(user_manifest, dst, should_exists=False)

        srcdirpath, scrfilename = src.rsplit('/', 1)
        dstdirpath, dstfilename = dst.rsplit('/', 1)

        srcobj = _retrieve_path(user_manifest, srcdirpath)
        dstobj = _retrieve_path(user_manifest, dstdirpath)
        dstobj.children[dstfilename] = srcobj.children[scrfilename]
        del srcobj.children[scrfilename]

        await self._commit_user_manifest(user_manifest, version)

    async def delete(self, path: str):
        user_manifest, version = await self._fetch_user_manifest()

        _check_path(user_manifest, path, should_exists=True)
        dirpath, leafname = path.rsplit('/', 1)
        obj = _retrieve_path(user_manifest, dirpath)
        del obj.children[leafname]

        await self._commit_user_manifest(user_manifest, version)


def _retrieve_file(workspace, path):
    fileobj = _retrieve_path(workspace, path)
    if not isinstance(fileobj, FileManifestAccess):
        raise InvalidPath("Path `%s` is not a file" % path)
    return fileobj


def _check_path(workspace, path, should_exists=True, type=None):
    if path == '/':
        if not should_exists or type not in ('folder', None):
            raise InvalidPath('Root `/` folder always exists')
        else:
            return
    dirpath, leafname = path.rsplit('/', 1)
    try:
        obj = _retrieve_path(workspace, dirpath)
    except InvalidPath:
        raise InvalidPath("Path `%s` doesn't exist" % (path if should_exists else dirpath))
    if not isinstance(obj, FolderEntry):
        raise InvalidPath("Path `%s` is not a folder" % path)
    try:
        leafobj = obj.children[leafname]
        if not should_exists:
            raise InvalidPath("Path `%s` already exist" % path)
        if (type == 'file' and not isinstance(leafobj, FileManifestAccess) or
                type == 'folder' and not isinstance(leafobj, FolderEntry)):
            raise InvalidPath("Path `%s` is not a %s" % (path, type))
    except KeyError:
        if should_exists:
            raise InvalidPath("Path `%s` doesn't exist" % path)


def _retrieve_path(workspace, path):
    if not path:
        return workspace
    if not path.startswith('/'):
        raise InvalidPath("Path must start with `/`")
    parent_dir = cur_dir = workspace
    reps = path.split('/')
    for rep in reps:
        if not rep or rep == '.':
            continue
        elif rep == '..':
            cur_dir = parent_dir
        else:
            try:
                parent_dir, cur_dir = cur_dir, cur_dir.children[rep]
            except KeyError:
                raise InvalidPath("Path `%s` doesn't exist" % path)
    return cur_dir
