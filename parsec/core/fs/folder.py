import attr
import pendulum

from parsec.core.fs.base import BaseEntry, FSInvalidPath, FSError
from parsec.core.fs.merge_folder import merge_folder_manifest, merge_children
from parsec.core.backend_storage import BackendConcurrencyError


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

        if not self._need_flush:
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
            id, rts, wts = await self._fs.manifests_manager.sync_new_entry_with_backend(
                key, manifest)
            self._base_version = 1
            self._access = self._fs._vlob_access_cls(id, rts, wts, key)

    async def sync(self, recursive=False):
        # TODO: if file is a placeholder but contains data we sync it two
        # times...
        await self.minimal_sync_if_placeholder()
        async with self.acquire_write():
            await self.flush_no_lock()
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

        # Convert placeholder children into proper synchronized children
        for name, entry in children.items():
            if recursive:
                await entry.minimal_sync_if_placeholder()
            else:
                await entry.sync(recursive=True)
            # TODO: Synchronize with up-to-date data and flush to avoid
            # having to re-synchronize placeholders
            manifest['children'][name] = entry._access.dump(with_type=False)

        # Upload the file manifest as new vlob version
        while True:
            try:
                await self._fs.manifests_manager.sync_with_backend(
                    access.id, access.wts, access.key, manifest)
                break
            except BackendConcurrencyError:
                base = await self._fs.manifests_manager.fetch_from_backend(
                    access.id, access.rts, access.key, version=manifest['version'] - 1)
                # Fetch last version from the backend and merge with it
                # before retrying the synchronization
                target = await self._fs.manifests_manager.fetch_from_backend(
                    access.id, access.rts, access.key)
                # 3-ways merge between base, modified and target versions
                manifest, _ = merge_folder_manifest(base, manifest, target)

        async with self.acquire_write():
            # Else update base_version
            self._base_version = manifest['version']
            await self.flush_no_lock()

            base = await self._fs.manifests_manager.fetch_from_backend(
                access.id, access.rts, access.key, self._base_version)
            target = await self._fs.manifests_manager.fetch_from_backend(
                access.id, access.rts, access.key)

            diverged = self
            _, modified = merge_children(base, diverged, target, inplace=diverged)
            self._need_sync = modified

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

    async def insert_child_as_access(self, name, access):
        async with self.acquire_write():
            return await self.insert_child_as_access(name, access)

    async def insert_child_as_access_no_lock(self, name, access):
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

        if not self._need_flush:
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
        await self._fs.manifests_manager.flush_user_manifest_on_local(manifest)
        self._need_flush = False

    async def sync(self, recursive=False):
        async with self.acquire_write():
            await self.flush_no_lock()
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

        # Convert placeholder children into proper synchronized children
        for name, entry in children.items():
            if recursive:
                await entry.minimal_sync_if_placeholder()
            else:
                await entry.sync(recursive=True)
            # TODO: Synchronize with up-to-date data and flush to avoid
            # having to re-synchronize placeholders
            manifest['children'][name] = entry._access.dump(with_type=False)

        # Upload the file manifest as new vlob version
        while True:
            try:
                await self._fs.manifests_manager.sync_user_manifest_with_backend(manifest)
                break

            except BackendConcurrencyError:
                base = await self._fs.manifests_manager.fetch_user_manifest_from_backend(
                    version=manifest['version'] - 1
                )
                if not base:
                    # base is version 0, which is never stored
                    base = {'children': {}}
                # Fetch last version from the backend and merge with it
                # before retrying the synchronization
                target = await self._fs.manifests_manager.fetch_user_manifest_from_backend()
                # 3-ways merge between base, modified and target versions
                manifest, _ = merge_folder_manifest(base, manifest, target)
        # TODO: If conflict, do a 3-ways merge between base, modified and target versions ?
        async with self.acquire_write():
            # Else update base_version
            self._base_version = manifest['version']
            await self.flush_no_lock()
            # TODO: what if the folder is modified during the sync ?
            # it is now marked as need_sync=False but needs synchro !
            self._need_sync = False

    async def minimal_sync_if_placeholder(self):
        raise RuntimeError("Don't do that on root !")
