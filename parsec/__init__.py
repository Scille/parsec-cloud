# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._version import __version__

# The parsec.utils module includes a bit of patching, let's make sure it is imported
__import__("parsec.utils")

try:
    import parsec._parsec  # noqa
except ImportError:
    raise RuntimeError("Missing parsec lib, missing `maturin develop` ?")

__all__ = [__version__]
