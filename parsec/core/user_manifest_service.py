import json
from functools import partial
from base64 import encodebytes, decodebytes
from datetime import datetime
from marshmallow import fields

from parsec.service import BaseService, service, cmd
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema, event_handler


class UserManifestError(ParsecError):
    pass


class UserManifestNotFound(UserManifestError):
    status = 'not_found'


class cmd_CREATE_FILE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    content = fields.String(missing='')
    group = fields.String(missing=None)


class cmd_RENAME_FILE_Schema(BaseCmdSchema):
    old_path = fields.String(required=True)
    new_path = fields.String(required=True)
    group = fields.String(missing=None)


class cmd_DELETE_FILE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    group = fields.String(missing=None)


class cmd_LIST_DIR_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    group = fields.String(missing=None)


class cmd_MAKE_DIR_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    group = fields.String(missing=None)


class cmd_REMOVE_DIR_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    group = fields.String(missing=None)


class cmd_SHOW_dustbin_Schema(BaseCmdSchema):
    path = fields.String(missing=None)
    group = fields.String(missing=None)


class cmd_RESTORE_Schema(BaseCmdSchema):
    vlob = fields.String(required=True)
    group = fields.String(missing=None)


class cmd_HISTORY_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    group = fields.String(missing=None)


class BaseUserManifestService(BaseService):

    name = 'UserManifestService'

    @cmd('user_manifest_create_file')
    async def _cmd_CREATE_FILE(self, session, msg):
        msg = cmd_CREATE_FILE_Schema().load(msg)
        file = await self.create_file(msg['path'], msg['content'], msg['group'])
        return {'status': 'ok', **file}

    @cmd('user_manifest_rename_file')
    async def _cmd_RENAME_FILE(self, session, msg):
        msg = cmd_RENAME_FILE_Schema().load(msg)
        await self.rename_file(msg['old_path'], msg['new_path'], msg['group'])
        return {'status': 'ok'}

    @cmd('user_manifest_delete_file')
    async def _cmd_DELETE_FILE(self, session, msg):
        msg = cmd_DELETE_FILE_Schema().load(msg)
        await self.delete_file(msg['path'], msg['group'])
        return {'status': 'ok'}

    @cmd('user_manifest_list_dir')
    async def _cmd_LIST_DIR(self, session, msg):
        msg = cmd_LIST_DIR_Schema().load(msg)
        current, children = await self.list_dir(msg['path'], msg['group'])
        return {'status': 'ok', 'current': current, 'children': children}

    @cmd('user_manifest_make_dir')
    async def _cmd_MAKE_DIR(self, session, msg):
        msg = cmd_MAKE_DIR_Schema().load(msg)
        await self.make_dir(msg['path'], msg['group'])
        return {'status': 'ok'}

    @cmd('user_manifest_remove_dir')
    async def _cmd_REMOVE_DIR(self, session, msg):
        msg = cmd_REMOVE_DIR_Schema().load(msg)
        await self.remove_dir(msg['path'], msg['group'])
        return {'status': 'ok'}

    @cmd('user_manifest_show_dustbin')
    async def _cmd_SHOW_dustbin(self, session, msg):
        msg = cmd_SHOW_dustbin_Schema().load(msg)
        dustbin = await self.show_dustbin(msg['path'], msg['group'])
        return {'status': 'ok', 'dustbin': dustbin}

    @cmd('user_manifest_restore')
    async def _cmd_RESTORE(self, session, msg):
        msg = cmd_RESTORE_Schema().load(msg)
        await self.restore(msg['vlob'], msg['group'])
        return {'status': 'ok'}

    @cmd('user_manifest_history')
    async def _cmd_HISTORY(self, session, msg):
        msg = cmd_HISTORY_Schema().load(msg)
        history = await self.history(msg['path'], msg['group'])
        return {'status': 'ok', 'history': history}

    @cmd('user_manifest_load')
    # TODO event when new identity loaded in indentity service
    async def _cmd_LOAD_USER_MANIFEST(self, session, msg):
        await self.load_user_manifest()
        return {'status': 'ok'}

    async def create_file(self, path, group):
        raise NotImplementedError()

    async def rename_file(self, old_path, new_path, group):
        raise NotImplementedError()

    async def delete_file(self, path, group):
        raise NotImplementedError()

    async def list_dir(self, path, group):
        raise NotImplementedError()

    async def make_dir(self, path, group):
        raise NotImplementedError()

    async def remove_dir(self, path, group):
        raise NotImplementedError()

    async def show_dustbin(self, path, group):
        raise NotImplementedError()

    async def restore(self, vlob, group):
        raise NotImplementedError()

    async def load_user_manifest(self):
        raise NotImplementedError()

    async def load_group_manifest(self):
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
        self.groups = {}
        self.entries = {'user_manifest': {}, 'group_manifests': {}}
        self.dustbins = {'user_manifest': [], 'group_manifests': {}}
        self.versions = {'user_manifest': 1, 'group_manifests': {}}
        self.handler = None

    async def get_entries_and_dustbin(self, group=None):
        subcategory = 'group_manifests' if group else 'user_manifest'
        try:
            if group:
                return self.entries[subcategory][group], self.dustbins[subcategory][group]
            else:
                return self.entries[subcategory], self.dustbins[subcategory]
        except Exception:
            raise UserManifestNotFound('Entries and dustbin not found.')

    async def create_group_manifest(self, group):
        if group in self.groups:
            raise(UserManifestError('Group already exists.'))
        self.entries['group_manifests'][group] = {}
        self.dustbins['group_manifests'][group] = []
        self.versions['group_manifests'][group] = 1
        vlob = await self.backend_api_service.vlob_create()
        key, _ = await self.crypto_service.sym_encrypt('')
        self.groups[group] = {'id': vlob['id'],
                              'key': encodebytes(key).decode(),
                              'read_trust_seed': vlob['read_trust_seed'],
                              'write_trust_seed': vlob['write_trust_seed']}
        await self.make_dir('/', group)
        await self.load_group_manifest(group)

    async def create_file(self, path, content='', group=None):
        manifest, _ = await self.get_entries_and_dustbin(group)
        if path in manifest:
            raise UserManifestError('already_exist', 'File already exists.')
        else:
            manifest[path] = await self.file_service.create(content)
            await self.save_manifest(group)
        return manifest[path]

    async def rename_file(self, old_path, new_path, group=None):
        manifest, _ = await self.get_entries_and_dustbin(group)
        manifest[new_path] = manifest[old_path]
        del manifest[old_path]
        await self.save_manifest(group)

    async def delete_file(self, path, group=None):
        manifest, dustbin = await self.get_entries_and_dustbin(group)
        try:
            entry = {'removed_date': datetime.utcnow().timestamp(), 'path': path}
            entry.update(manifest[path])
            dustbin.append(entry)
            del manifest[path]
            await self.save_manifest(group)
        except KeyError:
            raise UserManifestNotFound('File not found.')

    async def list_dir(self, path, group=None):
        manifest, _ = await self.get_entries_and_dustbin(group)
        if path != '/' and path not in manifest:
            raise UserManifestNotFound('Directory not found.')
        results = {}
        for entry in manifest:
            if entry != path and entry.startswith(path) and entry.count('/', len(path) + 1) == 0:
                results[entry.split('/')[-1]] = manifest[entry]
        return manifest[path], results

    async def make_dir(self, path, group=None):
        manifest, _ = await self.get_entries_and_dustbin(group)
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
        manifest, _ = await self.get_entries_and_dustbin(group)
        if path == '/':
            raise UserManifestError('cannot_remove_root', 'Cannot remove root directory.')
        for entry in manifest:
            if entry != path and entry.startswith(path):
                raise UserManifestError('directory_not_empty', 'Directory not empty.')
        try:
            del manifest[path]
            await self.save_manifest(group)
        except KeyError:
            raise(UserManifestNotFound('Directory not found.'))

    async def show_dustbin(self, path, group=None):
        _, dustbin = await self.get_entries_and_dustbin(group)
        if not path:
            return dustbin
        results = [entry for entry in dustbin if entry['path'] == path]
        if not results:
            raise(UserManifestNotFound('Path not found.'))
        return results

    async def restore(self, vlob, group=None):
        manifest, dustbin = await self.get_entries_and_dustbin(group)
        for entry in dustbin:
            if entry['id'] == vlob:
                path = entry['path']
                if path in manifest:
                    raise UserManifestNotFound('Restoration path already used.')
                del entry['path']
                del entry['removed_date']
                dustbin[:] = [item for item in dustbin if item['id'] != vlob]
                manifest[path] = entry  # TODO recreate subdirs if deleted since deletion
                return True
        raise(UserManifestNotFound('Vlob not found.'))

    async def load_user_manifest(self):
        if not self.handler:
            self.handler = partial(event_handler, self.reload_vlob, self)
        identity = await self.identity_service.get_identity()
        # TODO: Named vlob should use private key handshake instead of trust_seed
        try:
            vlob = await self.backend_api_service.named_vlob_read(id=identity, trust_seed='42')
        except Exception:
            vlob = await self.backend_api_service.named_vlob_create(id=identity)
            await self.make_dir('/')
            vlob = await self.backend_api_service.named_vlob_read(id=identity, trust_seed='42')
        blob = vlob['blob']
        content = await self.identity_service.decrypt(blob)
        content = content.decode()
        manifest = json.loads(content)
        if await self.check_consistency(manifest):
            self.entries['user_manifest'] = manifest['entries']
            self.groups = manifest['groups']
            self.dustbins['user_manifest'] = manifest['dustbin']
            self.versions = {'user_manifest': vlob['version'], 'group_manifests': {}}
            # Subscribe to events
            vlobs = list(self.groups.values())
            vlobs += list(self.entries['user_manifest'].values())
            vlobs += self.dustbins['user_manifest']
            # TODO where to unsubscribe?
            for vlob in vlobs:
                if vlob['id']:
                    await self.backend_api_service.connect_event('on_vlob_updated',
                                                                 vlob['id'],
                                                                 self.handler)
        else:
            raise(UserManifestError('User manifest not consistent.'))
        for group in manifest['groups']:
            await self.load_group_manifest(group)

    async def load_group_manifest(self, group):
        if not self.handler:
            self.handler = partial(event_handler, self.reload_vlob, self)
        properties = await self.get_properties(group=group)
        vlob = await self.backend_api_service.vlob_read(id=properties['id'],
                                                        trust_seed=properties['read_trust_seed'])
        blob = vlob['blob']
        version = vlob['version']
        key = decodebytes(properties['key'].encode())
        content = await self.crypto_service.sym_decrypt(blob, key)
        content = content.decode()
        manifest = json.loads(content)
        if await self.check_consistency(manifest):
            self.entries['group_manifests'][group] = manifest['entries']
            self.dustbins['group_manifests'][group] = manifest['dustbin']
            self.versions['group_manifests'][group] = version
            # Subscribe to events
            for vlob in list(manifest['entries'].values()) + manifest['dustbin']:
                if vlob['id']:
                    await self.backend_api_service.connect_event('on_vlob_updated',
                                                                 vlob['id'],
                                                                 self.handler)
        else:
            raise(UserManifestError('Group manifest not consistent.'))

    async def save_manifest(self, group):
        if group:
            await self.save_group_manifest(group)
        else:
            await self.save_user_manifest()

    async def save_user_manifest(self):
        identity = await self.identity_service.get_identity()
        self.versions['user_manifest'] += 1
        blob = json.dumps({'entries': self.entries['user_manifest'],
                           'groups': self.groups,
                           'dustbin': self.dustbins['user_manifest']}).encode()
        encrypted_blob = await self.identity_service.encrypt(blob)
        await self.backend_api_service.named_vlob_update(
            id=identity,
            version=self.versions['user_manifest'],
            blob=encrypted_blob.decode(),
            trust_seed='42')

    async def save_group_manifest(self, group):
        manifest, dustbin = await self.get_entries_and_dustbin(group)
        self.versions['group_manifests'][group] += 1
        blob = json.dumps({'entries': manifest,
                           'dustbin': dustbin}).encode()
        key = decodebytes(self.groups[group]['key'].encode())
        _, encrypted_blob = await self.crypto_service.sym_encrypt(blob, key)
        await self.backend_api_service.vlob_update(
            id=self.groups[group]['id'],
            version=self.versions['group_manifests'][group],
            trust_seed=self.groups[group]['write_trust_seed'],
            blob=encrypted_blob.decode()
        )

    async def import_group_manifest_vlob(self, vlob, group):
        # Import group manifest
        manifest_vlob = await self.backend_api_service.vlob_read(
            id=vlob['id'],
            trust_seed=vlob['read_trust_seed'])
        manifest_blob = manifest_vlob['blob']
        manifest_key = decodebytes(vlob['key'].encode())
        manifest_content = await self.crypto_service.sym_decrypt(manifest_blob, manifest_key)
        manifest_content = manifest_content.decode()
        manifest = json.loads(manifest_content)
        try:
            consistent = await self.check_consistency(manifest)
        except Exception:
            raise(UserManifestError('Invalid imported group manifest format.'))
        if not consistent:
            raise(UserManifestError('Group manifest not consistent.'))
        self.groups[group] = vlob
        await self.load_group_manifest(group)
        await self.save_user_manifest()

    async def import_file_vlob(self, vlob, path, group=None):
        if group:
            # Import a file in group manifest
            # TODO check path already exist
            self.entries['group_manifests'][group][path] = vlob
        else:
            # Import a file in user manifest
            # TODO check path already exist
            self.entries['user_manifest'][path] = vlob
        await self.save_manifest(group)

    async def reload_vlob(self, vlob_id):
        # vlob = get_properties(vlob_id)
        # TODO invalidate old cache
        print('RELOAD ', vlob_id)

    async def check_consistency(self, manifest):
        entries = list(manifest['entries'].values())
        entries += manifest['dustbin']
        for entry in entries:
            if entry['id']:
                try:
                    vlob = await self.backend_api_service.vlob_read(
                        id=entry['id'],
                        trust_seed=entry['read_trust_seed'])
                    encrypted_blob = vlob['blob']
                    encrypted_blob = decodebytes(encrypted_blob.encode())
                    key = decodebytes(entry['key'].encode()) if entry['key'] else None
                    await self.crypto_service.sym_decrypt(encrypted_blob, key)
                except Exception:
                    return False
        return True

    async def get_properties(self, path=None, id=None, group=None, dustbin=False):  # TODO refactor?
        if group and not id and not path:
            return self.groups[group]
        groups = [group] if group else [None] + list(self.groups.keys())
        for current_group in groups:
            manifest, manifest_dustbin = await self.get_entries_and_dustbin(current_group)
            item = manifest_dustbin if dustbin else manifest
            if path:
                try:
                    return item[path]
                except Exception:
                    raise(UserManifestNotFound('File not found.'))
            elif id:
                for entry in item.values():  # TODO bad complexity
                    if entry['id'] == id:
                        return entry
                raise(UserManifestNotFound('File not found.'))

    async def history(self):
        # TODO raise ParsecNotImplementedError
        pass
