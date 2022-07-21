# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec._version import __version__

# The parsec.utils module includes a bit of patching, let's make sure it is imported
__import__("parsec.utils")

try:
    import libparsec  # noqa
except ImportError:
    IS_OXIDIZED = False
else:
    IS_OXIDIZED = True

__all__ = ("__version__", "IS_OXIDIZED")
