import attr
import trio
import pendulum
from contextlib import contextmanager
from itertools import dropwhile
from math import inf

from parsec.core.fs.buffer_ordering import (
    quick_filter_block_accesses,
    Buffer,
    UncontiguousSpace,
    ContiguousSpace,
    InBufferSpace,
    merge_buffers,
    merge_buffers_with_limits,
    merge_buffers_with_limits_and_alignment,
)
from parsec.core.fs.utils import is_placeholder_access


def _shorten_data_repr(data):
    if len(data) > 100:
        return data[:40] + b"..." + data[-40:]
    else:
        return data


@attr.s(slots=True, repr=False)
class WriteCmd:
    offset = attr.ib()
    data = attr.ib()
    datetime = attr.ib(default=attr.Factory(pendulum.now))

    @property
    def end(self):
        return self.offset + len(self.data)

    def __repr__(self):
        return "%s(offset=%s, data=%s, datetime=%r)" % (
            type(self).__name__,
            self.offset,
            _shorten_data_repr(self.data),
            self.datetime,
        )


@attr.s(slots=True)
class TruncateCmd:
    length = attr.ib()


@attr.s(slots=True)
class MarkerCmd:
    file_size = attr.ib()
    new_manifest = attr.ib(default=None)
    datetime = attr.ib(default=attr.Factory(pendulum.now))
    synced_until_this_point = attr.ib(default=False)

    def resolve(self, new_manifest=None):
        if new_manifest is not None:
            self.new_manifest = new_manifest
        self.synced_until_this_point = attr.ib()


@attr.s(slots=True, repr=False)
class RamBuffer(Buffer):
    def __repr__(self):
        return "%s(start=%r, end=%r, data=%r)" % (
            type(self).__name__,
            self.start,
            self.end,
            _shorten_data_repr(self.data),
        )


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
        for bs in self.buffers:
            if isinstance(bs.buffer, (RamBuffer, DirtyBlockBuffer)):
                return True
            if bs.buffer.end > self.end:
                # Already synchronized block, but must be truncated
                return True
        return False


@attr.s()
class OpenedFile:

    access = attr.ib()
    _manifest = attr.ib()
    size = attr.ib(default=None)
    block_size = attr.ib(default=2 ** 16)
    _cmds_list = attr.ib(factory=list, repr=False)
    # To simplify concurrency, we use this event to prohibit flushes during
    # sync and multiple concurrent syncs
    _not_syncing_event = attr.ib(factory=trio.Event, repr=False)

    @property
    def base_version(self):
        return self._manifest["base_version"]

    def __attrs_post_init__(self):
        self.size = self._manifest["size"]
        self._not_syncing_event.set()

    def need_sync(self):
        return (
            is_placeholder_access(self.access) or self._manifest["need_sync"] or self.need_flush()
        )

    def need_flush(self):
        return self._manifest["size"] != self.size or any(
            x for x in self._cmds_list if isinstance(x, WriteCmd)
        )

    @contextmanager
    def start_syncing(self):
        if self.is_syncing():
            raise RuntimeError("Already syncing !")
        self._not_syncing_event.clear()
        try:
            marker = self.create_marker()
            yield marker

        finally:
            self.clean_marker(marker)
            self._not_syncing_event.set()

    def is_syncing(self):
        return not self._not_syncing_event.is_set()

    async def wait_not_syncing(self):
        return self._not_syncing_event.wait()

    def write(self, content: bytes, offset: int = -1):
        if not content:
            return

        if offset == -1 or offset > self.size:
            offset = self.size
        self._cmds_list.append(WriteCmd(offset, content))
        end = offset + len(content)
        if end > self.size:
            self.size = end

    def truncate(self, length):
        if length < self.size:
            # TODO: useful ?
            self._cmds_list.append(TruncateCmd(length))
            self.size = length

    def get_not_synced_bounds(self):
        if is_placeholder_access(self.access):
            return 0, self.size

        start = inf
        end = -inf

        for dba in self._manifest["dirty_blocks"]:
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

        if start == inf:
            start = 0
        try:
            # Retrieve the original size from the block access given the
            # size field is overwritten when flushing
            last_block = self._manifest["blocks"][-1]
            original_size = last_block["offset"] + last_block["size"]
            if original_size != self.size:
                end = self.size
        except IndexError:
            pass
        if end == -inf:
            end = 0
        # Needed to handle truncate on not synced data
        if end > self.size:
            end = self.size

        return start, end

    def _get_quickly_filtered_blocks(self, start, end):
        in_ram = [
            RamBuffer(x.offset, x.end, x.data) for x in self._cmds_list if isinstance(x, WriteCmd)
        ]
        dirty_blocks = [
            DirtyBlockBuffer(*x)
            for x in quick_filter_block_accesses(self._manifest["dirty_blocks"], start, end)
        ]
        blocks = [
            BlockBuffer(*x)
            for x in quick_filter_block_accesses(self._manifest["blocks"], start, end)
        ]

        return blocks + dirty_blocks + in_ram

    def get_read_map(self, size: int = -1, offset: int = 0):
        if offset >= self.size:
            return MultiLocationsContiguousSpace(offset, offset, [])

        if size < 0:
            size = self.size
        if offset + size > self.size:
            size = self.size - offset

        blocks = self._get_quickly_filtered_blocks(offset, offset + size)
        merged = merge_buffers_with_limits(blocks, offset, offset + size)
        assert len(merged.spaces) <= 1
        assert merged.size <= size
        assert merged.start == offset
        if merged.spaces:
            cs = merged.spaces[0]
            return MultiLocationsContiguousSpace(cs.start, cs.end, cs.buffers)
        else:
            return MultiLocationsContiguousSpace(offset, offset, [])

    def get_sync_map(self):
        start, end = self.get_not_synced_bounds()
        aligned_start = start - start % self.block_size
        if end % self.block_size:
            aligned_end = end + self.block_size - (end % self.block_size)
            if aligned_end < self.size:
                end = aligned_end
            else:
                end = self.size

        blocks = self._get_quickly_filtered_blocks(0, self.size)
        merged = merge_buffers_with_limits_and_alignment(
            blocks, aligned_start, end, self.block_size
        )

        spaces = []

        if aligned_start != 0:
            buffers = []
            for bm in self._manifest["blocks"]:
                block_start = bm["offset"]
                if block_start >= aligned_start:
                    continue
                block_end = bm["offset"] + bm["size"]
                buffers.append(
                    InBufferSpace(block_start, block_end, BlockBuffer(block_start, block_end, bm))
                )
            spaces.append(MultiLocationsContiguousSpace(0, aligned_start, buffers))

        spaces += [MultiLocationsContiguousSpace(x.start, x.end, x.buffers) for x in merged.spaces]

        if end != self.size:
            buffers = []
            for bm in self._manifest["blocks"]:
                block_start = bm["offset"]
                # We can do this given blocks are all aligned
                if block_start < end:
                    continue
                block_end = bm["offset"] + bm["size"]
                buffers.append(
                    InBufferSpace(block_start, block_end, BlockBuffer(block_start, block_end, bm))
                )
            spaces.append(MultiLocationsContiguousSpace(0, aligned_start, buffers))

        return UncontiguousSpace(0, self.size, spaces)

    def create_marker(self):
        if any(x for x in self._cmds_list if isinstance(x, MarkerCmd)):
            raise RuntimeError("A marker is already set !")

        marker = MarkerCmd(self.size)
        self._cmds_list.append(marker)
        return marker

    def clean_marker(self, marker):
        if marker.new_manifest is not None:
            self._manifest = marker.new_manifest
        if not marker.synced_until_this_point:
            # Just remove the marker
            tmp_list = [x for x in self._cmds_list if x is not marker]
        else:
            # Remove everything up to the marker
            marker_found = False

            def drop_until_marker_is_found(x):
                nonlocal marker_found
                if x is marker:
                    marker_found = True
                    return True  # Also drop the marker
                return not marker_found

            tmp_list = list(dropwhile(drop_until_marker_is_found, self._cmds_list))
            # Given sync must be done with a lock held, no concurrent marker that
            # could potentially remove our own marker is allowed
            assert marker_found

        assert not any(x for x in tmp_list if isinstance(x, MarkerCmd))
        self._cmds_list = tmp_list

    def get_flush_map(self):
        in_ram = [
            RamBuffer(x.offset, x.end, x.data) for x in self._cmds_list if isinstance(x, WriteCmd)
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


@attr.s
class OpenedFilesManager:
    device = attr.ib()
    manifests_manager = attr.ib()
    blocks_manager = attr.ib()
    block_size = attr.ib()
    opened_files = attr.ib(factory=dict)
    _resolved_placeholder_accesses = attr.ib(factory=dict)

    def is_opened(self, access):
        return self.opened_files.get(access["id"])

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
        else:
            assert fd.base_version == manifest["base_version"]
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
            fd = self.opened_files.pop(placeholder_access["id"])
            fd.access = resolved_access
            self.opened_files[resolved_access["id"]] = fd
        except KeyError:
            return
        self._resolved_placeholder_accesses[placeholder_access["id"]] = resolved_access["id"]

    def move_modifications(self, old_access, new_access):
        try:
            fd = self.opened_files.pop(old_access["id"])
        except KeyError:
            # Nothing to move
            return
        fd.access = new_access
        self.opened_files[new_access["id"]] = fd
