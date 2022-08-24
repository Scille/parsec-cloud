# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import os
from parsec._version import __version__

# The oscrypto library relies on `ctypes.util.find_library`,
# which doesn't work for snap classic enviornments.
# Hence, we rely on env variables similar to `FUSE_LIBRARY_PATH`
# to configure oscrypto correctly if those variables are provided.
SSL_LIBRARY_PATH = os.environ.get("SSL_LIBRARY_PATH")
CRYPTO_LIBRARY_PATH = os.environ.get("CRYPTO_LIBRARY_PATH")
if SSL_LIBRARY_PATH and CRYPTO_LIBRARY_PATH:
    import oscrypto

    oscrypto.use_openssl(
        libcrypto_path=CRYPTO_LIBRARY_PATH,
        libssl_path=SSL_LIBRARY_PATH,
    )

# The parsec.utils module includes a bit of patching, let's make sure it is imported
__import__("parsec.utils")

try:
    import parsec._parsec  # noqa
except ImportError:
    raise RuntimeError("Missing parsec lib, missing `maturin develop` ?")

__all__ = [__version__]
