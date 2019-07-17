# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from nacl.exceptions import BadSignatureError, CryptoError  # noqa: republishing

# Note to simplify things, we adopt `nacl.CryptoError` as our root error cls


class CryptoWrappedMsgValidationError(CryptoError):
    pass


class CryptoWrappedMsgPackingError(CryptoError):
    pass


class CryptoSignatureAuthorMismatchError(CryptoError):
    pass


class CryptoSignatureTimestampMismatchError(CryptoError):
    pass
