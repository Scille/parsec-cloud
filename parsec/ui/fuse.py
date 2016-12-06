from errno import EROFS, ENOENT, EBADFD, ENOTEMPTY
import os
from threading import Lock
from stat import S_IRWXU, S_IRWXG, S_IRWXO, S_IFDIR, S_IFREG
from logbook import debug, warning, StreamHandler
from fuse import FUSE, FuseOSError, Operations, LoggingMixIn

from ..abstract import BaseServer
from ..vfs import BaseVFSClient, VFSFileNotFoundError
from ..vfs.vfs_pb2 import Stat


class LockProxy:
    """
    Lock when calling the given object's methods to prevent concurrency issues.

    FUSE's C library make use of multiple threads, however given zmq req/rep
    calls must be synchronized we have to make use of locks.
    """
    def __init__(self, obj):
        self.lock = Lock()
        self.base_obj = obj

    def __getattr__(self, name):
        fn = getattr(self.base_obj, name)

        def wrapper(*args, **kwargs):
            try:
                self.lock.acquire()
                ret = fn(*args, **kwargs)
            finally:
                self.lock.release()
            return ret

        return wrapper


class VFSFile:

    def __init__(self, vfs, path, flags=0):
        self.path = path
        self._vfs = vfs
        self._need_flush = False
        self.flags = flags
        self._content = None

    def get_content(self, force=False):
        if not self._content or force:
            self._content = self._vfs.read_file(self.path).content
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
            self._vfs.write_file(self.path, self._content)
            self._need_flush = False


class FuseOperations(LoggingMixIn, Operations):

    def __init__(self, vfs):
        self._vfs = LockProxy(vfs)
        # self._vfs = vfs
        self.fds = {}
        self.next_fd_id = 0

    def _get_fd(self, fh):
        try:
            return self.fds[fh]
        except KeyError:
            raise FuseOSError(EBADFD)

    def getattr(self, path, fh=None):
        try:
            response = self._vfs.stat(path)
        except VFSFileNotFoundError:
            raise FuseOSError(ENOENT)
        stats = {
            'st_size': response.stat.size,
            'st_ctime': response.stat.ctime,
            'st_mtime': response.stat.mtime,
            'st_atime': response.stat.atime,
        }
        # Set it to 777 access
        stats['st_mode'] = 0
        if response.stat.type == Stat.DIRECTORY:
            stats['st_mode'] |= S_IFDIR
        else:
            stats['st_mode'] |= S_IFREG
        stats['st_mode'] |= S_IRWXU | S_IRWXG | S_IRWXO
        stats['st_nlink'] = 1
        stats['st_uid'] = os.getuid()
        stats['st_gid'] = os.getgid()
        return stats

    def readdir(self, path, fh):
        return ['.', '..'] + list(self._vfs.list_dir(path).list_dir)

    def create(self, path, mode):
        self._vfs.create_file(path)
        return self.open(path)

    def open(self, path, flags=0):
        fd_id = self.next_fd_id
        self.fds[fd_id] = VFSFile(self._vfs, path, flags)
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
        self._vfs.delete_file(path)

    def mkdir(self, path, mode):
        self._vfs.make_dir(path)
        return 0

    def rmdir(self, path):
        self._vfs.remove_dir(path)
        return 0

    def flush(self, path, fh):
        fd = self._get_fd(fh)
        fd.flush()
        return 0

    def fsync(self, path, datasync, fh):
        return 0  # TODO

    def fsyncdir(self, path, datasync, fh):
        return 0  # TODO


class FuseUIServer(BaseServer):
    def __init__(self, mountpoint: str, vfs: BaseVFSClient):
        self.mountpoint = mountpoint
        self.vfs = vfs

    def start(self):
        FUSE(FuseOperations(self.vfs), self.mountpoint, foreground=True)

    def stop(self):
        raise NotImplementedError()
