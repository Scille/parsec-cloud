from copy import deepcopy
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


class cmd_CREATE_GROUP_MANIFEST_Schema(BaseCmdSchema):
    group = fields.String()


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


class cmd_RESTORE_FILE_Schema(BaseCmdSchema):
    vlob = fields.String(required=True)
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


class cmd_HISTORY_Schema(BaseCmdSchema):
    first_version = fields.Integer(missing=1)
    last_version = fields.Integer(missing=None)
    summary = fields.Boolean(missing=False)
    group = fields.String(missing=None)


class cmd_RESTORE_MANIFEST_Schema(BaseCmdSchema):
    version = fields.Integer(missing=None)
    group = fields.String(missing=None)


class BaseUserManifestService(BaseService):

    name = 'UserManifestService'

    @cmd('user_manifest_create_group_manifest')
    async def _cmd_CREATE_GROUP_MANIFEST(self, session, msg):
        msg = cmd_CREATE_GROUP_MANIFEST_Schema().load(msg)
        await self.create_group_manifest(msg['group'])
        return {'status': 'ok'}

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

    @cmd('user_manifest_restore_file')
    async def _cmd_RESTORE(self, session, msg):
        msg = cmd_RESTORE_FILE_Schema().load(msg)
        await self.restore_file(msg['vlob'], msg['group'])
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

    @cmd('user_manifest_history')
    async def _cmd_HISTORY(self, session, msg):
        msg = cmd_HISTORY_Schema().load(msg)
        history = await self.history(msg['first_version'],
                                     msg['last_version'],
                                     msg['summary'],
                                     msg['group'])
        history['status'] = 'ok'
        return history

    @cmd('user_manifest_restore')
    async def _cmd_RESTORE_MANIFEST(self, session, msg):
        msg = cmd_RESTORE_MANIFEST_Schema().load(msg)
        await self.restore_manifest(msg['version'], msg['group'])
        return {'status': 'ok'}

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

    async def restore_file(self, vlob, group):
        raise NotImplementedError()

    async def list_dir(self, path, group):
        raise NotImplementedError()

    async def make_dir(self, path, group):
        raise NotImplementedError()

    async def remove_dir(self, path, group):
        raise NotImplementedError()

    async def show_dustbin(self, path, group):
        raise NotImplementedError()

    async def history(self, first_version, last_version, summary, group):
        raise NotImplementedError()

    async def restore_manifest(self, version, group):
        raise NotImplementedError()

    async def load_user_manifest(self):
        raise NotImplementedError()


class Manifest(object):

    def __init__(self, service, id=None, key=None, read_trust_seed=None, write_trust_seed=None):
        self.service = service
        self.id = id
        self.key = key
        self.read_trust_seed = read_trust_seed
        self.write_trust_seed = write_trust_seed
        self.version = 0
        self.entries = {'/': {'id': None,
                              'key': None,
                              'read_trust_seed': None,
                              'write_trust_seed': None}}
        self.dustbin = []
        self.original_manifest = {'entries': deepcopy(self.entries),
                                  'dustbin': deepcopy(self.dustbin)}
        self.handler = partial(event_handler, self.reload, self)

    async def reload(self):
        raise NotImplementedError()

    async def is_dirty(self):
        current_manifest = json.loads(await self.dumps())
        dirty = False
        diff = await self.diff(self.original_manifest, current_manifest)
        for category in diff.keys():
            for operation in diff[category].keys():
                if diff[category][operation]:
                    dirty = True
        return dirty

    async def diff(self, old_manifest, new_manifest):
        diff = {}
        for category in new_manifest.keys():
            if category == 'dustbin':
                continue
            added = {}
            changed = {}
            removed = {}
            for path, vlob in new_manifest[category].items():
                try:
                    ori_vlob = old_manifest[category][path]
                    if ori_vlob != vlob:
                        changed[path] = (ori_vlob, vlob)
                except KeyError:
                    added[path] = vlob
            for path, vlob in old_manifest[category].items():
                try:
                    new_manifest[category][path]
                except KeyError:
                    removed[path] = vlob
            diff.update({category: {'added': added, 'changed': changed, 'removed': removed}})
        # Dustbin
        added = []
        removed = []
        for vlob in new_manifest['dustbin']:
            if vlob not in old_manifest['dustbin']:
                added.append(vlob)
        for vlob in old_manifest['dustbin']:
            if vlob not in new_manifest['dustbin']:
                removed.append(vlob)
        diff.update({'dustbin': {'added': added, 'removed': removed}})
        return diff

    async def patch(self, manifest, diff):
        new_manifest = deepcopy(manifest)
        for category in diff.keys():
            if category == 'dustbin':
                continue
            for path, entry in diff[category]['added'].items():
                if path in new_manifest[category] and new_manifest[category][path] != entry:
                    new_manifest[category][path + '-conflict'] = new_manifest[category][path]
                new_manifest[category][path] = entry
            for path, entries in diff[category]['changed'].items():
                old_entry, new_entry = entries
                if path in new_manifest[category]:
                    current_entry = new_manifest[category][path]
                    if current_entry not in [old_entry, new_entry]:
                        new_manifest[category][path + '-conflict'] = current_entry
                    new_manifest[category][path] = new_entry
                else:
                    new_manifest[category][path + '-deleted'] = new_entry
            for path, entry in diff[category]['removed'].items():
                if path in new_manifest[category]:
                    if new_manifest[category][path] != entry:
                        new_manifest[category][path + '-recreated'] = new_manifest[category][path]
                    del new_manifest[category][path]
        for entry in diff['dustbin']['added']:
            if entry not in new_manifest['dustbin']:
                new_manifest['dustbin'].append(entry)
        for entry in diff['dustbin']['removed']:
            if entry in new_manifest['dustbin']:
                new_manifest['dustbin'].remove(entry)
        return new_manifest

    async def get_vlob(self):
        return {'id': self.id,
                'key': self.key,
                'read_trust_seed': self.read_trust_seed,
                'write_trust_seed': self.write_trust_seed}

    async def get_version(self):
        return self.version

    async def dumps(self, original_manifest=False):
        if original_manifest:
            return json.dumps(self.original_manifest)
        else:
            return json.dumps({'entries': self.entries,
                               'dustbin': self.dustbin})

    async def reload_vlob(self, vlob_id):
        # TODO invalidate old cache
        pass

    async def add_file(self, path, vlob):
        if path in self.entries:
            raise UserManifestError('already_exists', 'File already exists.')
        self.entries[path] = vlob

    async def rename_file(self, old_path, new_path):
        if new_path in self.entries:
            raise UserManifestError('already_exists', 'File already exists.')
        if old_path not in self.entries:
            raise UserManifestNotFound('File not found.')
        self.entries[new_path] = self.entries[old_path]
        del self.entries[old_path]

    async def delete_file(self, path):
        try:
            entry = self.entries[path]
        except KeyError:
            raise UserManifestNotFound('File not found.')
        if not entry['id']:
            raise UserManifestError('path_is_not_file', 'Path is not a file.')
        dustbin_entry = {'removed_date': datetime.utcnow().timestamp(), 'path': path}
        dustbin_entry.update(entry)
        self.dustbin.append(dustbin_entry)
        del self.entries[path]

    async def restore_file(self, vlob):
        for entry in self.dustbin:
            if entry['id'] == vlob:
                path = entry['path']
                if path in self.entries:
                    raise UserManifestError('already_exists', 'Restore path already used.')
                del entry['path']
                del entry['removed_date']
                self.dustbin[:] = [item for item in self.dustbin if item['id'] != vlob]
                self.entries[path] = entry  # TODO recreate subdirs if deleted since deletion
                return True
        raise UserManifestNotFound('Vlob not found.')

    async def list_dir(self, path, children=True):
        if path != '/' and path not in self.entries:
            raise UserManifestNotFound('Directory or file not found.')
        if not children:
            return self.entries[path]
        results = {}
        for entry in self.entries:
            if entry != path and entry.startswith(path) and entry.count('/', len(path) + 1) == 0:
                results[entry.split('/')[-1]] = self.entries[entry]
        return results

    async def make_dir(self, path):
        if path in self.entries:
            raise UserManifestError('already_exists', 'Directory already exists.')
        self.entries[path] = {'id': None,
                              'read_trust_seed': None,
                              'write_trust_seed': None,
                              'key': None}  # TODO set correct values
        return self.entries[path]

    async def remove_dir(self, path):
        if path == '/':
            raise UserManifestError('cannot_remove_root', 'Cannot remove root directory.')
        for entry, vlob in self.entries.items():
            if entry != path and entry.startswith(path):
                raise UserManifestError('directory_not_empty', 'Directory not empty.')
            elif entry == path and vlob['id']:
                raise UserManifestError('path_is_not_dir', 'Path is not a directory.')
        try:
            del self.entries[path]
        except KeyError:
            raise UserManifestNotFound('Directory not found.')

    async def show_dustbin(self, path=None):
        if not path:
            return self.dustbin
        results = [entry for entry in self.dustbin if entry['path'] == path]
        if not results:
            raise UserManifestNotFound('Path not found.')
        return results

    async def check_consistency(self, manifest):
        entries = [entry for entry in list(manifest['entries'].values()) if entry['id']]
        entries += manifest['dustbin']
        for entry in entries:
            try:
                vlob = await self.service.backend_api_service.vlob_read(
                    id=entry['id'],
                    trust_seed=entry['read_trust_seed'])
                encrypted_blob = vlob['blob']
                encrypted_blob = decodebytes(encrypted_blob.encode())
                key = decodebytes(entry['key'].encode()) if entry['key'] else None
                await self.service.crypto_service.sym_decrypt(encrypted_blob, key)
            except Exception:
                return False
        return True


class GroupManifest(Manifest):

    def __init__(self, service, id=None, key=None, read_trust_seed=None, write_trust_seed=None):
        super().__init__(service, id, key, read_trust_seed, write_trust_seed)

    async def update_vlob(self, new_vlob):
        self.id = new_vlob['id']
        self.key = new_vlob['key']
        self.read_trust_seed = new_vlob['read_trust_seed']
        self.write_trust_seed = new_vlob['write_trust_seed']

    async def diff_versions(self, old_version=None, new_version=None):
        empty_entries = {'/': {'id': None,
                               'key': None,
                               'read_trust_seed': None,
                               'write_trust_seed': None}}
        # Old manifest
        if old_version and old_version > 0:
            old_vlob = await self.service.backend_api_service.vlob_read(
                id=self.id,
                trust_seed=self.read_trust_seed,
                version=old_version)
            key = decodebytes(self.key.encode())
            content = await self.service.crypto_service.sym_decrypt(old_vlob['blob'], key)
            old_manifest = json.loads(content.decode())
        elif old_version == 0:
            old_manifest = {'entries': empty_entries, 'dustbin': []}
        else:
            old_manifest = self.original_manifest
        # New manifest
        if new_version and new_version > 0:
            new_vlob = await self.service.backend_api_service.vlob_read(
                id=self.id,
                trust_seed=self.read_trust_seed,
                version=new_version)
            key = decodebytes(self.key.encode())
            content = await self.service.crypto_service.sym_decrypt(new_vlob['blob'], key)
            new_manifest = json.loads(content.decode())
        elif new_version == 0:
            new_manifest = {'entries': empty_entries, 'dustbin': []}
        else:
            new_manifest = json.loads(await self.dumps())
        return await self.diff(old_manifest, new_manifest)

    async def reload(self, reset=False):
        if not self.id:
            raise UserManifestError('missing_id', 'Group manifest has no ID.')
        # Subscribe to events
        await self.service.backend_api_service.connect_event('on_vlob_updated',
                                                             self.id,
                                                             self.handler)
        vlob = await self.service.backend_api_service.vlob_read(id=self.id,
                                                                trust_seed=self.read_trust_seed)
        key = decodebytes(self.key.encode())
        content = await self.service.crypto_service.sym_decrypt(vlob['blob'], key)
        if not reset and vlob['version'] <= self.version:
            return
        new_manifest = json.loads(content.decode())
        backup_new_manifest = deepcopy(new_manifest)
        if not await self.check_consistency(new_manifest):
            raise UserManifestError('not_consistent', 'Group manifest not consistent.')
        if not reset:
            diff = await self.diff_versions()
            new_manifest = await self.patch(new_manifest, diff)
        self.entries = new_manifest['entries']
        self.dustbin = new_manifest['dustbin']
        self.version = vlob['version']
        self.original_manifest = backup_new_manifest

    async def save(self):
        if self.id and not await self.is_dirty():
            return
        blob = await self.dumps()
        if self.key:
            key = key = decodebytes(self.key.encode())
        else:
            key = None
        key, encrypted_blob = await self.service.crypto_service.sym_encrypt(blob.encode(), key)
        self.version += 1
        if self.id:
            await self.service.backend_api_service.vlob_update(
                id=self.id,
                version=self.version,
                trust_seed=self.write_trust_seed,
                blob=encrypted_blob.decode())
        else:
            vlob = await self.service.backend_api_service.vlob_create(blob=encrypted_blob.decode())
            self.id = vlob['id']
            self.key = encodebytes(key).decode()
            self.read_trust_seed = vlob['read_trust_seed']
            self.write_trust_seed = vlob['write_trust_seed']
        self.original_manifest = json.loads(blob)

    async def reencrypt(self):
        blob = await self.dumps()
        key, encrypted_blob = await self.service.crypto_service.sym_encrypt(blob.encode())
        new_vlob = await self.service.backend_api_service.vlob_create(blob=encrypted_blob.decode())
        self.id = new_vlob['id']
        self.key = encodebytes(key).decode()
        self.read_trust_seed = new_vlob['read_trust_seed']
        self.write_trust_seed = new_vlob['write_trust_seed']
        self.version = 1

    async def restore(self, version=None):
        if version is None:
            version = self.version - 1 if self.version > 1 else 1
        if version > 0 and version < self.version:
            vlob = await self.service.backend_api_service.vlob_read(id=self.id,
                                                                    trust_seed=self.read_trust_seed,
                                                                    version=version)
            await self.service.backend_api_service.vlob_update(
                id=self.id,
                version=self.version + 1,
                blob=vlob['blob'],
                trust_seed=self.write_trust_seed)
        elif version < 1 or version > self.version:
            raise UserManifestError('bad_version', 'Bad version number.')
        await self.reload(reset=True)


class UserManifest(Manifest):

    def __init__(self, service, id):
        super().__init__(service, id)
        self.group_manifests = {}
        self.original_manifest = {'entries': deepcopy(self.entries),
                                  'dustbin': deepcopy(self.dustbin),
                                  'groups': deepcopy(self.group_manifests)}

    async def diff_versions(self, old_version=None, new_version=None):
        empty_entries = {'/': {'id': None,
                               'key': None,
                               'read_trust_seed': None,
                               'write_trust_seed': None}}
        # Old manifest
        if old_version and old_version > 0:
            old_vlob = await self.service.backend_api_service.named_vlob_read(
                id=self.id,
                trust_seed='42',
                version=old_version)
            content = await self.service.identity_service.decrypt(old_vlob['blob'])
            old_manifest = json.loads(content.decode())
        elif old_version == 0:
            old_manifest = {'entries': empty_entries, 'groups': {}, 'dustbin': []}
        else:
            old_manifest = self.original_manifest
        # New manifest
        if new_version and new_version > 0:
            new_vlob = await self.service.backend_api_service.named_vlob_read(
                id=self.id,
                trust_seed='42',
                version=new_version)
            content = await self.service.identity_service.decrypt(new_vlob['blob'])
            new_manifest = json.loads(content.decode())
        elif new_version == 0:
            new_manifest = {'entries': empty_entries, 'groups': {}, 'dustbin': []}
        else:
            new_manifest = json.loads(await self.dumps())
        return await self.diff(old_manifest, new_manifest)

    async def dumps(self, original_manifest=False):
        if original_manifest:
            return json.dumps(self.original_manifest)
        else:
            return json.dumps({'entries': self.entries,
                               'dustbin': self.dustbin,
                               'groups': await self.get_group_vlobs()})

    async def get_group_vlobs(self, group=None):
        if group:
            groups = [group]
        else:
            groups = self.group_manifests.keys()
        results = {}
        try:
            for group in groups:
                results[group] = await self.group_manifests[group].get_vlob()
        except Exception:
            raise UserManifestNotFound('Group not found.')
        return results

    async def get_group_manifest(self, group):
        try:
            return self.group_manifests[group]
        except Exception:
            raise UserManifestNotFound('Group not found.')

    async def reencrypt_group_manifest(self, group):
        try:
            group_manifest = self.group_manifests[group]
        except Exception:
            raise UserManifestNotFound('Group not found.')
        await group_manifest.reencrypt()

    async def create_group_manifest(self, group):
        if group in self.group_manifests:
            raise UserManifestError('already_exists', 'Group already exists.')
        group_manifest = GroupManifest(self.service)
        self.group_manifests[group] = group_manifest

    async def import_group_vlob(self, group, vlob):
        if group in self.group_manifests:
            await self.group_manifests[group].update_vlob(vlob)
            await self.group_manifests[group].reload(reset=False)
        group_manifest = GroupManifest(self.service, **vlob)
        await group_manifest.reload(reset=True)
        self.group_manifests[group] = group_manifest

    async def remove_group(self, group):
        # TODO deleted group is not moved in dusbin, but hackers could continue to read/write files
        try:
            del self.group_manifests[group]
        except Exception:
            raise UserManifestNotFound('Group not found.')

    async def reload(self, reset=False):
        # TODO: Named vlob should use private key handshake instead of trust_seed
        try:
            vlob = await self.service.backend_api_service.named_vlob_read(id=self.id,
                                                                          trust_seed='42')
        except Exception:
            raise UserManifestNotFound('User manifest not found.')
        content = await self.service.identity_service.decrypt(vlob['blob'])
        if not reset and vlob['version'] <= self.version:
            return
        new_manifest = json.loads(content.decode())
        backup_new_manifest = deepcopy(new_manifest)
        if not await self.check_consistency(new_manifest):
            raise UserManifestError('not_consistent', 'User manifest not consistent.')
        if not reset:
            diff = await self.diff_versions()
            new_manifest = await self.patch(new_manifest, diff)
        self.entries = new_manifest['entries']
        self.dustbin = new_manifest['dustbin']
        self.version = vlob['version']
        for group, group_vlob in new_manifest['groups'].items():
            await self.import_group_vlob(group, group_vlob)
        self.original_manifest = backup_new_manifest
        # Update event subscriptions
        # TODO update events subscriptions
        # Subscribe to events
        # TODO where to unsubscribe?
        # await self.service.backend_api_service.connect_event('on_named_vlob_updated',
        #                                                      self.id,
        #                                                      self.handler)

    async def save(self, recursive=True):
        if self.version and not await self.is_dirty():
            return
        if recursive:
            for group_manifest in self.group_manifests.values():
                await group_manifest.save()
        blob = json.dumps({'entries': self.entries,
                           'dustbin': self.dustbin,
                           'groups': await self.get_group_vlobs(),
                           })
        encrypted_blob = await self.service.identity_service.encrypt(blob.encode())
        self.version += 1
        try:
            await self.service.backend_api_service.named_vlob_update(
                id=self.id,
                version=self.version,
                blob=encrypted_blob.decode(),
                trust_seed='42')
        except Exception:
            await self.service.backend_api_service.named_vlob_create(
                id=self.id,
                blob=encrypted_blob.decode())
        self.original_manifest = json.loads(blob)

    async def restore(self, version=None):
        if version is None:
            version = self.version - 1 if self.version > 1 else 1
        if version > 0 and version < self.version:
            vlob = await self.service.backend_api_service.named_vlob_read(id=self.id,
                                                                          trust_seed='42',
                                                                          version=version)
            await self.service.backend_api_service.named_vlob_update(
                id=self.id,
                version=self.version + 1,
                blob=vlob['blob'],
                trust_seed='42')
        elif version < 1 or version > self.version:
            raise UserManifestError('bad_version', 'Bad version number.')
        await self.reload(reset=True)

    async def check_consistency(self, manifest):
        if await super().check_consistency(manifest) is False:
            return False
        for group_manifest in self.group_manifests.values():
            entry = await group_manifest.get_vlob()
            try:
                vlob = await self.service.backend_api_service.vlob_read(
                    id=entry['id'],
                    trust_seed=entry['read_trust_seed'])
                encrypted_blob = vlob['blob']
                key = decodebytes(entry['key'].encode()) if entry['key'] else None
                await self.service.crypto_service.sym_decrypt(encrypted_blob, key)
            except Exception:
                return False
        return True


class UserManifestService(BaseUserManifestService):

    backend_api_service = service('BackendAPIService')
    crypto_service = service('CryptoService')
    file_service = service('FileService')
    identity_service = service('IdentityService')

    def __init__(self):
        super().__init__()
        self.user_manifest = None
        self.handler = None

    async def create_group_manifest(self, group):
        manifest = await self.get_manifest()
        vlob = await manifest.create_group_manifest(group)
        await manifest.save()
        return vlob

    async def reencrypt_group_manifest(self, group):
        manifest = await self.get_manifest()
        await manifest.reencrypt_group_manifest(group)
        await manifest.save()

    async def import_group_vlob(self, group, vlob):
        manifest = await self.get_manifest()
        await manifest.import_group_vlob(group, vlob)
        await manifest.save()

    async def import_file_vlob(self, path, vlob, group=None):
        manifest = await self.get_manifest(group)
        await manifest.add_file(path, vlob)
        if group:
            await manifest.save()
        else:
            await manifest.save()

    async def create_file(self, path, content=b'', group=None):
        manifest = await self.get_manifest(group)
        try:
            await manifest.list_dir(path, children=False)
        except Exception:
            vlob = await self.file_service.create(content)
            await manifest.add_file(path, vlob)
            await manifest.save()
            return vlob
        else:
            raise UserManifestError('already_exists', 'File already exists.')

    async def rename_file(self, old_path, new_path, group=None):
        manifest = await self.get_manifest(group)
        await manifest.rename_file(old_path, new_path)
        await manifest.save()

    async def delete_file(self, path, group=None):
        manifest = await self.get_manifest(group)
        await manifest.delete_file(path)
        await manifest.save()

    async def restore_file(self, vlob, group=None):
        manifest = await self.get_manifest(group)
        await manifest.restore_file(vlob)

    async def list_dir(self, path, group=None):
        manifest = await self.get_manifest(group)
        directory = await manifest.list_dir(path, children=False)
        children = await manifest.list_dir(path, children=True)
        return directory, children

    async def make_dir(self, path, group=None):
        manifest = await self.get_manifest(group)
        await manifest.make_dir(path)
        await manifest.save()

    async def remove_dir(self, path, group=None):
        manifest = await self.get_manifest(group)
        await manifest.remove_dir(path)
        await manifest.save()

    async def show_dustbin(self, path=None, group=None):
        manifest = await self.get_manifest(group)
        return await manifest.show_dustbin(path)

    async def history(self, first_version=1, last_version=None, summary=False, group=None):
        if first_version and last_version and first_version > last_version:
            raise UserManifestError('bad_versions', 'First version number greater than second one.')
        manifest = await self.get_manifest(group)
        if summary:
            diff = await manifest.diff_versions(first_version, last_version)
            return {'summary_history': diff}
        else:
            if not last_version:
                last_version = await manifest.get_version()
            history = []
            for current_version in range(first_version, last_version + 1):
                diff = await manifest.diff_versions(current_version - 1, current_version)
                diff['version'] = current_version
                history.append(diff)
            return {'detailed_history': history}

    async def restore_manifest(self, version=None, group=None):
        manifest = await self.get_manifest(group)
        await manifest.restore(version)

    async def load_user_manifest(self):
        identity = await self.identity_service.get_identity()
        self.user_manifest = UserManifest(self, identity)
        try:
            await self.user_manifest.reload(False)
        except UserManifestNotFound:
            await self.user_manifest.save()
            await self.user_manifest.reload(True)

    async def get_manifest(self, group=None):
        if not self.user_manifest:
            raise UserManifestNotFound('User manifest not found.')
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
                        return item
            else:
                if path in manifest.entries:
                    return manifest.entries[path]
                elif id:
                    for entry in manifest.entries.values():  # TODO bad complexity
                        if entry['id'] == id:
                            return entry
        raise UserManifestNotFound('File not found.')
