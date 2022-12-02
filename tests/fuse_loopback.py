# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import os.path
import time
from concurrent.futures import ThreadPoolExecutor
from errno import EACCES
from pathlib import Path

import pytest
from fuse import FUSE, FuseOSError, LoggingMixIn, Operations, fuse_exit


class Loopback(LoggingMixIn, Operations):
    def __init__(self, root):
        self.root = os.path.realpath(root)

    def __call__(self, op, path, *args):
        return super(Loopback, self).__call__(op, self.root + path, *args)

    def access(self, path, mode):
        if not os.access(path, mode):
            raise FuseOSError(EACCES)

    chmod = os.chmod
    chown = os.chown

    def create(self, path, mode):
        return os.open(path, os.O_RDWR | os.O_CREAT | os.O_TRUNC, mode)

    def flush(self, path, fh):
        return os.fsync(fh)

    def fsync(self, path, datasync, fh):
        if datasync != 0:
            return os.fdatasync(fh)
        else:
            return os.fsync(fh)

    def getattr(self, path, fh=None):
        st = os.lstat(path)
        return dict(
            (key, getattr(st, key))
            for key in (
                "st_atime",
                "st_ctime",
                "st_gid",
                "st_mode",
                "st_mtime",
                "st_nlink",
                "st_size",
                "st_uid",
            )
        )

    getxattr = None

    def link(self, target, source):
        return os.link(self.root + source, target)

    listxattr = None
    mkdir = os.mkdir
    mknod = os.mknod
    open = os.open

    def read(self, path, size, offset, fh):
        return os.pread(fh, size, offset)

    def readdir(self, path, fh):
        return [".", ".."] + os.listdir(path)

    readlink = os.readlink

    def release(self, path, fh):
        return os.close(fh)

    def rename(self, old, new):
        return os.rename(old, self.root + new)

    rmdir = os.rmdir

    def statfs(self, path):
        stv = os.statvfs(path)
        return dict(
            (key, getattr(stv, key))
            for key in (
                "f_bavail",
                "f_bfree",
                "f_blocks",
                "f_bsize",
                "f_favail",
                "f_ffree",
                "f_files",
                "f_flag",
                "f_frsize",
                "f_namemax",
            )
        )

    def symlink(self, target, source):
        return os.symlink(source, target)

    def truncate(self, path, length, fh=None):
        with open(path, "r+") as f:
            f.truncate(length)

    unlink = os.unlink
    utimens = os.utime

    def write(self, path, data, offset, fh):
        return os.pwrite(fh, data, offset)


class ControlledLoopback(Loopback):
    def __init__(self, source, mountpoint):
        self.path = mountpoint
        self.full = False
        self.number_of_write_before_full = None
        self.use_ns = True
        self._exiting = False
        super().__init__(source)

    def getattr(self, path, fh=None):
        if self._exiting:
            fuse_exit()
        return super().getattr(path, fh)

    def schedule_exit(self):
        self._exiting = True

    def write(self, path, data, offset, fh):
        if self.number_of_write_before_full is not None:
            if self.number_of_write_before_full == 0:
                self.full = True
                self.number_of_write_before_full = None
            elif self.number_of_write_before_full > 0:
                self.number_of_write_before_full -= 1
        if self.full:
            return 0
        return super().write(path, data, offset, fh)

    def ioctl(self, path, cmd, arg, fd, *args):
        return super().ioctl(path, cmd, arg, fd, *args)

    def __call__(self, op, *args):
        return super().__call__(op, *args)


@pytest.fixture
def loopback_fs(tmp_path):
    source_path = Path(tmp_path) / "source"
    mountpoint_path = Path(tmp_path) / "mountpoint"
    source_path.mkdir()
    mountpoint_path.mkdir()
    st_dev = mountpoint_path.stat().st_dev
    with ThreadPoolExecutor(max_workers=1) as executor:
        loopback = ControlledLoopback(source_path, mountpoint_path)
        future = executor.submit(
            lambda: FUSE(loopback, str(loopback.path), foreground=True, auto_unmount=True)
        )
        while mountpoint_path.stat().st_dev == st_dev:
            time.sleep(0.001)
        yield loopback
        loopback.schedule_exit()
        try:
            (mountpoint_path / "__exit__").exists()
        except OSError:
            pass
        future.result()
        while mountpoint_path.stat().st_dev != st_dev:
            time.sleep(0.001)
