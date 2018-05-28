import attr
import pendulum
from itertools import dropwhile
from math import inf

from parsec.core.fs2.buffer_ordering import (
    quick_filter_block_accesses,
    Buffer,
    ContiguousSpace,
    merge_buffers,
    merge_buffers_with_limits,
    merge_buffers_with_limits_and_alignment,
)


@attr.s(slots=True)
class WriteCmd:
    offset = attr.ib()
    buffer = attr.ib()
    datetime = attr.ib(default=attr.Factory(pendulum.now))

    @property
    def end(self):
        return self.offset + len(self.buffer)


@attr.s(slots=True)
class MarkerCmd:
    datetime = attr.ib(default=attr.Factory(pendulum.now))


@attr.s(slots=True)
class TruncateCmd:
    length = attr.ib()
    datetime = attr.ib(default=attr.Factory(pendulum.now))


@attr.s(slots=True)
class RamBuffer(Buffer):
    pass


@attr.s(slots=True)
class DirtyBlockBuffer(Buffer):
    pass


@attr.s(slots=True)
class BlockBuffer(Buffer):
    pass


@attr.s(slots=True)
class MultiLocationsContiguousSpace(ContiguousSpace):

    def get_blocks(self):
        return [bs for bs in self.buffers if isinstance(bs.buffer, BlockBuffer)]

    def get_dirty_blocks(self):
        return [bs for bs in self.buffers if isinstance(bs.buffer, DirtyBlockBuffer)]

    def get_in_ram_buffers(self):
        return [bs for bs in self.buffers if isinstance(bs.buffer, RamBuffer)]

    def need_sync(self):
        return self.get_dirty_blocks() or self.get_in_ram_buffers()


class OpenedFile:

    def __init__(self, access, manifest):
        self.block_size = 2 ** 16
        self.access = access
        self.manifest = manifest
        self.size = manifest["size"]
        self._cmds_list = []

    def write(self, content: bytes, offset: int = -1):
        if offset == -1 or offset > self.size:
            offset = self.size
        self._cmds_list.append(WriteCmd(offset, content))
        end = offset + len(content)
        if end > self.size:
            self.size = end

    def truncate(self, length):
        if length < self.size:
            self.size = length

    def compact(self):
        pass

    def get_not_synced_bounds(self):
        start = inf
        end = -inf

        for dba in self.manifest["dirty_blocks"]:
            if dba["offset"] < start:
                start = dba["offset"]
            dba_end = dba["offset"] + dba["size"]
            if dba_end > end:
                end = dba_end

        for cmd in self._cmds_list:
            if isinstance(cmd, WriteCmd):
                if cmd.offset < start:
                    start = cmd.offset
                if cmd.end > end:
                    end = cmd.end
            elif isinstance(cmd, TruncateCmd):
                if end > cmd.length:
                    end = cmd.length

        if start == inf:
            start = 0
        if end == -inf:
            end = 0

        return start, end

    def _get_quickly_filtered_blocks(self, start, end):
        in_ram = [
            RamBuffer(x.offset, x.end, x.buffer) for x in self._cmds_list if isinstance(x, WriteCmd)
        ]
        dirty_blocks = [
            DirtyBlockBuffer(*x)
            for x in quick_filter_block_accesses(self.manifest["dirty_blocks"], start, end)
        ]
        blocks = [
            BlockBuffer(*x)
            for x in quick_filter_block_accesses(self.manifest["blocks"], start, end)
        ]

        return blocks + dirty_blocks + in_ram

    def get_read_map(self, size: int = -1, offset: int = 0):
        if offset >= self.size:
            return MultiLocationsContiguousSpace(offset, offset, [])

        blocks = self._get_quickly_filtered_blocks(offset, offset + size)
        merged = merge_buffers_with_limits(blocks, offset, offset + size)
        assert len(merged.spaces) == 1
        assert merged.size <= size
        assert merged.start == offset

        cs = merged.spaces[0]
        return MultiLocationsContiguousSpace(cs.start, cs.end, cs.buffers)

        # opened_file_rm = []
        # dirty_blocks_rm = []
        # blocks_rm = []
        # for bs in merged.spaces[0].buffers:
        #     if isinstance(bs.buffer, RamBuffer):
        #         opened_file_rm.append(bs)
        #     elif isinstance(bs.buffer, DirtyBlockBuffer):
        #         dirty_blocks_rm.append(bs)
        #     else:  # BlockBuffer
        #         blocks_rm.append(bs)

        # return merged.size, opened_file_rm, dirty_blocks_rm, blocks_rm

    def get_sync_map(self):
        start, end = self.get_not_synced_bounds()
        aligned_start = start - start % self.block_size
        if not end % self.block_size:
            aligned_end = end
        else:
            aligned_end = end + (self.block_size - end % self.block_size)

        blocks = self._get_quickly_filtered_blocks(aligned_start, aligned_end)
        merged = merge_buffers_with_limits_and_alignment(
            blocks, aligned_start, aligned_end, self.block_size
        )
        assert len(merged.spaces) == 1
        assert merged.size == self.size
        assert merged.start == 0

        cs = merged.spaces[0]
        return MultiLocationsContiguousSpace(cs.start, cs.end, cs.buffers)

    def create_marker(self):
        marker = MarkerCmd()
        self._cmds_list.append(marker)
        return marker

    def drop_until_marker(self, marker):
        marker_found = False

        def drop_until_marker_is_found(x):
            nonlocal marker_found
            marker_found = x is marker
            return marker_found

        tmp_list = list(dropwhile(drop_until_marker_is_found, self._cmds_list))
        # In case two concurrent marker have been created, it's possible
        # the second has destroyed the first if it finishes first
        if marker_found:
            self._cmds_list = tmp_list

    def get_flush_map(self):
        in_ram = [
            RamBuffer(x.offset, x.end, x.buffer) for x in self._cmds_list if isinstance(x, WriteCmd)
        ]
        # TODO: we should determine which dirty block is no longer needed here

        merged = merge_buffers(in_ram)

        buffers = []
        for cs in merged.spaces:
            data = bytearray(cs.size)
            for bs in cs.buffers:
                data[bs.start - cs.start : bs.end - cs.start] = bs.buffer.data[
                    bs.buffer_slice_start : bs.buffer_slice_end
                ]
            buffers.append(Buffer(cs.start, cs.end, data))

        return self.size, buffers


def fast_forward_file_manifest(new_remote_base, current):
    # TODO: must be greatly improved...
    return {
        "type": "local_file_manifest",
        "blocks": new_remote_base["blocks"],
        "created": new_remote_base["created"],
        "device_name": new_remote_base["device_name"],
        "dirty_blocks": [],
        "need_sync": False,
        "base_version": new_remote_base["version"],
        "size": new_remote_base["size"],
        "updated": new_remote_base["updated"],
        "user_id": new_remote_base["user_id"],
    }


@attr.s
class OpenedFilesManager:
    device = attr.ib()
    manifests_manager = attr.ib()
    blocks_manager = attr.ib()
    opened_files = attr.ib(factory=dict)
    _resolved_placeholder_accesses = attr.ib(factory=dict)

    def is_opened(self, access):
        return access["id"] in self.opened_files

    def _file_lookup(self, id):
        try:
            return self.opened_files[id]
        except KeyError:
            try:
                return self.opened_files[self._resolved_placeholder_accesses[id]]
            except KeyError:
                return None

    def open_file(self, access, manifest):
        fd = self._file_lookup(access["id"])
        if not fd:
            fd = OpenedFile(access, manifest)
        self.opened_files[access["id"]] = fd
        return fd

    def close_file(self, access):
        id = access["id"]
        try:
            res = self.opened_files.pop(id)
            self._resolved_placeholder_accesses = {
                k: v for k, v in self._resolved_placeholder_accesses.items() if v != id
            }
            return res
        except KeyError:
            id = self._resolved_placeholder_accesses.pop(id)
            return self.opened_files.pop(id)

    def resolve_placeholder_access(self, placeholder_access, resolved_access):
        try:
            self.opened_files[resolved_access["id"]] = self.opened_files.pop(
                placeholder_access["id"]
            )
        except KeyError:
            return
        self._resolved_placeholder_accesses[placeholder_access["id"]] = resolved_access["id"]

    def flush_all(self):

        fd = self.opened_files.open_file(access, manifest)

        new_size, new_dirty_blocks = fd.get_flush_map()
        for ndb in new_dirty_blocks:
            ndba = new_dirty_block_access(ndb.start, ndb.size)
            self._blocks_manager.flush_on_local2(ndba["id"], ndba["key"], ndb.data)
            manifest["dirty_blocks"].append(ndba)
        manifest["size"] = new_size
        self._local_tree.update_entry(access, manifest)

        self.opened_files.close_file(access)

        # for id, fd in self.opened_files.items():
        #     self.manifests_manager.


#         # size, in_ram, in_local, in_remote = entry.get_read_map(size, offset)

#         return b""
#         # TODO: finish this...

#         # fd = self._open_file(access, manifest)

#         # # Start of atomic operations

#         # size, in_ram, in_local, in_remote = entry.get_read_map(size, offset)

#         # buffer = bytearray(size)

#         # for start, end, content in in_ram:
#         #     buffer[start, end] = content

#         # for start, end, id in in_local:
#         #     content = self.store.get_block(id)
#         #     buffer[start, end] = content

#         # # End of atomic operations

#         # for start, end, content in in_remote:
#         #     await self.blocks_manager.fetch_from_backend()
#         #     content = await self.store.get_remote_block(id)
#         #     buffer[start, end] = content

#         # return content
