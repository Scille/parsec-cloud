# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec_pyi.device import DeviceFileType
from parsec._parsec_pyi.ids import DeviceID, DeviceLabel, HumanHandle, OrganizationID

class DeviceFile:
    def __init__(
        self,
        type: DeviceFileType,
        ciphertext: bytes,
        human_handle: HumanHandle | None,
        device_label: DeviceLabel | None,
        device_id: DeviceID,
        organization_id: OrganizationID,
        slug: str,
        salt: bytes | None,
        encrypted_key: bytes | None,
        certificate_id: str | None,
        certificate_sha1: bytes | None,
    ) -> None: ...
    @property
    def type(self) -> DeviceFileType: ...
    @property
    def ciphertext(self) -> bytes: ...
    @property
    def human_handle(self) -> HumanHandle | None: ...
    @property
    def device_label(self) -> DeviceLabel | None: ...
    @property
    def device_id(self) -> DeviceID: ...
    @property
    def organization_id(self) -> OrganizationID: ...
    @property
    def slug(self) -> str: ...
    @property
    def salt(self) -> bytes: ...
    @property
    def encrypted_key(self) -> bytes: ...
    @property
    def certificate_id(self) -> str: ...
    @property
    def certificate_sha1(self) -> bytes | None: ...
    @classmethod
    def load(cls, data: bytes) -> DeviceFile: ...
    def dump(self) -> bytes: ...
