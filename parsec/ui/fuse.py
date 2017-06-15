from datetime import datetime
from dateutil import parser, tz
import os
import stat
import sys
import socket
import json
import click
import threading
from itertools import count
from errno import ENOENT
from stat import S_IRWXU, S_IRWXG, S_IRWXO, S_IFDIR, S_IFREG
from fuse import FUSE, FuseOSError, Operations, LoggingMixIn

from parsec.tools import logger, from_jsonb64, to_jsonb64


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


def convert_utc_time_to_local_timestamp(utc_time):
    from_zone = tz.tzutc()
    to_zone = tz.tzlocal()
    utc = parser.parse(utc_time)
    utc = utc.replace(tzinfo=from_zone)
    return utc.astimezone(to_zone).timestamp()


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
            req = json.dumps(msg).encode() + b'\n'
            logger.debug('Send: %r' % req)
            self.sock.send(req)
            raw_reps = self.sock.recv(4096)
            while raw_reps[-1] != ord(b'\n'):
                raw_reps += self.sock.recv(4096)
            logger.debug('Received: %r' % raw_reps)
            return json.loads(raw_reps[:-1].decode())

    def getattr(self, path, fh=None):
        response = self.send_cmd(cmd='stat', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        if response['type'] == 'file':
            stat = response
            stat['is_dir'] = False  # TODO remove this ?
        else:
            # TODO remove this?
            local_time = datetime.now().isoformat()
            stat = {'is_dir': True,
                    'size': 0,
                    'created': local_time,
                    'updated': local_time}
        fuse_stat = {
            'st_size': stat['size'],
            'st_ctime': convert_utc_time_to_local_timestamp(stat['created']),  # TODO change to local timezone
            'st_mtime': convert_utc_time_to_local_timestamp(stat['updated']),
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
        response = self.send_cmd(cmd='stat', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return ['.', '..'] + response['children']

    def create(self, path, mode):
        response = self.send_cmd(cmd='file_create', path=path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return 0

    def read(self, path, size, offset, fh):
        response = self.send_cmd(cmd='file_read', path=path, size=size, offset=offset)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return from_jsonb64(response['content'])

    def write(self, path, data, offset, fh):
        length = len(data)
        data = to_jsonb64(data)
        response = self.send_cmd(cmd='file_write', path=path, content=data, offset=offset)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return length

    def truncate(self, path, length, fh=None):
        if not fh:
            fh = self.open(path, flags=0)
            release_fh = True
        try:
            response = self.send_cmd(cmd='file_truncate', path=path, length=length)
            if response['status'] != 'ok':
                raise FuseOSError(ENOENT)
        finally:
            if release_fh:
                self.release(path, fh)

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

    def rename(self, old_path, new_path):
        response = self.send_cmd(cmd='move', old_path=old_path, new_path=new_path)
        if response['status'] != 'ok':
            raise FuseOSError(ENOENT)
        return 0


def start_fuse(socket_path: str, mountpoint: str, debug: bool=False, nothreads: bool=False):
    operations = FuseOperations(socket_path)
    FUSE(operations, mountpoint, foreground=True, nothreads=nothreads, debug=debug)
