import logging

from errno import EROFS, ENOENT, EBADFD
import os
import sys
from stat import S_IRWXU, S_IRWXG, S_IRWXO
from sys import argv, exit
import base64
from logbook import debug, warning, StreamHandler
from fuse import FUSE, FuseOSError, Operations, LoggingMixIn
import zmq


StreamHandler(sys.stdout).push_application()


def _content_wrap(content):
    return base64.encodebytes(content).decode()


def _content_unwrap(wrapped_content):
    return base64.decodebytes(wrapped_content.encode())


class DriveInterface:

    def __init__(self, addr='tcp://127.0.0.1:5000'):
        self._context = zmq.Context()
        self._socket = self._context.socket(zmq.REQ)
        self._socket.connect(addr)

    def __ll_com(self, cmd, params):
        self._socket.send_json({'cmd': cmd, 'params': params})
        ret = self._socket.recv_json()
        if not ret['ok']:
            reason = ret.get('reason')
            if reason == 'File not found':
                debug('%s: File not found (%s)' % (cmd, params))
                raise FuseOSError(ENOENT)
            else:
                msg = '`%s` has failed (reason: %s)' % (cmd, reason)
                warning(msg)
                raise FuseOSError(EROFS)
        return ret.get('data', None)

    def read_file(self, path):
        return _content_unwrap(self.__ll_com('READ_FILE', {'path': path})['content'])

    def create_file(self, path, content=b''):
        assert isinstance(content, bytes)
        return self.__ll_com('CREATE_FILE', {'path': path, 'content': _content_wrap(content)})

    def write_file(self, path, content):
        assert isinstance(content, bytes)
        return self.__ll_com('WRITE_FILE', {'path': path, 'content': _content_wrap(content)})

    def delete_file(self, path):
        return self.__ll_com('DELETE_FILE', {'path': path})

    def stat(self, path):
        return self.__ll_com('STAT', {'path': path})

    def list_dir(self, path):
        return self.__ll_com('LIST_DIR', {'path': path})

    def make_dir(self, path):
        return self.__ll_com('MAKE_DIR', {'path': path})


class DriveFile:

    def __init__(self, drive, path, flags=0):
        self.path = path
        self._drive = drive
        self._need_flush = False
        self.flags = flags
        self._content = None

    def get_content(self, force=False):
        if not self._content or force:
            self._content = self._drive.read_file(self.path)
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
            self._drive.write_file(self.path, self._content)
            self._need_flush = False


class FuseOperations(LoggingMixIn, Operations):

    def __init__(self):
        self._drive = DriveInterface()
        self.fds = {}
        self.next_fd_id = 0

    def _get_fd(self, fh):
        try:
            return self.fds[fh]
        except KeyError:
            raise FuseOSError(EBADFD)

    def getattr(self, path, fh=None):
        stats = self._drive.stat(path)
        # Set it to 777 access
        stats['st_mode'] |= S_IRWXU | S_IRWXG | S_IRWXO
        stats['st_atime'] = stats['st_mtime']
        stats['st_nlink'] = 1
        stats['st_uid'] = os.getuid()
        stats['st_gid'] = os.getgid()
        return stats

    def readdir(self, path, fh):
        return ['.', '..'] + self._drive.list_dir(path)['_items']

    def create(self, path, mode):
        self._drive.create_file(path)
        return self.open(path)

    def open(self, path, flags):
        fd_id = self.next_fd_id
        self.fds[fd_id] = DriveFile(self._drive, path, flags)
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
        self._drive.delete_file(path)

    def mkdir(self, path, mode):
        self._drive.make_dir(path)
        return 0

    def flush(self, path, fh):
        fd = self._get_fd(fh)
        fd.flush()
        return 0

    def fsync(self, path, datasync, fh):
        return 0  # TODO

    def fsyncdir(self, path, datasync, fh):
        return 0  # TODO


if __name__ == '__main__':
    if len(argv) != 2:
        print('usage: %s <mountpoint>' % argv[0])
        exit(1)

    logging.basicConfig(level=logging.DEBUG)
    fuse = FUSE(FuseOperations(), argv[1], foreground=True)
