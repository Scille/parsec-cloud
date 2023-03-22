# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec_pyi.backend_connection import AuthenticatedCmds
from parsec._parsec_pyi.certif import (
    DeviceCertificate,
    RealmRoleCertificate,
    RevokedUserCertificate,
    UserCertificate,
)
from parsec._parsec_pyi.enumerate import RealmRole
from parsec._parsec_pyi.ids import DeviceID, EntryID, UserID
from parsec._parsec_pyi.local_device import LocalDevice
from parsec._parsec_pyi.remote_devices_manager import RemoteDevicesManager
from parsec._parsec_pyi.time import DateTime

class UserRemoteLoader:
    def __init__(
        self,
        device: LocalDevice,
        workspace_id: EntryID,
        backend_cmds: AuthenticatedCmds,
        remote_devices_manager: RemoteDevicesManager,
    ) -> None: ...
    async def clear_realm_role_certificate_cache(self) -> None: ...
    async def load_realm_role_certificates(
        self, realm_id: EntryID | None = None
    ) -> list[RealmRoleCertificate]: ...
    async def load_realm_current_roles(
        self, realm_id: EntryID | None = None
    ) -> dict[UserID, RealmRole]: ...
    async def get_user(
        self, user_id: UserID, no_cache: bool = False
    ) -> tuple[UserCertificate, RevokedUserCertificate | None]: ...
    async def get_device(
        self, device_id: DeviceID, no_cache: bool = False
    ) -> DeviceCertificate: ...
    async def list_versions(self, entry_id: EntryID) -> dict[int, tuple[DateTime, DeviceID]]: ...
    async def create_realm(self, realm_id: EntryID) -> None: ...
