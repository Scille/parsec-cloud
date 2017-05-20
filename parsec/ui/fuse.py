import os
import stat
import sys
import socket
import json
import click
import threading
from base64 import decodebytes, encodebytes
from errno import ENOENT, EBADFD
from stat import S_IRWXU, S_IRWXG, S_IRWXO, S_IFDIR, S_IFREG
from fuse import FUSE, FuseOSError, Operations, LoggingMixIn

from parsec.tools import logger


CORE_UNIX_SOCKET = '/tmp/parsec'


@click.command()
@click.argument('mountpoint', type=click.Path(exists=True, file_okay=False))
@click.option('--identity', '-i', type=click.STRING, default=None)
@click.option('--debug', '-d', is_flag=True, default=False)
@click.option('--nothreads', is_flag=True, default=False)
@click.option('--socket', '-s', default=CORE_UNIX_SOCKET,
              help='Path to the UNIX socket (default: %s).' % CORE_UNIX_SOCKET)
def cli(mountpoint, identity, debug, nothreads, socket):
    # Do the import here in case fuse is not an available dependency
    start_fuse(socket, mountpoint, identity, debug=debug, nothreads=nothreads)


class File:

    def __init__(self, operations, id, version, flags=0):
        self.id = id
        self.version = version
        self._operations = operations
        self.flags = flags

    def read(self, size=None, offset=0):
        # TODO use flags
        response = self._operations.send_cmd(cmd='file_read',
                                             id=self.id,
                                             size=size,
                                             offset=offset)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        self.version = response['version']
        return decodebytes(response['content'].encode())

    def write(self, data, offset=0):
        # TODO use flags
        self.version += 1
        response = self._operations.send_cmd(
            cmd='file_write',
            id=self.id,
            version=self.version,
            content=encodebytes(data).decode(),
            offset=offset)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)

    def truncate(self, length):
        self.version += 1
        response = self._operations.send_cmd(cmd='file_truncate',
                                             id=self.id,
                                             version=self.version,
                                             length=length)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)

    def flush(self):
        pass


class FuseOperations(LoggingMixIn, Operations):

    def __init__(self, socket_path):
        self.fds = {}
        self.next_fd_id = 0
        self._socket_path = socket_path
        self._socket_lock = threading.Lock()
        self._socket = None

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
            req = json.dumps(msg).encode() + b'\n'
            logger.debug('Send: %r' % req)
            self.sock.send(req)
            raw_reps = self.sock.recv(4096)
            while raw_reps[-1] != ord(b'\n'):
                raw_reps += self.sock.recv(4096)
            logger.debug('Received: %r' % raw_reps)
            return json.loads(raw_reps[:-1].decode())

    def _get_fd(self, fh):
        try:
            return self.fds[fh]
        except KeyError:
            raise FuseOSError(EBADFD)

    def _get_file_id(self, path):
        response = self.send_cmd(cmd='user_manifest_list_dir', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return response['current']['id']

    def getattr(self, path, fh=None):
        response = self.send_cmd(cmd='user_manifest_list_dir', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        id = response['current']['id']
        if id:
            response = self.send_cmd(cmd='file_stat', id=id)
            if response['status'] != 'ok':
                raise FuseOSError(ENOENT)
            stat = response
            stat['is_dir'] = False  # TODO remove this ?
        else:
            # TODO remove this?
            stat = {'is_dir': True, 'size': 0, 'ctime': 0, 'mtime': 0, 'atime': 0}
        fuse_stat = {
            'st_size': stat['size'],
            'st_ctime': stat['ctime'],  # TOTO change to local timezone
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
        response = self.send_cmd(cmd='user_manifest_list_dir', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return ['.', '..'] + list(response['children'].keys())

    def create(self, path, mode):
        response = self.send_cmd(cmd='user_manifest_create_file', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return self.open(path)

    def open(self, path, flags=0):
        fd_id = self.next_fd_id
        id = self._get_file_id(path)
        response = self.send_cmd(cmd='user_manifest_list_dir', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        id = response['current']['id']
        response = self.send_cmd(cmd='file_read', id=id)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        version = response['version']
        file = File(self, id, version, flags)
        self.fds[fd_id] = file
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
            fd.truncate(length)
            for file in self.fds.values():
                if file.id == fd.id and file.version < fd.version:
                    file.version += 1
        finally:
            if release_fh:
                self.release(path, fh)

    def unlink(self, path):
        response = self.send_cmd(cmd='user_manifest_delete_file', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)

    def mkdir(self, path, mode):
        response = self.send_cmd(cmd='user_manifest_make_dir', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return 0

    def rmdir(self, path):
        response = self.send_cmd(cmd='user_manifest_remove_dir', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return 0

    def rename(self, old_path, new_path):
        response = self.send_cmd(cmd='user_manifest_rename_file',
                                 old_path=old_path,
                                 new_path=new_path)
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


def start_fuse(socket_path: str,
               mountpoint: str,
               identity: str,
               debug: bool=False,
               nothreads: bool=False):
    operations = FuseOperations(socket_path)
    response = operations.send_cmd(cmd='identity_load',
                                   identity=identity)
    if response['status'] != 'ok':
        raise FuseOSError(ENOENT)  # TODO change error message
    # TODO call this automatically
    response = operations.send_cmd(cmd='user_manifest_load')
    if response['status'] != 'ok':
        raise FuseOSError(ENOENT)  # TODO change error message
    FUSE(operations, mountpoint, foreground=True, nothreads=nothreads, debug=debug)
