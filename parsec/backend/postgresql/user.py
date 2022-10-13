# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import DateTime
from typing import Tuple, List, Optional

from parsec.api.protocol import UserID, DeviceID, OrganizationID
from parsec.backend.user import (
    BaseUserComponent,
    User,
    Device,
    Trustchain,
    GetUserAndDevicesResult,
    HumanFindResultItem,
)
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.user_queries import (
    query_create_user,
    query_create_device,
    query_find_humans,
    query_get_user,
    query_get_user_with_trustchain,
    query_get_user_with_device_and_trustchain,
    query_get_user_with_devices_and_trustchain,
    query_get_user_with_device,
    query_revoke_user,
    query_dump_users,
)


class PGUserComponent(BaseUserComponent):
    def __init__(self, dbh: PGHandler, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.dbh = dbh

    async def create_user(
        self, organization_id: OrganizationID, user: User, first_device: Device
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            await query_create_user(conn, organization_id, user, first_device)

    async def create_device(
        self, organization_id: OrganizationID, device: Device, encrypted_answer: bytes = b""
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            await query_create_device(conn, organization_id, device, encrypted_answer)

    async def get_user(self, organization_id: OrganizationID, user_id: UserID) -> User:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_user(conn, organization_id, user_id)

    async def get_user_with_trustchain(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> Tuple[User, Trustchain]:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_user_with_trustchain(conn, organization_id, user_id)

    async def get_user_with_device_and_trustchain(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> Tuple[User, Device, Trustchain]:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_user_with_device_and_trustchain(conn, organization_id, device_id)

    async def get_user_with_devices_and_trustchain(
        self, organization_id: OrganizationID, user_id: UserID, redacted: bool = False
    ) -> GetUserAndDevicesResult:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_user_with_devices_and_trustchain(
                conn, organization_id, user_id, redacted=redacted
            )

    async def get_user_with_device(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> Tuple[User, Device]:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_user_with_device(conn, organization_id, device_id)

    async def find_humans(
        self,
        organization_id: OrganizationID,
        query: Optional[str] = None,
        page: int = 1,
        per_page: int = 100,
        omit_revoked: bool = False,
        omit_non_human: bool = False,
    ) -> Tuple[List[HumanFindResultItem], int]:
        async with self.dbh.pool.acquire() as conn:
            return await query_find_humans(
                conn=conn,
                organization_id=organization_id,
                query=query,
                page=page,
                per_page=per_page,
                omit_revoked=omit_revoked,
                omit_non_human=omit_non_human,
            )

    async def revoke_user(
        self,
        organization_id: OrganizationID,
        user_id: UserID,
        revoked_user_certificate: bytes,
        revoked_user_certifier: DeviceID,
        revoked_on: Optional[DateTime] = None,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            return await query_revoke_user(
                conn,
                organization_id,
                user_id,
                revoked_user_certificate,
                revoked_user_certifier,
                revoked_on,
            )

    async def dump_users(self, organization_id: OrganizationID) -> Tuple[List[User], List[Device]]:
        async with self.dbh.pool.acquire() as conn:
            return await query_dump_users(conn, organization_id)
