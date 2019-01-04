import pendulum
import itertools
from triopg.exceptions import UniqueViolationError
from typing import Tuple, Dict, List

from parsec.types import UserID, DeviceID
from parsec.event_bus import EventBus
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
from parsec.backend.drivers.postgresql.handler import send_signal, PGHandler


class PGUserComponent(BaseUserComponent):
    def __init__(self, dbh: PGHandler, event_bus: EventBus):
        self.dbh = dbh
        self.event_bus = event_bus

    async def set_user_admin(self, user_id: UserID, is_admin: bool) -> None:
        user = await self.get_user(user_id)

        async with self.dbh.pool.acquire() as conn:
            result = await conn.execute(
                """
                UPDATE users SET
                    is_admin = $2
                WHERE user_id = $1
                """,
                user_id,
                is_admin,
            )
            if result != "UPDATE 1":
                raise UserError(f"Update error: {result}")

    async def create_user(self, user: User) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                await self._create_user(conn, user)
                await send_signal(conn, "user.created", user_id=user.user_id)

    @staticmethod
    async def _create_user(conn, user: User) -> None:
        try:
            result = await conn.execute(
                """
                INSERT INTO users (
                    user_id,
                    is_admin,
                    certified_user,
                    user_certifier,
                    created_on
                )
                VALUES ($1, $2, $3, $4, $5)
                """,
                user.user_id,
                user.is_admin,
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

    async def create_device(self, device: Device, encrypted_answer: bytes = b"") -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                await self._create_device(conn, device)
                await send_signal(
                    conn,
                    "device.created",
                    device_id=device.device_id,
                    encrypted_answer=encrypted_answer,
                )

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
            SELECT is_admin, certified_user, user_certifier, created_on
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
            is_admin=user_result[0],
            certified_user=user_result[1],
            user_certifier=user_result[2],
            created_on=user_result[3],
            devices=devices,
        )

    async def _get_trustchain(self, conn, user: User) -> Dict[DeviceID, Device]:
        # TODO: it's time to do a super awesome SQL query fetching everything
        # in one go...
        devices = {}
        devices_to_fetch = []
        if user.user_certifier:
            devices_to_fetch.append(user.user_certifier)
        for device in user.devices.values():
            if device.device_certifier:
                devices_to_fetch.append(device.device_certifier)
            if device.revocation_certifier:
                devices_to_fetch.append(device.revocation_certifier)

        while devices_to_fetch:
            results = await conn.fetch(
                """
            SELECT
                device_id,
                certified_device,
                device_certifier,
                created_on,
                revocated_on,
                certified_revocation,
                revocation_certifier
            FROM devices WHERE device_id = any($1::text[])
                """,
                devices_to_fetch,
            )

            for result in results:
                devices[result[0]] = Device(*result)

            devices_to_fetch = []
            for device in devices.values():
                if device.device_certifier and device.device_certifier not in devices:
                    devices_to_fetch.append(device.device_certifier)
                if device.revocation_certifier and device.revocation_certifier not in devices:
                    devices_to_fetch.append(device.revocation_certifier)

        return devices

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
        async with self.dbh.pool.acquire() as conn:
            if query:
                # LIKE only use % and _ as special tokens
                escaped_query = query.replace("!", "!!").replace("%", "!%").replace("_", "!_")
                all_results = await conn.fetch(
                    """
                    SELECT user_id FROM users
                    WHERE
                        user_id LIKE $1 ESCAPE '!'
                    ORDER BY user_id
                    """,
                    f"{escaped_query}%",
                )
            else:
                all_results = await conn.fetch(
                    """
                    SELECT user_id FROM users ORDER BY user_id
                    """
                )
            # TODO: should user LIMIT and OFFSET in the SQL query instead
            results = [x[0] for x in all_results[(page - 1) * per_page : page * per_page]]
        return results, len(all_results)

    async def _user_exists(self, conn, user_id: UserID) -> bool:
        user_result = await conn.fetchrow("SELECT true FROM users WHERE user_id = $1", user_id)
        return bool(user_result)

    async def _device_exists(self, conn, device_id: DeviceID) -> bool:
        user_result = await conn.fetchrow(
            "SELECT true FROM devices WHERE device_id = $1", device_id
        )
        return bool(user_result)

    async def create_user_invitation(self, invitation: UserInvitation) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                if await self._user_exists(conn, invitation.user_id):
                    raise UserAlreadyExistsError(f"User `{invitation.user_id}` already exists")

                result = await conn.execute(
                    """
                    INSERT INTO user_invitations (
                        user_id,
                        creator,
                        created_on
                    ) VALUES ($1, $2, $3)
                    ON CONFLICT (user_id)
                    DO UPDATE
                    SET creator = $2, created_on = $3
                    """,
                    invitation.user_id,
                    invitation.creator,
                    invitation.created_on,
                )

                if result not in ("INSERT 0 1", "UPDATE 1"):
                    raise UserError(f"Insertion error: {result}")

    async def get_user_invitation(self, user_id: UserID) -> UserInvitation:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                return await self._get_user_invitation(conn, user_id)

    async def _get_user_invitation(self, conn, user_id):
        if await self._user_exists(conn, user_id):
            raise UserAlreadyExistsError(f"User `{user_id}` already exists")

        result = await conn.fetchrow(
            """
            SELECT user_id, creator, created_on
            FROM user_invitations
            WHERE user_id = $1
            """,
            user_id,
        )
        if not result:
            raise UserNotFoundError(user_id)

        return UserInvitation(
            user_id=UserID(result[0]), creator=DeviceID(result[1]), created_on=result[2]
        )

    async def claim_user_invitation(
        self, user_id: UserID, encrypted_claim: bytes = b""
    ) -> UserInvitation:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                invitation = await self._get_user_invitation(conn, user_id)
                await send_signal(
                    conn,
                    "user.claimed",
                    user_id=invitation.user_id,
                    encrypted_claim=encrypted_claim,
                )
                return invitation

    async def cancel_user_invitation(self, user_id: UserID) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                if await self._user_exists(conn, user_id):
                    raise UserAlreadyExistsError(f"User `{user_id}` already exists")

                result = await conn.execute(
                    """
                    DELETE FROM user_invitations
                    WHERE user_id = $1
                    """,
                    user_id,
                )
                if result not in ("DELETE 1", "DELETE 0"):
                    raise UserError(f"Deletion error: {result}")

                await send_signal(conn, "user.invitation.cancelled", device_id=user_id)

    async def create_device_invitation(self, invitation: DeviceInvitation) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                if await self._device_exists(conn, invitation.device_id):
                    raise UserAlreadyExistsError(f"Device `{invitation.device_id}` already exists")

                result = await conn.execute(
                    """
                    INSERT INTO device_invitations (
                        device_id,
                        creator,
                        created_on
                    ) VALUES ($1, $2, $3)
                    ON CONFLICT (device_id)
                    DO UPDATE
                    SET creator = $2, created_on = $3
                    """,
                    invitation.device_id,
                    invitation.creator,
                    invitation.created_on,
                )

                if result not in ("INSERT 0 1", "UPDATE 1"):
                    raise UserError(f"Insertion error: {result}")

    async def get_device_invitation(self, device_id: DeviceID) -> DeviceInvitation:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                return await self._get_device_invitation(conn, device_id)

    async def _get_device_invitation(self, conn, device_id):
        if await self._device_exists(conn, device_id):
            raise UserAlreadyExistsError(f"Device `{device_id}` already exists")

        result = await conn.fetchrow(
            """
            SELECT device_id, creator, created_on
            FROM device_invitations
            WHERE device_id = $1
            """,
            device_id,
        )
        if not result:
            raise UserNotFoundError(device_id)

        return DeviceInvitation(
            device_id=DeviceID(result[0]), creator=DeviceID(result[1]), created_on=result[2]
        )

    async def claim_device_invitation(
        self, device_id: DeviceID, encrypted_claim: bytes = b""
    ) -> UserInvitation:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                invitation = await self._get_device_invitation(conn, device_id)
                await send_signal(
                    conn,
                    "device.claimed",
                    device_id=invitation.device_id,
                    encrypted_claim=encrypted_claim,
                )
                return invitation

    async def cancel_device_invitation(self, device_id: DeviceID) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                if await self._device_exists(conn, device_id):
                    raise UserAlreadyExistsError(f"Device `{device_id}` already exists")

                result = await conn.execute(
                    """
                    DELETE FROM device_invitations
                    WHERE device_id = $1
                    """,
                    device_id,
                )
                if result not in ("DELETE 1", "DELETE 0"):
                    raise UserError(f"Deletion error: {result}")

                await send_signal(conn, "device.invitation.cancelled", device_id=device_id)

    async def revoke_device(
        self, device_id: DeviceID, certified_revocation: bytes, revocation_certifier: DeviceID
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                result = await conn.execute(
                    """
                    UPDATE devices SET
                        certified_revocation = $2,
                        revocation_certifier = $3,
                        revocated_on = $4
                    WHERE device_id = $1 AND revocated_on IS NULL
                    """,
                    device_id,
                    certified_revocation,
                    revocation_certifier,
                    pendulum.now(),
                )

                if result != "UPDATE 1":
                    # TODO: avoid having to do another query to find the error
                    err_result = await conn.fetchrow(
                        """
                        SELECT revocated_on FROM devices WHERE device_id = $1
                    """,
                        device_id,
                    )
                    if not err_result:
                        raise UserNotFoundError(device_id)
                    if err_result[0]:
                        raise UserAlreadyRevokedError()

                    else:
                        raise UserError(f"Update error: {result}")
