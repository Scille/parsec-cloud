from parsec.core.base import BaseAsyncComponent
from parsec.core.fs2.local_tree import LocalTree
from parsec.core.fs2.opened_file import OpenedFilesManager


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
        # TODO
        self._opened_files.flush_all()
