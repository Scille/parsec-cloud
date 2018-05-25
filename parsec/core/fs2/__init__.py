from parsec.core.fs2.base import FSBase
from parsec.core.fs2.ops import FSOpsMixin
from parsec.core.fs2.sync import FSSyncMixin
from parsec.core.fs2.exceptions import InvalidPath


class FS2(FSOpsMixin, FSSyncMixin, FSBase):
    pass

    # def _debug_pocav(self, name="DUMP FS"):
    #     from pprint import pprint

    #     print("~~~ %s ~~~" % name)
    #     pprint(self._dump_local_fs())
    #     pprint(self._opened_files)
    #     pprint(self._manifests_cache)
    #     print("~~~ END %s ~~~" % name)
