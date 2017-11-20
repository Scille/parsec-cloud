import attr
import pendulum
from uuid import uuid4
import json

from parsec.utils import ParsecError, from_jsonb64, to_jsonb64, generate_sym_key


class InvalidManifestError(ParsecError):
    status = 'invalid_manifest'


class InvalidSignatureError(ParsecError):
    status = 'invalid_signature'


def load_manifest(raw):
    # TODO catch json decoding error and missing fields
    data = json.loads(raw.decode())
    if data['format'] != 1:
        raise InvalidManifestError('Wrong format version: %r' % data)
    if data['type'] == 'local_file_manifest':
        return _load_local_file_manifest(data)
    elif data['type'] == 'local_folder_manifest':
        return _load_local_folder_manifest(data)
    elif data['type'] == 'local_user_manifest':
        return _load_local_user_manifest(data)
    if data['type'] == 'file_manifest':
        return _load_file_manifest(data)
    elif data['type'] == 'folder_manifest':
        return _load_folder_manifest(data)
    elif data['type'] == 'user_manifest':
        return _load_user_manifest(data)
    else:
        raise InvalidManifestError()


def _load_local_entry(data):
    if data['type'] == 'synced_entry':
        return SyncedEntry(
            id=data['id'],
            key=from_jsonb64(data['key']),
            rts=data['read_trust_seed'],
            wts=data['write_trust_seed'],
        )
    elif data['type'] == 'placeholder_entry':
        return PlaceHolderEntry(
            id=data['id'],
            key=from_jsonb64(data['key']),
        )
    else:
        raise InvalidManifestError('Unknown entry type: %r' % data)


def _load_local_user_manifest(data):
    return LocalUserManifest(
        base_version=data['base_version'],
        updated=pendulum.parse(data['updated']),
        created=pendulum.parse(data['created']),
        need_sync=data['need_sync'],
        children={k: _load_local_entry(v) for k, v in data['children'].items()}
    )


def _load_local_folder_manifest(data):
    return LocalFolderManifest(
        base_version=data['base_version'],
        updated=pendulum.parse(data['updated']),
        created=pendulum.parse(data['created']),
        need_sync=data['need_sync'],
        children={k: _load_local_entry(v) for k, v in data['children'].items()}
    )


def _load_local_file_manifest(data):
    return LocalFileManifest(
        base_version=data['base_version'],
        updated=pendulum.parse(data['updated']),
        created=pendulum.parse(data['created']),
        need_sync=data['need_sync'],
        size=data['size'],
        blocks=data['blocks'],
        dirty_blocks=data['dirty_blocks'],
    )


def _load_entry(data):
    return SyncedEntry(
        id=data['id'],
        key=from_jsonb64(data['key']),
        rts=data['read_trust_seed'],
        wts=data['write_trust_seed'],
    )


def _load_user_manifest(data):
    return UserManifest(
        version=data['version'],
        updated=pendulum.parse(data['updated']),
        created=pendulum.parse(data['created']),
        children={k: _load_entry(v) for k, v in data['children'].items()}
    )


def _load_folder_manifest(data):
    return FolderManifest(
        version=data['version'],
        updated=pendulum.parse(data['updated']),
        created=pendulum.parse(data['created']),
        children={k: _load_entry(v) for k, v in data['children'].items()}
    )


def _load_file_manifest(data):
    return FileManifest(
        version=data['version'],
        updated=pendulum.parse(data['updated']),
        created=pendulum.parse(data['created']),
        size=data['size'],
        blocks=data['blocks']
    )


def dump_manifest(manifest):
    return json.dumps(manifest.dumps()).encode()


@attr.s(slots=True)
class BaseEntry:
    id = attr.ib(default=attr.Factory(lambda: uuid4().hex))


@attr.s(slots=True)
class SyncedEntry(BaseEntry):
    key = attr.ib(default=None)
    rts = attr.ib(default=None)
    wts = attr.ib(default=None)

    def dumps(self):
        return {
            'type': 'synced_entry',
            'id': self.id,
            'read_trust_seed': self.rts,
            'write_trust_seed': self.wts,
            'key': to_jsonb64(self.key)
        }


@attr.s(slots=True)
class NewlySyncedEntry(SyncedEntry):
    placeholder_id = attr.ib(default=None)


@attr.s(slots=True)
class PlaceHolderEntry(BaseEntry):
    key = attr.ib(default=attr.Factory(generate_sym_key))

    def dumps(self):
        return {
            'type': 'placeholder_entry',
            'id': self.id,
            'key': to_jsonb64(self.key)
        }


@attr.s(slots=True)
class LocalFolderManifest:
    base_version = attr.ib(default=0)
    updated = attr.ib(default=attr.Factory(pendulum.utcnow))
    created = attr.ib(default=attr.Factory(pendulum.utcnow))
    children = attr.ib(default=attr.Factory(dict))
    need_sync = attr.ib(default=False)

    def dumps(self):
        return {
            'format': 1,
            'type': 'local_folder_manifest',
            'base_version': self.base_version,
            'updated': self.updated.isoformat(),
            'created': self.created.isoformat(),
            'need_sync': self.need_sync,
            'children': {k: v.dumps() for k, v in self.children.items()}
        }

    def to_sync_manifest(self):
        return FolderManifest(
            version=self.base_version + 1,
            updated=self.updated,
            created=self.created,
            children=self.children
        )


@attr.s(slots=True)
class LocalFileManifest:
    base_version = attr.ib(default=0)
    updated = attr.ib(default=attr.Factory(pendulum.utcnow))
    created = attr.ib(default=attr.Factory(pendulum.utcnow))
    size = attr.ib(default=0)
    blocks = attr.ib(default=attr.Factory(list))
    dirty_blocks = attr.ib(default=attr.Factory(list))
    need_sync = attr.ib(default=False)

    def dumps(self):
        return {
            'format': 1,
            'type': 'local_file_manifest',
            'base_version': self.base_version,
            'updated': self.updated.isoformat(),
            'created': self.created.isoformat(),
            'need_sync': self.need_sync,
            'size': self.size,
            'blocks': self.blocks,
            'dirty_blocks': self.dirty_blocks,
        }

    def to_sync_manifest(self):
        if self.dirty_blocks:
            raise RuntimeError('Cannot convert a file manifest with dirty blocks')
        return FileManifest(
            version=self.base_version + 1,
            updated=self.updated,
            created=self.created,
            size=self.size,
            blocks=self.blocks
        )


@attr.s(slots=True)
class LocalUserManifest(LocalFolderManifest):
    def dumps(self):
        return {
            'format': 1,
            'type': 'local_user_manifest',
            'base_version': self.base_version,
            'updated': self.updated.isoformat(),
            'created': self.created.isoformat(),
            'need_sync': self.need_sync,
            'children': {k: v.dumps() for k, v in self.children.items()}
        }

    def to_sync_manifest(self):
        return UserManifest(
            version=self.base_version + 1,
            updated=self.updated,
            created=self.created,
            children=self.children
        )


def simple_rename(base, diverged, target, entry_name):
    resolved = {
        entry_name: target.children[entry_name]
    }
    name = entry_name
    while True:
        name = '%s.conflict' % name
        if name not in target.children and name not in diverged.children:
            resolved[name] = diverged.children[entry_name]
            break
    return resolved


def merge_folder_manifest(base, diverged, target, on_conflict=simple_rename):
    # If entry is in base but not in diverged and target, it is then already
    # resolved.
    all_entries = diverged.children.keys() | target.children.keys()
    resolved = {}
    for entry in all_entries:
        base_entry = base.children.get(entry)
        target_entry = target.children.get(entry)
        diverged_entry = diverged.children.get(entry)
        if diverged_entry == target_entry:
            # No modifications or same modification on both sides, either case
            # just keep things like this
            resolved[entry] = target_entry
        elif target_entry == base_entry:
            if diverged_entry:
                # Entry has been modified on diverged side only
                resolved[entry] = diverged_entry
        elif diverged_entry == base_entry:
            # Entry has been modified en target side only
            if target_entry:
                resolved[entry] = target_entry
        else:
            # Entry modified on both side...
            if not target_entry:
                # Entry removed on target side, apply diverged modification
                # not to loose it (unless it is a remove of course)
                if diverged_entry:
                    resolved[entry] = diverged_entry
            elif not diverged_entry:
                # Entry removed on diverged side and modified (no remove) on
                # target side, just apply them
                resolved[entry] = target_entry
            else:
                # Entry modified on both side (no remove), last chance to
                # resolve this is if the entry is a placeholder that has been
                # synchronized
                if target_entry.id == diverged_entry.id:
                    # Keep the synced entry, forget about the placeholder one
                    resolved[entry] = target_entry
                else:
                    # Conflict !
                    resolved.update(on_conflict(base, diverged, target, entry))

    return attr.evolve(target, children=resolved)


@attr.s(slots=True)
class FileManifest:
    version = attr.ib(default=1)
    updated = attr.ib(default=attr.Factory(pendulum.utcnow))
    created = attr.ib(default=attr.Factory(pendulum.utcnow))
    size = attr.ib(default=0)
    blocks = attr.ib(default=attr.Factory(list))

    def dumps(self):
        return {
            'format': 1,
            'type': 'file_manifest',
            'version': self.version,
            'updated': self.updated.isoformat(),
            'created': self.created.isoformat(),
            'size': self.size,
            'blocks': self.blocks,
        }


@attr.s(slots=True)
class FolderManifest:
    version = attr.ib(default=1)
    updated = attr.ib(default=attr.Factory(pendulum.utcnow))
    created = attr.ib(default=attr.Factory(pendulum.utcnow))
    children = attr.ib(default=attr.Factory(dict))

    def dumps(self):
        children = {}
        for k, v in self.children.items():
            if isinstance(v, PlaceHolderEntry):
                raise RuntimeError('Sync manifest cannot contains placeholder entries')
            children[k] = {
                'id': v.id,
                'read_trust_seed': v.rts,
                'write_trust_seed': v.wts,
                'key': to_jsonb64(v.key)
            }
        return {
            'format': 1,
            'type': 'folder_manifest',
            'version': self.version,
            'updated': self.updated.isoformat(),
            'created': self.created.isoformat(),
            'children': children
        }


@attr.s(slots=True)
class UserManifest(FolderManifest):

    def dumps(self):
        children = {}
        for k, v in self.children.items():
            if isinstance(v, PlaceHolderEntry):
                raise RuntimeError('Sync manifest cannot contains placeholder entries')
            children[k] = {
                'id': v.id,
                'read_trust_seed': v.rts,
                'write_trust_seed': v.wts,
                'key': to_jsonb64(v.key)
            }
        return {
            'format': 1,
            'type': 'user_manifest',
            'version': self.version,
            'updated': self.updated.isoformat(),
            'created': self.created.isoformat(),
            'children': children
        }


def merge_folder_manifest2(base, diverged, target, on_conflict=simple_rename):
    # Terminator 2, Back to the future 2... all the best things have a 2 in them !
    # Except Jurassic park 2 :'-(

    # If entry is in base but not in diverged and target, it is then already
    # resolved.
    all_entries = diverged.children.keys() | target.children.keys()
    resolved = {}
    for entry in all_entries:
        base_entry = base.children.get(entry)
        target_entry = target.children.get(entry)
        diverged_entry = diverged.children.get(entry)
        if diverged_entry == target_entry:
            # No modifications or same modification on both sides, either case
            # just keep things like this
            resolved[entry] = target_entry
        elif target_entry == base_entry:
            if diverged_entry:
                # Entry has been modified on diverged side only
                resolved[entry] = diverged_entry
        elif diverged_entry == base_entry:
            # Entry has been modified en target side only
            if target_entry:
                resolved[entry] = target_entry
        else:
            # Entry modified on both side...
            if not target_entry:
                # Entry removed on target side, apply diverged modification
                # not to loose it (unless it is a remove of course)
                if diverged_entry:
                    resolved[entry] = diverged_entry
            elif not diverged_entry:
                # Entry removed on diverged side and modified (no remove) on
                # target side, just apply them
                resolved[entry] = target_entry
            else:
                # Entry modified on both side (no remove), last chance to
                # resolve this is if the entry is a placeholder that has been
                # synchronized
                if target_entry.id == diverged_entry.id:
                    # Keep the synced entry, forget about the placeholder one
                    resolved[entry] = target_entry
                else:
                    # Conflict !
                    resolved.update(on_conflict(base, diverged, target, entry))

    return LocalFolderManifest(
        created=base.created,
        updated=diverged.updated,
        base_version=target.version,
        need_sync=diverged.updated > target.updated,
        children=resolved
    )
