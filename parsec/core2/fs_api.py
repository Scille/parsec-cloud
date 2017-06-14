import asyncio
import arrow
from marshmallow import fields, validate

from parsec.core2.workspace import Workspace, Reader, Writer
from parsec.service import event, cmd, service, ServiceMixin
from parsec.tools import BaseCmdSchema, to_jsonb64, async_callback
from parsec.exceptions import InvalidPath


class PathOnlySchema(BaseCmdSchema):
    path = fields.String(required=True)
path_only_schema = PathOnlySchema()


class cmd_FILE_READ_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    offset = fields.Int(missing=0, validate=validate.Range(min=0))
    size = fields.Int(missing=-1, validate=validate.Range(min=0))


class cmd_FILE_WRITE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    offset = fields.Int(missing=0, validate=validate.Range(min=0))
    content = fields.Base64Bytes(required=True)


class cmd_MOVE_Schema(BaseCmdSchema):
    src = fields.String(required=True)
    dst = fields.String(required=True)


class cmd_FILE_TRUNCATE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    length = fields.Int(required=True, validate=validate.Range(min=0))


class BaseFSAPIMixin(ServiceMixin):

    on_file_changed = event('file_changed')
    on_folder_changed = event('folder_changed')

    @cmd('file_create')
    async def _cmd_FILE_CREATE(self, session, msg):
        msg = path_only_schema.load(msg)
        await self.file_create(msg['path'])
        return {'status': 'ok'}

    @cmd('file_write')
    async def _cmd_FILE_WRITE(self, session, msg):
        msg = cmd_FILE_WRITE_Schema().load(msg)
        await self.file_write(msg['path'], msg['content'], msg['offset'])
        return {'status': 'ok'}

    @cmd('file_read')
    async def _cmd_FILE_READ(self, session, msg):
        msg = cmd_FILE_READ_Schema().load(msg)
        ret = await self.file_read(msg['path'], msg['offset'], msg['size'])
        return {'status': 'ok', 'content': to_jsonb64(ret)}

    @cmd('stat')
    async def _cmd_STAT(self, session, msg):
        msg = path_only_schema.load(msg)
        ret = await self.stat(msg['path'])
        return {'status': 'ok', **ret}

    @cmd('folder_create')
    async def _cmd_FOLDER_CREATE(self, session, msg):
        msg = path_only_schema.load(msg)
        await self.folder_create(msg['path'])
        return {'status': 'ok'}

    @cmd('move')
    async def _cmd_MOVE(self, session, msg):
        msg = cmd_MOVE_Schema().load(msg)
        await self.move(msg['src'], msg['dst'])
        return {'status': 'ok'}

    @cmd('delete')
    async def _cmd_DELETE(self, session, msg):
        msg = path_only_schema.load(msg)
        await self.delete(msg['path'])
        return {'status': 'ok'}

    @cmd('file_truncate')
    async def _cmd_FILE_TRUNCATE(self, session, msg):
        msg = cmd_FILE_TRUNCATE_Schema().load(msg)
        await self.file_truncate(msg['path'], msg['length'])
        return {'status': 'ok'}

    async def wait_for_ready(self):
        raise NotImplementedError()

    async def file_create(self, path: str):
        raise NotImplementedError()

    async def file_write(self, path: str, content: bytes, offset: int=0):
        raise NotImplementedError()

    async def file_read(self, path: str, offset: int=0, size: int=-1):
        raise NotImplementedError()

    async def stat(self, path: str):
        raise NotImplementedError()

    async def folder_create(self, path: str):
        raise NotImplementedError()

    async def move(self, src: str, dst: str):
        raise NotImplementedError()

    async def delete(self, path: str):
        raise NotImplementedError()

    async def file_truncate(self, path: str, length: int):
        raise NotImplementedError()


class MockedFSAPIMixin(BaseFSAPIMixin):

    identity = service('IdentityService')

    def __init__(self):
        now = arrow.get()
        self._fs = {
            'type': 'folder',
            'children': {},
            'stat': {'created': now, 'updated': now}
        }

    async def wait_for_ready(self):
        return

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
        try:
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
        except InvalidPath:
            raise InvalidPath("Path `%s` doesn't exist" % (path if should_exists else dirpath))

    def _retrieve_path(self, path):
        if not path:
            return self._fs
        if not path.startswith('/'):
            raise InvalidPath("Path must start with `/`")
        cur_dir = self._fs
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

    async def file_create(self, path: str):
        self.identity.id  # Trigger exception if identity is not loaded
        self._check_path(path, should_exists=False)
        dirpath, name = path.rsplit('/', 1)
        dirobj = self._retrieve_path(dirpath)
        now = arrow.get()
        dirobj['children'][name] = {
            'type': 'file', 'data': b'', 'stat': {'created': now, 'updated': now}
        }

    async def file_write(self, path: str, content: bytes, offset: int=0):
        self.identity.id  # Trigger exception if identity is not loaded
        self._check_path(path, should_exists=True, type='file')
        fileobj = self._retrieve_file(path)
        fileobj['data'] = (fileobj['data'][:offset] + content +
                           fileobj['data'][offset + len(content):])
        fileobj['stat']['updated'] = arrow.get()

    async def file_read(self, path: str, offset: int=0, size: int=-1):
        self.identity.id  # Trigger exception if identity is not loaded
        self._check_path(path, should_exists=True, type='file')
        fileobj = self._retrieve_file(path)
        if size < 0:
            return fileobj['data'][offset:]
        else:
            return fileobj['data'][offset:offset + size]

    async def stat(self, path: str):
        self.identity.id  # Trigger exception if identity is not loaded
        self._check_path(path, should_exists=True)
        obj = self._retrieve_path(path)
        if obj['type'] == 'folder':
            return {**obj['stat'], 'type': obj['type'], 'children': list(obj['children'].keys())}
        else:
            return {**obj['stat'], 'type': obj['type'], 'size': len(obj['data'])}

    async def folder_create(self, path: str):
        self.identity.id  # Trigger exception if identity is not loaded
        self._check_path(path, should_exists=False)
        dirpath, name = path.rsplit('/', 1)
        dirobj = self._retrieve_path(dirpath)
        now = arrow.get()
        dirobj['children'][name] = {
            'type': 'folder', 'children': {}, 'stat': {'created': now, 'updated': now}}

    async def move(self, src: str, dst: str):
        self.identity.id  # Trigger exception if identity is not loaded
        self._check_path(src, should_exists=True)
        self._check_path(dst, should_exists=False)

        srcdirpath, scrfilename = src.rsplit('/', 1)
        dstdirpath, dstfilename = dst.rsplit('/', 1)

        srcobj = self._retrieve_path(srcdirpath)
        dstobj = self._retrieve_path(dstdirpath)
        dstobj['children'][dstfilename] = srcobj['children'][scrfilename]
        del srcobj['children'][scrfilename]

    async def delete(self, path: str):
        self.identity.id  # Trigger exception if identity is not loaded
        self._check_path(path, should_exists=True)
        dirpath, leafname = path.rsplit('/', 1)
        obj = self._retrieve_path(dirpath)
        del obj['children'][leafname]

    async def file_truncate(self, path: str, length: int):
        self.identity.id  # Trigger exception if identity is not loaded
        self._check_path(path, should_exists=True, type='file')
        fileobj = self._retrieve_file(path)
        fileobj['data'] = fileobj['data'][:length]
        fileobj['stat']['updated'] = arrow.get()


class FSAPIMixin(BaseFSAPIMixin):

    identity = service('IdentityService')
    backend = service('BackendAPIService')

    def __init__(self):
        self._workspace = None
        self._reader = None
        self._writer = None
        self._ready_future = asyncio.futures.Future()

    async def wait_for_ready(self):
        await self._ready_future

    async def bootstrap(self):
        self._reader = Reader(self.identity, self.backend)
        self._writer = Writer(self.identity, self.backend)
        self.identity.on_identity_loaded.connect(
            async_callback(lambda x: self._load_workspace()), weak=False)
        self.identity.on_identity_unloaded.connect(
            async_callback(lambda x: self._unload_workspace()), weak=False)

    async def _load_workspace(self):
        # self._workspace = workspace_factory()
        self._workspace = Workspace()
        self._ready_future.set_result(None)

    async def _unload_workspace(self):
        self._ready_future = asyncio.futures.Future()
        self._workspace = None

    async def file_create(self, path: str):
        await self._writer.file_create(self._workspace, path)

    async def file_write(self, path: str, content: bytes, offset: int=0):
        await self._writer.file_write(self._workspace, path, content, offset)

    async def file_read(self, path: str, offset: int=0, size: int=-1):
        return await self._reader.file_read(self._workspace, path, offset, size)

    async def stat(self, path: str):
        return await self._reader.stat(self._workspace, path)

    async def folder_create(self, path: str):
        await self._writer.folder_create(self._workspace, path)

    async def move(self, src: str, dst: str):
        await self._writer.move(self._workspace, src, dst)

    async def delete(self, path: str):
        await self._writer.delete(self._workspace, path)

    async def file_truncate(self, path: str, length: int):
        await self._writer.file_truncate(self._workspace, path, length)
