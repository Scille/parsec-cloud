# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from contextlib import contextmanager
from errno import ENETDOWN, EBADF, ENOTDIR
from stat import S_IRWXU, S_IRWXG, S_IRWXO, S_IFDIR, S_IFREG
from fuse import FuseOSError, Operations, LoggingMixIn, fuse_get_context, fuse_exit

from parsec.core.fs import FSInvalidFileDescriptor
from parsec.core.backend_connection import BackendNotAvailable


@contextmanager
def translate_error():
    try:
        yield

    except BackendNotAvailable as exc:
        raise FuseOSError(ENETDOWN) from exc

    except FSInvalidFileDescriptor as exc:
        raise FuseOSError(EBADF) from exc

    except OSError as exc:
        raise FuseOSError(exc.errno) from exc


class FuseOperations(LoggingMixIn, Operations):
    def __init__(self, workspace, fs_access):
        super().__init__()
        self.workspace = workspace
        self.fs_access = fs_access
        self.fds = {}
        self._need_exit = False

    def _localize_path(self, path):
        return f"/{self.workspace}/{path}"

    def schedule_exit(self):
        # TODO: Currently call fuse_exit from a non fuse thread is not possible
        # (see https://github.com/fusepy/fusepy/issues/116).
        self._need_exit = True

    def init(self, path):
        pass

    def getattr(self, path, fh=None):
        path = self._localize_path(path)

        if self._need_exit:
            fuse_exit()

        with translate_error():
            stat = self.fs_access.stat(path)

        fuse_stat = {}
        # Set it to 777 access
        fuse_stat["st_mode"] = 0
        if stat["is_folder"]:
            fuse_stat["st_mode"] |= S_IFDIR
            fuse_stat["st_size"] = 4096  # Because why not ?
            fuse_stat["st_nlink"] = 2
        else:
            fuse_stat["st_mode"] |= S_IFREG
            fuse_stat["st_size"] = stat["size"]
            fuse_stat["st_nlink"] = 1
        fuse_stat["st_blocks"] = fuse_stat["st_size"] // 512
        if fuse_stat["st_size"] % 512:
            fuse_stat["st_blocks"] += 1
        fuse_stat["st_mode"] |= S_IRWXU | S_IRWXG | S_IRWXO
        fuse_stat["st_ctime"] = stat["created"].timestamp()  # TODO change to local timezone
        fuse_stat["st_mtime"] = stat["updated"].timestamp()
        fuse_stat["st_atime"] = stat["updated"].timestamp()  # TODO not supported ?
        uid, gid, _ = fuse_get_context()
        fuse_stat["st_uid"] = uid
        fuse_stat["st_gid"] = gid
        return fuse_stat

    def readdir(self, path, fh):
        path = self._localize_path(path)

        with translate_error():
            stat = self.fs_access.stat(path)

        if not stat["is_folder"]:
            raise FuseOSError(ENOTDIR)

        return [".", ".."] + list(stat["children"])

    def create(self, path, mode):
        path = self._localize_path(path)

        with translate_error():
            return self.fs_access.file_create(path)

    def open(self, path, flags=0):
        path = self._localize_path(path)

        with translate_error():
            return self.fs_access.file_fd_open(path)

    def release(self, path, fh):
        path = self._localize_path(path)

        with translate_error():
            self.fs_access.file_fd_close(fh)

    def read(self, path, size, offset, fh):
        path = self._localize_path(path)

        with translate_error():
            ret = self.fs_access.file_fd_read(fh, size, offset)
            # Fuse wants bytes but file_fd_read returns a bytearray
            return bytes(ret)

    def write(self, path, data, offset, fh):
        path = self._localize_path(path)

        with translate_error():
            return self.fs_access.file_fd_write(fh, data, offset)

    def truncate(self, path, length, fh=None):
        path = self._localize_path(path)

        with translate_error():
            if fh:
                self.fs_access.file_fd_truncate(fh, length)
            else:
                self.fs_access.file_truncate(path, length)

    def unlink(self, path):
        path = self._localize_path(path)

        with translate_error():
            self.fs_access.file_delete(path)

    def mkdir(self, path, mode):
        path = self._localize_path(path)

        with translate_error():
            self.fs_access.folder_create(path)

        return 0

    def rmdir(self, path):
        path = self._localize_path(path)

        with translate_error():
            self.fs_access.folder_delete(path)

        return 0

    def rename(self, src, dst):
        src = self._localize_path(src)
        dst = self._localize_path(dst)

        # Unix allows to overwrite the destination, so make sure to have
        # space before calling the move
        with translate_error():
            self.fs_access.move(src, dst, overwrite=True)

        return 0

    def flush(self, path, fh):
        path = self._localize_path(path)

        with translate_error():
            self.fs_access.file_fd_flush(fh)

        return 0

    def fsync(self, path, datasync, fh):
        path = self._localize_path(path)

        with translate_error():
            self.fs_access.file_fd_flush(fh)

        return 0

    def fsyncdir(self, path, datasync, fh):
        path = self._localize_path(path)

        return 0  # TODO
