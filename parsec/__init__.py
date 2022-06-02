# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec._version import __version__

# The parsec.utils module includes a bit of patching, let's make sure it is imported
import parsec.utils  # noqa

__all__ = ("__version__",)

try:
    import libparsec  # noqa
except:
    IS_OXIDIZED = False
else:
    IS_OXIDIZED = True
