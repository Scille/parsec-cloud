# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

# Monkeypath to improve debugging on File/Dir nodes
from SCons.Node.FS import Base

Base.__repr__ = lambda self: f"<{type(self).__module__}.{type(self).__name__} {str(self)}>"
