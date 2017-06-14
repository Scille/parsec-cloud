import arrow
import json

from parsec.exceptions import InvalidPath
from parsec.tools import to_jsonb64, from_jsonb64


class UserManifest:
    def __init__(self):
        pass


class FileManifest:
    def __init__(self):
        pass


class FileBlock:
    def __init__(self):
        pass


class File:
    def __init__(self, created=None, updated=None, data=None):
        self.data = data or b''
        now = arrow.get()
        self.created = created or now
        self.updated = updated or now

    def dump(self):
        return json.dumps({
            'data': to_jsonb64(self.data),
            'created': self.created.toisoformat(),
            'updated': self.updated.toisoformat()
        })

    @classmethod
    def load(cls, payload):
        jsonpayload = json.loads(payload)
        return cls(
            created=arrow.get(jsonpayload['created']),
            updated=arrow.get(jsonpayload['updated']),
            data=from_jsonb64(jsonpayload['data'])
        )


class Folder:
    def __init__(self, created=None, updated=None, children=None):
        self.children = children or {}
        now = arrow.get()
        self.created = created or now
        self.updated = updated or now

    # def dump(self):
    #     return json.dumps({
    #         'children': {k: v.dump() for k, v in },
    #         'created': self.created.toisoformat(),
    #         'updated': self.updated.toisoformat()
    #     })

    # @classmethod
    # def load(cls, payload):
    #     jsonpayload = json.loads(payload)
    #     return cls(
    #         created=arrow.get(jsonpayload['created']),
    #         updated=arrow.get(jsonpayload['updated']),
    #         data=from_jsonb64(jsonpayload['data'])
    #     )

class Workspace(Folder):
    pass
    # def dump(self):
    #     pass

    # def load(self):
    #     pass


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

    async def file_read(self, workspace: Workspace, path: str, offset: int=0, size: int=-1):
        _check_path(workspace, path, should_exists=True, type='file')
        fileobj = _retrieve_file(workspace, path)
        return fileobj.data[offset:offset + size]

    async def stat(self, workspace: Workspace, path: str):
        _check_path(workspace, path, should_exists=True)
        obj = _retrieve_path(workspace, path)
        if isinstance(obj, Folder):
            return {'created': obj.created, 'updated': obj.updated,
                    'type': 'folder', 'children': list(obj.children.keys())}
        else:
            return {'created': obj.created, 'updated': obj.updated,
                    'type': 'file', 'size': len(obj.data)}


class Writer:

    async def file_create(self, workspace, path: str):
        _check_path(workspace, path, should_exists=False)
        dirpath, name = path.rsplit('/', 1)
        dirobj = _retrieve_path(workspace, dirpath)
        dirobj.children[name] = File()

    async def file_write(self, workspace, path: str, content: bytes, offset: int=0):
        _check_path(workspace, path, should_exists=True, type='file')
        fileobj = _retrieve_file(workspace, path)
        fileobj.data = (fileobj.data[:offset] + content +
                           fileobj.data[offset + len(content):])
        fileobj.updated = arrow.get()

    async def folder_create(self, workspace, path: str):
        _check_path(workspace, path, should_exists=False)
        dirpath, name = path.rsplit('/', 1)
        dirobj = _retrieve_path(workspace, dirpath)
        dirobj.children[name] = Folder()

    async def move(self, workspace, src: str, dst: str):
        _check_path(workspace, src, should_exists=True)
        _check_path(workspace, dst, should_exists=False)

        srcdirpath, scrfilename = src.rsplit('/', 1)
        dstdirpath, dstfilename = dst.rsplit('/', 1)

        srcobj = _retrieve_path(workspace, srcdirpath)
        dstobj = _retrieve_path(workspace, dstdirpath)
        dstobj.children[dstfilename] = srcobj.children[scrfilename]
        del srcobj.children[scrfilename]

    async def delete(self, workspace, path: str):
        _check_path(workspace, path, should_exists=True)
        dirpath, leafname = path.rsplit('/', 1)
        obj = _retrieve_path(workspace, dirpath)
        del obj.children[leafname]

    async def file_truncate(self, workspace, path: str, length: int):
        _check_path(workspace, path, should_exists=True, type='file')
        fileobj = _retrieve_file(workspace, path)
        fileobj.data = fileobj.data[:length]
        fileobj.updated = arrow.get()


class BaseWorkspaceSerializer:
    pass


def workspace_factory(user_manifest):
    pass

