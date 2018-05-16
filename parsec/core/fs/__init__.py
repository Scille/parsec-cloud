from parsec.core.fs.access import BasePlaceHolderAccess, BaseVlobAccess, BaseUserVlobAccess
from parsec.core.fs.base import BaseNotLoadedEntry, FSInvalidPath
from parsec.core.fs.block import BaseBlock, BaseBlockAccess, BaseDirtyBlockAccess
from parsec.core.fs.file import BaseFileEntry
from parsec.core.fs.folder import BaseFolderEntry, BaseRootEntry


class FS:

    def __init__(self, device, manifests_manager, blocks_manager):
        self.device = device
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

    async def init(self, nursery):
        access = self._user_vlob_access_cls(None)  # TODO...
        # Note we don't try to get the user manifest from the backend here
        # The reason is we already know version 0 of the manifest (i.e. empty
        # user manifest), so we fallback to it if there is nothing better in
        # the local storage. This way init can be done no matter if the
        # backend is not available.
        user_manifest = await self.manifests_manager.fetch_user_manifest_from_local()
        user_id = self.manifests_manager.device.user_id
        device_name = self.manifests_manager.device.device_name
        if not user_manifest:
            self.root = self._root_entry_cls(
                access, user_id, device_name, name="", need_flush=False, need_sync=False
            )
        else:
            self.root = self._load_entry(
                access, user_id, device_name, name="", parent=None, manifest=user_manifest
            )

    async def teardown(self):
        # TODO: if should be handled by parent
        # Flush what needs to be before leaving to avoid data loss
        if self.root:
            await self.root.flush(recursive=True)

    async def fetch_path(self, path):
        if not path.startswith("/"):
            raise FSInvalidPath("Path must be absolute")

        hops = [n for n in path.split("/") if n]
        entry = self.root
        for hop in hops:
            if not isinstance(entry, BaseFolderEntry):
                raise FSInvalidPath("Path `%s` doesn't exists" % path)

            entry = await entry.fetch_child(hop)
        return entry

    async def update_last_processed_message(self, offset):
        async with self.root.acquire_write():
            self.root._last_processed_message = offset
            self.root._modified()

    def _load_entry(self, access, user_id, device_name, name, parent, manifest):
        if manifest["type"] == "file_manifest":
            blocks_accesses = [self._block_access_cls(**v) for v in manifest["blocks"]]
            return self._file_entry_cls(
                access=access,
                user_id=user_id,
                device_name=device_name,
                need_flush=False,
                need_sync=False,
                name=name,
                parent=parent,
                created=manifest["created"],
                updated=manifest["updated"],
                base_version=manifest["version"],
                size=manifest["size"],
                blocks_accesses=blocks_accesses,
            )

        elif manifest["type"] == "local_file_manifest":
            blocks_accesses = [self._block_access_cls(**v) for v in manifest["blocks"]]
            dirty_blocks_accesses = [
                self._dirty_block_access_cls(**v) for v in manifest["dirty_blocks"]
            ]
            return self._file_entry_cls(
                access=access,
                user_id=user_id,
                device_name=device_name,
                need_flush=False,
                need_sync=manifest["need_sync"],
                name=name,
                parent=parent,
                created=manifest["created"],
                updated=manifest["updated"],
                base_version=manifest["base_version"],
                size=manifest["size"],
                blocks_accesses=blocks_accesses,
                dirty_blocks_accesses=dirty_blocks_accesses,
            )

        elif manifest["type"] in ("folder_manifest", "user_manifest"):
            children_accesses = {
                k: self._vlob_access_cls(**v) for k, v in manifest["children"].items()
            }
            if manifest["type"] == "folder_manifest":
                entry_cls = self._folder_entry_cls
                extra_kwargs = {}
            else:
                entry_cls = self._root_entry_cls
                extra_kwargs = {"last_processed_message": manifest["last_processed_message"]}
            return entry_cls(
                access=access,
                user_id=user_id,
                device_name=device_name,
                need_flush=False,
                need_sync=False,
                name=name,
                parent=parent,
                created=manifest["created"],
                updated=manifest["updated"],
                base_version=manifest["version"],
                children_accesses=children_accesses,
                **extra_kwargs
            )

        elif manifest["type"] in ("local_folder_manifest", "local_user_manifest"):
            children_accesses = {}
            for k, v in manifest["children"].items():
                vtype = v.pop("type")
                if vtype == "vlob":
                    children_accesses[k] = self._vlob_access_cls(**v)
                elif vtype == "placeholder":
                    children_accesses[k] = self._placeholder_access_cls(**v)
                else:
                    raise RuntimeError("Unknown entry type `%s`" % vtype)

            if manifest["type"] == "local_folder_manifest":
                entry_cls = self._folder_entry_cls
                extra_kwargs = {}
            else:
                entry_cls = self._root_entry_cls
                extra_kwargs = {"last_processed_message": manifest["last_processed_message"]}
            return entry_cls(
                access=access,
                user_id=user_id,
                device_name=device_name,
                need_flush=False,
                need_sync=manifest["need_sync"],
                name=name,
                parent=parent,
                created=manifest["created"],
                updated=manifest["updated"],
                base_version=manifest["base_version"],
                children_accesses=children_accesses,
                **extra_kwargs
            )

        else:
            raise RuntimeError("Invalid manifest type `%s`", manifest["type"])
