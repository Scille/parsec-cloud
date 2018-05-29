from parsec.core.base import BaseAsyncComponent
from parsec.core.fs.local_tree import LocalTree
from parsec.core.fs.opened_file import OpenedFilesManager
from parsec.core.fs.utils import new_dirty_block_access


class FSInvalidPath(Exception):
    pass


class FSBase(BaseAsyncComponent):

    def __init__(self, device, manifests_manager, blocks_manager):
        super().__init__()
        self._device = device
        self._manifests_manager = manifests_manager
        self._blocks_manager = blocks_manager
        self._local_tree = None
        self._opened_files = None

    async def _init(self, nursery):
        # Fetch from local storage the data tree
        self._local_tree = LocalTree(self._device, self._manifests_manager)
        self._opened_files = OpenedFilesManager(
            self._device, self._manifests_manager, self._blocks_manager
        )

    async def _teardown(self):
        # TODO: not really elegant way to do this...
        # Flush all opened file before leaving
        for fd in self._opened_files.opened_files.values():
            # No concurrent coroutine should be working at this point
            assert not fd.is_syncing()

            try:
                manifest = self._local_tree.retrieve_entry_by_access(fd.access)
            except KeyError:
                continue
            if not fd.need_flush(manifest):
                continue
            new_size, new_dirty_blocks = fd.get_flush_map()
            for ndb in new_dirty_blocks:
                ndba = new_dirty_block_access(ndb.start, ndb.size)
                self._blocks_manager.flush_on_local(ndba["id"], ndba["key"], ndb.data)
                manifest["dirty_blocks"].append(ndba)
            # TODO: clean useless dirty blocks
            manifest["size"] = new_size
            self._local_tree.update_entry(fd.access, manifest)
