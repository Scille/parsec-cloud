import os
import sys
import socket
import json
import threading
from base64 import decodebytes, encodebytes
from errno import ENOENT, EBADFD
from stat import S_IRWXU, S_IRWXG, S_IRWXO, S_IFDIR, S_IFREG
from fuse import FUSE, FuseOSError, Operations, LoggingMixIn
from logbook import Logger, StreamHandler


LOG_FORMAT = '[{record.time:%Y-%m-%d %H:%M:%S.%f%z}] ({record.thread_name}) {record.level_name}: {record.channel}: {record.message}'
log = Logger('Parsec-FUSE')


class VFSFile:

    def __init__(self, operations, path, flags=0):
        self.path = path
        self._operations = operations
        self._need_flush = False
        self.flags = flags
        self._content = None

    def get_content(self, force=False):
        if not self._content or force:
            response = self._operations.send_cmd(cmd='vfs:read_file', path=self.path)
            if response['status'] != 'ok':
                raise FuseOSError(ENOENT)
            self._content = decodebytes(response['content'].encode())
        return self._content

    def read(self, size=None, offset=0):
        # TODO use flags
        content = self.get_content()
        if size is not None:
            return content[offset:offset + size]
        else:
            return content

    def write(self, data, offset=0):
        # TODO use flags
        if offset == 0:
            content = b''
        else:
            content = self.get_content()
        self._content = content[:offset] + data
        self._need_flush = True

    def flush(self):
        if self._need_flush:
            response = self._operations.send_cmd(
                cmd='vfs:write_file', path=self.path,
                content=encodebytes(self._content).decode())
            if response['status'] != 'ok':
                raise FuseOSError(ENOENT)
            self._need_flush = False


class FuseOperations(LoggingMixIn, Operations):

    def __init__(self, socket_path):
        self.perthread = threading.local()
        self.fds = {}
        self.next_fd_id = 0
        self._socket_path = socket_path

    @property
    def sock(self):
        if not hasattr(self.perthread, 'sock'):
            self._init_socket()
        return self.perthread.sock

    def _init_socket(self):
        sock = socket.socket(socket.AF_UNIX, type=socket.SOCK_STREAM)
        sock.connect(self._socket_path)
        log.debug('Init socket')
        self.perthread.sock = sock

    def send_cmd(self, **msg):
        req = json.dumps(msg).encode() + b'\n'
        log.debug('Send: %r' % req)
        self.sock.send(req)
        raw_reps = self.sock.recv(4096)
        while raw_reps[-1] != ord(b'\n'):
            raw_reps += self.sock.recv(4096)
        log.debug('Received: %r' % raw_reps)
        return json.loads(raw_reps.decode())

    def _get_fd(self, fh):
        try:
            return self.fds[fh]
        except KeyError:
            raise FuseOSError(EBADFD)

    def getattr(self, path, fh=None):
        response = self.send_cmd(cmd='vfs:stat', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        stat = response['stat']
        fuse_stat = {
            'st_size': stat['size'],
            'st_ctime': stat['ctime'],
            'st_mtime': stat['mtime'],
            'st_atime': stat['atime'],
        }
        # Set it to 777 access
        fuse_stat['st_mode'] = 0
        if stat['is_dir']:
            fuse_stat['st_mode'] |= S_IFDIR
        else:
            fuse_stat['st_mode'] |= S_IFREG
        fuse_stat['st_mode'] |= S_IRWXU | S_IRWXG | S_IRWXO
        fuse_stat['st_nlink'] = 1
        fuse_stat['st_uid'] = os.getuid()
        fuse_stat['st_gid'] = os.getgid()
        return fuse_stat

    def readdir(self, path, fh):
        response = self.send_cmd(cmd='vfs:list_dir', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return ['.', '..'] + response['list']

    def create(self, path, mode):
        response = self.send_cmd(cmd='vfs:create_file', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return self.open(path)

    def open(self, path, flags=0):
        fd_id = self.next_fd_id
        self.fds[fd_id] = VFSFile(self, path, flags)
        self.next_fd_id += 1
        return fd_id

    def release(self, path, fh):
        try:
            del self.fds[fh]
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
        release_fh = False
        if not fh:
            fh = self.open(path, flags=0)
            release_fh = True
        try:
            fd = self._get_fd(fh)
            return fd.write(fd.read(length))
        finally:
            if release_fh:
                self.release(path, fh)

    def unlink(self, path):
        response = self.send_cmd(cmd='vfs:delete_file', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)

    def mkdir(self, path, mode):
        response = self.send_cmd(cmd='vfs:make_dir', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return 0

    def rmdir(self, path):
        response = self.send_cmd(cmd='vfs:remove_dir', path=path)
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
    StreamHandler(sys.stdout, format_string=LOG_FORMAT).push_application()
    operations = FuseOperations(socket_path)
    FUSE(operations, mountpoint, foreground=True, nothreads=nothreads, debug=debug)
