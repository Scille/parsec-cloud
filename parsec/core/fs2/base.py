from parsec.core.base import BaseAsyncComponent
from parsec.core.fs2.local_tree import LocalTree
from parsec.core.fs2.opened_file import OpenedFilesManager
from parsec.core.fs2.utils import new_dirty_block_access


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
        # Flush all opened file before leaving
        for fd in self._opened_files.opened_files.values():

            new_size, new_dirty_blocks = fd.get_flush_map()
            for ndb in new_dirty_blocks:
                ndba = new_dirty_block_access(ndb.start, ndb.size)
                self._blocks_manager.flush_on_local2(ndba["id"], ndba["key"], ndb.data)
                fd.manifest["dirty_blocks"].append(ndba)
            fd.manifest["size"] = new_size
            self._local_tree.update_entry(fd.access, fd.manifest)
