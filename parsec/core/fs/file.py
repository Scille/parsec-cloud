import attr
import pendulum

from parsec.core.fs.base import BaseEntry
from parsec.utils import generate_sym_key


@attr.s(slots=True, init=False)
class BaseFileEntry(BaseEntry):
    _need_flush = attr.ib()
    _need_sync = attr.ib()
    _created = attr.ib()
    _updated = attr.ib()
    _size = attr.ib()
    _blocks = attr.ib()
    _dirty_blocks = attr.ib()
    _rwlock = attr.ib()

    def __init__(self, access, need_flush=True, need_sync=True, created=None, updated=None,
                 name='', parent=None, base_version=0, size=0, blocks_accesses=None,
                 dirty_blocks_accesses=None):
        super().__init__(access, name, parent)
        self._created = created or pendulum.utcnow()
        self._updated = updated or self.created
        self._need_flush = need_flush
        self._need_sync = need_sync
        self._size = size
        self._base_version = base_version
        self._blocks = []
        if blocks_accesses:
            for access in blocks_accesses:
                self._blocks.append(self._fs._block_cls(access))
        self._dirty_blocks = []
        if dirty_blocks_accesses:
            for access in dirty_blocks_accesses:
                self._dirty_blocks.append(self._fs._block_cls(access))

    @property
    def created(self):
        return self._created

    @property
    def updated(self):
        return self._updated

    @property
    def size(self):
        return self._size

    @property
    def base_version(self):
        return self._base_version

    @property
    def need_flush(self):
        return self._need_flush

    @property
    def need_sync(self):
        return self._need_sync

    def _modified(self):
        self._need_flush = True
        self._updated = pendulum.utcnow()

    async def flush_no_lock(self):
        # Convert the patches into dirty manifests
        # Save the new dirty manifests and remove the deprecated ones
        # Serialize the file as local file manifest
        # Save the local file manifest
        if not self._need_flush:
            return
        for dirty_block in self._dirty_blocks:
            await dirty_block.flush_data()
        # Serialize as local folder manifest
        manifest = {
            'format': 1,
            'type': 'local_file_manifest',
            'need_sync': self._need_sync,
            'base_version': self._base_version,
            'created': self._created,
            'updated': self._updated,
            'blocks': [v._access.dump() for v in self._blocks],
            'dirty_blocks': [v._access.dump() for v in self._dirty_blocks],
        }
        # Save the local folder manifest
        access = self._access
        await self._fs.manifests_manager.flush_on_local(access.id, access.key, manifest)
        self._need_flush = False

    async def flush(self):
        if not self._need_flush:
            return
        async with self.acquire_write():
            await self.flush_no_lock()

    async def read_no_lock(self, size=None, offset=0):
        # Determine the blocks and dirty blocks needed for the read
        # Fetch the blocks
        # Fetch the dirty blocks
        # Combine everything together
        size = size if (size is not None and 0 < size < self.size) else self.size
        end = offset + size

        # Get all buffer
        out = bytearray(size)
        # Retrieve each block that could be part of the final buffer
        for block in self._blocks:
            if block.end <= offset or block.offset >= end:
                continue
            else:
                data = await block.fetch_data()
                if block.offset <= offset and block.end < end:
                    out[:block.end - offset] = data[offset - block.offset:]
                elif block.offset > offset and block.end <= end:
                    out[block.offset - offset:block.end - offset] = data
                elif block.offset > offset and block.end > end:
                    out[block.offset - offset:] = data[:end - block.offset]
                else:
                    out[:] = data[offset - block.offset:end - block.offset]
        # Dirty block are historically ordered, so they can be applied
        # one on top of another and we get the right result in the end
        for block in self._dirty_blocks:
            if block.end <= offset or block.offset >= end:
                continue
            else:
                data = await block.fetch_data()
                if block.offset <= offset and block.end < end:
                    out[:block.end - offset] = data[offset - block.offset:]
                elif block.offset > offset and block.end <= end:
                    out[block.offset - offset:block.end - offset] = data
                elif block.offset > offset and block.end > end:
                    out[block.offset - offset:] = data[:end - block.offset]
                else:
                    out[:] = data[offset - block.offset:end - block.offset]

        return out

    async def read(self, size=None, offset=0):
        async with self.acquire_read():
            return await self.read_no_lock(size, offset)

    async def write_no_lock(self, buffer, offset=0):
        # Create a new dirty block
        access = self._fs._dirty_block_access_cls(offset=offset, size=len(buffer))
        block = self._fs._block_cls(access, data=buffer)
        self._dirty_blocks.append(block)
        if offset + len(buffer) > self._size:
            self._size = offset + len(buffer)
        self._modified()

    async def write(self, buffer, offset=0):
        async with self.acquire_write():
            await self.write_no_lock(buffer, offset)

    async def truncate_no_lock(self, length):
        if self._size < length:
            return
        self._size = length
        self._modified()

    async def truncate(self, length):
        async with self.acquire_write():
            await self.truncate_no_lock(length)

    async def minimal_sync_if_placeholder(self):
        if not self.is_placeholder:
            return
        # Don't actually synchronize the data to save to, otherwise
        # consider version 1 is the newly created pristine object
        async with self.acquire_write():
            manifest = {
                'format': 1,
                'type': 'file_manifest',
                'version': 1,
                'created': self._created,
                'updated': self._created,
                'blocks': {},
                'size': 0,
            }
            key = self._access.key
            id, rts, wts = await self._fs.manifests_manager.sync_new_entry_with_backend(
                key, manifest)
            self._base_version = 1
            self._access = self._fs._vlob_access_cls(id, rts, wts, key)

    async def sync(self):
        if not self._need_sync:
            return
        # Make sure we are not a placeholder
        async with self.acquire_write():
            if not self._need_sync:
                return
            # Make sure data are flushed on disk
            await self.flush_no_lock()
            # TODO: Protect the dirty blocks from beeing deleted
            manifest = {
                'format': 1,
                'type': 'file_manifest',
                'version': self._base_version + 1,
                'created': self._created,
                'updated': self._updated,
            }
            # TODO: we should precisely determine what part of the file should
            # be reuploaded instead of replacing everything like this...
            big_ass_buffer = await self.read_no_lock()
            dirty_blocks_count = len(self._dirty_blocks)
        # Upload the new blocks
        key = generate_sym_key()
        id = await self._fs.blocks_manager.sync_new_block_with_backend(key, big_ass_buffer)
        big_ass_access = self._fs._dirty_block_access_cls(
            id=id, key=key, offset=0, size=len(big_ass_buffer))

        # Flush data here given we don't want to lose the upload blocks
        # TODO...

        # Upload the file manifest as new vlob version
        await self._fs.manifests_manager.sync_with_backend(
            self._access.id, self._access.wts, self._access.key, manifest)
        print('------------------------------------')
        # If conflict, raise exception and let parent folder copy ourself somewhere else
        async with self.acquire_write():
            # Merge our synced data with up-to-date dirty_blocks and patches
            self._blocks = [self._fs._block_cls(big_ass_access)]
            # Forget about all the blocks that was part of our synchronization
            self._dirty_blocks = self._dirty_blocks[dirty_blocks_count:]
            # TODO: notify blocks_managers the dirty blocks are no longer useful ?
            self._base_version = manifest['version']
            self.flush_no_lock()
            self._need_sync = bool(self._dirty_blocks)
