# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


class ThreadFSAccess:
    def __init__(self, portal, fs):
        self.fs = fs
        self._portal = portal

    def stat(self, path):
        return self._portal.run(self.fs.stat, path)

    def delete(self, path):
        return self._portal.run(self.fs.delete, path)

    def move(self, src, dst):
        return self._portal.run(self.fs.move, src, dst)

    def file_create(self, path):
        async def _do(path):
            await self.fs.file_create(path)
            return await self.fs.file_fd_open(path)

        return self._portal.run(_do, path)

    def folder_create(self, path):
        return self._portal.run(self.fs.folder_create, path)

    def file_truncate(self, path, length):
        return self._portal.run(self.fs.file_truncate, path, length)

    def file_fd_open(self, path):
        return self._portal.run(self.fs.file_fd_open, path)

    def file_fd_close(self, fh):
        return self._portal.run(self.fs.file_fd_close, fh)

    def file_fd_seek(self, fh, offset):
        return self._portal.run(self.fs.file_fd_seek, fh, offset)

    def file_fd_read(self, fh, size, offset):
        return self._portal.run(self.fs.file_fd_read, fh, size, offset)

    def file_fd_write(self, fh, data, offset):
        return self._portal.run(self.fs.file_fd_write, fh, data, offset)

    def file_fd_truncate(self, fh, length):
        return self._portal.run(self.fs.file_fd_truncate, fh, length)

    def file_fd_flush(self, fh):
        return self._portal.run(self.fs.file_fd_flush, fh)
