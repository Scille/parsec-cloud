import json
from base64 import encodebytes, decodebytes
from marshmallow import fields

from parsec.service import BaseService, service, cmd
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


class UserManifestError(ParsecError):
    pass


class UserManifestNotFound(UserManifestError):
    status = 'not_found'


class cmd_CREATE_FILE_Schema(BaseCmdSchema):
    path = fields.String(required=True)


class cmd_RENAME_FILE_Schema(BaseCmdSchema):
    old_path = fields.String(required=True)
    new_path = fields.String(required=True)


class cmd_DELETE_FILE_Schema(BaseCmdSchema):
    path = fields.String(required=True)


class cmd_LIST_DIR_Schema(BaseCmdSchema):
    path = fields.String(required=True)


class cmd_MAKE_DIR_Schema(BaseCmdSchema):
    path = fields.String(required=True)


class cmd_REMOVE_DIR_Schema(BaseCmdSchema):
    path = fields.String(required=True)


class cmd_HISTORY_Schema(BaseCmdSchema):
    path = fields.String(required=True)


class BaseUserManifestService(BaseService):

    name = 'UserManifestService'

    @cmd('user_manifest_create_file')
    async def _cmd_CREATE_FILE(self, session, msg):
        msg = cmd_CREATE_FILE_Schema().load(msg)
        file = await self.create_file(msg['path'])
        return {'status': 'ok', **file}

    @cmd('user_manifest_rename_file')
    async def _cmd_RENAME_FILE(self, session, msg):
        msg = cmd_RENAME_FILE_Schema().load(msg)
        await self.rename_file(msg['old_path'], msg['new_path'])
        return {'status': 'ok'}

    @cmd('user_manifest_delete_file')
    async def _cmd_DELETE_FILE(self, session, msg):
        msg = cmd_DELETE_FILE_Schema().load(msg)
        await self.delete_file(msg['path'])
        return {'status': 'ok'}

    @cmd('user_manifest_list_dir')
    async def _cmd_LIST_DIR(self, session, msg):
        msg = cmd_LIST_DIR_Schema().load(msg)
        current, childrens = await self.list_dir(msg['path'])
        return {'status': 'ok', 'current': current, 'childrens': childrens}

    @cmd('user_manifest_make_dir')
    async def _cmd_MAKE_DIR(self, session, msg):
        msg = cmd_MAKE_DIR_Schema().load(msg)
        await self.make_dir(msg['path'])
        return {'status': 'ok'}

    @cmd('user_manifest_remove_dir')
    async def _cmd_REMOVE_DIR(self, session, msg):
        msg = cmd_REMOVE_DIR_Schema().load(msg)
        await self.remove_dir(msg['path'])
        return {'status': 'ok'}

    @cmd('user_manifest_history')
    async def _cmd_HISTORY(self, session, msg):
        msg = cmd_HISTORY_Schema().load(msg)
        history = await self.history(msg['path'])
        return {'status': 'ok', 'history': history}

    @cmd('user_manifest_load')
    # TODO event when new identity loaded in indentity service
    async def _cmd_LOAD_USER_MANIFEST(self, session, msg):
        await self.load_user_manifest()
        return {'status': 'ok'}

    async def create_file(self, path):
        raise NotImplementedError()

    async def rename_file(self, old_path, new_path):
        raise NotImplementedError()

    async def delete_file(self, path):
        raise NotImplementedError()

    async def list_dir(self, path):
        raise NotImplementedError()

    async def make_dir(self, path):
        raise NotImplementedError()

    async def remove_dir(self, path):
        raise NotImplementedError()

    async def load_user_manifest(self):
        raise NotImplementedError()

    async def save_manifest(self):
        raise NotImplementedError()

    async def check_consistency(self, manifest):
        raise NotImplementedError()

    async def get_properties(self, id):
        raise NotImplementedError()

    async def history(self):
        raise NotImplementedError()


class UserManifestService(BaseUserManifestService):

    backend_api_service = service('BackendAPIService')
    crypto_service = service('CryptoService')
    file_service = service('FileService')
    identity_service = service('IdentityService')
    share_service = service('ShareService')

    def __init__(self):
        super().__init__()
        self.user_manifest = {}
        self.group_manifests = {}
        self.groups = {}
        self.versions = {None: 0}

    async def get_manifest(self, group=None):
        if group:
            try:
                return self.group_manifests[group]
            except Exception:
                raise UserManifestNotFound('Group manifest not found.')
        else:
            return self.user_manifest

    async def create_group_manifest(self, group):
        if group in self.groups:
            raise(UserManifestError('Group already exists.'))
        self.group_manifests[group] = {}
        vlob = await self.backend_api_service.vlob_create()
        key, _ = await self.crypto_service.sym_encrypt('')
        self.groups[group] = {'id': vlob.id,
                              'key': encodebytes(key).decode(),
                              'read_trust_seed': vlob.read_trust_seed,
                              'write_trust_seed': vlob.write_trust_seed}
        self.versions[group] = 0
        await self.make_dir('/', group)
        vlob = await self.backend_api_service.vlob_read(id=vlob.id)
        self.versions[group] = len(vlob.blob_versions)
        blob = vlob.blob_versions[self.versions[group] - 1]
        content = await self.crypto_service.sym_decrypt(blob, key)
        content = content.decode()
        manifest = json.loads(content)
        self.group_manifests[group] = manifest

    async def create_file(self, path, group=None):
        manifest = await self.get_manifest(group)
        if path in manifest:
            raise UserManifestError('already_exist', 'File already exists.')
        else:
            ret = await self.file_service.create()
            file = {}
            for key in ['id', 'read_trust_seed', 'write_trust_seed']:
                file[key] = ret[key]
            key, _ = await self.crypto_service.sym_encrypt('')
            file['key'] = encodebytes(key).decode()
            manifest[path] = file
            await self.save_manifest(group)
        return manifest[path]

    async def rename_file(self, old_path, new_path, group=None):
        manifest = await self.get_manifest(group)
        manifest[new_path] = manifest[old_path]
        del manifest[old_path]
        await self.save_manifest(group)

    async def delete_file(self, path, group=None):
        manifest = await self.get_manifest(group)
        try:
            del manifest[path]
            await self.save_manifest(group)
        except KeyError:
            raise UserManifestNotFound('File not found.')

    async def list_dir(self, path, group=None):
        manifest = await self.get_manifest(group)
        if path != '/' and path not in manifest:
            raise UserManifestNotFound('Directory not found.')
        results = {}
        for entry in manifest:
            if entry != path and entry.startswith(path) and entry.count('/', len(path) + 1) == 0:
                results[entry.split('/')[-1]] = manifest[entry]
        return manifest[path], results

    async def make_dir(self, path, group=None):
        manifest = await self.get_manifest(group)
        if path in manifest:
            raise UserManifestError('already_exist', 'Directory already exists.')
        else:
            manifest[path] = {'id': None,
                              'read_trust_seed': None,
                              'write_trust_seed': None,
                              'key': None}  # TODO set correct values
            await self.save_manifest(group)
        return manifest[path]

    async def remove_dir(self, path, group=None):
        manifest = await self.get_manifest(group)
        if path == '/':
            raise UserManifestError('cannot_remove_root', 'Cannot remove root directory.')
        for entry in manifest:
            if entry != path and entry.startswith(path):
                raise UserManifestError('directory_not_empty', 'Directory not empty.')
        try:
            del manifest[path]
            await self.save_manifest(group)
        except KeyError:
            raise UserManifestNotFound('Directory not found.')

    async def load_user_manifest(self):
        await self.share_service.listen_shared_vlob()
        identity = await self.identity_service.get_identity()
        try:
            vlob = await self.backend_api_service.named_vlob_read(id=identity)
        except Exception:
            vlob = await self.backend_api_service.named_vlob_create(id=identity)
            await self.make_dir('/')
            vlob = await self.backend_api_service.named_vlob_read(id=identity)
        self.versions[None] = len(vlob.blob_versions)
        blob = vlob.blob_versions[self.versions[None] - 1]
        content = await self.identity_service.decrypt(blob)
        content = content.decode()
        manifest = json.loads(content)
        if await self.check_user_manifest_consistency(manifest):
            self.user_manifest = manifest['user_manifest']
        else:
            raise(UserManifestError('User manifest not consistent.'))
        for group in manifest['groups']:
            await self.load_group_manifest(group)

    async def load_group_manifest(self, group):
        properties = await self.get_properties(group=group)
        vlob = await self.backend_api_service.vlob_read(id=properties['id'])
        self.versions[group] = len(vlob.blob_versions)
        blob = vlob.blob_versions[self.versions[group] - 1]
        key = decodebytes(properties['key'].encode())
        content = await self.crypto_service.sym_decrypt(blob, key)
        content = content.decode()
        manifest = json.loads(content)
        if await self.check_group_manifest_consistency(manifest):
            self.group_manifests[group] = manifest
        else:
            raise(UserManifestError('Group manifest not consistent.'))

    async def save_manifest(self, group):
        if group:
            await self.save_group_manifest(group)
        else:
            await self.save_user_manifest()

    async def save_user_manifest(self):
        identity = await self.identity_service.get_identity()
        self.versions[None] += 1
        blob = json.dumps({'user_manifest': self.user_manifest, 'groups': self.groups}).encode()
        encrypted_blob = await self.identity_service.encrypt(blob)
        await self.backend_api_service.named_vlob_update(
            id=identity,
            next_version=self.versions[None],
            blob=encrypted_blob.decode())

    async def save_group_manifest(self, group):
        manifest = await self.get_manifest(group)
        self.versions[group] += 1
        blob = json.dumps(manifest)
        blob = blob.encode()
        key = decodebytes(self.groups[group]['key'].encode())
        _, encrypted_blob = await self.crypto_service.sym_encrypt(blob, key)
        await self.backend_api_service.vlob_update(
            id=self.groups[group]['id'],
            next_version=self.versions[group],
            blob=encrypted_blob.decode())

    async def import_vlob(self, vlob, path=None, group=None):
        # TODO check vlob is manifest if group
        # TODO check path
        if group:
            self.groups[group] = vlob
            await self.load_group_manifest(group)
        else:
            self.user_manifest[path] = vlob
        await self.save_manifest(group)

    async def check_user_manifest_consistency(self, manifest):
        for category in ['user_manifest', 'groups']:
            for _, entry in manifest[category].items():
                if entry['id']:
                    try:
                        await self.file_service.stat(entry['id'], entry['read_trust_seed'])
                    except Exception:
                        return False
        return True

    async def check_group_manifest_consistency(self, manifest):
        for _, entry in manifest.items():
            if entry['id']:
                try:
                    await self.file_service.stat(entry['id'], entry['read_trust_seed'])
                except Exception:
                    return False
        return True

    async def get_properties(self, path=None, id=None, group=None):  # TODO refactor?
        manifest = await self.get_manifest(group)
        if path:
            try:
                return manifest[path]
            except Exception:
                raise(UserManifestNotFound('File not found.'))
        elif id:
            for entry in manifest.values():  # TODO bad complexity
                if entry['id'] == id:
                    return entry
            raise(UserManifestNotFound('File not found.'))
        elif group:
            return self.groups[group]

    async def history(self):
        # TODO raise ParsecNotImplementedError
        pass
