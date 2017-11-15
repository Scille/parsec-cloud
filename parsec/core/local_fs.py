import attr
import json
import pendulum
from uuid import uuid4
from marshmallow import fields, validate
from nacl.public import Box
from nacl.secret import SecretBox

from .local_storage import LocalStorage
from .local_user_manifest import (
    LocalUserManifest, decrypt_and_load_local_user_manifest,
    dump_and_encrypt_local_user_manifest
)
from .file_manager import FileManager
from .backend_connection import BackendConnection
from .synchronizer import Synchronizer
from parsec.utils import BaseCmdSchema, from_jsonb64, to_jsonb64, abort, ParsecError


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


class LocalFS:
    def __init__(self, user, backend_addr):
        self.user = user
        self.local_storage = LocalStorage()
        self.backend_conn = BackendConnection(user, backend_addr)
        self.synchronizer = Synchronizer(self, self.backend_conn)
        self.local_user_manifest = None
        self.files_manager = FileManager(self.local_storage)

    async def init(self, nursery):
        ciphered = self.local_storage.get_local_user_manifest()
        if not ciphered:
            self.local_user_manifest = LocalUserManifest()
        else:
            self.local_user_manifest = decrypt_and_load_local_user_manifest(
                self.user.privkey, ciphered)
        await self.backend_conn.init(nursery)
        await self.synchronizer.init(nursery)

    async def teardown(self):
        await self.backend_conn.teardown()

    def _sync_local_user_manifest(self):
        ciphered = dump_and_encrypt_local_user_manifest(
            self.user.privkey, self.local_user_manifest)
        self.local_storage.save_local_user_manifest(ciphered)

    async def _cmd_FILE_CREATE(self, req):
        req = PathOnlySchema().load(req)
        path = req['path']
        self.local_user_manifest.check_path(path, should_exists=False)
        file, key = self.files_manager.create_placeholder_file()
        dirpath, name = path.rsplit('/', 1)
        dirobj = self.local_user_manifest.retrieve_path(dirpath)
        dirobj['children'][name] = {
            'type': 'placeholder_file',
            'id': file.id,
            'key': to_jsonb64(key)
        }
        self.local_user_manifest.file_placeholders.add(path)
        self.local_user_manifest.is_dirty = True
        self._sync_local_user_manifest()
        file.sync()
        return {'status': 'ok'}

    async def _cmd_FILE_READ(self, req):
        req = cmd_FILE_READ_Schema().load(req)
        path = req['path']
        self.local_user_manifest.check_path(path, should_exists=True, type='file')
        obj = self.local_user_manifest.retrieve_path(path)
        key = from_jsonb64(obj['key'])
        if obj['type'] == 'placeholder_file':
            file = self.files_manager.get_placeholder_file(obj['id'], key)
        else:
            file = await self.files_manager.get_file(obj['id'], obj['read_trust_seed'], obj['write_trust_seed'], key)
        if not file:
            # Data not in local and backend is offline
            abort('unavailable_resource')
        content = await file.read(req['size'], req['offset'])
        return {'status': 'ok', 'content': to_jsonb64(content)}

    async def _cmd_FILE_WRITE(self, req):
        req = cmd_FILE_WRITE_Schema().load(req)
        path = req['path']
        self.local_user_manifest.check_path(path, should_exists=True, type='file')
        obj = self.local_user_manifest.retrieve_path(path)
        key = from_jsonb64(obj['key'])
        if obj['type'] == 'placeholder_file':
            file = self.files_manager.get_placeholder_file(obj['id'], key)
        else:
            file = await self.files_manager.get_file(obj['id'], obj['read_trust_seed'], obj['write_trust_seed'], key)
        if not file:
            # Data not in local and backend is offline
            abort('unavailable_resource')
        file.write(req['content'], req['offset'])
        if obj['type'] == 'file':
            self.local_user_manifest.dirty_files.add(path)
        return {'status': 'ok'}

    async def _cmd_FILE_TRUNCATE(self, req):
        req = cmd_FILE_TRUNCATE_Schema().load(req)
        path = req['path']
        self.local_user_manifest.check_path(path, should_exists=True, type='file')
        obj = self.local_user_manifest.retrieve_path(path)
        key = from_jsonb64(obj['key'])
        if obj['type'] == 'placeholder_file':
            file = self.files_manager.get_placeholder_file(obj['id'], key)
        else:
            file = await self.files_manager.get_file(obj['id'], obj['read_trust_seed'], obj['write_trust_seed'], key)
        if not file:
            # Data not in local and backend is offline
            abort('unavailable_resource')
        file.truncate(req['length'])
        if obj['type'] == 'file':
            self.local_user_manifest.dirty_files.add(path)
        return {'status': 'ok'}

    async def _cmd_FILE_SYNC(self, req):
        req = PathOnlySchema().load(req)
        path = req['path']
        self.local_user_manifest.check_path(path, should_exists=True, type='file')
        obj = self.local_user_manifest.retrieve_path(path)
        key = from_jsonb64(obj['key'])
        if obj['type'] == 'placeholder_file':
            file = self.files_manager.get_placeholder_file(obj['id'], key)
        else:
            file = await self.files_manager.get_file(obj['id'], obj['read_trust_seed'], obj['write_trust_seed'], key)
        if not file:
            # Data not in local and backend is offline
            abort('unavailable_resource')
        file.sync()
        return {'status': 'ok'}

    async def _cmd_STAT(self, req):
        req = PathOnlySchema().load(req)
        path = req['path']
        self.local_user_manifest.check_path(path, should_exists=True)
        obj = self.local_user_manifest.retrieve_path(path)
        if obj['type'] == 'folder':
            return {
                'status': 'ok',
                'type': 'folder',
                'created': obj['created'],
                'children': list(sorted(obj['children'].keys()))
            }
        elif obj['type'] == 'file':
            key = from_jsonb64(obj['key'])
            file = await self.files_manager.get_file(obj['id'], obj['read_trust_seed'], obj['write_trust_seed'], key)
            if not file:
                # Data not in local and backend is offline
                abort('unavailable_resource')
            return {
                'status': 'ok',
                'type': 'file',
                'created': file.created,
                'updated': file.updated,
                'version': file.version,
                'is_dirty': file.is_dirty,
                'is_placeholder': False,
                'size': file.size
            }
        else:  # placeholder file
            key = from_jsonb64(obj['key'])
            file = self.files_manager.get_placeholder_file(obj['id'], key)
            if not file:
                # Data not in local and backend is offline, should never
                # happened with placeholder !
                abort('unavailable_resource')
            return {
                'status': 'ok',
                'type': 'file',
                'created': file.created,
                'updated': file.updated,
                'version': file.version,
                'is_dirty': True,
                'is_placeholder': True,
                'size': file.size
            }

    async def _cmd_FOLDER_CREATE(self, req):
        req = PathOnlySchema().load(req)
        path = req['path']
        self.local_user_manifest.check_path(path, should_exists=False)
        dirpath, name = path.rsplit('/', 1)
        dirobj = self.local_user_manifest.retrieve_path(dirpath)
        now = pendulum.utcnow().isoformat()
        dirobj['children'][name] = {
            'type': 'folder', 'children': {}, 'stat': {'created': now, 'updated': now}}
        self._sync_local_user_manifest()
        self.local_user_manifest.is_dirty = True
        return {'status': 'ok'}

    async def _cmd_MOVE(self, req):
        req = cmd_MOVE_Schema().load(req)
        src = req['src']
        dst = req['dst']

        self.local_user_manifest.check_path(src, should_exists=True)
        self.local_user_manifest.check_path(dst, should_exists=False)

        srcdirpath, scrfilename = src.rsplit('/', 1)
        dstdirpath, dstfilename = dst.rsplit('/', 1)

        srcobj = self.local_user_manifest.retrieve_path(srcdirpath)
        dstobj = self.local_user_manifest.retrieve_path(dstdirpath)
        dstobj['children'][dstfilename] = srcobj['children'][scrfilename]
        del srcobj['children'][scrfilename]
        self.local_user_manifest.is_dirty = True
        self._sync_local_user_manifest()
        return {'status': 'ok'}

    async def _cmd_DELETE(self, req):
        req = PathOnlySchema().load(req)
        path = req['path']
        self.local_user_manifest.check_path(path, should_exists=True)
        dirpath, leafname = path.rsplit('/', 1)
        obj = self.local_user_manifest.retrieve_path(dirpath)
        del obj['children'][leafname]
        self.local_user_manifest.is_dirty = True
        self._sync_local_user_manifest()
        return {'status': 'ok'}

    async def _cmd_SYNCHRONISE(self, req):
        BaseCmdSchema().load(req)
        # TODO: sync file and user manifest if placeholder file
        return {'status': 'not_implemented'}
