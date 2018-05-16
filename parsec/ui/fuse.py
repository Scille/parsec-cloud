import os
import socket
import click
import logbook
import threading
from dateutil.parser import parse as dateparse
from itertools import count
from errno import ENOENT

try:
    from errno import EBADFD
except ImportError:
    from errno import EBADF as EBADFD
from stat import S_IRWXU, S_IRWXG, S_IRWXO, S_IFDIR, S_IFREG
from fuse import FUSE, FuseOSError, Operations, LoggingMixIn, fuse_get_context, fuse_exit

from parsec.utils import from_jsonb64, to_jsonb64, ejson_dumps, ejson_loads


logger = logbook.Logger("parsec.fuse")

DEFAULT_CORE_UNIX_SOCKET = "tcp://127.0.0.1:6776"

# TODO: Currently call fuse_exit from a non fuse thread is not possible
# (see https://github.com/fusepy/fusepy/issues/116).

_need_closing = False


def shutdown_fuse_if_needed():
    if _need_closing:
        fuse_exit()
        raise FuseOSError(ENOENT)


def shutdown_fuse(mountpoint):
    global _need_closing
    _need_closing = True
    # Ask for dummy file just to force a fuse operation that will
    # call the `fuse_exit` from a valid context
    try:
        os.path.exists("%s/__shutdown_fuse__" % mountpoint)
    except OSError:
        pass


class CoreConectionLostError(Exception):
    pass


def _socket_init(socket_address):
    logger.debug("Init socket on %s" % socket_address)
    sock = socket.socket(socket.AF_INET, type=socket.SOCK_STREAM)
    ip = socket_address.split(":")[1][2:]
    port = int(socket_address.split(":")[2])
    sock.connect((ip, port))
    return sock


def _socket_send_cmd(sock, msg):
    req = ejson_dumps(msg).encode() + b"\n"
    logger.debug("Send: %r" % req)
    sock.send(req)
    raw_reps = sock.recv(4096)
    if not raw_reps:
        raise CoreConectionLostError()

    while raw_reps[-1] != ord(b"\n"):
        buff = sock.recv(4096)
        if not buff:
            raise CoreConectionLostError()

        raw_reps += buff
    logger.debug("Received: %r" % raw_reps)
    return ejson_loads(raw_reps[:-1].decode())


def start_shutdown_watcher(socket_address, mountpoint):
    def _shutdown_watcher():
        logger.debug("Starting shutdown watcher")
        sock = _socket_init(socket_address)
        _socket_send_cmd(
            sock,
            {"cmd": "event_subscribe", "event": "fuse_mountpoint_need_stop", "subject": mountpoint},
        )
        logger.debug("Shutdown watcher Started")
        while True:
            try:
                rep = _socket_send_cmd(sock, {"cmd": "event_listen"})
                assert rep["status"] == "ok"
                if rep["event"] == "fuse_mountpoint_need_stop" and rep["subject"] == mountpoint:
                    logger.warning("Received need stop event, exiting...")
                    break

            except CoreConectionLostError as exc:
                logger.warning("Connection with core has been lost, exiting...")
                break

        shutdown_fuse(mountpoint)

    # fuse_exit()

    threading.Thread(target=_shutdown_watcher, daemon=True).start()


class ContentBuilder:
    def __init__(self):
        self.contents = {}

    def write(self, data, offset):
        end_offset = offset + len(data)
        offsets_to_delete = []
        new_data = data
        for current_offset in self.contents:
            current_content = self.contents[current_offset]
            # Insert inside
            if offset >= current_offset and end_offset <= current_offset + len(current_content):
                new_data = current_content[: offset - current_offset]
                new_data += data
                new_data += current_content[offset - current_offset + len(data) :]
                offset = current_offset
            # Insert before and merge
            elif offset <= current_offset and end_offset >= current_offset:
                new_data = data + current_content[offset + len(data) - current_offset :]
                offsets_to_delete.append(current_offset)
            # Insert after
            elif offset == current_offset + len(current_content):
                new_data = current_content[:offset] + new_data
                offset = current_offset
        for offset_to_delete in offsets_to_delete:
            del self.contents[offset_to_delete]
        self.contents[offset] = new_data

    def truncate(self, length):
        offsets_to_delete = []
        for current_offset in self.contents:
            if current_offset > length:
                offsets_to_delete.append(current_offset)
            elif current_offset + len(self.contents[current_offset]) > length:
                data = self.contents[current_offset][: length - current_offset]
                self.contents[current_offset] = data
        for offset_to_delete in offsets_to_delete:
            del self.contents[offset_to_delete]


@click.command()
@click.argument(
    "mountpoint",
    type=click.Path(
        **({"exists": True, "file_okay": False} if os.name == "posix" else {"exists": False})
    ),
)
@click.option("--debug", "-d", is_flag=True, default=False)
@click.option(
    "--log-level", "-l", default="WARNING", type=click.Choice(("DEBUG", "INFO", "WARNING", "ERROR"))
)
@click.option("--log-file", "-o")
@click.option("--nothreads", is_flag=True, default=False)
@click.option(
    "--socket",
    "-s",
    default=DEFAULT_CORE_UNIX_SOCKET,
    help="Path to the UNIX socket (default: %s)." % DEFAULT_CORE_UNIX_SOCKET,
)
def cli(mountpoint, debug, log_level, log_file, nothreads, socket):
    if log_file:
        file_handler = logbook.FileHandler(log_file, level=log_level.upper())
        # Push globally the log handler make it work across threads
        file_handler.push_application()
    else:
        log_handler = logbook.StderrHandler(level=log_level.upper())
        log_handler.push_application()
    start_fuse(socket, mountpoint, debug=debug, nothreads=nothreads)


class File:
    def __init__(self, operations, path, fd, flags=0):
        self.fd = fd
        self.path = path
        self._operations = operations
        self.flags = flags
        self.modifications = []
        self.written_data = 0
        self.max_written_data = 500000

    def read(self, size=None, offset=0):
        self.flush()
        # TODO use flags
        response = self._operations.send_cmd(
            cmd="file_read", path=self.path, size=size, offset=offset
        )
        if response["status"] != "ok":
            raise FuseOSError(ENOENT)

        return from_jsonb64(response["content"])

    def write(self, data, offset=0):
        self.modifications.append((self.write, data, offset))
        self.written_data += len(data)
        if self.written_data > self.max_written_data:
            self.flush()
            self.written_data = 0

    def truncate(self, length):
        self.modifications.append((self.truncate, length))

    def flush(self):
        if not self.modifications:
            return

        # Merge all modifications to build final content
        builder = ContentBuilder()
        shortest_truncate = None
        for modification in self.modifications:
            if modification[0] == self.write:
                builder.write(*modification[1:])
            elif modification[0] == self.truncate:
                builder.truncate(modification[1])
                if not shortest_truncate or shortest_truncate > modification[1]:
                    shortest_truncate = modification[1]
            else:
                raise NotImplementedError()

        self.modifications = []
        # Truncate file
        if shortest_truncate is not None:
            response = self._operations.send_cmd(
                cmd="file_truncate", path=self.path, length=shortest_truncate
            )
            if response["status"] != "ok":
                raise FuseOSError(ENOENT)

        # Write new contents
        for offset, content in builder.contents.items():
            # TODO use flags
            response = self._operations.send_cmd(
                cmd="file_write", path=self.path, content=to_jsonb64(content), offset=offset
            )
            if response["status"] != "ok":
                raise FuseOSError(ENOENT)


class FuseOperations(LoggingMixIn, Operations):
    def __init__(self, socket_address):
        self.fds = {}
        self._fs_id_generator = count(1)
        self._socket_address = socket_address
        # TODO: create a per-thread socket instead of using a lock
        self._socket_lock = threading.Lock()
        self._socket = None

    def get_fd_id(self):
        return next(self._fs_id_generator)

    @property
    def sock(self):
        shutdown_fuse_if_needed()
        if not self._socket:
            self._socket = _socket_init(self._socket_address)
        return self._socket

    def send_cmd(self, **msg):
        with self._socket_lock:
            return _socket_send_cmd(self.sock, msg)

    def _get_fd(self, fh):
        try:
            return self.fds[fh]

        except KeyError:
            raise FuseOSError(EBADFD)

    def _get_file_id(self, path):
        response = self.send_cmd(cmd="list_dir", path=path)
        if response["status"] != "ok":
            raise FuseOSError(ENOENT)

        return response["current"]["id"]

    def getattr(self, path, fh=None):
        stat = self.send_cmd(cmd="stat", path=path)
        if stat["status"] != "ok":
            raise FuseOSError(ENOENT)

        fuse_stat = {}
        # Set it to 777 access
        fuse_stat["st_mode"] = 0
        if stat["type"] == "folder":
            fuse_stat["st_mode"] |= S_IFDIR
            fuse_stat["st_nlink"] = 2
        else:
            fuse_stat["st_mode"] |= S_IFREG
            fuse_stat["st_size"] = stat.get("size", 0)
            fuse_stat["st_ctime"] = dateparse(
                stat["created"]
            ).timestamp()  # TODO change to local timezone
            fuse_stat["st_mtime"] = dateparse(stat["updated"]).timestamp()
            fuse_stat["st_atime"] = dateparse(stat["updated"]).timestamp()  # TODO not supported ?
            fuse_stat["st_nlink"] = 1
        fuse_stat["st_mode"] |= S_IRWXU | S_IRWXG | S_IRWXO
        uid, gid, _ = fuse_get_context()
        fuse_stat["st_uid"] = uid
        fuse_stat["st_gid"] = gid
        return fuse_stat

    def readdir(self, path, fh):
        resp = self.send_cmd(cmd="stat", path=path)
        # TODO: make sure error code for path is not a folder is ENOENT
        if resp["status"] != "ok" or resp["type"] != "folder":
            raise FuseOSError(ENOENT)

        return [".", ".."] + list(resp["children"])

    def create(self, path, mode):
        response = self.send_cmd(cmd="file_create", path=path)
        if response["status"] != "ok":
            raise FuseOSError(ENOENT)

        return self.open(path)

    def open(self, path, flags=0):
        fd_id = self.get_fd_id()
        resp = self.send_cmd(cmd="stat", path=path)
        if resp["status"] != "ok" or resp["type"] != "file":
            raise FuseOSError(ENOENT)

        file = File(self, path, fd_id, flags)
        self.fds[fd_id] = file
        return fd_id

    def release(self, path, fh):
        try:
            file = self.fds.pop(fh)
            file.flush()
        except KeyError:
            raise FuseOSError(EBADFD)

    def read(self, path, size, offset, fh):
        fd = self._get_fd(fh)
        return fd.read(size, offset)

    def write(self, path, data, offset, fh):
        fd = self._get_fd(fh)
        fd.write(data, offset)
        return len(data)

    def truncate(self, path, length, fh=None):
        if not fh:
            try:
                fh = self.open(path, flags=0)
                fd = self._get_fd(fh)
                fd.truncate(length)
            finally:
                self.release(path, fh)
        else:
            fd = self._get_fd(fh)
            fd.truncate(length)

    def unlink(self, path):
        # TODO: check path is a file
        response = self.send_cmd(cmd="delete", path=path)
        if response["status"] != "ok":
            raise FuseOSError(ENOENT)

    def mkdir(self, path, mode):
        response = self.send_cmd(cmd="folder_create", path=path)
        if response["status"] != "ok":
            raise FuseOSError(ENOENT)

        return 0

    def rmdir(self, path):
        # TODO: check directory is empty
        # TODO: check path is a directory
        response = self.send_cmd(cmd="delete", path=path)
        if response["status"] != "ok":
            raise FuseOSError(ENOENT)

        return 0

    def rename(self, src, dst):
        # Unix allows to overwrite the destination, so make sure to have
        # space before calling the move
        self.send_cmd(cmd="delete", path=dst)
        response = self.send_cmd(cmd="move", src=src, dst=dst)
        if response["status"] != "ok":
            raise FuseOSError(ENOENT)

        return 0

    def flush(self, path, fh):
        fd = self._get_fd(fh)
        fd.flush()
        return 0

    def fsync(self, path, datasync, fh):
        return 0  # TODO

    def fsyncdir(self, path, datasync, fh):
        return 0  # TODO


def start_fuse(socket_address: str, mountpoint: str, debug: bool = False, nothreads: bool = False):
    operations = FuseOperations(socket_address)
    start_shutdown_watcher(socket_address, mountpoint)
    FUSE(operations, mountpoint, foreground=True, nothreads=nothreads, debug=debug)


# TODO: shutdown watcher should be able to send here a command to the
# core to signify it has been successfully closed
