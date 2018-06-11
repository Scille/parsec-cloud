import attr
import trio
from contextlib import contextmanager

from parsec.core.base import BaseAsyncComponent
from parsec.core.fs.local_tree import LocalTree
from parsec.core.fs.opened_file import OpenedFilesManager
from parsec.core.fs.utils import new_dirty_block_access


class FSInvalidPath(Exception):
    pass


class FSBase(BaseAsyncComponent):
    def __init__(self, device, manifests_manager, blocks_manager, block_size):
        super().__init__()
        # To simplify concurrency, we prohibit per-entry flushes during sync
        # and multiple concurrent syncs
        self._sync_locks = ResourcesLocker()
        self._device = device
        self._manifests_manager = manifests_manager
        self._blocks_manager = blocks_manager
        self._local_tree = None
        self._opened_files = None
        self._block_size = block_size

    async def _init(self, nursery):
        # Fetch from local storage the data tree
        self._local_tree = LocalTree(self._device, self._manifests_manager)
        self._opened_files = OpenedFilesManager(
            self._device, self._manifests_manager, self._blocks_manager, self._block_size
        )

    async def _teardown(self):
        # TODO: not really elegant way to do this...

        # No concurrent coroutine should be working at this point
        assert not self._sync_locks._locks

        # Flush all opened file before leaving
        for fd in self._opened_files.opened_files.values():
            try:
                manifest = self._local_tree.retrieve_entry_by_access(fd.access)
            except KeyError:
                continue
            if not fd.need_flush():
                continue
            new_size, new_dirty_blocks = fd.get_flush_map()
            for ndb in new_dirty_blocks:
                ndba = new_dirty_block_access(ndb.start, ndb.size)
                self._blocks_manager.flush_on_local(ndba["id"], ndba["key"], ndb.data)
                manifest["dirty_blocks"].append(ndba)
            # TODO: clean useless dirty blocks
            manifest["size"] = new_size
            self._local_tree.update_entry(fd.access, manifest)

    def update_last_processed_message(self, index):
        root_access, root_manifest = self._local_tree.retrieve_entry_sync("/")
        assert index > root_manifest["last_processed_message"]
        root_manifest["last_processed_message"] = index
        self._local_tree.update_entry(root_access, root_manifest)

    def get_last_processed_message(self):
        _, root_manifest = self._local_tree.retrieve_entry_sync("/")
        return root_manifest["last_processed_message"]


@attr.s
class ResourcesLocker:
    _locks = attr.ib(factory=dict)

    @contextmanager
    def lock(self, key):
        if key in self._locks:
            raise RuntimeError("%r is already syncing !" % key)
        self._locks[key] = trio.Event()
        try:
            yield

        finally:
            lock_released = self._locks.pop(key)
            lock_released.set()

    def is_locked(self, key):
        try:
            return not self._locks[key].is_set()
        except KeyError:
            # Not currently locked
            return False

    async def wait_not_locked(self, key):
        try:
            await self._locks[key].wait()
        except KeyError:
            # Not currently syncing
            pass
