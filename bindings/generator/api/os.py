# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .common import Variant


class OS(Variant):
    class Linux:
        pass

    class MacOs:
        pass

    class Windows:
        pass

    class Android:
        pass


def get_os() -> OS:
    raise NotImplementedError
