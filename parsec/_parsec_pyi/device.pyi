# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

class DeviceFileType:
    PASSWORD: DeviceFileType
    SMARTCARD: DeviceFileType
    RECOVERY: DeviceFileType

    def values(self) -> tuple[DeviceFileType, ...]: ...
    def str(self) -> str: ...
    def dump(self) -> bytes: ...
    @classmethod
    def load(cls, data: bytes) -> DeviceFileType: ...
