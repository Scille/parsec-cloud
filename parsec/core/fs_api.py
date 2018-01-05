from parsec.core.fs import BaseFolderEntry, BaseFileEntry
from parsec.utils import to_jsonb64
from parsec.schema import BaseCmdSchema, fields, validate


class PathOnlySchema(BaseCmdSchema):
    path = fields.String(required=True)


class cmd_CREATE_GROUP_MANIFEST_Schema(BaseCmdSchema):
    group = fields.String()


class cmd_SHOW_dustbin_Schema(BaseCmdSchema):
    path = fields.String(missing=None)


class cmd_HISTORY_Schema(BaseCmdSchema):
    first_version = fields.Integer(missing=1, validate=lambda n: n >= 1)
    last_version = fields.Integer(missing=None, validate=lambda n: n >= 1)
    summary = fields.Boolean(missing=False)


class cmd_RESTORE_MANIFEST_Schema(BaseCmdSchema):
    version = fields.Integer(missing=None, validate=lambda n: n >= 1)


class cmd_FILE_READ_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    offset = fields.Int(missing=0, validate=validate.Range(min=0))
    size = fields.Int(missing=None, validate=validate.Range(min=0))


class cmd_FILE_WRITE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    offset = fields.Int(missing=0, validate=validate.Range(min=0))
    content = fields.Base64Bytes(required=True)


class cmd_FILE_TRUNCATE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    length = fields.Int(required=True, validate=validate.Range(min=0))


class cmd_FILE_HISTORY_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    first_version = fields.Int(missing=1, validate=validate.Range(min=1))
    last_version = fields.Int(missing=None, validate=validate.Range(min=1))


class cmd_FILE_RESTORE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    version = fields.Int(required=True, validate=validate.Range(min=1))


class cmd_MOVE_Schema(BaseCmdSchema):
    src = fields.String(required=True)
    dst = fields.String(required=True)


class cmd_UNDELETE_Schema(BaseCmdSchema):
    vlob = fields.String(required=True)


class FSApi:
    def __init__(self, fs=None):
        self.fs = fs

    async def file_create(self, req):
        if not self.fs:
            return {'status': 'login_required'}

        req = PathOnlySchema().load_or_abort(req)
        dirpath, filename = req['path'].rsplit('/', 1)
        parent = await self.fs.fetch_path(dirpath or '/')
        if not isinstance(parent, BaseFolderEntry):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a directory' % parent.path}
        new_file = await parent.create_file(filename)
        await new_file.flush()
        await parent.flush()
        return {'status': 'ok'}

    async def file_read(self, req):
        if not self.fs:
            return {'status': 'login_required'}

        req = cmd_FILE_READ_Schema().load_or_abort(req)
        file = await self.fs.fetch_path(req['path'])
        if not isinstance(file, BaseFileEntry):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a file' % file.path}
        content = await file.read(req['size'], req['offset'])
        return {'status': 'ok', 'content': to_jsonb64(content)}

    async def file_write(self, req):
        if not self.fs:
            return {'status': 'login_required'}

        req = cmd_FILE_WRITE_Schema().load_or_abort(req)
        file = await self.fs.fetch_path(req['path'])
        if not isinstance(file, BaseFileEntry):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a file' % file.path}
        await file.write(req['content'], req['offset'])
        return {'status': 'ok'}

    async def file_truncate(self, req):
        if not self.fs:
            return {'status': 'login_required'}

        req = cmd_FILE_TRUNCATE_Schema().load_or_abort(req)
        file = await self.fs.fetch_path(req['path'])
        if not isinstance(file, BaseFileEntry):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a file' % file.path}
        await file.truncate(req['length'])
        return {'status': 'ok'}

    async def stat(self, req):
        if not self.fs:
            return {'status': 'login_required'}

        req = PathOnlySchema().load_or_abort(req)
        obj = await self.fs.fetch_path(req['path'])
        if isinstance(obj, BaseFolderEntry):
            return {
                'status': 'ok',
                'type': 'folder',
                'created': obj.created.isoformat(),
                'updated': obj.updated.isoformat(),
                'base_version': obj.base_version,
                'is_placeholder': obj.is_placeholder,
                'need_sync': obj.need_sync,
                'need_flush': obj.need_flush,
                'children': list(sorted(obj.keys()))
            }
        else:
            return {
                'status': 'ok',
                'type': 'file',
                'created': obj.created.isoformat(),
                'updated': obj.updated.isoformat(),
                'base_version': obj.base_version,
                'is_placeholder': obj.is_placeholder,
                'need_sync': obj.need_sync,
                'need_flush': obj.need_flush,
                'size': obj.size
            }

    async def folder_create(self, req):
        if not self.fs:
            return {'status': 'login_required'}

        req = PathOnlySchema().load_or_abort(req)
        dirpath, name = req['path'].rsplit('/', 1)
        parent = await self.fs.fetch_path(dirpath or '/')
        if not isinstance(parent, BaseFolderEntry):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a directory' % parent.path}
        child = await parent.create_folder(name)
        await child.flush()
        await parent.flush()
        return {'status': 'ok'}

    async def move(self, req):
        if not self.fs:
            return {'status': 'login_required'}

        req = cmd_MOVE_Schema().load_or_abort(req)
        if req['src'] == '/':
            return {'status': 'invalid_path', 'reason': "Cannot move `/` root folder"}
        if req['dst'] == '/':
            return {'status': 'invalid_path', 'reason': "Path `/` already exists"}
        srcdirpath, srcfilename = req['src'].rsplit('/', 1)
        dstdirpath, dstfilename = req['dst'].rsplit('/', 1)

        srcparent = await self.fs.fetch_path(srcdirpath or '/')
        dstparent = await self.fs.fetch_path(dstdirpath or '/')

        if not isinstance(srcparent, BaseFolderEntry):
            return {
                'status': 'invalid_path',
                'reason': 'Path `%s` is not a directory' % srcparent.path
            }
        if not isinstance(dstparent, BaseFolderEntry):
            return {
                'status': 'invalid_path',
                'reason': 'Path `%s` is not a directory' % dstparent.path
            }

        if srcfilename not in srcparent:
            return {'status': 'invalid_path', 'reason': "Path `%s` doesn't exists" % req['src']}
        if dstfilename in dstparent:
            return {'status': 'invalid_path', 'reason': "Path `%s` already exists" % req['dst']}
        obj = await srcparent.delete_child(srcfilename)
        await dstparent.insert_child(dstfilename, obj)

        await dstparent.flush()
        if srcparent != dstparent:
            await srcparent.flush()
        return {'status': 'ok'}

    async def delete(self, req):
        if not self.fs:
            return {'status': 'login_required'}

        req = PathOnlySchema().load_or_abort(req)
        dirpath, name = req['path'].rsplit('/', 1)
        parent = await self.fs.fetch_path(dirpath or '/')
        if not isinstance(parent, BaseFolderEntry):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a directory' % parent.path}
        await parent.delete_child(name)
        await parent.flush()
        return {'status': 'ok'}

    async def flush(self, req):
        if not self.fs:
            return {'status': 'login_required'}

        req = PathOnlySchema().load_or_abort(req)
        obj = await self.fs.fetch_path(req['path'])
        await obj.flush()
        return {'status': 'ok'}

    async def synchronize(self, req):
        if not self.fs:
            return {'status': 'login_required'}

        req = PathOnlySchema().load_or_abort(req)
        obj = await self.fs.fetch_path(req['path'])
        to_sync = [obj]
        curr_path = req['path']
        while to_sync[-1].is_placeholder:
            curr_path, _ = curr_path.rsplit('/', 1)
            if not curr_path:
                curr_path = '/'
            to_sync.append(await self.fs.fetch_path(curr_path))
        for obj in reversed(to_sync):
            # Note we must explicitly sync children even if parent are sync !
            await obj.sync()
        return {'status': 'ok'}
