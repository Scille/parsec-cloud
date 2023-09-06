# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .common import ErrorVariant, Handle, Result


def new_canceller() -> Handle:
    raise NotImplementedError


class CancelError(ErrorVariant):
    class NotBound:
        pass

    class Internal:
        pass


def cancel(canceller: Handle) -> Result[None, CancelError]:
    raise NotImplementedError
