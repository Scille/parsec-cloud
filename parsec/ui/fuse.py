import os
import stat
import sys
import socket
import click
import threading
from dateutil.parser import parse as dateparse
from itertools import count
from logbook import WARNING
from errno import ENOENT, EBADFD
from stat import S_IRWXU, S_IRWXG, S_IRWXO, S_IFDIR, S_IFREG
from fuse import FUSE, FuseOSError, Operations, LoggingMixIn

from parsec.core.file import ContentBuilder
from parsec.tools import logger, logger_stream, from_jsonb64, to_jsonb64, ejson_dumps, ejson_loads


DEFAULT_CORE_UNIX_SOCKET = '/tmp/parsec'


@click.command()
@click.argument('mountpoint', type=click.Path(exists=True, file_okay=False))
@click.option('--debug', '-d', is_flag=True, default=False)
@click.option('--nothreads', is_flag=True, default=False)
@click.option('--socket', '-s', default=DEFAULT_CORE_UNIX_SOCKET,
              help='Path to the UNIX socket (default: %s).' % DEFAULT_CORE_UNIX_SOCKET)
def cli(mountpoint, debug, nothreads, socket):
    # Do the import here in case fuse is not an available dependency
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
            cmd='file_read', path=self.path, size=size, offset=offset)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return from_jsonb64(response['content'])

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
                cmd='file_truncate', path=self.path, length=shortest_truncate)
            if response['status'] != 'ok':
                raise FuseOSError(ENOENT)
        # Write new contents
        for offset, content in builder.contents.items():
            # TODO use flags
            response = self._operations.send_cmd(
                cmd='file_write',
                path=self.path,
                content=to_jsonb64(content),
                offset=offset)
            if response['status'] != 'ok':
                raise FuseOSError(ENOENT)


class FuseOperations(LoggingMixIn, Operations):

    def __init__(self, socket_path):
        self.fds = {}
        self._fs_id_generator = count(1)
        self._socket_path = socket_path
        self._socket_lock = threading.Lock()
        self._socket = None

    def get_fd_id(self):
        return next(self._fs_id_generator)

    @property
    def sock(self):
        if not self._socket:
            self._init_socket()
        return self._socket

    def _init_socket(self):
        sock = socket.socket(socket.AF_UNIX, type=socket.SOCK_STREAM)
        if (not os.path.exists(self._socket_path) or
                not stat.S_ISSOCK(os.stat(self._socket_path).st_mode)):
            logger.error("File %s doesn't exist or isn't a socket. Is Parsec Core running?" %
                         self._socket_path)
            sys.exit(1)
        sock.connect(self._socket_path)
        logger.debug('Init socket')
        self._socket = sock

    def send_cmd(self, **msg):
        with self._socket_lock:
            req = ejson_dumps(msg).encode() + b'\n'
            logger.debug('Send: %r' % req)
            self.sock.send(req)
            raw_reps = self.sock.recv(4096)
            while raw_reps[-1] != ord(b'\n'):
                raw_reps += self.sock.recv(4096)
            logger.debug('Received: %r' % raw_reps)
            return ejson_loads(raw_reps[:-1].decode())

    def _get_fd(self, fh):
        try:
            return self.fds[fh]
        except KeyError:
            raise FuseOSError(EBADFD)

    def _get_file_id(self, path):
        response = self.send_cmd(cmd='list_dir', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return response['current']['id']

    def getattr(self, path, fh=None):
        stat = self.send_cmd(cmd='stat', path=path)
        if stat['status'] != 'ok':
            raise FuseOSError(ENOENT)
        fuse_stat = {}
        # Set it to 777 access
        fuse_stat['st_mode'] = 0
        if stat['type'] == 'folder':
            fuse_stat['st_mode'] |= S_IFDIR
        else:
            fuse_stat['st_mode'] |= S_IFREG
            fuse_stat['st_size'] = stat.get('size', 0)
            fuse_stat['st_ctime'] = dateparse(stat['created']).timestamp()  # TODO change to local timezone
            fuse_stat['st_mtime'] = dateparse(stat['updated']).timestamp()
            fuse_stat['st_atime'] = dateparse(stat['updated']).timestamp()  # TODO not supported ?
        fuse_stat['st_mode'] |= S_IRWXU | S_IRWXG | S_IRWXO
        fuse_stat['st_nlink'] = 1
        fuse_stat['st_uid'] = os.getuid()
        fuse_stat['st_gid'] = os.getgid()
        return fuse_stat

    def readdir(self, path, fh):
        resp = self.send_cmd(cmd='stat', path=path)
        # TODO: make sure error code for path is not a folder is ENOENT
        if resp['status'] != 'ok' or resp['type'] != 'folder':
            raise FuseOSError(ENOENT)
        return ['.', '..'] + list(resp['children'])

    def create(self, path, mode):
        response = self.send_cmd(cmd='file_create', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return self.open(path)

    def open(self, path, flags=0):
        fd_id = self.get_fd_id()
        resp = self.send_cmd(cmd='stat', path=path)
        if resp['status'] != 'ok' or resp['type'] != 'file':
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
        response = self.send_cmd(cmd='delete', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)

    def mkdir(self, path, mode):
        response = self.send_cmd(cmd='folder_create', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return 0

    def rmdir(self, path):
        # TODO: check directory is empty
        # TODO: check path is a directory
        response = self.send_cmd(cmd='delete', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return 0

    def rename(self, src, dst):
        # Unix allows to overwrite the destination, so make sure to have
        # space before calling the move
        self.send_cmd(cmd='delete', path=dst)
        response = self.send_cmd(cmd='move', src=src, dst=dst)
        if response['status'] != 'ok':
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


def start_fuse(socket_path: str, mountpoint: str, debug: bool=False, nothreads: bool=False):
    operations = FuseOperations(socket_path)
    if not debug:
        logger_stream.level = WARNING
    mountpoint = os.path.join(os.getcwd(), mountpoint)
    operations.send_cmd(cmd='register_mountpoint', path=mountpoint)
    FUSE(operations, mountpoint, foreground=True, nothreads=nothreads, debug=debug)
    operations.send_cmd(cmd='unregister_mountpoint', path=mountpoint)
