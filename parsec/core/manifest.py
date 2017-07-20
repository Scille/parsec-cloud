from copy import deepcopy
from functools import partial
from datetime import datetime
import os

from effect2 import Effect, do

from parsec.core.file import File
from parsec.core.synchronizer import (
    EUserVlobSynchronize, EUserVlobRead, EUserVlobUpdate, EVlobCreate, EVlobList, EVlobRead,
    EVlobUpdate, EVlobSynchronize)
from parsec.crypto import generate_sym_key, load_private_key, load_sym_key
from parsec.exceptions import FileError, ManifestError, ManifestNotFound, VlobNotFound
from parsec.tools import event_handler, from_jsonb64, to_jsonb64, ejson_loads, ejson_dumps


class Manifest:

    def __init__(self, id=None):
        self.id = id
        self.version = 0
        self.entries = {'/': None}
        self.dustbin = []
        self.original_manifest = {'entries': deepcopy(self.entries),
                                  'dustbin': deepcopy(self.dustbin),
                                  'versions': {}}
        self.handler = partial(event_handler, self.reload, reset=False)

    def reload(self):
        raise NotImplementedError()

    @do
    def is_dirty(self):
        dump = yield self.dumps()
        current_manifest = ejson_loads(dump)
        diff = self.diff(self.original_manifest, current_manifest)
        for category in diff.keys():
            for operation in diff[category].keys():
                if diff[category][operation]:
                    return True
        return False

    def diff(self, old_manifest, new_manifest):
        diff = {}
        for category in new_manifest.keys():
            if category == 'dustbin':
                continue
            added = {}
            changed = {}
            removed = {}
            for key, value in new_manifest[category].items():
                try:
                    ori_value = old_manifest[category][key]
                    if ori_value != value:
                        changed[key] = (ori_value, value)
                except KeyError:
                    added[key] = value
            for key, value in old_manifest[category].items():
                try:
                    new_manifest[category][key]
                except KeyError:
                    removed[key] = value
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

    def patch(self, manifest, diff):
        new_manifest = deepcopy(manifest)
        for category in diff.keys():
            if category in ['dustbin', 'versions']:
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

    def diff_versions(self, old_version=None, new_version=None):
        raise NotImplementedError()

    @do
    def history(self, first_version=1, last_version=None, summary=False):
        if first_version and last_version and first_version > last_version:
            raise ManifestError('bad_versions',
                                'First version number higher than the second one.')
        if summary:
            diff = yield self.diff_versions(first_version, last_version)
            return {'summary_history': diff}
        else:
            if not last_version:
                last_version = self.version
            history = []
            for current_version in range(first_version, last_version + 1):
                diff = yield self.diff_versions(current_version - 1, current_version)
                diff['version'] = current_version
                history.append(diff)
            return {'detailed_history': history}

    @do
    def get_version(self):
        is_dirty = yield self.is_dirty()
        return self.version + 1 if is_dirty else self.version

    @do
    def get_vlobs_versions(self):
        versions = {}
        for entry in [self.entries[entry] for entry in sorted(self.entries)] + self.dustbin:
            if entry:
                try:
                    vlob = yield Effect(EVlobRead(entry['id'], entry['read_trust_seed']))
                except VlobNotFound:
                    versions[entry['id']] = None
                else:
                    versions[entry['id']] = vlob['version']
        return versions

    @do
    def dumps(self, original_manifest=False):
        if original_manifest:
            return ejson_dumps(self.original_manifest)
        else:
            versions = yield self.get_vlobs_versions()
            return ejson_dumps({'entries': self.entries,
                                'dustbin': self.dustbin,
                                'versions': versions})

    def add_file(self, path, vlob):
        path = '/' + path.strip('/')
        parent_folder = os.path.dirname(path)
        if parent_folder not in self.entries:
            raise ManifestNotFound('Destination Folder not found.')
        if path in self.entries:
            raise ManifestError('already_exists', 'File already exists.')
        self.entries[path] = vlob

    def move(self, old_path, new_path):
        old_path = '/' + old_path.strip('/')
        new_path = '/' + new_path.strip('/')
        new_parent_folder = os.path.dirname(new_path)
        if old_path not in self.entries:
            raise ManifestNotFound('File not found.')
        if new_parent_folder not in self.entries:
            raise ManifestNotFound('Destination Folder not found.')
        if new_path in self.entries:
            raise ManifestError('already_exists', 'File already exists.')
        for entry, vlob in self.entries.items():
            if entry.startswith(old_path):
                new_entry = new_path + entry[len(old_path):]
                self.entries[new_entry] = vlob
                del self.entries[entry]

    @do
    def delete(self, path):
        path = '/' + path.strip('/')
        deleted_paths = []
        for entry in self.entries:
            if entry == path or entry.startswith(path + '/') or path == '/':
                deleted_paths.append(entry)
        for path in deleted_paths:
            entry = self.entries[path]
            if entry:
                file = yield File.load(entry['id'],
                                       entry['key'],
                                       entry['read_trust_seed'],
                                       entry['write_trust_seed'])
                discarded = yield file.discard()  # TODO discard or not?
                if not discarded:
                    dustbin_entry = {'removed_date': datetime.utcnow().isoformat(), 'path': path}
                    dustbin_entry.update(entry)
                    self.dustbin.append(dustbin_entry)
            if path != '/':
                del self.entries[path]
        if not deleted_paths:
            raise ManifestNotFound('File or directory not found.')

    def undelete_file(self, vlob):
        for entry in self.dustbin:
            if entry['id'] == vlob:
                path = entry['path']
                if path in self.entries:
                    raise ManifestError('already_exists', 'Restore path already used.')
                del entry['path']
                del entry['removed_date']
                self.dustbin[:] = [item for item in self.dustbin if item['id'] != vlob]
                self.entries[path] = entry
                folder = os.path.dirname(path)
                self.create_folder(folder, parents=True)
                return
        raise ManifestNotFound('Vlob not found.')

    @do
    def reencrypt_file(self, path):
        path = '/' + path.strip('/')
        try:
            entry = self.entries[path]
        except KeyError:
            raise ManifestNotFound('File not found.')
        file = yield File.load(**entry)
        yield file.reencrypt()
        self.entries[path] = file.get_vlob()

    @do
    def stat(self, path):
        path = '/' + path.strip('/')
        if path != '/' and path not in self.entries:
            raise ManifestNotFound('Folder or file not found.')
        entry = self.entries[path]
        if entry:
            file = yield File.load(**entry)
            stat = yield file.stat()
            return stat
        else:
            # Skip mtime and size given that they are too complicated to provide for folder
            # TODO time except mtime
            children = {}
            for entry in self.entries:
                if (entry != path and
                        entry.startswith(path) and
                        entry.count('/', len(path) + 1) == 0):
                    children[os.path.basename(entry)] = deepcopy(self.entries[entry])
            return {
                'type': 'folder',
                'children': sorted(list(children.keys()))
            }

    def create_folder(self, path, parents=False):
        path = '/' + path.strip('/')
        if path in self.entries:
            if parents:
                return self.entries[path]
            else:
                raise ManifestError('already_exists', 'Folder already exists.')
        parent_folder = os.path.dirname(path)
        if parent_folder not in self.entries:
            if parents:
                self.create_folder(parent_folder, parents=True)
            else:
                raise ManifestNotFound("Parent folder doesn't exists.")
        self.entries[path] = None
        return self.entries[path]

    def show_dustbin(self, path=None):
        if not path:
            return self.dustbin
        else:
            path = '/' + path.strip('/')
        results = [entry for entry in self.dustbin if entry['path'] == path]
        if not results:
            raise ManifestNotFound('Path not found.')
        return results

    @do
    def check_consistency(self, manifest):
        entries = [entry for entry in list(manifest['entries'].values()) if entry]
        entries += manifest['dustbin']
        for entry in entries:
            try:
                vlob = yield Effect(EVlobRead(entry['id'],
                                              entry['read_trust_seed'],
                                              manifest['versions'][entry['id']]))
                encrypted_blob = vlob['blob']
                encrypted_blob = from_jsonb64(encrypted_blob)
                key = from_jsonb64(entry['key']) if entry['key'] else None
                encryptor = load_sym_key(key)
                encryptor.decrypt(encrypted_blob)  # TODO check exception
            except VlobNotFound:
                return False
        return True


class GroupManifest(Manifest):

    @classmethod
    @do
    def create(cls):
        vlob = yield Effect(EVlobCreate())
        self = GroupManifest(vlob['id'])
        self.read_trust_seed = vlob['read_trust_seed']
        self.write_trust_seed = vlob['write_trust_seed']
        self.encryptor = generate_sym_key()
        self.version = 0
        blob = yield self.dumps()
        encrypted_blob = self.encryptor.encrypt(blob.encode())
        encrypted_blob = to_jsonb64(encrypted_blob)
        yield Effect(EVlobUpdate(vlob['id'], vlob['write_trust_seed'], 1, encrypted_blob))
        return self

    @classmethod
    @do
    def load(cls, id, key, read_trust_seed, write_trust_seed):
        self = GroupManifest(id)
        self.read_trust_seed = read_trust_seed
        self.write_trust_seed = write_trust_seed
        self.encryptor = load_sym_key(from_jsonb64(key))
        yield self.reload(reset=True)
        return self

    def get_vlob(self):
        return {'id': self.id,
                'key': to_jsonb64(self.encryptor.key),
                'read_trust_seed': self.read_trust_seed,
                'write_trust_seed': self.write_trust_seed}

    def update_vlob(self, new_vlob):
        self.id = new_vlob['id']
        self.encryptor = load_sym_key(from_jsonb64(new_vlob['key']))
        self.read_trust_seed = new_vlob['read_trust_seed']
        self.write_trust_seed = new_vlob['write_trust_seed']

    @do
    def diff_versions(self, old_version=None, new_version=None):
        empty_entries = {'/': None}
        empty_manifest = {'entries': empty_entries, 'dustbin': [], 'versions': {}}
        # Old manifest
        if old_version and old_version > 0:
            old_vlob = yield Effect(EVlobRead(self.id, self.read_trust_seed, old_version))
            old_blob = from_jsonb64(old_vlob['blob'])
            content = self.encryptor.decrypt(old_blob)
            old_manifest = ejson_loads(content.decode())
        elif old_version == 0:
            old_manifest = empty_manifest
        else:
            old_manifest = self.original_manifest
        # New manifest
        if new_version and new_version > 0:
            new_vlob = yield Effect(EVlobRead(self.id, self.read_trust_seed, new_version))
            blob = from_jsonb64(new_vlob['blob'])
            content = self.encryptor.decrypt(blob)
            new_manifest = ejson_loads(content.decode())
        elif new_version == 0:
            new_manifest = empty_manifest
        else:
            dump = yield self.dumps()
            new_manifest = ejson_loads(dump)
        return self.diff(old_manifest, new_manifest)

    @do
    def reload(self, reset=False):
        # Subscribe to events
        # yield Effect(EConnectEvent('on_vlob_updated', self.id, self.handler)) # TODO call
        vlob = yield Effect(EVlobRead(self.id, self.read_trust_seed))
        blob = from_jsonb64(vlob['blob'])
        content = self.encryptor.decrypt(blob)
        if not reset and vlob['version'] <= self.version:
            return
        new_manifest = ejson_loads(content.decode())
        backup_new_manifest = deepcopy(new_manifest)
        consistency = yield self.check_consistency(new_manifest)
        if not consistency:
            raise ManifestError('not_consistent', 'Group manifest not consistent.')
        if not reset:
            diff = yield self.diff_versions()
            new_manifest = self.patch(new_manifest, diff)
        self.entries = new_manifest['entries']
        self.dustbin = new_manifest['dustbin']
        self.version = vlob['version']
        self.original_manifest = backup_new_manifest
        versions = new_manifest['versions']
        file_vlob = None
        for vlob_id, version in sorted(versions.items()):
            for entry in self.entries.values():
                if entry and entry['id'] == vlob_id:
                    file_vlob = entry
                    break
            if not file_vlob:
                for entry in self.dustbin:
                    if entry['id'] == vlob_id:
                        file_vlob = {'id': entry['id'],
                                     'read_trust_seed': entry['read_trust_seed'],
                                     'write_trust_seed': entry['write_trust_seed'],
                                     'key': entry['key']}
                        break
            if file_vlob:
                file_vlob = None
                file = yield File.load(entry['id'],
                                       entry['key'],
                                       entry['read_trust_seed'],
                                       entry['write_trust_seed'])
                try:
                    yield file.restore(version)
                except FileError:
                    pass

    @do
    def commit(self):
        is_dirty = yield self.is_dirty()
        if self.version != 0 and not is_dirty:
            return
        # Update manifest entries with new file vlobs (dustbin entries are already commited)
        vlob_list = yield Effect(EVlobList())
        for entry in self.entries.values():
            if entry and entry['id'] in vlob_list:
                file = yield File.load(entry['id'],
                                       entry['key'],
                                       entry['read_trust_seed'],
                                       entry['write_trust_seed'])
                new_vlob = yield file.commit()
                if new_vlob and new_vlob is not True:
                    entry['id'] = new_vlob['id']
                    entry['read_trust_seed'] = new_vlob['read_trust_seed']
                    entry['write_trust_seed'] = new_vlob['write_trust_seed']
        # Commit manifest
        blob = yield self.dumps()
        encrypted_blob = self.encryptor.encrypt(blob.encode())
        encrypted_blob = to_jsonb64(encrypted_blob)
        yield Effect(EVlobUpdate(self.id, self.write_trust_seed, self.version + 1, encrypted_blob))
        self.original_manifest = ejson_loads(blob)
        new_vlob = yield Effect(EVlobSynchronize(self.id))
        if new_vlob:
            if new_vlob is not True:
                self.id = new_vlob['id']
                self.read_trust_seed = new_vlob['read_trust_seed']
                self.write_trust_seed = new_vlob['write_trust_seed']
                new_vlob = self.get_vlob()
            self.version += 1
        return new_vlob

    @do
    def reencrypt(self):
        # Reencrypt files
        for path, entry in self.entries.items():
            if entry:
                file = yield File.load(**entry)
                yield file.reencrypt()
                new_vlob = file.get_vlob()
                self.entries[path] = new_vlob
        for index, entry in enumerate(self.dustbin):
            entry = deepcopy(entry)
            path = entry['path']
            removed_date = entry['removed_date']
            for key in ['path', 'removed_date']:
                del entry[key]
            file = yield File.load(**entry)
            yield file.reencrypt()
            new_vlob = file.get_vlob()
            new_vlob['path'] = path
            new_vlob['removed_date'] = removed_date
            self.dustbin[index] = new_vlob
        # Reencrypt manifest
        blob = yield self.dumps()
        self.encryptor = generate_sym_key()
        encrypted_blob = self.encryptor.encrypt(blob.encode())
        encrypted_blob = to_jsonb64(encrypted_blob)
        new_vlob = yield Effect(EVlobCreate(encrypted_blob))
        self.id = new_vlob['id']
        self.read_trust_seed = new_vlob['read_trust_seed']
        self.write_trust_seed = new_vlob['write_trust_seed']
        self.version = 0

    @do
    def restore(self, version=None):
        if version is None:
            version = self.version - 1 if self.version > 1 else 1
        if version > 0 and version < self.version:
            vlob = yield Effect(EVlobRead(self.id, self.read_trust_seed, version))
            yield Effect(EVlobUpdate(self.id, self.write_trust_seed, self.version, vlob['blob']))
        elif version < 1 or version > self.version:
            raise ManifestError('bad_version', 'Bad version number.')
        yield self.reload(reset=True)


class UserManifest(Manifest):

    @classmethod
    @do
    def load(cls, private_key):  # TODO retrieve key from id
        self = UserManifest('USER')
        self.encryptor = load_private_key(private_key)
        try:
            yield self.reload(reset=True)
        except ManifestNotFound:
            self.version = 0
            self.group_manifests = {}
            self.original_manifest = {'entries': deepcopy(self.entries),
                                      'dustbin': deepcopy(self.dustbin),
                                      'groups': deepcopy(self.group_manifests),
                                      'versions': {}}
            blob = yield self.dumps()
            encrypted_blob = self.encryptor.pub_key.encrypt(blob.encode())
            encrypted_blob = to_jsonb64(encrypted_blob)
            yield Effect(EUserVlobUpdate(1, encrypted_blob))
        return self

    @do
    def diff_versions(self, old_version=None, new_version=None):
        empty_entries = {'/': None}
        empty_manifest = {'entries': empty_entries, 'groups': {}, 'dustbin': [], 'versions': {}}
        # Old manifest
        if old_version and old_version > 0:
            old_vlob = yield Effect(EUserVlobRead(old_version))
            old_blob = from_jsonb64(old_vlob['blob'])
            content = self.encryptor.decrypt(old_blob)
            old_manifest = ejson_loads(content.decode())
        elif old_version == 0:
            old_manifest = empty_manifest
        else:
            old_manifest = self.original_manifest
        # New manifest
        if new_version and new_version > 0:
            new_vlob = yield Effect(EUserVlobRead(new_version))
            new_blob = from_jsonb64(new_vlob['blob'])
            content = self.encryptor.decrypt(new_blob)
            new_manifest = ejson_loads(content.decode())
        elif new_version == 0:
            new_manifest = empty_manifest
        else:
            dump = yield self.dumps()
            new_manifest = ejson_loads(dump)
        return self.diff(old_manifest, new_manifest)

    @do
    def dumps(self, original_manifest=False):
        if original_manifest:
            manifest = deepcopy(self.original_manifest)
            manifest['groups'] = self.get_group_vlobs()
            return ejson_dumps(manifest)
        else:
            versions = yield self.get_vlobs_versions()
            return ejson_dumps({'entries': self.entries,
                                'dustbin': self.dustbin,
                                'groups': self.get_group_vlobs(),
                                'versions': versions})

    def get_group_vlobs(self, group=None):
        if group:
            groups = [group]
        else:
            groups = self.group_manifests.keys()
        results = {}
        try:
            for group in groups:
                results[group] = self.group_manifests[group].get_vlob()
        except KeyError:
            raise ManifestNotFound('Group not found.')
        return results

    def get_group_manifest(self, group):
        try:
            return self.group_manifests[group]
        except KeyError:
            raise ManifestNotFound('Group not found.')

    @do
    def reencrypt_group_manifest(self, group):
        try:
            group_manifest = self.group_manifests[group]
        except KeyError:
            raise ManifestNotFound('Group not found.')
        yield group_manifest.reencrypt()

    @do
    def create_group_manifest(self, group):
        if group in self.group_manifests:
            raise ManifestError('already_exists', 'Group already exists.')
        group_manifest = yield GroupManifest.create()
        self.group_manifests[group] = group_manifest

    @do
    def import_group_vlob(self, group, vlob):
        if group in self.group_manifests:
            self.group_manifests[group].update_vlob(vlob)
            yield self.group_manifests[group].reload(reset=False)
        group_manifest = yield GroupManifest.load(**vlob)
        self.group_manifests[group] = group_manifest

    def remove_group(self, group):
        # TODO deleted group is not moved in dusbin, but hackers could continue to read/write files
        try:
            del self.group_manifests[group]
        except KeyError:
            raise ManifestNotFound('Group not found.')

    @do
    def reload(self, reset=False):
        vlob = yield Effect(EUserVlobRead())
        if not vlob['blob']:
            raise ManifestNotFound('User manifest not found.')
        blob = from_jsonb64(vlob['blob'])
        content = self.encryptor.decrypt(blob)
        if not reset and vlob['version'] <= self.version:
            return
        new_manifest = ejson_loads(content.decode())
        backup_new_manifest = deepcopy(new_manifest)
        consistency = yield self.check_consistency(new_manifest)
        if not consistency:
            raise ManifestError('not_consistent', 'User manifest not consistent.')
        if not reset:
            diff = yield self.diff_versions()
            new_manifest = self.patch(new_manifest, diff)
        self.entries = new_manifest['entries']
        self.dustbin = new_manifest['dustbin']
        self.version = vlob['version']
        self.group_manifests = {}
        for group, group_vlob in new_manifest['groups'].items():
            self.import_group_vlob(group, group_vlob)
        self.original_manifest = backup_new_manifest
        versions = new_manifest['versions']
        file_vlob = None
        for vlob_id, version in sorted(versions.items()):
            for entry in self.entries.values():
                if entry and entry['id'] == vlob_id:
                    file_vlob = entry
                    break
            if not file_vlob:
                for entry in self.dustbin:
                    if entry['id'] == vlob_id:
                        file_vlob = {'id': entry['id'],
                                     'read_trust_seed': entry['read_trust_seed'],
                                     'write_trust_seed': entry['write_trust_seed'],
                                     'key': entry['key']}
                        break
            if file_vlob:
                file_vlob = None
                file = yield File.load(entry['id'],
                                       entry['key'],
                                       entry['read_trust_seed'],
                                       entry['write_trust_seed'])
                try:
                    yield file.restore(version)
                except FileError:
                    pass
        # Update event subscriptions
        # TODO update events subscriptions
        # Subscribe to events
        # TODO where to unsubscribe?

    @do
    def commit(self, recursive=True):
        is_dirty = yield self.is_dirty()
        if self.version != 0 and not is_dirty:
            return
        # Update manifest with new group vlobs
        vlob_list = yield Effect(EVlobList())
        if recursive:
            for group_manifest in self.group_manifests.values():
                new_vlob = yield group_manifest.commit()
                if new_vlob and new_vlob is not True:
                    old_vlob = group_manifest.get_vlob()
                    new_vlob['key'] = old_vlob['key']
                    group_manifest.update_vlob(new_vlob)
        # Update manifest entries with new file vlobs (dustbin entries are already commited)
        for entry in self.entries.values():
            if entry and entry['id'] in vlob_list:
                file = yield File.load(entry['id'],
                                       entry['key'],
                                       entry['read_trust_seed'],
                                       entry['write_trust_seed'])
                new_vlob = yield file.commit()
                if new_vlob and new_vlob is not True:
                    entry['id'] = new_vlob['id']
                    entry['read_trust_seed'] = new_vlob['read_trust_seed']
                    entry['write_trust_seed'] = new_vlob['write_trust_seed']
        # Commit manifest
        blob = yield self.dumps()
        encrypted_blob = self.encryptor.pub_key.encrypt(blob.encode())
        encrypted_blob = to_jsonb64(encrypted_blob)
        yield Effect(EUserVlobUpdate(self.version + 1, encrypted_blob))
        self.original_manifest = ejson_loads(blob)
        synchronized = yield Effect(EUserVlobSynchronize())
        if synchronized:
            self.version += 1

    @do
    def restore(self, version=None):
        if version is None:
            version = self.version - 1 if self.version > 1 else 1
        if version > 0 and version < self.version:
            vlob = yield Effect(EUserVlobRead(version))
            yield Effect(EUserVlobUpdate(self.version, vlob['blob']))
        elif version < 1 or version > self.version:
            raise ManifestError('bad_version', 'Bad version number.')
        yield self.reload(reset=True)

    @do
    def check_consistency(self, manifest):
        consistency = yield super().check_consistency(manifest)
        if consistency is False:
            return False
        for entry in manifest['groups'].values():
            try:
                vlob = yield Effect(EVlobRead(entry['id'], entry['read_trust_seed']))
                encrypted_blob = vlob['blob']
                key = from_jsonb64(entry['key']) if entry['key'] else None
                encryptor = load_sym_key(key)
                encryptor.decrypt(encrypted_blob)
            except VlobNotFound:
                return False
        return True
