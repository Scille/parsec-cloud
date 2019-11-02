# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


class ThreadFSAccess:
    def __init__(self, portal, workspace_fs):
        self.workspace_fs = workspace_fs
        self._portal = portal

    # Rights check

    def check_read_rights(self, path):
        return self._portal.run_sync(self.workspace_fs.transactions.check_read_rights, path)

    def check_write_rights(self, path):
        return self._portal.run_sync(self.workspace_fs.transactions.check_write_rights, path)

    # Entry transactions

    def entry_info(self, path):
        return self._portal.run(self.workspace_fs.transactions.entry_info, path)

    def entry_rename(self, source, destination, *, overwrite):
        return self._portal.run(
            self.workspace_fs.transactions.entry_rename, source, destination, overwrite
        )

    # Folder transactions

    def folder_create(self, path):
        return self._portal.run(self.workspace_fs.transactions.folder_create, path)

    def folder_delete(self, path):
        return self._portal.run(self.workspace_fs.transactions.folder_delete, path)

    # File transactions

    def file_create(self, path, *, open):
        return self._portal.run(self.workspace_fs.transactions.file_create, path, open)

    def file_open(self, path, *, mode):
        return self._portal.run(self.workspace_fs.transactions.file_open, path, mode)

    def file_delete(self, path):
        return self._portal.run(self.workspace_fs.transactions.file_delete, path)

    def file_resize(self, path, length):
        return self._portal.run(self.workspace_fs.transactions.file_resize, path, length)

    # File descriptor transactions

    def fd_close(self, fh):
        return self._portal.run(self.workspace_fs.transactions.fd_close, fh)

    def fd_seek(self, fh, offset):
        return self._portal.run(self.workspace_fs.transactions.fd_seek, fh, offset)

    def fd_read(self, fh, size, offset):
        return self._portal.run(self.workspace_fs.transactions.fd_read, fh, size, offset)

    def fd_write(self, fh, data, offset):
        return self._portal.run(self.workspace_fs.transactions.fd_write, fh, data, offset)

    def fd_resize(self, fh, length):
        return self._portal.run(self.workspace_fs.transactions.fd_resize, fh, length)

    def fd_flush(self, fh):
        return self._portal.run(self.workspace_fs.transactions.fd_flush, fh)
