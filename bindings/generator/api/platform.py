# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .common import Enum, EnumItemUnit


class Platform(Enum):
    Linux = EnumItemUnit
    MacOS = EnumItemUnit
    Windows = EnumItemUnit
    Android = EnumItemUnit
    Web = EnumItemUnit


def get_platform() -> Platform:
    raise NotImplementedError
