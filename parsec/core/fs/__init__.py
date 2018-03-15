from parsec.core.fs.access import *
from parsec.core.fs.base import *
from parsec.core.fs.block import *
from parsec.core.fs.file import *
from parsec.core.fs.folder import *
from parsec.core.manifests_manager import ManifestsManager
from parsec.core.blocks_manager import BlocksManager
from parsec.core.local_storage import LocalStorage
from parsec.core.backend_storage import BackendStorage


async def fs_factory(device, config, backend_conn):
    local_storage = LocalStorage(device.local_storage_db_path)
    local_storage.init()  # TODO: don't forget teardown !
    backend_storage = BackendStorage(backend_conn)
    manifests_manager = ManifestsManager(device, local_storage, backend_storage)
    blocks_manager = BlocksManager(local_storage, backend_storage)
    # await manifests_manager.init()
    # await blocks_manager.init()
    fs = FS(manifests_manager, blocks_manager)
    await fs.init()
    return fs


class FS:

    def __init__(self, manifests_manager, blocks_manager):
        self.manifests_manager = manifests_manager
        self.blocks_manager = blocks_manager
        self._entry_cls_factory()
        self.root = None

    def _entry_cls_factory(self):

        class FileEntry(BaseFileEntry):
            _fs = self

        class FolderEntry(BaseFolderEntry):
            _fs = self

        class NotLoadedEntry(BaseNotLoadedEntry):
            _fs = self

        class RootEntry(BaseRootEntry):
            _fs = self

        class VlobAccess(BaseVlobAccess):
            _fs = self

        class PlaceHolderAccess(BasePlaceHolderAccess):
            _fs = self

        class UserVlobAccess(BaseUserVlobAccess):
            _fs = self

        class Block(BaseBlock):
            _fs = self

        class BlockAccess(BaseBlockAccess):
            _fs = self

        class DirtyBlockAccess(BaseDirtyBlockAccess):
            _fs = self

        self._file_entry_cls = FileEntry
        self._folder_entry_cls = FolderEntry
        self._not_loaded_entry_cls = NotLoadedEntry
        self._root_entry_cls = RootEntry
        self._vlob_access_cls = VlobAccess
        self._placeholder_access_cls = PlaceHolderAccess
        self._user_vlob_access_cls = UserVlobAccess
        self._block_cls = Block
        self._block_access_cls = BlockAccess
        self._dirty_block_access_cls = DirtyBlockAccess

    async def init(self):
        access = self._user_vlob_access_cls(None)  # TODO...
        # Note we don't try to get the user manifest from the backend here
        # The reason is we already know version 0 of the manifest (i.e. empty
        # user manifest), so we fallback to it if there is nothing better in
        # the local storage. This way init can be done no matter if the
        # backend is not available.
        user_manifest = await self.manifests_manager.fetch_user_manifest_from_local()
        if not user_manifest:
            self.root = self._root_entry_cls(
                access, name='', need_flush=True, need_sync=True)
        else:
            self.root = self._load_entry(access, '', None, user_manifest)

    async def teardown(self):
        # TODO: too deeeeeeeep
        # Flush what needs to be before leaving to avoid data loss
        await self.root.flush(recursive=True)
        await self.manifests_manager._backend_storage.backend_conn.teardown()

    async def fetch_path(self, path):
        if not path.startswith('/'):
            raise FSInvalidPath('Path must be absolute')
        hops = [n for n in path.split('/') if n]
        entry = self.root
        for hop in hops:
            if not isinstance(entry, BaseFolderEntry):
                raise FSInvalidPath("Path `%s` doesn't exists" % path)
            entry = await entry.fetch_child(hop)
        return entry

    def _load_entry(self, access, name, parent, manifest):
        if manifest['type'] == 'file_manifest':
            blocks_accesses = [self._block_access_cls(**v) for v in manifest['blocks']]
            return self._file_entry_cls(
                access=access,
                need_flush=False,
                need_sync=False,
                name=name,
                parent=parent,
                created=manifest['created'],
                updated=manifest['updated'],
                base_version=manifest['version'],
                size=manifest['size'],
                blocks_accesses=blocks_accesses,
            )
        elif manifest['type'] == 'local_file_manifest':
            blocks_accesses = [self._block_access_cls(**v) for v in manifest['blocks']]
            dirty_blocks_accesses = [self._dirty_block_access_cls(**v)
                            for v in manifest['dirty_blocks']]
            return self._file_entry_cls(
                access=access,
                need_flush=False,
                need_sync=manifest['need_sync'],
                name=name,
                parent=parent,
                created=manifest['created'],
                updated=manifest['updated'],
                base_version=manifest['base_version'],
                size=manifest['size'],
                blocks_accesses=blocks_accesses,
                dirty_blocks_accesses=dirty_blocks_accesses,
            )
        elif manifest['type'] in ('folder_manifest', 'user_manifest'):
            children_accesses = {k: self._vlob_access_cls(**v)
                                for k, v in manifest['children'].items()}
            if manifest['type'] == 'folder_manifest':
                entry_cls = self._folder_entry_cls
            else:
                entry_cls = self._root_entry_cls
            return entry_cls(
                access=access,
                need_flush=False,
                need_sync=False,
                name=name,
                parent=parent,
                created=manifest['created'],
                updated=manifest['updated'],
                base_version=manifest['version'],
                children_accesses=children_accesses,
            )
        elif manifest['type'] in ('local_folder_manifest', 'local_user_manifest'):
            children_accesses = {}
            for k, v in manifest['children'].items():
                vtype = v.pop('type')
                if vtype == 'vlob':
                    children_accesses[k] = self._vlob_access_cls(**v)
                elif vtype == 'placeholder':
                    children_accesses[k] = self._placeholder_access_cls(**v)
                else:
                    raise RuntimeError('Unknown entry type `%s`' % vtype)
            if manifest['type'] == 'local_folder_manifest':
                entry_cls = self._folder_entry_cls
            else:
                entry_cls = self._root_entry_cls
            return entry_cls(
                access=access,
                need_flush=False,
                need_sync=manifest['need_sync'],
                name=name,
                parent=parent,
                created=manifest['created'],
                updated=manifest['updated'],
                base_version=manifest['base_version'],
                children_accesses=children_accesses,
            )
        else:
            raise RuntimeError('Invalid manifest type `%s`', manifest['type'])
