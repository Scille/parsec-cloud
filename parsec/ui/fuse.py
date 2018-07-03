import os
from os import fsencode, fsdecode
from collections import defaultdict
import faulthandler
import signal
import socket
import click
import logbook
from raven.handlers.logbook import SentryHandler
import threading
from dateutil.parser import parse as dateparse
from itertools import count
from errno import ENOENT

from stat import S_IRWXU, S_IRWXG, S_IRWXO, S_IFDIR, S_IFREG
import llfuse
from llfuse import FUSEError

from parsec.core.config import CoreConfig
from parsec.utils import from_jsonb64, to_jsonb64, ejson_dumps, ejson_loads


logger = logbook.Logger("parsec.fuse")

DEFAULT_CORE_UNIX_SOCKET = "tcp://127.0.0.1:6776"


faulthandler.enable()


# apt-get install libfuse-dev libattr1-dev
# python setup.py install --user
# export PYTHONPATH=$PYTHONPATH:/home/rossigneux/.local/lib/python3.6/site-packages/


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


def start_shutdown_watcher(operations, socket_address, mountpoint):
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

        operations.fuse_exit()

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
        log_handler = logbook.FileHandler(log_file, level=log_level.upper())
    else:
        log_handler = logbook.StderrHandler(level=log_level.upper())
    # Push globally the log handler make it work across threads
    log_handler.push_application()

    config = CoreConfig()
    if config.sentry_url:
        sentry_handler = SentryHandler(config.sentry_url, level="WARNING")
        sentry_handler.push_application()

    start_fuse(socket, mountpoint, debug=debug, nothreads=nothreads)


class File:
    def __init__(self, operations, path, flags=0):
        self.attr = None
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
            raise FUSEError(ENOENT)

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
                raise FUSEError(ENOENT)

        # Write new contents
        for offset, content in builder.contents.items():
            # TODO use flags
            response = self._operations.send_cmd(
                cmd="file_write", path=self.path, content=to_jsonb64(content), offset=offset
            )
            if response["status"] != "ok":
                raise FUSEError(ENOENT)
        # Flush
        response = self._operations.send_cmd(cmd="flush", path=self.path)
        if response["status"] != "ok":
            raise FUSEError(ENOENT)


class Operations(llfuse.Operations):
    def __init__(self, socket_address):
        super().__init__()
        self._socket_lock = threading.Lock()
        self._socket = None
        self._socket_address = socket_address
        self._inode_path_map = {llfuse.ROOT_INODE: "/"}
        self._path_inode_map = {"/": llfuse.ROOT_INODE}
        self._fd_inode_map = dict()
        self._inode_fd_map = dict()
        self._fd_fh_map = dict()
        self._fh_fd_map = dict()
        self._lookup_count = defaultdict(lambda: 0)
        self._fd_open_count = defaultdict(lambda: 0)
        self._fd_id_generator = count(2)
        self._inode_generator = count(2)

    @property
    def sock(self):
        if not self._socket:
            self._socket = _socket_init(self._socket_address)
        return self._socket

    def send_cmd(self, **msg):
        with self._socket_lock:
            return _socket_send_cmd(self.sock, msg)

    def fuse_exit(self):
        os.kill(os.getpid(), signal.SIGTERM)

    def get_fh(self):
        return next(self._fd_id_generator)

    def get_inode(self):
        return next(self._inode_generator)

    def _add_inode_path(self, inode, path):
        self._lookup_count[inode] += 1
        self._inode_path_map[inode] = path
        self._path_inode_map[path] = inode

    def _add_fd_inode(self, fd, fh):
        self._fd_open_count[fd] += 1
        self._fd_inode_map[fd] = fh
        self._inode_fd_map[fh] = fd

    def _add_fd_fh(self, fd, fh):
        self._fd_fh_map[fd] = fh
        self._fh_fd_map[fh] = fd

    def forget(self, inode_list):
        for (inode, nlookup) in inode_list:
            if self._lookup_count[inode] > nlookup:
                self._lookup_count[inode] -= nlookup
                continue
            assert inode not in self._inode_fd_map
            del self._lookup_count[inode]
            try:
                path = self._inode_path_map[inode]
                del self._inode_path_map[inode]
                del self._path_inode_map[path]
            except KeyError:  # may have been deleted
                pass

    def lookup(self, inode_p, name, ctx=None):
        name = fsdecode(name)
        path = os.path.join(self._inode_path_map[inode_p], name)
        attr = self._getattr(path=path)
        if name != "." and name != "..":
            self._add_inode_path(attr.st_ino, path)
        return attr

    def getattr(self, inode, ctx=None):
        if inode in self._inode_fd_map:
            return self._getattr(fd=self._inode_fd_map[inode])
        else:
            return self._getattr(path=self._inode_path_map[inode])

    def _getattr(self, path=None, fd=None):
        if fd:
            path = self._inode_path_map[self._fd_inode_map[fd]]
        stat = self.send_cmd(cmd="stat", path=path)
        if stat["status"] != "ok":
            raise FUSEError(ENOENT)

        entry = llfuse.EntryAttributes()
        # Set it to 777 access
        entry.st_mode = 0
        if path not in self._path_inode_map:
            inode = next(self._inode_generator)
            self._add_inode_path(inode, path)
        entry.st_ino = self._path_inode_map[path]
        if stat["type"] == "folder":
            entry.st_mode |= S_IFDIR
            entry.st_nlink = 2
        else:
            entry.st_mode |= S_IFREG
            entry.st_size = stat.get("size", 0)
            entry.st_ctime_ns = (
                dateparse(stat["created"]).timestamp() * 10e9
            )  # TODO change to local timezone
            entry.st_mtime_ns = dateparse(stat["updated"]).timestamp() * 10e9
            entry.st_atime_ns = (
                dateparse(stat["updated"]).timestamp() * 10e9
            )  # TODO not supported ?
            entry.st_nlink = 1
        entry.st_mode |= S_IRWXU | S_IRWXG | S_IRWXO
        entry.st_uid = os.getuid()
        entry.st_gid = os.getgid()
        return entry

    def setattr(self, inode, attr, fields, fh, ctx=None):
        if fields.update_size:
            fd = self._inode_fd_map[inode]
            fd.truncate(attr.st_size)
            fd.flush()
        return self.getattr(inode)

    def opendir(self, inode, ctx):
        return inode

    def readdir(self, inode, off):
        path = self._inode_path_map[inode]
        resp = self.send_cmd(cmd="stat", path=path)
        # TODO: make sure error code for path is not a folder is ENOENT
        if resp["status"] != "ok" or resp["type"] != "folder":
            raise FUSEError(ENOENT)
        entries = []
        for name in list(resp["children"]):
            try:
                attr = self._getattr(path=os.path.join(path, name))
            except FUSEError:
                pass
            else:
                entries.append((attr.st_ino, name, attr))
        for (ino, name, attr) in sorted(entries):
            if ino <= off:
                continue
            yield (fsencode(name), attr, attr.st_ino)

    def unlink(self, inode_p, name, ctx):
        name = fsdecode(name)
        parent = self._inode_path_map[inode_p]
        path = os.path.join(parent, name)
        response = self.send_cmd(cmd="delete", path=path)
        if response["status"] != "ok":
            raise FUSEError(ENOENT)

        inode = self._path_inode_map[path]
        del self._inode_path_map[inode]
        del self._path_inode_map[path]

    def rmdir(self, inode_p, name, ctx):
        name = fsdecode(name)
        parent = self._inode_path_map[inode_p]
        path = os.path.join(parent, name)
        response = self.send_cmd(cmd="delete", path=path)
        if response["status"] != "ok":
            raise FUSEError(ENOENT)

        inode = self._path_inode_map[path]
        del self._inode_path_map[inode]
        del self._path_inode_map[path]

    def rename(self, inode_p_old, name_old, inode_p_new, name_new, ctx):
        name_old = fsdecode(name_old)
        name_new = fsdecode(name_new)
        parent_old = self._inode_path_map[inode_p_old]
        parent_new = self._inode_path_map[inode_p_new]
        path_old = os.path.join(parent_old, name_old)
        path_new = os.path.join(parent_new, name_new)

        self.send_cmd(cmd="delete", path=path_new)
        response = self.send_cmd(cmd="move", src=path_old, dst=path_new)
        if response["status"] != "ok":
            raise FUSEError(ENOENT)

        inode = self._path_inode_map[path_old]
        self._path_inode_map[path_new] = inode
        del self._path_inode_map[path_old]
        self._inode_path_map[inode] = path_new

        for path in list(self._path_inode_map):
            if path.startswith(path_old):
                new_path = path_new + path[len(path_old) :]
                inode = self._path_inode_map[path]
                del self._inode_path_map[inode]
                del self._path_inode_map[path]
                self._inode_path_map[inode] = new_path
                self._path_inode_map[new_path] = inode

    def mkdir(self, inode_p, name, mode, ctx):
        path = os.path.join(self._inode_path_map[inode_p], fsdecode(name))
        response = self.send_cmd(cmd="folder_create", path=path)
        if response["status"] != "ok":
            raise FUSEError(ENOENT)
        attr = self._getattr(path=path)
        self._add_inode_path(attr.st_ino, path)
        return attr

    def open(self, inode, flags, ctx):
        if inode in self._inode_fd_map:
            fd = self._inode_fd_map[inode]
            self._fd_open_count[fd] += 1
            return self._fd_fh_map[fd]

        resp = self.send_cmd(cmd="stat", path=self._inode_path_map[inode])
        if resp["status"] != "ok" or resp["type"] != "file":
            raise FUSEError(ENOENT)

        try:
            fd = self._inode_fd_map[inode]
        except KeyError:
            fd = File(self, self._inode_path_map[inode], flags)
            self._add_fd_inode(fd, inode)
        try:
            fh = self._fd_fh_map[fd]
        except KeyError:
            fh = self.get_fh()
            self._add_fd_fh(fd, fh)
        return fh

    def create(self, inode_p, name, mode, flags, ctx):
        path = os.path.join(self._inode_path_map[inode_p], fsdecode(name))

        response = self.send_cmd(cmd="file_create", path=path)
        if response["status"] != "ok":
            raise FUSEError(ENOENT)

        new_inode = next(self._inode_generator)
        self._add_inode_path(new_inode, path)
        fh = self.open(new_inode, flags, ctx)
        attr = self._getattr(path=path)
        return (fh, attr)

    def read(self, fh, offset, length):
        return self._fh_fd_map[fh].read(length, offset)

    def write(self, fh, offset, buf):
        self._fh_fd_map[fh].write(buf, offset)
        return len(buf)

    def flush(self, fh):
        fd = self._fh_fd_map[fh]
        fd.flush()

    def release(self, fh):
        fd = self._fh_fd_map[fh]
        if self._fd_open_count[fd] > 1:
            self._fd_open_count[fd] -= 1
            return

        if fd in self._fd_open_count:
            del self._fd_open_count[fd]
        inode = self._fd_inode_map[fd]
        del self._inode_fd_map[inode]
        del self._fd_inode_map[fd]

        fh = self._fd_fh_map[fd]
        del self._fd_fh_map[fd]
        del self._fh_fd_map[fh]

    def releasedir(self, fh):
        pass


def start_fuse(socket_address: str, mountpoint: str, debug: bool = False, nothreads: bool = False):
    operations = Operations(socket_address)
    start_shutdown_watcher(operations, socket_address, mountpoint)

    fuse_options = set(llfuse.default_options)
    fuse_options.discard("default_permissions")
    if debug:
        fuse_options.add("debug")
    llfuse.init(operations, mountpoint, fuse_options)
    try:
        llfuse.main(workers=1)
    except:
        llfuse.close(unmount=False)
        raise

    llfuse.close()


# TODO: shutdown watcher should be able to send here a command to the
# core to signify it has been successfully closed
