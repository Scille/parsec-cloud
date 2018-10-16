from parsec.core.fs.folder_syncer import FolderSyncerMixin
from parsec.core.fs.file_syncer import FileSyncerMixin


class Syncer(FolderSyncerMixin, FileSyncerMixin):
    pass
