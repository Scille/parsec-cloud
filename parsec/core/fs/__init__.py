from parsec.core.fs.base import FSBase
from parsec.core.fs.ops import FSOpsMixin
from parsec.core.fs.sync import FSSyncMixin
from parsec.core.fs.sync_file import FSSyncFileMixin
from parsec.core.fs.sync_folder import FSSyncFolderMixin
from parsec.core.fs.utils import FSInvalidPath


class FS(FSOpsMixin, FSSyncMixin, FSSyncFolderMixin, FSSyncFileMixin, FSBase):
    pass


__all__ = ("FSInvalidPath", "FS")
