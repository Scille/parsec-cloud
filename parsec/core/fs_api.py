from copy import deepcopy
from marshmallow import fields, validate
from datetime import datetime, timezone

from parsec.core.file import File
from parsec.core.manifest import UserManifest
from parsec.service import event, cmd, ServiceMixin
from parsec.tools import BaseCmdSchema, to_jsonb64
from parsec.exceptions import FileError, FileNotFound, InvalidPath, ManifestError, ManifestNotFound, VlobNotFound


class PathOnlySchema(BaseCmdSchema):
    path = fields.String(required=True)
path_only_schema = PathOnlySchema()


class cmd_FILE_READ_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    offset = fields.Int(missing=0, validate=validate.Range(min=0))
    size = fields.Int(missing=None, validate=validate.Range(min=0))


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


class cmd_FILE_RESTORE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    version = fields.Int(required=True, validate=validate.Range(min=1))


class cmd_FILE_HISTORY_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    first_version = fields.Int(missing=1, validate=validate.Range(min=1))
    last_version = fields.Int(missing=None, validate=validate.Range(min=1))


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

    @cmd('undelete')
    async def _cmd_UNDELETE(self, session, msg):
        msg = path_only_schema.load(msg)
        await self.undelete(msg['path'])
        return {'status': 'ok'}

    @cmd('restore')
    async def _cmd_RESTORE(self, session, msg):
        msg = cmd_FILE_RESTORE_Schema().load(msg)
        await self.file_restore(msg['path'], msg['version'])
        return {'status': 'ok'}

    @cmd('history')
    async def _cmd_HISTORY(self, session, msg):
        msg = cmd_FILE_HISTORY_Schema().load(msg)
        ret = await self.history(msg['path'], msg['first_version'], msg['last_version'])
        return {'status': 'ok', 'history': ret}

    # @cmd('file_reencrypt')
    # async def _cmd_REENCRYPT(self, session, msg):
    #     msg = cmd_REENCRYPT_FILE_Schema().load(msg)
    #     file = await self.file_reencrypt(msg['id'])
    #     file.update({'status': 'ok'})
    #     return file

    @cmd('file_truncate')
    async def _cmd_FILE_TRUNCATE(self, session, msg):
        msg = cmd_FILE_TRUNCATE_Schema().load(msg)
        await self.file_truncate(msg['path'], msg['length'])
        return {'status': 'ok'}

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

    async def undelete(self, path: str):
        raise NotImplementedError()

    async def file_truncate(self, path: str, length: int):
        raise NotImplementedError()

    async def file_restore(self, path: str, version: int):
        raise NotImplementedError()


class MockedFSAPIMixin(BaseFSAPIMixin):

    pass

    # def __init__(self):
    #     now = datetime.now(timezone.utc).isoformat()
    #     self._fs = {
    #         'type': 'folder',
    #         'children': {},
    #         'stat': {'created': now, 'updated': now}
    #     }

    # def _retrieve_file(self, path):
    #     fileobj = self._retrieve_path(path)
    #     if fileobj['type'] != 'file':
    #         raise InvalidPath("Path `%s` is not a file" % path)
    #     return fileobj

    # def _check_path(self, path, should_exists=True, type=None):
    #     if path == '/':
    #         if not should_exists or type not in ('folder', None):
    #             raise InvalidPath('Root `/` folder always exists')
    #         else:
    #             return
    #     dirpath, leafname = path.rsplit('/', 1)
    #     try:
    #         obj = self._retrieve_path(dirpath)
    #         if obj['type'] != 'folder':
    #             raise InvalidPath("Path `%s` is not a folder" % path)
    #         try:
    #             leafobj = obj['children'][leafname]
    #             if not should_exists:
    #                 raise InvalidPath("Path `%s` already exist" % path)
    #             if type is not None and leafobj['type'] != type:
    #                 raise InvalidPath("Path `%s` is not a %s" % (path, type))
    #         except KeyError:
    #             if should_exists:
    #                 raise InvalidPath("Path `%s` doesn't exist" % path)
    #     except InvalidPath:
    #         raise InvalidPath("Path `%s` doesn't exist" % (path if should_exists else dirpath))

    # def _retrieve_path(self, path):
    #     if not path:
    #         return self._fs
    #     if not path.startswith('/'):
    #         raise InvalidPath("Path must start with `/`")
    #     cur_dir = self._fs
    #     reps = path.split('/')
    #     for rep in reps:
    #         if not rep or rep == '.':
    #             continue
    #         elif rep == '..':
    #             cur_dir = cur_dir['parent']
    #         else:
    #             try:
    #                 cur_dir = cur_dir['children'][rep]
    #             except KeyError:
    #                 raise InvalidPath("Path `%s` doesn't exist" % path)
    #     return cur_dir

    # async def file_create(self, path: str):
    #     self._check_path(path, should_exists=False)
    #     dirpath, name = path.rsplit('/', 1)
    #     dirobj = self._retrieve_path(dirpath)
    #     now = datetime.now(timezone.utc).isoformat()
    #     dirobj['children'][name] = {
    #         'type': 'file', 'data': b'', 'stat': {'created': now, 'updated': now}
    #     }

    # async def file_write(self, path: str, content: bytes, offset: int=0):
    #     self._check_path(path, should_exists=True, type='file')
    #     fileobj = self._retrieve_file(path)
    #     fileobj['data'] = (fileobj['data'][:offset] + content +
    #                        fileobj['data'][offset + len(content):])
    #     fileobj['stat']['updated'] = datetime.now(timezone.utc).isoformat()

    # async def file_read(self, path: str, offset: int=0, size: int=-1):
    #     self._check_path(path, should_exists=True, type='file')
    #     fileobj = self._retrieve_file(path)
    #     return fileobj['data'][offset:offset + size]

    # async def stat(self, path: str):
    #     self._check_path(path, should_exists=True)
    #     obj = self._retrieve_path(path)
    #     if obj['type'] == 'folder':
    #         return {**obj['stat'], 'type': obj['type'], 'children': list(obj['children'].keys())}
    #     else:
    #         return {**obj['stat'], 'type': obj['type'], 'size': len(obj['data'])}

    # async def folder_create(self, path: str):
    #     self._check_path(path, should_exists=False)
    #     dirpath, name = path.rsplit('/', 1)
    #     dirobj = self._retrieve_path(dirpath)
    #     now = datetime.now(timezone.utc).isoformat()
    #     dirobj['children'][name] = {
    #         'type': 'folder', 'children': {}, 'stat': {'created': now, 'updated': now}}

    # async def move(self, src: str, dst: str):
    #     self._check_path(src, should_exists=True)
    #     self._check_path(dst, should_exists=False)

    #     srcdirpath, scrfilename = src.rsplit('/', 1)
    #     dstdirpath, dstfilename = dst.rsplit('/', 1)

    #     srcobj = self._retrieve_path(srcdirpath)
    #     dstobj = self._retrieve_path(dstdirpath)
    #     dstobj['children'][dstfilename] = srcobj['children'][scrfilename]
    #     del srcobj['children'][scrfilename]

    # async def delete(self, path: str):
    #     self._check_path(path, should_exists=True)
    #     dirpath, leafname = path.rsplit('/', 1)
    #     obj = self._retrieve_path(dirpath)
    #     del obj['children'][leafname]

    # async def file_truncate(self, path: str, length: int):
    #     self._check_path(path, should_exists=True, type='file')
    #     fileobj = self._retrieve_file(path)
    #     fileobj['data'] = fileobj['data'][:length]
    #     fileobj['stat']['updated'] = datetime.now(timezone.utc).isoformat()


class FSAPIMixin(BaseFSAPIMixin):

    def __init__(self):
        self.user_manifest = None

    async def file_create(self, path, group=None):
        manifest = await self.get_manifest(group)
        file = await File.create(self.synchronizer)
        vlob = await file.get_vlob()
        try:
            await manifest.add_file(path, vlob)
        except (ManifestError, ManifestNotFound) as ex:
            await file.discard()
            raise ex
        return vlob

    async def file_write(self, path, data, offset, group=None):
        file = await self._get_file(path, group)
        await file.write(data, offset)

    async def file_read(self, path, offset=0, size=None, version=None, group=None):
        file = await self._get_file(path, group)
        return await file.read(size, offset)

    async def stat(self, path, version=None, group=None):
        manifest = await self.get_manifest(group)
        return await manifest.stat(path, version)

    async def folder_create(self, path, parents=False, group=None):
        manifest = await self.get_manifest(group)
        await manifest.make_folder(path, parents)
        await manifest.commit()

    async def move(self, old_path, new_path, group=None):
        manifest = await self.get_manifest(group)
        await manifest.rename_file(old_path, new_path)
        await manifest.commit()

    async def delete(self, path, group=None):
        manifest = await self.get_manifest(group)
        try:
            await manifest.delete_file(path)
        except ManifestError as ex:
            await manifest.remove_folder(path)
        await manifest.commit()

    async def undelete(self, vlob, group=None):
        manifest = await self.get_manifest(group)
        await manifest.undelete_file(vlob)
        await manifest.commit()

    async def file_truncate(self, path, length, group=None):
        file = await self._get_file(path, group)
        await file.truncate(length)

    async def file_restore(self, path, version, group=None):
        file = await self._get_file(path, group)
        await file.restore(version)

    async def _get_file(self, path, group):
        if not self.user_manifest:
            await self.get_manifest(group)
        try:
            properties = await self.get_properties(path=path, group=group)
        except FileNotFound:
            try:
                properties = await self.get_properties(path=path, dustbin=True, group=group)
            except FileNotFound:
                raise FileNotFound('Vlob not found.')
        return await File.load(self.synchronizer,
                               properties['id'],
                               properties['key'],
                               properties['read_trust_seed'],
                               properties['write_trust_seed'])

    async def history(self, path, first_version, last_version):
        if first_version and last_version and first_version > last_version:
            raise FileError('bad_versions',
                            'First version number higher than the second one.')
        history = []
        if not last_version:
            stat = await self.stat(path)
            last_version = stat['version']
        for current_version in range(first_version, last_version + 1):
            stat = await self.stat(path, current_version)
            del stat['id']
            history.append(stat)
        return history

    async def restore(self, path, version=None):  # TODO id in dustbin ?
        try:
            properties = await self.get_properties(path=path)
        except FileNotFound:
            try:
                properties = await self.get_properties(path=path, dustbin=True)
            except FileNotFound:
                raise FileNotFound('Vlob not found.')
        file = await File.load(**properties)
        await file.restore(version)

    # async def file_reencrypt(self, id):
    #     try:
    #         properties = await self.get_properties(id=id)
    #     except FileNotFound:
    #         try:
    #             properties = await self.get_properties(id=id, dustbin=True)
    #         except FileNotFound:
    #             raise FileNotFound('Vlob not found.')
    #     old_vlob = await self.service.vlob_read(properties['id'], properties['read_trust_seed'])
    #     old_blob = old_vlob['blob']
    #     old_encrypted_blob = decodebytes(old_blob.encode())
    #     old_blob_key = decodebytes(properties['key'].encode())
    #     encryptor = load_sym_key(old_blob_key)
    #     new_blob = encryptor.decrypt(old_encrypted_blob)
    #     encryptor = generate_sym_key()
    #     new_encrypted_blob = encryptor.encrypt(new_blob)
    #     new_encrypted_blob = encodebytes(new_encrypted_blob).decode()
    #     new_key = encodebytes(encryptor.key).decode()
    #     new_vlob = await self.service.vlob_create(new_encrypted_blob)
    #     del new_vlob['status']
    #     new_vlob['key'] = new_key
    #     return new_vlob




    # MOVE FOLLOWING

    async def reencrypt_group_manifest(self, group):
        manifest = await self.get_manifest()
        await manifest.reencrypt_group_manifest(group)
        await manifest.commit()

    async def import_group_vlob(self, group, vlob):
        manifest = await self.get_manifest()
        await manifest.import_group_vlob(group, vlob)
        await manifest.commit()

    async def import_file_vlob(self, path, vlob, group=None):
        manifest = await self.get_manifest(group)
        await manifest.add_file(path, vlob)
        await manifest.commit()

    async def restore_file(self, vlob, group=None):
        manifest = await self.get_manifest(group)
        await manifest.restore_file(vlob)

    async def get_manifest(self, group=None):
        identity = self.identity.id
        if not self.user_manifest or self.user_manifest.id != identity:
            self.user_manifest = await UserManifest.load(self)
        if group:
            return await self.user_manifest.get_group_manifest(group)
        else:
            return self.user_manifest

    async def get_properties(self, path=None, id=None, dustbin=False, group=None):  # TODO refactor?
        if group and not id and not path:
            manifest = await self.get_manifest(group)
            return await manifest.get_vlob()
        groups = [group] if group else [None] + list(await self.user_manifest.get_group_vlobs())
        for current_group in groups:
            manifest = await self.get_manifest(current_group)
            if dustbin:
                for item in manifest.dustbin:
                    if path == item['path'] or id == item['id']:
                        return deepcopy(item)
            else:
                if path in manifest.entries:
                    return deepcopy(manifest.entries[path])
                elif id:
                    for entry in manifest.entries.values():  # TODO bad complexity
                        if entry and entry['id'] == id:
                            return deepcopy(entry)
        raise FileNotFound('File not found.')
