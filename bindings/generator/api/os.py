# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .common import Enum, EnumItemUnit


class OS(Enum):
    Linux = EnumItemUnit
    MacOs = EnumItemUnit
    Windows = EnumItemUnit
    Android = EnumItemUnit


def get_os() -> OS:
    raise NotImplementedError
