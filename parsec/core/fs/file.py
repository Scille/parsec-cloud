import attr
import bisect
import nacl.encoding
import nacl.hash
import pendulum

from parsec.core.fs.base import BaseEntry
from parsec.core.backend_connection import BackendConcurrencyError
from huepy import good, bad, run, que


@attr.s(slots=True, init=False)
class BaseFileEntry(BaseEntry):
    _user_id = attr.ib()
    _device_name = attr.ib()
    _need_flush = attr.ib()
    _need_sync = attr.ib()
    _created = attr.ib()
    _updated = attr.ib()
    _size = attr.ib()
    _blocks = attr.ib()
    _dirty_blocks = attr.ib()
    _rwlock = attr.ib()

    def __init__(
        self,
        access,
        user_id,
        device_name,
        need_flush=True,
        need_sync=True,
        created=None,
        updated=None,
        name="",
        parent=None,
        base_version=0,
        size=0,
        blocks_accesses=None,
        dirty_blocks_accesses=None,
    ):
        super().__init__(access, name, parent)
        self._user_id = user_id
        self._device_name = device_name
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
    def user_id(self):
        return self._user_id

    @property
    def device_name(self):
        return self._device_name

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
        self._need_sync = True
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
            "format": 1,
            "type": "local_file_manifest",
            "user_id": self._user_id,
            "device_name": self._device_name,
            "need_sync": self._need_sync,
            "base_version": self._base_version,
            "created": self._created,
            "updated": self._updated,
            "size": self._size,
            "blocks": [v._access.dump() for v in self._blocks],
            "dirty_blocks": [v._access.dump() for v in self._dirty_blocks],
        }
        # Save the local folder manifest
        access = self._access
        print(good("flush %s %s" % (self.path, manifest)))
        await self._fs.manifests_manager.flush_on_local(access.id, access.key, manifest)
        self._need_flush = False

    async def flush(self, recursive=False):
        # Note recursive argument is not needed here
        if not self._need_flush:
            return

        async with self.acquire_write():
            await self.flush_no_lock()

    async def read_no_lock(self, size=None, offset=0):
        # Determine the blocks and dirty blocks needed for the read
        # Fetch the blocks
        # Fetch the dirty blocks
        # Combine everything together
        size = size if (
            size is not None and 0 <= size + offset < self.size
        ) else self.size - offset
        if size < 0:
            size = 0
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
        if size == 0:
            return b""

        async with self.acquire_read():
            return await self.read_no_lock(size, offset)

    async def write_no_lock(self, buffer, offset=0):
        if offset > self.size:
            offset = self.size
        # Create a new dirty block
        digest = nacl.hash.sha256(bytes(buffer), encoder=nacl.encoding.Base64Encoder)
        access = self._fs._dirty_block_access_cls(
            offset=offset, size=len(buffer), digest=digest
        )
        block = self._fs._block_cls(access, data=buffer)
        self._dirty_blocks.append(block)
        if offset + len(buffer) > self._size:
            self._size = offset + len(buffer)
        self._modified()

    async def write(self, buffer, offset=0):
        if not buffer:
            return

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
                "format": 1,
                "type": "file_manifest",
                "user_id": self._user_id,
                "device_name": self._device_name,
                "version": 1,
                "created": self._created,
                "updated": self._created,
                "blocks": [],
                "size": 0,
            }
            key = self._access.key
            print(run("min sync %s %s" % (self.path, manifest)))
            id, rts, wts = await self._fs.manifests_manager.sync_new_entry_with_backend(
                key, manifest
            )
            self._base_version = 1
            self._access = self._fs._vlob_access_cls(id, rts, wts, key)
            self._need_flush = True
            await self.flush_no_lock()

    async def sync(self, recursive=False):
        # Note recursive argument is not needed here
        # Make sure we are not a placeholder
        await self.minimal_sync_if_placeholder()

        async with self.acquire_write():
            if not self._need_sync:
                manifest = await self._fs.manifests_manager.fetch_from_backend(
                    self._access.id, self._access.rts, self._access.key
                )
                if manifest["version"] != self.base_version:
                    self._user_id = manifest["user_id"]
                    self._device_name = manifest["device_name"]
                    self._created = manifest["created"]
                    self._updated = manifest["updated"]
                    self._base_version = manifest["version"]
                    self._size = manifest["size"]
                    self._blocks = [
                        self._fs._block_cls(self._fs._block_access_cls(**v))
                        for v in manifest["blocks"]
                    ]
                    return

            # Make sure data are flushed on disk
            await self.flush_no_lock()
            # TODO: Protect the dirty blocks from beeing deleted
            manifest = {
                "format": 1,
                "type": "file_manifest",
                "user_id": self._user_id,
                "device_name": self._device_name,
                "version": self._base_version + 1,
                "created": self._created,
                "updated": self._updated,
                "size": self._size,
                "blocks": None,  # Will be computed later
            }
            merged_blocks = []
            curr_file_offset = 0
            for block, offset, end in get_merged_blocks(
                self._blocks, self._dirty_blocks, self._size
            ):
                # TODO: block taken verbatim are rewritten
                buffer = await block.fetch_data()
                buffer = buffer[offset:end]
                access = self._fs._dirty_block_access_cls(
                    offset=curr_file_offset,
                    size=len(buffer),
                    digest=block._access.digest,
                )
                block = self._fs._block_cls(access, data=buffer)
                curr_file_offset += access.size
                merged_blocks.append(block)
            normalized_blocks = []
            curr_file_offset = 0
            for block_group in get_normalized_blocks(merged_blocks, 4096):
                if len(block_group) == 1:
                    block, offset, end = block_group[0]
                    if offset == 0 and end == block.size:
                        # Current block hasn't been changed
                        curr_file_offset += block.size
                        normalized_blocks.append(block)
                        continue

                buffer = bytearray()
                for block, offset, end in block_group:
                    buffer += (await block.fetch_data())[offset:end]

                digest = nacl.hash.sha256(
                    bytes(buffer), encoder=nacl.encoding.Base64Encoder
                )
                access = self._fs._dirty_block_access_cls(
                    offset=curr_file_offset, size=len(buffer), digest=digest
                )
                block = self._fs._block_cls(access, data=buffer)

                curr_file_offset += len(buffer)
                normalized_blocks.append(block)

            dirty_blocks_count = len(self._dirty_blocks)
        # TODO: Flush manifest here given we don't want to lose the upload blocks
        # TODO: block upload in parallel ?
        for normalized_block in normalized_blocks:
            await normalized_block.sync()
        manifest["blocks"] = [v._access.dump() for v in normalized_blocks]

        # Upload the file manifest as new vlob version
        try:
            await self._fs.manifests_manager.sync_with_backend(
                self._access.id, self._access.wts, self._access.key, manifest
            )
            print(que("sync %s %s" % (self.path, manifest)))
        except BackendConcurrencyError:
            # File already modified, must rename ourself in the parent directory
            # to avoid losing data !
            print(bad("concurrency error sync %s" % self.path))
            original_access = self._access
            original_name = self._name

            async with self.acquire_write(), self.parent.acquire_write():

                self._access = self._fs._placeholder_access_cls()
                entry = self._parent._children.pop(self._name)
                while True:
                    self._name += ".conflict"
                    if self._name not in self._parent._children:
                        self._parent._children[self._name] = entry
                        break

                self._parent._children[original_name] = self._fs._not_loaded_entry_cls(
                    original_access
                )
                # Merge&flush
                self._blocks = normalized_blocks
                self._dirty_blocks = self._dirty_blocks[dirty_blocks_count:]
                self._base_version = manifest["version"]
                self._need_flush = True
                await self.flush_no_lock()
                return

        async with self.acquire_write():
            # Merge our synced data with up-to-date dirty_blocks and patches
            self._blocks = normalized_blocks
            # Forget about all the blocks that was part of our synchronization
            self._dirty_blocks = self._dirty_blocks[dirty_blocks_count:]
            # TODO: notify blocks_managers the dirty blocks are no longer useful ?
            self._base_version = manifest["version"]
            self._need_flush = True
            self._need_sync = self._updated != manifest["updated"]
            await self.flush_no_lock()


def get_merged_blocks(file_blocks, file_dirty_blocks, file_size):
    # TODO: add docstring
    # TODO: file_size currently used to handle truncate, remove it ?
    events = []
    for index, block in enumerate(file_blocks):
        if not block.size:
            continue

        bisect.insort(events, (block.offset, index, "start", block))
        bisect.insort(events, (block.end, index, "end", block))
    delta = len(file_blocks)
    for index, block in enumerate(file_dirty_blocks):
        if not block.size:
            continue

        bisect.insort(events, (block.offset, index + delta, "start", block))
        bisect.insort(events, (block.end, index + delta, "end", block))
    blocks = []
    block_stack = []
    start_offset = {}
    for event in events:
        offset, index, action, block = event
        if offset > file_size:
            if action == "end" and index in start_offset:
                offset = file_size
            else:
                continue

        top_index = -1
        if block_stack:
            top_index = block_stack[-1][0]
        if action == "start" and index > top_index:
            if len(block_stack) > 0:
                index_ended, block_ended = block_stack[-1]
                block_ended_start_data = start_offset[index_ended] - block_ended.offset
                block_ended_end_data = offset - block_ended.offset
                if block_ended_start_data != block_ended_end_data:
                    blocks.append(
                        (block_ended, block_ended_start_data, block_ended_end_data)
                    )
            bisect.insort(block_stack, (index, block))
            start_offset[index] = offset
        elif action == "start" and index < top_index:
            bisect.insort(block_stack, (index, block))
        elif action == "end" and index == top_index:
            block_start_data = start_offset[index] - block.offset
            block_end_data = offset - block.offset
            if block_start_data != block_end_data:
                if block.offset == block_start_data and block.end == block_end_data:
                    blocks.append((block, 0, block.size))
                else:
                    blocks.append((block, block_start_data, block_end_data))
            del start_offset[index]
            block_stack.pop()
            if block_stack:
                restart_index, _ = block_stack[-1]
                start_offset[restart_index] = offset
        elif action == "end" and index < top_index:
            block_stack.remove((index, block))
    return blocks


def get_normalized_blocks(blocks, block_size=4096):
    size = sum([block.size for block in blocks])
    if size <= block_size:
        block_list = [(block, 0, block.size) for block in blocks if block.size]
        return [block_list] if block_list else []

    offset_splits = list(range(block_size, size, block_size))
    offset_splits.append(offset_splits[-1] + block_size)
    final_blocks = []
    block_group = []
    blocks = [(block, block.offset, block.end) for block in blocks if block.size]
    while blocks:
        block, offset, end = blocks.pop(0)
        if offset < offset_splits[0] and end > offset_splits[0]:
            block_group.append(
                (block, offset - block.offset, offset_splits[0] - block.offset)
            )
            final_blocks.append(block_group)
            block_group = []
            blocks.insert(0, (block, offset_splits[0], block.end))
            offset_splits.pop(0)
        else:
            if offset == block.offset and end == block.end:
                block_group.append((block, 0, block.size))
            else:
                block_group.append((block, offset - block.offset, end - block.offset))
            if end == offset_splits[0]:
                final_blocks.append(block_group)
                block_group = []
                offset_splits.pop(0)
    if block_group:
        final_blocks.append(block_group)
    return final_blocks
