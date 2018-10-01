import attr
from typing import NewType
from math import inf

from parsec.core.local_db import LocalDBMissingEntry
from parsec.core.fs.utils import is_file_manifest, new_block_access
from parsec.core.fs.buffer_ordering import (
    quick_filter_block_accesses,
    Buffer,
    NullFillerBuffer,
    merge_buffers_with_limits,
)
from parsec.core.fs.local_folder_fs import mark_manifest_modified


FileDescriptor = NewType("FileDescriptor", int)


def _shorten_data_repr(data):
    if len(data) > 100:
        return data[:40] + b"..." + data[-40:]
    else:
        return data


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
    access = attr.ib()
    data = attr.ib(default=None)


@attr.s(slots=True)
class BlockBuffer(Buffer):
    access = attr.ib()
    data = attr.ib(default=None)


class FSBlocksLocalMiss(Exception):
    def __init__(self, accesses):
        super().__init__(accesses)
        self.accesses = accesses


@attr.s(slots=True)
class FileCursor:
    access = attr.ib()
    offset = attr.ib(default=0)


@attr.s(slots=True)
class HotFile:
    size = attr.ib()
    base_version = attr.ib()
    pending_writes = attr.ib(factory=list)
    cursors = attr.ib(factory=set)


class FSInvalidFileDescriptor(Exception):
    pass


class LocalFileFS:
    def __init__(self, device, local_folder_fs, event_bus):
        self.event_bus = event_bus
        self.local_folder_fs = local_folder_fs
        self._local_db = device.local_db
        self._opened_cursors = {}
        self._hot_files = {}
        self._next_fd = 1
        # TODO: handle fs.entry.moved events coming from sync

    def get_block(self, access):
        return self._local_db.get(access)

    def set_block(self, access, block):
        return self._local_db.set(access, block)

    def _get_cursor_from_fd(self, fd):
        try:
            return self._opened_cursors[fd]
        except KeyError:
            raise FSInvalidFileDescriptor(fd)

    def _get_quickly_filtered_blocks(self, manifest, start, end):
        dirty_blocks = [
            DirtyBlockBuffer(*x)
            for x in quick_filter_block_accesses(manifest["dirty_blocks"], start, end)
        ]
        blocks = [
            BlockBuffer(*x) for x in quick_filter_block_accesses(manifest["blocks"], start, end)
        ]

        return blocks + dirty_blocks

    def open(self, access) -> FileDescriptor:
        cursor = FileCursor(access)
        # Sanity check
        manifest = self.local_folder_fs.get_manifest(access)
        if not is_file_manifest(manifest):
            raise IsADirectoryError(21, "Is a directory")

        hf = self._ensure_hot_file(access, manifest)
        hf.cursors.add(id(cursor))
        fd = self._next_fd
        self._opened_cursors[fd] = cursor
        self._next_fd += 1
        return fd

    def _ensure_hot_file(self, access, manifest):
        hf = self._hot_files.get(access["id"])
        if not hf:
            hf = HotFile(manifest["size"], manifest["base_version"])
            self._hot_files[access["id"]] = hf
        else:
            assert hf.base_version == manifest["base_version"]

        return hf

    def _get_hot_file(self, access):
        return self._hot_files[access["id"]]

    def _delete_hot_file(self, access):
        del self._hot_files[access["id"]]

    def close(self, fd: FileDescriptor):
        self.flush(fd)
        cursor = self._opened_cursors.pop(fd)
        hf = self._get_hot_file(cursor.access)
        hf.cursors.remove(id(cursor))
        if not hf.cursors:
            self._delete_hot_file(cursor.access)

    def seek(self, fd: FileDescriptor, offset: int):
        cursor = self._get_cursor_from_fd(fd)

        if offset < 0:
            hf = self._get_hot_file(cursor.access)
            cursor.offset = hf.size
        else:
            cursor.offset = offset

    def write(self, fd: FileDescriptor, content: bytes):
        cursor = self._get_cursor_from_fd(fd)

        if not content:
            return

        hf = self._get_hot_file(cursor.access)
        empty_gap = cursor.offset - hf.size
        if empty_gap > 0:
            content = b"\x00" * empty_gap + content
            start = hf.size
            cursor.offset -= empty_gap
        else:
            start = cursor.offset
        end = start + len(content)
        hf.pending_writes.append(RamBuffer(start, end, content))

        cursor.offset += len(content)
        if hf.size < cursor.offset:
            hf.size = cursor.offset

    def truncate(self, fd: FileDescriptor, length: int):
        cursor = self._get_cursor_from_fd(fd)

        hf = self._get_hot_file(cursor.access)
        if hf.size < length:
            hf.pending_writes.append(NullFillerBuffer(hf.size, length))
        hf.size = length
        hf.pending_writes = [pw for pw in hf.pending_writes if pw.start < length]

    def read(self, fd: FileDescriptor, size: int = inf):
        cursor = self._get_cursor_from_fd(fd)

        hf = self._get_hot_file(cursor.access)
        if cursor.offset > hf.size:
            return b""

        manifest = self.local_folder_fs.get_manifest(cursor.access)
        assert is_file_manifest(manifest)

        start = cursor.offset
        end = cursor.offset + size
        if end > hf.size:
            end = hf.size

        blocks = self._get_quickly_filtered_blocks(manifest, start, end)
        blocks += hf.pending_writes

        merged = merge_buffers_with_limits(blocks, start, end)
        assert merged.size <= size
        assert merged.start == start

        missing = []
        data = bytearray(end - start)
        for cs in merged.spaces:
            for bs in cs.buffers:
                if isinstance(bs.buffer, DirtyBlockBuffer):
                    access = bs.buffer.access
                    try:
                        buff = self.get_block(access)
                    except LocalDBMissingEntry as exc:
                        raise RuntimeError(f"Unknown local block `{access['id']}`") from exc

                    data[bs.start - cs.start : bs.end - cs.start] = buff[
                        bs.buffer_slice_start : bs.buffer_slice_end
                    ]

                elif isinstance(bs.buffer, BlockBuffer):
                    access = bs.buffer.access
                    try:
                        buff = self.get_block(access)
                    except LocalDBMissingEntry:
                        missing.append(access)
                        continue

                    data[bs.start - cs.start : bs.end - cs.start] = buff[
                        bs.buffer_slice_start : bs.buffer_slice_end
                    ]

                else:
                    data[bs.start - start : bs.end - start] = bs.get_data()

        if missing:
            raise FSBlocksLocalMiss(missing)

        cursor.offset += len(data)
        return data

    def flush(self, fd):
        cursor = self._get_cursor_from_fd(fd)

        manifest = self.local_folder_fs.get_manifest(cursor.access)
        assert is_file_manifest(manifest)

        hf = self._get_hot_file(cursor.access)
        if manifest["size"] == hf.size and not hf.pending_writes:
            return

        new_dirty_blocks = []
        for pw in hf.pending_writes:
            block_access = new_block_access(pw.data, pw.start)
            self.set_block(block_access, pw.data)
            new_dirty_blocks.append(block_access)

        # TODO: clean overwritten dirty blocks
        manifest["dirty_blocks"] += new_dirty_blocks
        manifest["size"] = hf.size
        mark_manifest_modified(manifest)

        self.local_folder_fs.set_manifest(cursor.access, manifest)

        hf.pending_writes.clear()
        self.event_bus.send("fs.entry.updated", id=cursor.access["id"])
