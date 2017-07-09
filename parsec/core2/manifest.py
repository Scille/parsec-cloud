import json
import attr
import arrow

from parsec.exceptions import InvalidManifest
from parsec.crypto import load_sym_key
from parsec.tools import from_jsonb64, to_jsonb64, ejson_dumps, ejson_loads


@attr.s
class BlockAccess:
    id = attr.ib()
    key = attr.ib()
    signature = attr.ib()
    size = attr.ib()


@attr.s
class FileManifest:
    created = attr.ib(default=None)
    updated = attr.ib(default=None)
    blocks = attr.ib(default=attr.Factory(list))
    version = attr.ib(default=0)

    def __attrs_post_init__(self):
        self.created = self.created or arrow.get()
        self.updated = self.updated or self.created


@attr.s
class FileManifestAccess:
    id = attr.ib()
    key = attr.ib()
    signature = attr.ib()
    read_trust_seed = attr.ib()
    write_trust_seed = attr.ib()


@attr.s
class FolderEntry:
    created = attr.ib(default=None)
    updated = attr.ib(default=None)
    children = attr.ib(default=attr.Factory(dict))

    def __attrs_post_init__(self):
        self.created = self.created or arrow.get()
        self.updated = self.updated or self.created


@attr.s
class UserManifestAccess:
    identity = attr.ib()


class UserManifest(FolderEntry):
    pass


def _load_datetime(path, fieldname, raw):
    try:
        return arrow.get(raw)
    except arrow.parser.ParserError:
        raise InvalidManifest('Entry %s contains invalid %s date `%s`' %
            (path, fieldname, raw))


def _load_file(path, entry):
    keys = set(entry.keys())
    expected_keys = {'type', 'id', 'key', 'signature', 'read_trust_seed', 'write_trust_seed'}
    if keys - expected_keys:
        raise InvalidManifest('Entry for path %s contains unknown fields: %s' %
            (path, keys - expected_keys))
    if expected_keys - keys:
        raise InvalidManifest('Entry for path %s miss mandatory fields: %s' %
            (path, expected_keys - keys))
    if entry['type'] != 'file':
        raise InvalidManifest('Entry %s should be a file' % path)
    for k, v in entry.items():
        if not isinstance(v, str):
            raise InvalidManifest('Entry %s contains non-string `%s` field' % k)
    return FileManifestAccess(
        id=entry['id'],
        key=load_sym_key(from_jsonb64(entry['key'])),
        signature=from_jsonb64(entry['signature']),
        read_trust_seed=entry['read_trust_seed'],
        write_trust_seed=entry['write_trust_seed']
    )


def _load_folder(path, entry, folder_cls=FolderEntry):
    keys = set(entry.keys())
    expected_keys = {'type', 'updated', 'created', 'children'}
    if keys - expected_keys:
        raise InvalidManifest('Entry for path %s contains unknown fields: %s' %
            (path, keys - expected_keys))
    if expected_keys - keys:
        raise InvalidManifest('Entry for path %s miss mandatory fields: %s' %
            (path, expected_keys - keys))
    if entry['type'] != 'folder':
        raise InvalidManifest('Entry %s should be a folder' % path)
    if not isinstance(entry['children'], dict):
        raise InvalidManifest('Entry %s contains non-dict `children` field')
    children = {}
    for name, child in entry['children'].items():
        subpath = '%s/%s' % (path, name)
        if child.get('type') == 'file':
            children[name] = _load_file(subpath, child)
        else:
            children[name] = _load_folder(subpath, child)
    return folder_cls(
        _load_datetime(path, 'created', entry['created']),
        _load_datetime(path, 'updated', entry['updated']),
        children)


def load_user_manifest(raw: bytes):
    if not raw:
        return UserManifest()
    try:
        wksp = ejson_loads(raw.decode())
    except json.JSONDecodeError as exc:
        raise InvalidManifest(str(exc))
    return _load_folder('/', wksp, folder_cls=UserManifest)


def dump_user_manifest(wksp: UserManifest):

    def _dump(item):
        if isinstance(item, FileManifestAccess):
            return {
                'type': 'file',
                'id': item.id,
                'key': to_jsonb64(item.key.key),
                'signature': to_jsonb64(item.signature),
                'read_trust_seed': item.read_trust_seed,
                'write_trust_seed': item.write_trust_seed
            }
        elif isinstance(item, FolderEntry):
            return {
                'type': 'folder',
                'updated': item.updated.isoformat(),
                'created': item.created.isoformat(),
                'children': {k: _dump(v) for k, v in item.children.items()}
            }
        else:
            raise RuntimeError('Invalid node type %s' % item)

    wksp_as_dict = _dump(wksp)
    return ejson_dumps(wksp_as_dict).encode()


def load_file_manifest(raw: bytes):
    try:
        data = ejson_loads(raw.decode())
    except json.JSONDecodeError as exc:
        raise InvalidManifest(str(exc))
    blocks = []
    for block in data['blocks']:
        blocks.append(BlockAccess(
            id=block['id'],
            key=load_sym_key(from_jsonb64(block['key'])),
            signature=from_jsonb64(block['signature']),
            size=block['size']
        ))
    return FileManifest(
        created=data['created'],
        updated=data['updated'],
        blocks=blocks
    )


def dump_file_manifest(fm: FileManifest):
    blocks_data = []
    for block in fm.blocks:
        blocks_data.append({
            'id': block.id,
            'key': to_jsonb64(block.key.key),
            'signature': to_jsonb64(block.signature),
            'size': block.size,
        })
    data = {
        'created': fm.created,
        'updated': fm.updated,
        'blocks': blocks_data
    }
    return ejson_dumps(data).encode()
