import pendulum
import itertools
from triopg.exceptions import UniqueViolationError
from typing import Tuple, Dict, List

from parsec.types import UserID, DeviceID
from parsec.backend.user import (
    BaseUserComponent,
    User,
    Device,
    DevicesMapping,
    UserInvitation,
    DeviceInvitation,
    UserError,
    UserAlreadyExistsError,
    UserAlreadyRevokedError,
    UserNotFoundError,
)
from parsec.backend.drivers.postgresql.handler import PGHandler


class PGUserComponent(BaseUserComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def create_user(self, user: User) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                await self._create_user(conn, user)

    async def _create_user(self, conn, user: User) -> None:
        try:
            result = await conn.execute(
                """
                INSERT INTO users (
                    user_id,
                    certified_user,
                    user_certifier,
                    created_on
                )
                VALUES ($1, $2, $3, $4)
                """,
                user.user_id,
                user.certified_user,
                user.user_certifier,
                user.created_on,
            )
        except UniqueViolationError as exc:
            raise UserAlreadyExistsError(f"User `{user.user_id}` already exists")

        if result != "INSERT 0 1":
            raise UserError(f"Insertion error: {result}")

        await conn.executemany(
            """INSERT INTO devices (
                device_id,
                user_id,
                certified_device,
                device_certifier,
                created_on,
                revocated_on,
                certified_revocation,
                revocation_certifier
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            """,
            [
                (
                    device.device_id,
                    device.user_id,
                    device.certified_device,
                    device.device_certifier,
                    device.created_on,
                    device.revocated_on,
                    device.certified_revocation,
                    device.revocation_certifier,
                )
                for device in user.devices.values()
            ],
        )

    async def create_device(self, device: Device) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                await self._create_device(conn, device)

    async def _create_device(self, conn, device: Device) -> None:
        existing_devices = await conn.fetch(
            "SELECT device_id FROM devices WHERE user_id = $1", device.user_id
        )
        if not existing_devices:
            raise UserNotFoundError(f"User `{device.user_id}` doesn't exists")

        if device.device_id in itertools.chain(*existing_devices):
            raise UserAlreadyExistsError(f"Device `{device.device_id}` already exists")

        result = await conn.execute(
            """INSERT INTO devices (
                device_id,
                user_id,
                certified_device,
                device_certifier,
                created_on,
                revocated_on,
                certified_revocation,
                revocation_certifier
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            """,
            device.device_id,
            device.user_id,
            device.certified_device,
            device.device_certifier,
            device.created_on,
            device.revocated_on,
            device.certified_revocation,
            device.revocation_certifier,
        )

        if result != "INSERT 0 1":
            raise UserError(f"Insertion error: {result}")

    async def get_user(self, user_id: UserID) -> User:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                return await self._get_user(conn, user_id)

    async def _get_user(self, conn, user_id: UserID) -> User:
        user_result = await conn.fetchrow(
            """
            SELECT certified_user, user_certifier, created_on
            FROM users
            WHERE user_id = $1
            """,
            user_id,
        )
        if not user_result:
            raise UserNotFoundError(user_id)

        devices_results = await conn.fetch(
            """
            SELECT 
                device_id,
                certified_device,
                device_certifier,
                created_on,
                revocated_on,
                certified_revocation,
                revocation_certifier
            FROM devices WHERE user_id = $1
            """,
            user_id,
        )
        devices = DevicesMapping(
            *[
                Device(DeviceID(device_result[0]), *device_result[1:])
                for device_result in devices_results
            ]
        )

        return User(
            user_id=UserID(user_id),
            certified_user=user_result[0],
            user_certifier=user_result[1],
            created_on=user_result[2],
            devices=devices,
        )

    async def _get_trustchain(self, conn, user: User) -> List[Device]:
        certifiers = [user.user_certifier]
        for device in user.devices.values():
            certifiers.append(device.device_certifier)
            certifiers.append(device.revocation_certifier)

        devices_results = await conn.fetch(
            """
WITH RECURSIVE trustchain(
    SELECT 
        device_id,
        certified_device,
        device_certifier,
        created_on,
        revocated_on,
        certified_revocation,
        revocation_certifier
    FROM devices WHERE device_id IN $1

    UNION SELECT
        d.device_id,
        d.certified_device,
        d.device_certifier,
        d.created_on,
        d.revocated_on,
        d.certified_revocation,
        d.revocation_certifier
    FROM devices d

    INNER JOIN trustchain t ON t.device_id IN (d.device_certifier, d.revocation_certifier)
) SELECT * FROM trustchain;
            """
        )
        return [Device(*device_result) for device_result in devices_results]

    async def get_user_with_trustchain(
        self, user_id: UserID
    ) -> Tuple[User, Dict[DeviceID, Device]]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                user = await self._get_user(conn, user_id)
                trustchain = await self._get_trustchain(conn, user)
                return user, trustchain

    # async def get_device(self, device_id: DeviceID) -> Device:
    #     raise NotImplementedError()

    # async def get_device_with_trustchain(
    #     self, device_id: DeviceID
    # ) -> Tuple[Device, Dict[DeviceID, Device]]:
    #     raise NotImplementedError()

    async def find(
        self, query: str = None, page: int = 1, per_page: int = 100
    ) -> Tuple[List[UserID], int]:
        raise NotImplementedError()

    async def create_user_invitation(self, invitation: UserInvitation) -> None:
        raise NotImplementedError()

    async def get_user_invitation(self, user_id: UserID) -> UserInvitation:
        raise NotImplementedError()

    async def user_cancel_invitation(self, user_id: UserID) -> None:
        raise NotImplementedError()

    async def create_device_invitation(self, invitation: DeviceInvitation) -> None:
        raise NotImplementedError()

    async def get_device_invitation(self, user_id: UserID) -> DeviceInvitation:
        raise NotImplementedError()

    async def device_cancel_invitation(self, device_id: DeviceID) -> None:
        raise NotImplementedError()

    async def revoke_device(
        self, device_id: DeviceID, certified_revocation: bytes, revocation_certifier: DeviceID
    ) -> None:
        raise NotImplementedError()
