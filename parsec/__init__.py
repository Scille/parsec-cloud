# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec._version import __version__

# The parsec.utils module includes a bit of patching, let's make sure it is imported
__import__("parsec.utils")

import parsec._parsec  # noqa

__all__ = [__version__]
