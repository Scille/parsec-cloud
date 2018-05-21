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


def fast_forward_file_manifest(new_base, current):
    # TODO: must be greatly improved...
    merged = deepcopy(current)
    merged["base_version"] = new_base["version"]
    merged["need_sync"] = False
    return merged
