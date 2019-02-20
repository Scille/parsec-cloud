# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
import itertools
from triopg.exceptions import UniqueViolationError
from typing import Tuple, Dict, List, Optional

from parsec.types import UserID, DeviceID, OrganizationID
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

    async def set_user_admin(
        self, organization_id: OrganizationID, user_id: UserID, is_admin: bool
    ) -> None:
        await self.get_user(organization_id, user_id)

        async with self.dbh.pool.acquire() as conn:
            result = await conn.execute(
                """
UPDATE users SET
    is_admin = $3
WHERE
    organization = get_organization_internal_id($1)
    AND user_id = $2
""",
                organization_id,
                user_id,
                is_admin,
            )
            if result != "UPDATE 1":
                raise UserError(f"Update error: {result}")

    async def create_user(self, organization_id: OrganizationID, user: User) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                await self._create_user(conn, organization_id, user)
                await send_signal(
                    conn, "user.created", organization_id=organization_id, user_id=user.user_id
                )

    @staticmethod
    async def _create_user(conn, organization_id: OrganizationID, user: User) -> None:
        try:
            result = await conn.execute(
                """
INSERT INTO users (
    organization,
    user_id,
    is_admin,
    certified_user,
    user_certifier,
    created_on
)
SELECT
    get_organization_internal_id($1),
    $2, $3, $4,
    get_device_internal_id($1, $5),
    $6
""",
                organization_id,
                user.user_id,
                user.is_admin,
                user.certified_user,
                user.user_certifier,
                user.created_on,
            )
        except UniqueViolationError:
            raise UserAlreadyExistsError(f"User `{user.user_id}` already exists")

        if result != "INSERT 0 1":
            raise UserError(f"Insertion error: {result}")

        await conn.executemany(
            """
INSERT INTO devices (
    organization,
    user_,
    device_id,
    certified_device,
    device_certifier,
    created_on,
    revocated_on,
    certified_revocation,
    revocation_certifier
)
SELECT
    get_organization_internal_id($1),
    get_user_internal_id($1, $2),
    $3, $4,
    get_device_internal_id($1, $5),
    $6, $7, $8,
    get_device_internal_id($1, $9)
""",
            [
                (
                    organization_id,
                    device.user_id,
                    device.device_id,
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

    async def create_device(
        self, organization_id: OrganizationID, device: Device, encrypted_answer: bytes = b""
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                await self._create_device(conn, organization_id, device)
                await send_signal(
                    conn,
                    "device.created",
                    organization_id=organization_id,
                    device_id=device.device_id,
                    encrypted_answer=encrypted_answer,
                )

    async def _create_device(self, conn, organization_id: OrganizationID, device: Device) -> None:
        existing_devices = await conn.fetch(
            """
SELECT device_id FROM devices
WHERE user_ = get_user_internal_id($1, $2)
            """,
            organization_id,
            device.user_id,
        )
        if not existing_devices:
            raise UserNotFoundError(f"User `{device.user_id}` doesn't exists")

        if device.device_id in itertools.chain(*existing_devices):
            raise UserAlreadyExistsError(f"Device `{device.device_id}` already exists")

        result = await conn.execute(
            """
INSERT INTO devices (
    organization,
    user_,
    device_id,
    certified_device,
    device_certifier,
    created_on,
    revocated_on,
    certified_revocation,
    revocation_certifier
)
SELECT
    get_organization_internal_id($1),
    get_user_internal_id($1, $2),
    $3, $4,
    get_device_internal_id($1, $5),
    $6, $7, $8,
    get_device_internal_id($1, $9)
""",
            organization_id,
            device.user_id,
            device.device_id,
            device.certified_device,
            device.device_certifier,
            device.created_on,
            device.revocated_on,
            device.certified_revocation,
            device.revocation_certifier,
        )

        if result != "INSERT 0 1":
            raise UserError(f"Insertion error: {result}")

    async def get_user(self, organization_id: OrganizationID, user_id: UserID) -> User:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                return await self._get_user(conn, organization_id, user_id)

    async def _get_user(self, conn, organization_id: OrganizationID, user_id: UserID) -> User:
        user_result = await conn.fetchrow(
            """
SELECT
    is_admin,
    certified_user,
    get_device_id(user_certifier) as user_certifier,
    created_on
FROM users
WHERE
    organization = get_organization_internal_id($1)
    AND user_id = $2
""",
            organization_id,
            user_id,
        )
        if not user_result:
            raise UserNotFoundError(user_id)

        devices_results = await conn.fetch(
            """
SELECT
    d1.device_id,
    d1.certified_device,
    get_device_id(d1.device_certifier) as device_certifier,
    d1.created_on,
    d1.revocated_on,
    d1.certified_revocation,
    get_device_id(d1.revocation_certifier) as revocation_certifier
FROM devices as d1
WHERE user_ = get_user_internal_id($1, $2)
""",
            organization_id,
            user_id,
        )
        devices = DevicesMapping(
            *[Device(DeviceID(d_id), *d_data) for d_id, *d_data in devices_results]
        )

        return User(
            user_id=UserID(user_id),
            is_admin=user_result[0],
            certified_user=user_result[1],
            user_certifier=user_result[2],
            created_on=user_result[3],
            devices=devices,
        )

    async def _get_trustchain(
        self, conn, organization_id: OrganizationID, user: User
    ) -> Dict[DeviceID, Device]:
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
    d1.device_id,
    d1.certified_device,
    get_device_id(d1.device_certifier) as device_certifier,
    d1.created_on,
    d1.revocated_on,
    d1.certified_revocation,
    get_device_id(d1.revocation_certifier) as revocation_certifier
FROM devices as d1
WHERE
    organization = get_organization_internal_id($1)
    AND device_id = any($2::text[])
""",
                organization_id,
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
        self, organization_id: OrganizationID, user_id: UserID
    ) -> Tuple[User, Dict[DeviceID, Device]]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                user = await self._get_user(conn, organization_id, user_id)
                trustchain = await self._get_trustchain(conn, organization_id, user)
                return user, trustchain

    # async def get_device(self, device_id: DeviceID) -> Device:
    #     raise NotImplementedError()

    # async def get_device_with_trustchain(
    #     self, device_id: DeviceID
    # ) -> Tuple[Device, Dict[DeviceID, Device]]:
    #     raise NotImplementedError()

    async def find(
        self,
        organization_id: OrganizationID,
        query: str = None,
        page: int = 1,
        per_page: int = 100,
        omit_revocated: bool = False,
    ) -> Tuple[List[UserID], int]:
        async with self.dbh.pool.acquire() as conn:
            now = pendulum.now()
            if query:
                # LIKE only use % and _ as special tokens
                escaped_query = query.replace("!", "!!").replace("%", "!%").replace("_", "!_")
                all_results = await conn.fetch(
                    """
SELECT user_id FROM users
WHERE
    organization = get_organization_internal_id($1)
    AND user_id LIKE $2 ESCAPE '!'
    AND (
        NOT $3 OR EXISTS (
            SELECT TRUE FROM devices
            WHERE
                user_ = users._id
                AND (
                    revocated_on IS NULL
                    OR revocated_on > $4
                )
        )
    )
ORDER BY user_id
""",
                    organization_id,
                    f"{escaped_query}%",
                    omit_revocated,
                    now,
                )
            else:
                all_results = await conn.fetch(
                    """
SELECT user_id FROM users
WHERE
    organization = get_organization_internal_id($1)
    AND (
        NOT $2 OR EXISTS (
            SELECT TRUE FROM devices
            WHERE
                user_ = users._id
                AND (
                    revocated_on IS NULL
                    OR revocated_on > $3
                )
        )
    )
ORDER BY user_id
""",
                    organization_id,
                    omit_revocated,
                    now,
                )
            # TODO: should user LIMIT and OFFSET in the SQL query instead
            results = [UserID(x[0]) for x in all_results[(page - 1) * per_page : page * per_page]]
        return results, len(all_results)

    async def _user_exists(self, conn, organization_id: OrganizationID, user_id: UserID) -> bool:
        user_result = await conn.fetchrow(
            """
SELECT true FROM users
WHERE  _id = get_user_internal_id($1, $2)
""",
            organization_id,
            user_id,
        )
        return bool(user_result)

    async def _device_exists(
        self, conn, organization_id: OrganizationID, device_id: DeviceID
    ) -> bool:
        user_result = await conn.fetchrow(
            """
SELECT true
FROM devices
WHERE
    organization = get_organization_internal_id($1)
    AND device_id = $2
""",
            organization_id,
            device_id,
        )
        return bool(user_result)

    async def create_user_invitation(
        self, organization_id: OrganizationID, invitation: UserInvitation
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                if await self._user_exists(conn, organization_id, invitation.user_id):
                    raise UserAlreadyExistsError(f"User `{invitation.user_id}` already exists")

                result = await conn.execute(
                    """
INSERT INTO user_invitations (
    organization,
    creator,
    user_id,
    created_on
) VALUES (
    get_organization_internal_id($1),
    get_device_internal_id($1, $2),
    $3, $4
)
ON CONFLICT (organization, user_id)
DO UPDATE
SET
    organization = excluded.organization,
    creator = excluded.creator,
    created_on = excluded.created_on
""",
                    organization_id,
                    invitation.creator,
                    invitation.user_id,
                    invitation.created_on,
                )

                if result not in ("INSERT 0 1", "UPDATE 1"):
                    raise UserError(f"Insertion error: {result}")

    async def get_user_invitation(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> UserInvitation:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                return await self._get_user_invitation(conn, organization_id, user_id)

    async def _get_user_invitation(self, conn, organization_id: OrganizationID, user_id: UserID):
        if await self._user_exists(conn, organization_id, user_id):
            raise UserAlreadyExistsError(f"User `{user_id}` already exists")

        result = await conn.fetchrow(
            """
SELECT user_id, get_device_id(creator), created_on
FROM user_invitations
WHERE
    organization = get_organization_internal_id($1)
    AND user_id = $2
""",
            organization_id,
            user_id,
        )
        if not result:
            raise UserNotFoundError(user_id)

        return UserInvitation(
            user_id=UserID(result[0]), creator=DeviceID(result[1]), created_on=result[2]
        )

    async def claim_user_invitation(
        self, organization_id: OrganizationID, user_id: UserID, encrypted_claim: bytes = b""
    ) -> UserInvitation:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                invitation = await self._get_user_invitation(conn, organization_id, user_id)
                await send_signal(
                    conn,
                    "user.claimed",
                    organization_id=organization_id,
                    user_id=invitation.user_id,
                    encrypted_claim=encrypted_claim,
                )
                return invitation

    async def cancel_user_invitation(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                if await self._user_exists(conn, organization_id, user_id):
                    raise UserAlreadyExistsError(f"User `{user_id}` already exists")

                result = await conn.execute(
                    """
DELETE FROM user_invitations
WHERE
    organization = get_organization_internal_id($1)
    AND user_id = $2
""",
                    organization_id,
                    user_id,
                )
                if result not in ("DELETE 1", "DELETE 0"):
                    raise UserError(f"Deletion error: {result}")

                await send_signal(
                    conn,
                    "user.invitation.cancelled",
                    organization_id=organization_id,
                    device_id=user_id,
                )

    async def create_device_invitation(
        self, organization_id: OrganizationID, invitation: DeviceInvitation
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                if await self._device_exists(conn, organization_id, invitation.device_id):
                    raise UserAlreadyExistsError(f"Device `{invitation.device_id}` already exists")

                result = await conn.execute(
                    """
INSERT INTO device_invitations (
    organization,
    creator,
    device_id,
    created_on
)
VALUES (
    get_organization_internal_id($1),
    get_device_internal_id($1, $2),
    $3, $4
)
ON CONFLICT (organization, device_id)
DO UPDATE
SET
    organization = excluded.organization,
    creator = excluded.creator,
    created_on = excluded.created_on
""",
                    organization_id,
                    invitation.creator,
                    invitation.device_id,
                    invitation.created_on,
                )

                if result not in ("INSERT 0 1", "UPDATE 1"):
                    raise UserError(f"Insertion error: {result}")

    async def get_device_invitation(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> DeviceInvitation:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                return await self._get_device_invitation(conn, organization_id, device_id)

    async def _get_device_invitation(
        self, conn, organization_id: OrganizationID, device_id: DeviceID
    ):
        if await self._device_exists(conn, organization_id, device_id):
            raise UserAlreadyExistsError(f"Device `{device_id}` already exists")

        result = await conn.fetchrow(
            """
SELECT device_id, get_device_id(creator), created_on
FROM device_invitations
WHERE
    organization = get_organization_internal_id($1)
    AND device_id = $2
""",
            organization_id,
            device_id,
        )
        if not result:
            raise UserNotFoundError(device_id)

        return DeviceInvitation(
            device_id=DeviceID(result[0]), creator=DeviceID(result[1]), created_on=result[2]
        )

    async def claim_device_invitation(
        self, organization_id: OrganizationID, device_id: DeviceID, encrypted_claim: bytes = b""
    ) -> UserInvitation:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                invitation = await self._get_device_invitation(conn, organization_id, device_id)
                await send_signal(
                    conn,
                    "device.claimed",
                    organization_id=organization_id,
                    device_id=invitation.device_id,
                    encrypted_claim=encrypted_claim,
                )
                return invitation

    async def cancel_device_invitation(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                if await self._device_exists(conn, organization_id, device_id):
                    raise UserAlreadyExistsError(f"Device `{device_id}` already exists")

                result = await conn.execute(
                    """
DELETE FROM device_invitations
WHERE
    organization = get_organization_internal_id($1)
    AND device_id = $2
""",
                    organization_id,
                    device_id,
                )
                if result not in ("DELETE 1", "DELETE 0"):
                    raise UserError(f"Deletion error: {result}")

                await send_signal(
                    conn,
                    "device.invitation.cancelled",
                    organization_id=organization_id,
                    device_id=device_id,
                )

    async def revoke_device(
        self,
        organization_id: OrganizationID,
        device_id: DeviceID,
        certified_revocation: bytes,
        revocation_certifier: DeviceID,
    ) -> Optional[pendulum.Pendulum]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                result = await conn.execute(
                    """
UPDATE devices SET
    certified_revocation = $3,
    revocation_certifier = get_device_internal_id($1, $4),
    revocated_on = $5
WHERE
    organization = get_organization_internal_id($1)
    AND device_id = $2
    AND revocated_on IS NULL
""",
                    organization_id,
                    device_id,
                    certified_revocation,
                    revocation_certifier,
                    pendulum.now(),
                )

                if result != "UPDATE 1":
                    # TODO: avoid having to do another query to find the error
                    err_result = await conn.fetchrow(
                        """
SELECT revocated_on
FROM devices
WHERE
    organization = get_organization_internal_id($1)
    AND device_id = $2
""",
                        organization_id,
                        device_id,
                    )
                    if not err_result:
                        raise UserNotFoundError(device_id)
                    if err_result[0]:
                        raise UserAlreadyRevokedError()

                    else:
                        raise UserError(f"Update error: {result}")

                # Determine if the user has bee revoked (i.e. all
                # his devices are revoked)
                result = await conn.fetch(
                    """
SELECT revocated_on FROM devices
WHERE user_ = get_user_internal_id($1, $2)
            """,
                    organization_id,
                    device_id.user_id,
                )

        revocations = [r[0] for r in result]
        if None in revocations:
            return None
        else:
            return sorted(revocations)[-1]
