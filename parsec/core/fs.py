from copy import deepcopy

import attr
import blinker
from effect import TypeDispatcher
from effect2 import Effect, do

from parsec.core.identity import EIdentityGet
from parsec.core.file import File
from parsec.core.manifest import UserManifest
from parsec.core.synchronizer import ESynchronize as ESynchronizerSynchronize
from parsec.exceptions import FileNotFound, ManifestError, ManifestNotFound


@attr.s
class ESynchronize:
    pass


@attr.s
class EGroupCreate:
    group = attr.ib()


@attr.s
class EDustbinShow:
    path = attr.ib(default=None)


@attr.s
class EManifestHistory:
    first_version = attr.ib(default=1)
    last_version = attr.ib(default=None)
    summary = attr.ib(default=False)


@attr.s
class EManifestRestore:
    version = attr.ib(default=None)


@attr.s
class EFileCreate:
    path = attr.ib()


@attr.s
class EFileRead:
    path = attr.ib()
    offset = attr.ib(default=0)
    size = attr.ib(default=None)


@attr.s
class EFileWrite:
    path = attr.ib()
    content = attr.ib()
    offset = attr.ib()


@attr.s
class EFileTruncate:
    path = attr.ib()
    length = attr.ib()


@attr.s
class EFileHistory:
    path = attr.ib()
    first_version = attr.ib()
    last_version = attr.ib()


@attr.s
class EFileRestore:
    version = attr.ib()


@attr.s
class EFolderCreate:
    path = attr.ib()


@attr.s
class EStat:
    path = attr.ib()


@attr.s
class EMove:
    src = attr.ib()
    dst = attr.ib()


@attr.s
class EDelete:
    path = attr.ib()


@attr.s
class EUndelete:
    vlob = attr.ib()


class FSComponent:

    def __init__(self):
        self.user_manifest = None
        blinker.signal('app_start').connect(self.on_app_start)

    @do
    def on_app_start(self, app):
        self.app = app
        self.user_manifest = yield self._get_manifest()

    @do
    def perform_synchronize(self, intent):
        yield self.user_manifest.commit()
        yield Effect(ESynchronizerSynchronize())

    @do
    def perform_group_create(self, intent):
        yield self.user_manifest.create_group_manifest(intent.group)

    @do
    def perform_dustbin_show(self, intent):
        dustbin = yield self.user_manifest.show_dustbin(intent.path)
        return dustbin

    @do
    def perform_manifest_history(self, intent):
        history = self.user_manifest.history(intent.first_version,
                                             intent.last_version,
                                             intent.summary)
        return history

    @do
    def perform_manifest_restore(self, intent):
        yield self.user_manifest.history(intent.version)

    @do
    def perform_file_create(self, intent):
        file = yield File.create()
        vlob = file.get_vlob()
        try:
            self.user_manifest.add_file(intent.path, vlob)
        except (ManifestError, ManifestNotFound) as ex:
            yield file.discard()
            raise ex

    @do
    def perform_file_read(self, intent):
        file = yield self._get_file(intent.path)
        ret = yield file.read(intent.size, intent.offset)
        return ret

    @do
    def perform_file_write(self, intent):
        file = yield self._get_file(intent.path)
        yield file.write(intent.content, intent.offset)

    @do
    def perform_file_truncate(self, intent):
        file = yield self._get_file(intent.path)
        yield file.truncate(intent.length)

    @do
    def perform_file_history(self, intent):
        file = yield self._get_file(intent.path)
        history = yield file.history(intent.first_version, intent.last_version)
        return history

    @do
    def perform_file_restore(self, intent):
        file = yield self._get_file(intent.path)
        yield file.restore(intent.version)

    @do
    def perform_folder_create(self, intent):
        self.user_manifest.create_folder(intent.path)

    @do
    def perform_stat(self, intent):
        stat = yield self.user_manifest.stat(intent.path)
        return stat

    @do
    def perform_move(self, intent):
        yield self.user_manifest.stat(intent.src, intent.dst)

    @do
    def perform_delete(self, intent):
        yield self.user_manifest.delete(intent.path)

    @do
    def perform_undelete(self, intent):
        yield self.user_manifest.undelete(intent.vlob)

    @do
    def _get_file(self, path, group=None):
        try:
            properties = yield self._get_properties(path=path, group=group)
        except FileNotFound:
            try:
                properties = yield self._get_properties(path=path, dustbin=True, group=group)
            except FileNotFound:
                raise FileNotFound('Vlob not found.')
        file = yield File.load(properties['id'],
                               properties['key'],
                               properties['read_trust_seed'],
                               properties['write_trust_seed'])
        return file

    @do
    def _get_manifest(self, group=None):
        if (not self.user_manifest or
            self.user_manifest.encryptor._hazmat_private_key !=
                self.app.identity.private_key._hazmat_private_key):
            manifest = yield UserManifest.load(self.app.identity.private_key._hazmat_private_key)
        else:
            manifest = self.user_manifest
        if group:
            return manifest.get_group_manifest(group)
        else:
            return manifest

    @do
    def _get_properties(self, path=None, id=None, dustbin=False, group=None):  # TODO refactor?
        if group and not id and not path:
            manifest = yield self._get_manifest(group)
            return manifest.get_vlob()
        groups = [group] if group else [None] + list(self.user_manifest.get_group_vlobs())
        for current_group in groups:
            manifest = yield self._get_manifest(current_group)
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

    def get_dispatcher(self):
        return TypeDispatcher({
            ESynchronize: self.perform_synchronize,
            EGroupCreate: self.perform_group_create,
            EDustbinShow: self.perform_dustbin_show,
            EManifestHistory: self.perform_manifest_history,
            EManifestRestore: self.perform_manifest_restore,
            EFileCreate: self.perform_file_create,
            EFileRead: self.perform_file_read,
            EFileWrite: self.perform_file_write,
            EFileTruncate: self.perform_file_truncate,
            EFileHistory: self.perform_file_history,
            EFileRestore: self.perform_file_restore,
            EFolderCreate: self.perform_folder_create,
            EStat: self.perform_stat,
            EMove: self.perform_move,
            EDelete: self.perform_delete,
            EUndelete: self.perform_undelete
        })
