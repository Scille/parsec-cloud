from marshmallow import fields, validate

from parsec.core.fs import BaseFolder, BaseFile
from parsec.utils import BaseCmdSchema, to_jsonb64


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
    def __init__(self, fs):
        self.fs = fs

    async def _cmd_FILE_CREATE(self, req):
        req = PathOnlySchema().load(req)
        dirpath, filename = req['path'].rsplit('/', 1)
        parent = await self.fs.fetch_path(dirpath or '/')
        if not isinstance(parent, BaseFolder):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a directory' % parent.path}
        new_file = parent.create_file(filename)
        new_file.flush()
        parent.flush()
        return {'status': 'ok'}

    async def _cmd_FILE_READ(self, req):
        req = cmd_FILE_READ_Schema().load(req)
        file = await self.fs.fetch_path(req['path'])
        if not isinstance(file, BaseFile):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a file' % file.path}
        content = await file.read(req['size'], req['offset'])
        return {'status': 'ok', 'content': to_jsonb64(content)}

    async def _cmd_FILE_WRITE(self, req):
        req = cmd_FILE_WRITE_Schema().load(req)
        file = await self.fs.fetch_path(req['path'])
        if not isinstance(file, BaseFile):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a file' % file.path}
        file.write(req['content'], req['offset'])
        return {'status': 'ok'}

    async def _cmd_FILE_TRUNCATE(self, req):
        req = cmd_FILE_TRUNCATE_Schema().load(req)
        file = await self.fs.fetch_path(req['path'])
        if not isinstance(file, BaseFile):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a file' % file.path}
        file.truncate(req['length'])
        return {'status': 'ok'}

    async def _cmd_STAT(self, req):
        req = PathOnlySchema().load(req)
        obj = await self.fs.fetch_path(req['path'])
        if isinstance(obj, BaseFolder):
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

    async def _cmd_FOLDER_CREATE(self, req):
        req = PathOnlySchema().load(req)
        dirpath, name = req['path'].rsplit('/', 1)
        parent = await self.fs.fetch_path(dirpath or '/')
        if not isinstance(parent, BaseFolder):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a directory' % parent.path}
        child = parent.create_folder(name)
        child.flush()
        parent.flush()
        return {'status': 'ok'}

    async def _cmd_MOVE(self, req):
        req = cmd_MOVE_Schema().load(req)
        srcdirpath, srcfilename = req['src'].rsplit('/', 1)
        dstdirpath, dstfilename = req['dst'].rsplit('/', 1)

        srcparent = await self.fs.fetch_path(srcdirpath or '/')
        dstparent = await self.fs.fetch_path(dstdirpath or '/')

        if not isinstance(srcparent, BaseFolder):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a directory' % srcparent.path}
        if not isinstance(dstparent, BaseFolder):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a directory' % dstparent.path}

        obj = await srcparent.fetch_child(srcfilename)
        dstparent.insert_child(dstfilename, obj)
        srcparent.delete_child(srcfilename)

        dstparent.flush()
        if srcparent != dstparent:
            srcparent.flush()
        return {'status': 'ok'}

    async def _cmd_DELETE(self, req):
        req = PathOnlySchema().load(req)
        dirpath, name = req['path'].rsplit('/', 1)
        parent = await self.fs.fetch_path(dirpath or '/')
        if not isinstance(parent, BaseFolder):
            return {'status': 'invalid_path', 'reason': 'Path `%s` is not a directory' % parent.path}
        parent.delete_child(name)
        parent.flush()
        return {'status': 'ok'}

    async def _cmd_FLUSH(self, req):
        req = PathOnlySchema().load(req)
        obj = await self.fs.fetch_path(req['path'])
        obj.flush()
        return {'status': 'ok'}

    async def _cmd_SYNCHRONISE(self, req):
        req = PathOnlySchema().load(req)
        obj = await self.fs.fetch_path(req['path'])
        await obj.sync()
        return {'status': 'ok'}
