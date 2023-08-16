# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from .common import ErrorVariant, Handle, Result


def new_canceller() -> Handle:
    ...


class CancelError(ErrorVariant):
    class NotBinded:
        pass

    class Internal:
        pass


def cancel(canceller: Handle) -> Result[None, CancelError]:
    ...
