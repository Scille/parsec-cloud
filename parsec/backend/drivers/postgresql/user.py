import itertools
import pendulum
from triopg.exceptions import UniqueViolationError
from typing import List

# from parsec.utils import ParsecError
from parsec.backend.user import BaseUserComponent

# from parsec.backend.exceptions import (
#     AlreadyExistsError,
#     AlreadyRevokedError,
#     NotFoundError,
#     UserClaimError,
#     OutOfDateError,
# )


class PGUserComponent(BaseUserComponent):
    def __init__(self, dbh, event_bus):
        super().__init__(event_bus)
        self.dbh = dbh

    async def create_invitation(self, invitation_token: str, user_id: str):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                # TODO: combine this in the next SQL request
                user = await conn.fetchrow("SELECT 1 FROM users WHERE user_id = $1", user_id)

                if user:
                    raise AlreadyExistsError("User `%s` already exists" % user_id)

                result = await conn.execute(
                    """
                    INSERT INTO user_invitations (
                        status,
                        user_id,
                        invited_on,
                        invitation_token,
                        claim_tries
                    ) VALUES (
                        'pending',
                        $1,
                        $2,
                        $3,
                        0
                    )
                    """,
                    user_id,
                    pendulum.now(),
                    invitation_token,
                )
                if result != "INSERT 0 1":
                    raise ParsecError("Insertion error.")

    async def claim_invitation(
        self,
        user_id: str,
        invitation_token: str,
        broadcast_key: bytes,
        device_name: str,
        device_verify_key: bytes,
    ):
        assert isinstance(broadcast_key, (bytes, bytearray))
        assert isinstance(device_verify_key, (bytes, bytearray))

        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                rows = await conn.fetch(
                    """
                    SELECT
                        _id,
                        status,
                        invited_on,
                        invitation_token,
                        claim_tries
                    FROM user_invitations WHERE
                        user_id = $1
                    """,
                    user_id,
                )

                if not rows:
                    raise NotFoundError(f"No invitation for user `{user_id}`")

                for (rowid, status, invited_on, expected_invitation_token, claim_tries) in rows:
                    if expected_invitation_token == invitation_token:
                        break
                else:
                    await conn.executemany(
                        """
                        UPDATE user_invitations SET status = $2, claim_tries = $3 WHERE _id = $1
                        """,
                        rowid,
                        "pending" if claim_tries < 3 else "rejected",
                        claim_tries,
                    )
                    await conn.commit()
                    raise UserClaimError("Invalid invitation token")

                now = pendulum.now()
                if (now - invited_on) > pendulum.interval(hours=1):
                    raise OutOfDateError("Claim code is too old.")

                claim_tries += 1
                if status == "claimed":
                    raise UserClaimError(f"User `{user_id}` has already been registered")
                elif status == "rejected":
                    raise UserClaimError("Invalid invitation token")

                result = await conn.execute(
                    """
                    UPDATE user_invitations SET status = 'claimed', claim_tries = $2, claimed_on = $3 WHERE _id = $1
                    """,
                    rowid,
                    claim_tries,
                    now,
                )
                if result != "UPDATE 1":
                    raise ParsecError("Update error.")

                await self._create_user(
                    conn, now, user_id, broadcast_key, devices=[(device_name, device_verify_key)]
                )

    async def configure_device(self, user_id: str, device_name: str, device_verify_key: bytes):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                row = await conn.fetchrow(
                    """
                    SELECT
                        created_on
                    FROM unconfigured_devices
                    WHERE user_id = $1 AND device_name = $2
                    """,
                    user_id,
                    device_name,
                )
            if not row:
                raise NotFoundError(f"Device `{user_id}@{device_name}` doesn't exists")

            await self._create_device(conn, user_id, device_name, row[0], device_verify_key)

            result = await conn.execute(
                """
                DELETE FROM unconfigured_devices
                WHERE user_id = $1 AND device_name = $2
                """,
                user_id,
                device_name,
            )
            if result != "DELETE 1":
                raise ParsecError("Update error.")

    async def declare_unconfigured_device(self, token: str, user_id: str, device_name: str):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                devices = await conn.fetch(
                    "SELECT device_name FROM devices WHERE user_id = $1", user_id
                )
                if not devices:
                    raise NotFoundError("User `%s` doesn't exists" % user_id)

                if device_name in itertools.chain(*devices):
                    raise AlreadyExistsError(
                        "Device `%s@%s` already exists" % (user_id, device_name)
                    )

                result = await conn.execute(
                    """
                    INSERT INTO unconfigured_devices (
                        user_id,
                        device_name,
                        created_on,
                        configure_token
                    ) VALUES (
                        $1,
                        $2,
                        $3,
                        $4
                    ) ON CONFLICT (user_id, device_name) DO UPDATE SET
                        created_on=EXCLUDED.created_on,
                        configure_token=EXCLUDED.configure_token
                    """,
                    user_id,
                    device_name,
                    pendulum.now(),
                    token,
                )
                if result != "INSERT 0 1":
                    raise ParsecError("Insertion error.")

    async def get_unconfigured_device(self, user_id: str, device_name: str):
        async with self.dbh.pool.acquire() as conn:
            row = await conn.fetchrow(
                """
                SELECT
                    created_on,
                    configure_token
                FROM unconfigured_devices WHERE
                    user_id = $1 AND device_name = $2
                """,
                user_id,
                device_name,
            )

            if not row:
                raise NotFoundError(f"No invitation for device `{user_id@device_name}`")

            return {"created_on": row[0], "configure_token": row[1]}

    async def register_device_configuration_try(
        self,
        config_try_id: str,
        user_id: str,
        device_name: str,
        device_verify_key: bytes,
        exchange_cipherkey: bytes,
        salt: bytes,
    ):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                result = await conn.execute(
                    """
                    INSERT INTO device_configuration_tries (
                        config_try_id,
                        status,
                        user_id,
                        device_name,
                        device_verify_key,
                        exchange_cipherkey,
                        salt
                    ) VALUES (
                        $1,
                        'waiting_answer',
                        $2,
                        $3,
                        $4,
                        $5,
                        $6
                    ) ON CONFLICT (user_id, config_try_id) DO UPDATE SET
                        status=EXCLUDED.status,
                        device_name=EXCLUDED.device_name,
                        device_verify_key=EXCLUDED.device_verify_key,
                        exchange_cipherkey=EXCLUDED.exchange_cipherkey,
                        salt=EXCLUDED.salt
                    """,
                    config_try_id,
                    user_id,
                    device_name,
                    device_verify_key,
                    exchange_cipherkey,
                    salt,
                )
                if result != "INSERT 0 1":
                    raise ParsecError("Insertion error.")

    async def retrieve_device_configuration_try(self, config_try_id: str, user_id: str):
        async with self.dbh.pool.acquire() as conn:
            row = await conn.fetchrow(
                """
                SELECT
                    status,
                    refused_reason,
                    device_name,
                    device_verify_key,
                    exchange_cipherkey,
                    salt,
                    ciphered_user_privkey,
                    ciphered_user_manifest_access
                FROM device_configuration_tries WHERE
                    user_id = $1 AND config_try_id = $2
                """,
                user_id,
                config_try_id,
            )

            if not row:
                raise NotFoundError()

            return {
                "status": row[0],
                "refused_reason": row[1],
                "device_name": row[2],
                "device_verify_key": row[3],
                "exchange_cipherkey": row[4],
                "salt": row[5],
                "ciphered_user_privkey": row[6],
                "ciphered_user_manifest_access": row[7],
            }

    async def accept_device_configuration_try(
        self,
        config_try_id: str,
        user_id: str,
        ciphered_user_privkey: bytes,
        ciphered_user_manifest_access: bytes,
    ):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                row = await conn.fetchrow(
                    """
                    SELECT
                        status, device_name, device_verify_key
                    FROM device_configuration_tries
                    WHERE user_id = $1 AND config_try_id = $2
                    """,
                    user_id,
                    config_try_id,
                )
                if not row:
                    raise NotFoundError()
                status, device_name, device_verify_key = row
                if status != "waiting_answer":
                    raise AlreadyExistsError("Device configuration try already done.")

                result = await conn.execute(
                    """
                    UPDATE device_configuration_tries SET
                        status = 'accepted',
                        ciphered_user_privkey = $1,
                        ciphered_user_manifest_access = $2
                    WHERE user_id = $3 AND config_try_id = $4
                    """,
                    ciphered_user_privkey,
                    ciphered_user_manifest_access,
                    user_id,
                    config_try_id,
                )
                if result != "UPDATE 1":
                    raise ParsecError("Update error.")

    async def refuse_device_configuration_try(self, config_try_id: str, user_id: str, reason: str):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                row = await conn.fetchrow(
                    """
                    SELECT status FROM device_configuration_tries WHERE user_id = $1 AND config_try_id = $2
                    """,
                    user_id,
                    config_try_id,
                )
                if not row:
                    raise NotFoundError()
                if row[0] != "waiting_answer":
                    raise AlreadyExistsError("Device configuration try already done.")

                result = await conn.execute(
                    """
                    UPDATE device_configuration_tries SET
                        status = 'refused',
                        refused_reason = $1
                    WHERE user_id = $2 AND config_try_id = $3
                    """,
                    reason,
                    user_id,
                    config_try_id,
                )
                if result != "UPDATE 1":
                    raise ParsecError("Update error.")

    async def _create_user(self, conn, created_on, user_id, broadcast_key, devices):
        assert isinstance(broadcast_key, (bytes, bytearray))

        if isinstance(devices, dict):
            devices = list(devices.items())

        for _, key in devices:
            assert isinstance(key, (bytes, bytearray))

        try:
            result = await conn.execute(
                """
                INSERT INTO users (user_id, created_on, broadcast_key)
                VALUES ($1, $2, $3)
                """,
                user_id,
                created_on,
                broadcast_key,
            )
        except UniqueViolationError as exc:
            raise AlreadyExistsError("User `%s` already exists" % user_id)

        if result != "INSERT 0 1":
            raise ParsecError("Insertion error.")

        await conn.executemany(
            """INSERT INTO devices (
                device_id, user_id, device_name, created_on, verify_key
            )
            VALUES ($1, $2, $3, $4, $5)
            """,
            [(f"{user_id}@{name}", user_id, name, created_on, key) for name, key in devices],
        )

    async def _create_device(self, conn, user_id, device_name, created_on, verify_key):
        devices = await conn.fetch("SELECT device_name FROM devices WHERE user_id = $1", user_id)
        if not devices:
            raise NotFoundError("User `%s` doesn't exists" % user_id)

        if device_name in itertools.chain(*devices):
            raise AlreadyExistsError("Device `%s@%s` already exists" % (user_id, device_name))

        result = await conn.execute(
            "INSERT INTO devices (device_id, user_id, device_name, created_on, verify_key) VALUES ($1, $2, $3, $4, $5)",
            f"{user_id}@{device_name}",
            user_id,
            device_name,
            pendulum.now(),
            verify_key,
        )
        if result != "INSERT 0 1":
            raise ParsecError("Insertion error.")

    async def create(self, user_id: str, broadcast_key: bytes, devices: List[str]):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                await self._create_user(conn, pendulum.now(), user_id, broadcast_key, devices)

    async def create_device(self, user_id: str, device_name: str, verify_key: bytes):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                await self._create_device(conn, user_id, device_name, pendulum.now(), verify_key)

    async def get(self, user_id: str):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                results = await conn.fetchrow(
                    "SELECT created_on, broadcast_key FROM users WHERE user_id = $1", user_id
                )
                if results:
                    created_on, broadcast_key = results
                else:
                    raise NotFoundError(user_id)

                user = {
                    "user_id": user_id,
                    "broadcast_key": broadcast_key,
                    "created_on": pendulum.instance(created_on),
                    "devices": {},
                }
                user_devices = await conn.fetch(
                    """
                    SELECT device_name, created_on, verify_key, revocated_on
                    FROM devices WHERE user_id = $1
                    """,
                    user_id,
                )
                for (d_name, d_created_on, d_verify_key, d_revocated_on) in user_devices:
                    user["devices"][d_name] = {
                        "created_on": pendulum.instance(d_created_on),
                        "verify_key": d_verify_key if d_verify_key else None,
                        "revocated_on": (
                            pendulum.instance(d_revocated_on) if d_revocated_on else None
                        ),
                    }

        return user

    async def revoke_user(self, user_id: str):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                results = await conn.fetchrow(
                    "SELECT user_id FROM users WHERE user_id = $1", user_id
                )
                if not results:
                    raise NotFoundError("User `%s` doesn't exists" % user_id)
                result = await conn.execute(
                    """
                    UPDATE devices SET revocated_on = $1
                    WHERE user_id = $2 AND revocated_on IS NULL
                    """,
                    pendulum.now(),
                    user_id,
                )
                if result == "UPDATE 0":
                    raise AlreadyRevokedError("User `%s` already revoked" % user_id)

    async def revoke_device(self, user_id: str, device_name: str):
        device_id = f"{user_id}@{device_name}"
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                results = await conn.fetchrow(
                    "SELECT device_id FROM devices WHERE device_id = $1", device_id
                )
                if not results:
                    raise NotFoundError("Device `%s` doesn't exists" % device_id)
                result = await conn.execute(
                    """
                    UPDATE devices SET revocated_on = $1
                    WHERE device_id = $2 AND revocated_on IS NULL
                    """,
                    pendulum.now(),
                    device_id,
                )
                if result == "UPDATE 0":
                    raise AlreadyRevokedError("Device `%s` already revoked" % device_id)

    async def find(self, query: str = None, page: int = 0, per_page: int = 100):
        async with self.dbh.pool.acquire() as conn:
            if query:
                all_results = await conn.fetch(
                    "SELECT user_id FROM users WHERE user_id LIKE $1 ORDER BY user_id", f"{query}%"
                )
            else:
                all_results = await conn.fetch("SELECT user_id FROM users ORDER BY user_id")
            # TODO: should use LIMIT and OFFSET in the SQL query instead
            results = [x[0] for x in all_results[(page - 1) * per_page : page * per_page]]
            return results, len(all_results)
