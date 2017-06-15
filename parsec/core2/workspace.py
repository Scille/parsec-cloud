import arrow
import json
import struct

from parsec.exceptions import InvalidPath
from parsec.tools import to_jsonb64, from_jsonb64, json_dumps
from parsec.crypto import generate_sym_key, load_sym_key


class VlobAccess:
    def __init__(self, id, key, signature, read_trust_seed, write_trust_seed):
        self.id = id
        self.key = key
        self.signature = signature
        self.read_trust_seed = read_trust_seed
        self.write_trust_seed = write_trust_seed


class File:
    def __init__(self, created=None, updated=None, data=None, version=1, vlob_access=None):
        self.data = data or b''
        now = arrow.get()
        self.version = version
        self.created = created or now
        self.updated = updated or now
        self.vlob_access = vlob_access


class Folder:
    def __init__(self, created=None, updated=None, children=None):
        self.children = children or {}
        now = arrow.get()
        self.created = created or now
        self.updated = updated or now


class Workspace(Folder):
    def __init__(self, created=None, updated=None, children=None, version=0):
        super().__init__(created, updated, children)
        self.version = version


def _retrieve_file(workspace, path):
    fileobj = _retrieve_path(workspace, path)
    if not isinstance(fileobj, File):
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
        if not isinstance(obj, Folder):
            raise InvalidPath("Path `%s` is not a folder" % path)
        try:
            leafobj = obj.children[leafname]
            if not should_exists:
                raise InvalidPath("Path `%s` already exist" % path)
            if (type == 'file' and not isinstance(leafobj, File) or
                    type == 'folder' and not isinstance(leafobj, Folder)):
                raise InvalidPath("Path `%s` is not a %s" % (path, type))
        except KeyError:
            if should_exists:
                raise InvalidPath("Path `%s` doesn't exist" % path)
    except InvalidPath:
        raise InvalidPath("Path `%s` doesn't exist" % (path if should_exists else dirpath))


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



class Reader:
    def __init__(self, identity, backend):
        self._identity = identity
        self._backend = backend

    async def file_read(self, workspace: Workspace, path: str, offset: int=0, size: int=-1):
        self._identity.id
        _check_path(workspace, path, should_exists=True, type='file')
        fileobj = _retrieve_file(workspace, path)
        if size < 0:
            return fileobj.data[offset:]
        else:
            return fileobj.data[offset:offset + size]

    async def stat(self, workspace: Workspace, path: str):
        self._identity.id
        _check_path(workspace, path, should_exists=True)
        obj = _retrieve_path(workspace, path)
        if isinstance(obj, Folder):
            return {'created': obj.created, 'updated': obj.updated,
                    'type': 'folder', 'children': list(obj.children.keys())}
        else:
            return {'created': obj.created, 'updated': obj.updated,
                    'type': 'file', 'size': len(obj.data)}


class Writer:
    def __init__(self, identity, backend):
        self._identity = identity
        self._backend = backend

    async def file_create(self, workspace, path: str):
        self._identity.id
        _check_path(workspace, path, should_exists=False)
        dirpath, name = path.rsplit('/', 1)
        dirobj = _retrieve_path(workspace, dirpath)
        dirobj.children[name] = File()

    async def file_write(self, workspace, path: str, content: bytes, offset: int=0):
        self._identity.id
        _check_path(workspace, path, should_exists=True, type='file')
        fileobj = _retrieve_file(workspace, path)
        fileobj.data = (fileobj.data[:offset] + content +
                           fileobj.data[offset + len(content):])
        fileobj.updated = arrow.get()

    async def folder_create(self, workspace, path: str):
        self._identity.id
        _check_path(workspace, path, should_exists=False)
        dirpath, name = path.rsplit('/', 1)
        dirobj = _retrieve_path(workspace, dirpath)
        dirobj.children[name] = Folder()

    async def move(self, workspace, src: str, dst: str):
        self._identity.id
        _check_path(workspace, src, should_exists=True)
        _check_path(workspace, dst, should_exists=False)

        srcdirpath, scrfilename = src.rsplit('/', 1)
        dstdirpath, dstfilename = dst.rsplit('/', 1)

        srcobj = _retrieve_path(workspace, srcdirpath)
        dstobj = _retrieve_path(workspace, dstdirpath)
        dstobj.children[dstfilename] = srcobj.children[scrfilename]
        del srcobj.children[scrfilename]

    async def delete(self, workspace, path: str):
        self._identity.id
        _check_path(workspace, path, should_exists=True)
        dirpath, leafname = path.rsplit('/', 1)
        obj = _retrieve_path(workspace, dirpath)
        del obj.children[leafname]

    async def file_truncate(self, workspace, path: str, length: int):
        self._identity.id
        _check_path(workspace, path, should_exists=True, type='file')
        fileobj = _retrieve_file(workspace, path)
        fileobj.data = fileobj.data[:length]
        fileobj.updated = arrow.get()


class SyncWriter(Writer):
    """Writer that directly synchronize with the backend"""

    async def file_create(self, workspace, path: str):
        await super().file_create(workspace, path)
        ret = await self._backend.vlob_create(b'')
        obj = _retrieve_file(workspace, path)
        key = generate_sym_key()
        obj.vlob_access = VlobAccess(
            id=ret['id'],
            key=key,
            signature=self._identity.private_key.sign(b''),
            read_trust_seed=ret['read_trust_seed'],
            write_trust_seed=ret['write_trust_seed']
        )

    async def file_write(self, workspace, path: str, content: bytes, offset: int=0):
        await super().file_write(workspace, path, content, offset)
        await self._save_file_manifest(workspace, path)

    async def _save_file_manifest(self, workspace, path):
        obj = _retrieve_file(workspace, path)
        assert obj.vlob_access is not None
        signature = self._identity.private_key.sign(obj.data)
        raw = _serialize_data_and_signature(obj.data, signature)
        ciphered = obj.vlob_access.key.encrypt(raw)
        await self._backend.vlob_update(
            obj.vlob_access.id, obj.version + 1,
            obj.vlob_access.write_trust_seed, ciphered)
        obj.version += 1

    async def file_truncate(self, workspace, path: str, length: int):
        await super().file_truncate(workspace, path, length)
        await self._save_file_manifest(workspace, path)

    async def folder_create(self, workspace, path: str):
        await super().folder_create(workspace, path)
        await save_workspace(self._identity, self._backend, workspace)

    async def move(self, workspace, src: str, dst: str):
        await super().move(workspace, src, dst)
        await save_workspace(self._identity, self._backend, workspace)

    async def delete(self, workspace, path: str):
        await super().delete(workspace, path)
        await save_workspace(self._identity, self._backend, workspace)


class SyncReader(Reader):

    async def file_read(self, workspace: Workspace, path: str, offset: int=0, size: int=-1):
        obj = _retrieve_file(workspace, path)
        assert obj.vlob_access is not None
        ret = await self._backend.vlob_read(obj.vlob_access.id, obj.vlob_access.read_trust_seed)
        ciphered = from_jsonb64(ret['blob'])
        raw = obj.vlob_access.key.decrypt(ciphered)
        data, signature = _deserialize_data_and_signature(raw)
        # TODO: use pubkey service to get the key to verify
        self._identity.public_key.verify(signature, data)
        obj.data = data
        obj.version = ret['version']
        return await super().file_read(workspace, path, offset, size)

    async def stat(self, workspace: Workspace, path: str):
        return await super().stat(workspace, path)


def _load_file(entry):
    assert isinstance(entry, dict)
    vlob_access = VlobAccess(
        id=entry['id'],
        key=load_sym_key(from_jsonb64(entry['key'])),
        signature=from_jsonb64(entry['signature']),
        read_trust_seed=entry['read_trust_seed'],
        write_trust_seed=entry['write_trust_seed']
    )
    return File(
        vlob_access=vlob_access
    )


def _load_folder(entry, folder_cls=Folder):
    assert isinstance(entry, dict)
    assert isinstance(entry['children'], dict)
    children = {}
    for name, child in entry['children'].items():
        if child['type'] == 'file':
            children[name] = _load_file(child)
        else:
            children[name] = _load_folder(child)
    return folder_cls(arrow.get(entry['created']), arrow.get(entry['updated']), children)


def workspace_factory(user_manifest):
    assert isinstance(user_manifest, dict)
    assert user_manifest['type'] == 'folder'
    return _load_folder(user_manifest, folder_cls=Workspace)


def _dump_user_manifest(workspace):

    def _dump(item):
        if isinstance(item, File):
            return {
                'type': 'file',
                'id': item.vlob_access.id,
                'key': to_jsonb64(item.vlob_access.key.key),
                'signature': to_jsonb64(item.vlob_access.signature),
                'read_trust_seed': item.vlob_access.read_trust_seed,
                'write_trust_seed': item.vlob_access.write_trust_seed
            }
        elif isinstance(item, Folder):
            return {
                'type': 'folder',
                'updated': item.updated.isoformat(),
                'created': item.created.isoformat(),
                'children': {k: _dump(v) for k, v in item.children.items()}
            }
        else:
            raise RuntimeError('Invalid node type %s' % item)

    assert isinstance(workspace, Workspace)
    return _dump(workspace)


def _serialize_data_and_signature(data, signature):
    return struct.pack('>I', len(signature)) + signature + data


def _deserialize_data_and_signature(raw):
    sign_len, = struct.unpack('>I', raw[:4])
    signature = raw[4:sign_len + 4]
    data = raw[sign_len + 4:]
    return data, signature


def pretty_print_workspace(workspace):
    out = []

    def _pretty_print(item, path):
        out.append('/' + path)
        if isinstance(item, Folder):
            for k, v in item.children.items():
                _pretty_print(v, '%s/%s' % (path, k))

    _pretty_print(workspace, '')
    return '\n'.join(out)


async def save_workspace(identity, backend, workspace):
    wksp_data = json_dumps(_dump_user_manifest(workspace)).encode()
    signature = identity.private_key.sign(wksp_data)
    vlobatom_data = _serialize_data_and_signature(wksp_data, signature)
    vlobatom_ciphered_data = identity.public_key.encrypt(vlobatom_data)
    await backend.user_vlob_update(workspace.version + 1, vlobatom_ciphered_data)
    workspace.version += 1


async def load_or_create_workspace(identity, backend):
    # TODO: concurrency and error handling not done
    vlobatom = await backend.user_vlob_read()
    if vlobatom['version'] == 0:
        # Create the workspace
        workspace = Workspace()
        await save_workspace(identity, backend, workspace)
        return workspace
    else:
        # TODO: backend_api_service should return cooked objects
        vlobatom_ciphered_data = from_jsonb64(vlobatom['blob'])
        vlobatom_data = identity.private_key.decrypt(vlobatom_ciphered_data)
        wksp_data, signature = _deserialize_data_and_signature(vlobatom_data)
        identity.public_key.verify(signature, wksp_data)
        return workspace_factory(json.loads(wksp_data.decode()))
