from parsec.core.fs.base import FSBase
from parsec.core.fs.ops import FSOpsMixin
from parsec.core.fs.sync import FSSyncMixin
from parsec.core.fs.utils import FSInvalidPath


class FS(FSOpsMixin, FSSyncMixin, FSBase):
    pass


__all__ = ("FSInvalidPath", "FS")
