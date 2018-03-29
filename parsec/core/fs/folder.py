import attr
import pendulum

from parsec.core.fs.base import BaseEntry, BaseNotLoadedEntry, FSInvalidPath, FSError
from parsec.core.fs.merge_folder import merge_folder_manifest, merge_children
from parsec.core.backend_storage import BackendConcurrencyError
from huepy import *


async def _recursive_need_sync(entry):
    if isinstance(entry, BaseNotLoadedEntry):
        # TODO: Do we really need to reload the entire fs tree to retrieve
        # what need to be sync ? couldn't we just interrogate local_storage ?
        entry = await entry.load()
    if isinstance(entry, BaseFolderEntry):
        if entry.need_sync:
            return True
        for child_entry in entry._children.values():
            if await _recursive_need_sync(child_entry):
                return True
    elif entry.need_sync:
        return True
    return False


def _recursive_need_flush(entry):
    if isinstance(entry, BaseNotLoadedEntry):
        return
    if entry.need_flush:
        return True
    if isinstance(entry, BaseFolderEntry):
        for child_entry in entry._children.values():
            if _recursive_need_flush(child_entry):
                return True
    return False


class BaseFolderEntry(BaseEntry):
    _created = attr.ib()
    _updated = attr.ib()
    _children = attr.ib()
    _need_flush = attr.ib()
    _need_sync = attr.ib()
    _base_version = attr.ib()

    def __init__(self, access, need_flush=True, need_sync=True, created=None,
                 updated=None, base_version=0, name='', parent=None,
                 children_accesses=None):
        super().__init__(access, name, parent)
        self._need_flush = need_flush
        self._need_sync = need_sync
        self._created = created or pendulum.utcnow()
        self._updated = updated or self.created
        self._base_version = base_version
        self._children = {}
        if children_accesses:
            for name, access in children_accesses.items():
                self._children[name] = self._fs._not_loaded_entry_cls(
                    access=access,
                    name=name,
                    parent=self
                )

    @property
    def created(self):
        return self._created

    @property
    def updated(self):
        return self._updated

    @property
    def base_version(self):
        return self._base_version

    @property
    def need_flush(self):
        return self._need_flush

    @property
    def need_sync(self):
        return self._need_sync

    def keys(self):
        return self._children.keys()

    def __getitem__(self, name):
        return self._children[name]

    def __contains__(self, name):
        return name in self._children

    def _modified(self):
        self._need_flush = True
        self._need_sync = True
        self._updated = pendulum.utcnow()

    async def flush_no_lock(self, recursive=False):
        if recursive:
            # TODO: recursive lock maybe too violent here (e.g.
            # `flush('/', recursive=True)` means we cannot modify / when
            # recursively flush children, which maybe numerous...)
            for child in self._children.values():
                await child.flush(recursive=True)

        if not _recursive_need_flush(self):
            return

        # Serialize as local folder manifest
        manifest = {
            'format': 1,
            'type': 'local_folder_manifest',
            'need_sync': self._need_sync,
            'base_version': self._base_version,
            'created': self._created,
            'updated': self._updated,
            'children': {k: v._access.dump(with_type=True)
                         for k, v in self._children.items()}
        }
        # Save the local folder manifest
        access = self._access
        print(good(f'flush {self.path} {manifest}'))
        await self._fs.manifests_manager.flush_on_local(access.id, access.key, manifest)
        self._need_flush = False

    async def flush(self, recursive=False):
        if not self._need_flush and not recursive:
            return
        async with self.acquire_write():
            await self.flush_no_lock(recursive)

    async def minimal_sync_if_placeholder(self):
        if not self.is_placeholder:
            return
        # Don't actually synchronize the data to save to, otherwise
        # consider version 1 is the newly created pristine object
        async with self.acquire_write():
            manifest = {
                'format': 1,
                'type': 'folder_manifest',
                'version': 1,
                'created': self._created,
                'updated': self._created,
                'children': {}
            }
            key = self._access.key
            print(run(f'min sync {self.path} {manifest}'))
            id, rts, wts = await self._fs.manifests_manager.sync_new_entry_with_backend(
                key, manifest)
            self._base_version = 1
            self._access = self._fs._vlob_access_cls(id, rts, wts, key)
            self._need_flush = True
            await self.flush_no_lock()

    async def sync(self, recursive=False, ignore_placeholders=False):
        # TODO: if file is a placeholder but contains data we sync it two
        # times...
        await self.minimal_sync_if_placeholder()

        async with self.acquire_write():

            if not await _recursive_need_sync(self):
                # This folder (and it children) hasn't been modified locally,
                # just download last version from the backend if any.
                manifest = await self._fs.manifests_manager.fetch_from_backend(
                    self._access.id, self._access.rts, self._access.key)
                if manifest['version'] != self.base_version:
                    self._created = manifest['created']
                    self._updated = manifest['updated']
                    self._base_version = manifest['version']
                    self._children = {
                        k: self._fs._not_loaded_entry_cls(
                            name=k, parent=self, access=self._fs._vlob_access_cls(**v))
                        for k, v in manifest['children'].items()
                    }
                    return

            # TODO: useful ?
            # await self.flush_no_lock()
            # Make a snapshot of ourself to avoid concurrency
            # Serialize as local folder manifest
            manifest = {
                'format': 1,
                'type': 'folder_manifest',
                'version': self._base_version + 1,
                'created': self._created,
                'updated': self._updated,
                'children': {}
            }
            children = self._children.copy()
            access = self._access

            # TODO: should release the lock here

            # Convert placeholder children into proper synchronized children
            for name, entry in children.items():
                if ignore_placeholders and entry.is_placeholder:
                    continue
                else:
                    if recursive:
                        await entry.sync(recursive=True)
                    else:
                        await entry.minimal_sync_if_placeholder()
                # TODO: Synchronize with up-to-date data and flush to avoid
                # having to re-synchronize placeholders
                manifest['children'][name] = entry._access.dump(with_type=False)

            # Upload the file manifest as new vlob version
            while True:
                try:
                    await self._fs.manifests_manager.sync_with_backend(
                        access.id, access.wts, access.key, manifest)
                    print(que(f'sync {self.path} {manifest}'))
                    break
                except BackendConcurrencyError:
                    print(bad(f'concurrency error sync {self.path}'))
                    print(info('manifest %s' % manifest))
                    base = await self._fs.manifests_manager.fetch_from_backend(
                        access.id, access.rts, access.key, version=manifest['version'] - 1)
                    print(info('base %s' % base))
                    # Fetch last version from the backend and merge with it
                    # before retrying the synchronization
                    target = await self._fs.manifests_manager.fetch_from_backend(
                        access.id, access.rts, access.key)
                    print(info('target %s' % target))
                    # 3-ways merge between base, modified and target versions
                    manifest, _ = merge_folder_manifest(base, manifest, target)
                    print(info('merged %s' % manifest))

            # TODO: should re-acquire the lock here and fix the following
            # Else update base_version
            self._base_version = manifest['version']

            # base = await self._fs.manifests_manager.fetch_from_backend(
            #     access.id, access.rts, access.key, self._base_version-1)
            # target = manifest
            # # target = await self._fs.manifests_manager.fetch_from_backend(
            # #     access.id, access.rts, access.key)
            # diverged = self

            # _, modified = merge_children(base, diverged, target, inplace=self)
            # for k, v in self._children.items():
            #     # TODO: merge_children should take care of this !
            #     if isinstance(v, dict):
            #         self._children[k] = self._fs._not_loaded_entry_cls(
            #             self._vlob_access_cls(**v)
            #         )
            # self._need_sync = modified

            self._children = {
                k: self._fs._not_loaded_entry_cls(self._fs._vlob_access_cls(**v))
                for k, v in manifest['children'].items()
            }
            self._need_sync = False
            self._need_flush = True
            await self.flush_no_lock()

    def _get_child(self, name):
        try:
            return self._children[name]
        except KeyError:
            raise FSInvalidPath("Path `%s/%s` doesn't exists" % (self.path, name))

    async def fetch_child(self, name):
        async with self.acquire_read():
            # Check entry exists and return it if already loaded
            entry = self._get_child(name)
        # If entry hasn't been loaded yet, we must do it now
        if not entry.is_loaded:
            not_loaded_entry = entry
            entry = await not_loaded_entry.load()
            # Finally, update the parent to take into account this entry
            async with self.acquire_write():
                # As usual we must make sure the entry hasn't been tempered
                # in the meantime (e.g. entry deleted or moved)
                if self._children.get(name) is not_loaded_entry:
                    self._children[name] = entry
        return entry

    async def delete_child(self, name):
        async with self.acquire_write():
            return await self.delete_child_no_lock(name)

    async def delete_child_no_lock(self, name):
        entry = self._children.pop(name, None)
        if not entry:
            raise FSInvalidPath("Path `%s/%s` doesn't exists" % (self.path, name))
        self._modified()
        # Disconnect entry to be able to re-insert it somewhere else
        entry._parent = None
        return entry

    async def insert_child_from_access(self, name, access):
        async with self.acquire_write():
            return await self.insert_child_from_access_no_lock(name, access)

    async def insert_child_from_access_no_lock(self, name, access):
        if name in self._children:
            raise FSInvalidPath("Path `%s/%s` already exists" % (self.path, name))
        child = self._fs._not_loaded_entry_cls(
            access=access,
            name=name,
            parent=self
        )
        self._children[name] = child
        self._modified()
        return child

    async def insert_child(self, name, child):
        async with self.acquire_write():
            await self.insert_child_no_lock(name, child)

    async def insert_child_no_lock(self, name, child):
        if child._parent:
            raise FSError("Cannot insert %r in %r given it already has a parent" % (child, self))
        if name in self._children:
            raise FSInvalidPath("Path `%s/%s` already exists" % (self.path, name))
        child._parent = self
        self._children[name] = child
        self._modified()

    async def create_folder(self, name):
        async with self.acquire_write():
            return await self.create_folder_no_lock(name)

    async def create_folder_no_lock(self, name):
        if name in self._children:
            raise FSInvalidPath("Path `%s/%s` already exists" % (self.path, name))
        entry = self._fs._folder_entry_cls(
            access=self._fs._placeholder_access_cls(),
            name=name,
            parent=self,
            need_sync=True,
            need_flush=True,
        )
        self._children[name] = entry
        self._modified()
        return entry

    async def create_file(self, name):
        async with self.acquire_write():
            return await self.create_file_no_lock(name)

    async def create_file_no_lock(self, name):
        if name in self._children:
            raise FSInvalidPath("Path `%s/%s` already exists" % (self.path, name))
        entry = self._fs._file_entry_cls(
            access=self._fs._placeholder_access_cls(),
            name=name,
            parent=self,
            need_sync=True,
            need_flush=True,
        )
        self._children[name] = entry
        self._modified()
        return entry


class BaseRootEntry(BaseFolderEntry):
    async def flush_no_lock(self, recursive=False):
        if recursive:
            # TODO: recursive lock maybe too violent here (e.g.
            # `flush('/', recursive=True)` means we cannot modify / when
            # recursively flush children, which maybe numerous...)
            for child in self._children.values():
                await child.flush(recursive=True)

        if not _recursive_need_flush(self):
            return

        # Serialize as local folder manifest
        manifest = {
            'format': 1,
            'type': 'local_user_manifest',
            'need_sync': self._need_sync,
            'base_version': self._base_version,
            'created': self._created,
            'updated': self._updated,
            'children': {k: v._access.dump(with_type=True)
                         for k, v in self._children.items()}
        }
        # Save the local folder manifest
        print(good(f'flush {self.path} {manifest}'))
        await self._fs.manifests_manager.flush_user_manifest_on_local(manifest)
        self._need_flush = False

    async def sync(self, recursive=False, ignore_placeholders=False):
        async with self.acquire_write():

            if not await _recursive_need_sync(self):
                # This folder (and it children) hasn't been modified locally,
                # just download last version from the backend if any.
                manifest = await self._fs.manifests_manager.fetch_user_manifest_from_backend()
                if manifest and manifest['version'] != self.base_version:
                    self._created = manifest['created']
                    self._updated = manifest['updated']
                    self._base_version = manifest['version']
                    self._children = {
                        k: self._fs._not_loaded_entry_cls(
                            name=k, parent=self, access=self._fs._vlob_access_cls(**v))
                        for k, v in manifest['children'].items()
                    }
                return

            # TODO: seems not useful...
            # await self.flush_no_lock()
            # Make a snapshot of ourself to avoid concurrency
            # Serialize as local folder manifest
            manifest = {
                'format': 1,
                'type': 'user_manifest',
                'version': self._base_version + 1,
                'created': self._created,
                'updated': self._updated,
                'children': {}
            }
            children = self._children.copy()

            # TODO: should release the lock here

            # Convert placeholder children into proper synchronized children
            for name, entry in children.items():
                if ignore_placeholders and entry.is_placeholder:
                    continue
                else:
                    if recursive:
                        await entry.sync(recursive=True)
                    else:
                        await entry.minimal_sync_if_placeholder()
                # TODO: Synchronize with up-to-date data and flush to avoid
                # having to re-synchronize placeholders
                manifest['children'][name] = entry._access.dump(with_type=False)

            # Upload the file manifest as new vlob version
            while True:
                try:
                    await self._fs.manifests_manager.sync_user_manifest_with_backend(manifest)
                    print(que(f'sync {self.path} {manifest}'))
                    break

                except BackendConcurrencyError:
                    print(bad('concurrency error sync in root'))
                    print(info('manifest %s' % manifest))
                    base = await self._fs.manifests_manager.fetch_user_manifest_from_backend(
                        version=manifest['version'] - 1
                    )
                    if not base:
                        # base is version 0, which is never stored
                        base = {'children': {}}
                    print(info('base %s' % base))
                    # Fetch last version from the backend and merge with it
                    # before retrying the synchronization
                    target = await self._fs.manifests_manager.fetch_user_manifest_from_backend()
                    print(info('target %s' % target))
                    # 3-ways merge between base, modified and target versions
                    manifest, _ = merge_folder_manifest(base, manifest, target)
                    print(info('merged %s' % manifest))

            # TODO: should re-acquire the lock here and fix the following
            # TODO: If conflict, do a 3-ways merge between base, modified and target versions ?
            # Else update base_version
            self._base_version = manifest['version']

            # target = manifest
            # base = await self._fs.manifests_manager.fetch_user_manifest_from_backend(self._base_version-1)
            # if not base:
            #     # Fake v0 version
            #     base = {
            #     'format': 1,
            #     'type': 'user_manifest',
            #     'version': 0,
            #     'created': target['created'],
            #     'updated': target['updated'],
            #     'children': {}
            # }
            # # target = await self._fs.manifests_manager.fetch_user_manifest_from_backend()
            # diverged = self

            # _, modified = merge_children(base, diverged, target, inplace=self)
            # for k, v in self._children.items():
            #     # TODO: merge_children should take care of this !
            #     if isinstance(v, dict):
            #         self._children[k] = self._fs._not_loaded_entry_cls(
            #             self._fs._vlob_access_cls(**v)
            #         )
            # self._need_sync = modified

            self._children = {
                k: self._fs._not_loaded_entry_cls(self._fs._vlob_access_cls(**v))
                for k, v in manifest['children'].items()
            }
            self._need_sync = False
            self._need_flush = True
            await self.flush_no_lock()

    async def minimal_sync_if_placeholder(self):
        raise RuntimeError("Don't do that on root !")
