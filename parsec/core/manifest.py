import attr
import pendulum
from uuid import uuid4
import json

from parsec.utils import ParsecError, from_jsonb64, to_jsonb64, generate_sym_key


class InvalidManifestError(ParsecError):
    status = 'invalid_manifest'


class InvalidSignatureError(ParsecError):
    status = 'invalid_signature'


def load_local_manifest(raw):
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
    else:
        raise InvalidManifestError()


def _load_entry(data):
    if data['type'] == 'synced_entry':
        return SyncedEntry(
            # TODO: local_id and syncid are pretty misleading...
            id=data['local_id'],
            syncid=data['id'],
            key=from_jsonb64(data['key']),
            rts=data['read_trust_seed'],
            wts=data['write_trust_seed'],
        )
    elif data['type'] == 'placeholder_entry':
        return PlaceHolderEntry(
            id=data['local_id'],
            key=from_jsonb64(data['key']),
        )
    else:
        raise InvalidManifestError('Unknown entry type: %r' % data)


def _load_local_user_manifest(data):
    return LocalUserManifest(
        base_version=data['base_version'],
        updated=pendulum.parse(data['updated']),
        created=pendulum.parse(data['created']),
        children={k: _load_entry(v) for k, v in data['children'].items()}
    )


def _load_local_folder_manifest(data):
    return LocalFolderManifest(
        base_version=data['base_version'],
        updated=pendulum.parse(data['updated']),
        created=pendulum.parse(data['created']),
        children={k: _load_entry(v) for k, v in data['children'].items()}
    )


def _load_local_file_manifest(data):
    return LocalFileManifest(
        base_version=data['base_version'],
        updated=pendulum.parse(data['updated']),
        created=pendulum.parse(data['created']),
        size=data['size'],
        blocks=data['blocks'],
        dirty_blocks=data['dirty_blocks'],
    )


def dump_local_manifest(manifest):
    return json.dumps(manifest.dumps()).encode()


@attr.s(slots=True)
class BaseEntry:
    # Local id, not to be mistaken with the syncid
    id = attr.ib(default=attr.Factory(lambda: uuid4().hex))


@attr.s(slots=True)
class SyncedEntry(BaseEntry):
    syncid = attr.ib(default=None)
    key = attr.ib(default=None)
    rts = attr.ib(default=None)
    wts = attr.ib(default=None)

    def dumps(self):
        return {
            'type': 'synced_entry',
            'local_id': self.id,
            'id': self.syncid,
            'read_trust_seed': self.rts,
            'write_trust_seed': self.wts,
            'key': to_jsonb64(self.key)
        }


@attr.s(slots=True)
class PlaceHolderEntry(BaseEntry):
    key = attr.ib(default=attr.Factory(generate_sym_key))

    def dumps(self):
        return {
            'type': 'placeholder_entry',
            'local_id': self.id,
            'key': to_jsonb64(self.key)
        }

@attr.s(slots=True)
class LocalFolderManifest:
    base_version = attr.ib(default=0)
    updated = attr.ib(default=attr.Factory(pendulum.utcnow))
    created = attr.ib(default=attr.Factory(pendulum.utcnow))
    children = attr.ib(default=attr.Factory(dict))

    def dumps(self):
        return {
            'format': 1,
            'type': 'local_folder_manifest',
            'base_version': self.base_version,
            'updated': self.updated.isoformat(),
            'created': self.created.isoformat(),
            'children': {k: v.dumps() for k, v in self.children.items()}
        }

    def to_sync_manifest(self):
        return {
            'format': 1,
            'type': 'folder_manifest',
            'version': self.base_version + 1,
            'updated': self.updated.isoformat(),
            'created': self.created.isoformat(),
            'children': {k: v.dumps() for k, v in self.children.items()}
        }

    @classmethod
    def load_from_sync_manifest(cls, data):
        return cls(
            base_version=data['version'],
            updated=pendulum.parse(data['updated']),
            created=pendulum.parse(data['created']),
            children=data['children'],
        )


@attr.s(slots=True)
class LocalFileManifest:
    base_version = attr.ib(default=0)
    updated = attr.ib(default=attr.Factory(pendulum.utcnow))
    created = attr.ib(default=attr.Factory(pendulum.utcnow))
    size = attr.ib(default=0)
    blocks = attr.ib(default=attr.Factory(list))
    dirty_blocks = attr.ib(default=attr.Factory(list))

    def dumps(self):
        return {
            'format': 1,
            'type': 'local_file_manifest',
            'base_version': self.base_version,
            'updated': self.updated.isoformat(),
            'created': self.created.isoformat(),
            'size': self.size,
            'blocks': self.blocks,
            'dirty_blocks': self.dirty_blocks,
        }


@attr.s(slots=True)
class LocalUserManifest(LocalFolderManifest):
    def dumps(self):
        return {
            'format': 1,
            'type': 'local_user_manifest',
            'base_version': self.base_version,
            'updated': self.updated.isoformat(),
            'created': self.created.isoformat(),
            'children': {k: v.dumps() for k, v in self.children.items()}
        }


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
