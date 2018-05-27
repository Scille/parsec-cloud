import attr
import pendulum
from copy import deepcopy


@attr.s(slots=True)
class WriteCmd:
    offset = attr.ib()
    buffer = attr.ib()
    datetime = attr.ib(default=attr.Factory(pendulum.now))


@attr.s(slots=True)
class SnapshotCmd:
    datetime = attr.ib(default=attr.Factory(pendulum.now))


@attr.s(slots=True)
class TruncateCmd:
    length = attr.ib()
    datetime = attr.ib(default=attr.Factory(pendulum.now))


@attr.s(slots=True)
class OpenedFile:
    access = attr.ib()
    manifest = attr.ib()
    _cmds_list = attr.ib(default=attr.Factory(list))

    def write(self, content: bytes, offset: int = -1):
        self._cmds_list.append(WriteCmd(content, offset))

    def truncate(self, length):
        self._cmds_list.append(TruncateCmd(length))

    def compact(self):
        pass

    def get_read_map(self, size: int = -1, offset: int = 0):
        return (0, [], [], [])

    def snapshot(self):
        self._cmds_list.append(SnapshotCmd())

    def get_flush_map(self):
        pass


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
    _opened_files = attr.ib(default=attr.Factory(dict))

    def is_opened(self, access):
        return access["id"] in self._opened_files

    def open_file(self, access, manifest):
        fd = self._opened_files.get(access["id"])
        if not fd:
            fd = OpenedFile(access, manifest)
        self._opened_files[access["id"]] = fd
        return fd

    def flush_all(self):
        pass
        # for id, fd in self._opened_files.items():
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
