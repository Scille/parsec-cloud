import itertools
import pendulum

from parsec.utils import ParsecError
from parsec.backend.user import BaseUserComponent
from parsec.backend.exceptions import (
    AlreadyExistsError,
    NotFoundError,
    UserClaimError,
    OutOfDateError,
)


class PGUserComponent(BaseUserComponent):
    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    async def claim_invitation(
        self, user_id, invitation_token, broadcast_key, device_name, device_verify_key
    ):
        assert isinstance(broadcast_key, (bytes, bytearray))
        assert isinstance(device_verify_key, (bytes, bytearray))

        error = None
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                results = await conn.fetchrow(
                    """SELECT ts, author, invitation_token, claim_tries
                    FROM invitations WHERE user_id = $1
                    """,
                    user_id,
                )
                if results:
                    ts, author, retrieved_invitation_token, claim_tries = results
                else:
                    raise NotFoundError("No invitation for user `%s`" % user_id)

                ts = pendulum.from_timestamp(ts)
                user = await conn.fetchrow("SELECT 1 FROM users WHERE user_id = $1", user_id)

                if user or retrieved_invitation_token != invitation_token:
                    claim_tries = claim_tries + 1

                    if claim_tries > 3:
                        result = await conn.execute(
                            "DELETE FROM invitations WHERE user_id = $1", user_id
                        )
                        if result != "DELETE 1":
                            raise ParsecError("Deletion error.")

                    else:
                        result = await conn.execute(
                            "UPDATE invitations SET claim_tries = $1 WHERE user_id = $2",
                            claim_tries,
                            user_id,
                        )
                        if result != "UPDATE 1":
                            raise ParsecError("Update error.")

                    if user:
                        error = "User `%s` has already been registered" % user_id
                    else:
                        error = "Invalid invitation token"

                else:
                    now = pendulum.now()

                    if (now - ts) > pendulum.interval(hours=1):
                        raise OutOfDateError("Claim code is too old.")

                if not error:
                    await self.create(
                        author, user_id, broadcast_key, devices=[(device_name, device_verify_key)]
                    )
        if error:
            raise UserClaimError(error)

    async def create_invitation(self, invitation_token, author, user_id):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                user = await conn.fetchrow("SELECT 1 FROM users WHERE user_id = $1", user_id)

                if user:
                    raise AlreadyExistsError("User `%s` already exists" % user_id)

                # Overwrite previous invitation if any
                result = await conn.execute(
                    """
                    INSERT INTO invitations (
                        user_id, ts, author, invitation_token, claim_tries
                    ) VALUES ($1, $2, $3, $4, 0)
                    ON CONFLICT (user_id) DO UPDATE SET
                        ts=EXCLUDED.ts,
                        author=EXCLUDED.author,
                        invitation_token=EXCLUDED.invitation_token,
                        claim_tries=EXCLUDED.claim_tries
                    """,
                    user_id,
                    pendulum.now().int_timestamp,
                    author,
                    invitation_token,
                )
                if result != "INSERT 0 1":
                    raise ParsecError("Insertion error.")

    async def create(self, author, user_id, broadcast_key, devices):
        assert isinstance(broadcast_key, (bytes, bytearray))

        if isinstance(devices, dict):
            devices = list(devices.items())

        for _, key in devices:
            assert isinstance(key, (bytes, bytearray))

        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                user = await conn.fetchrow("SELECT 1 FROM users WHERE user_id = $1", user_id)

                if user:
                    raise AlreadyExistsError("User `%s` already exists" % user_id)

                now = pendulum.now().int_timestamp

                result = await conn.execute(
                    """INSERT INTO users (user_id, created_on, created_by, broadcast_key)
                    VALUES ($1, $2, $3, $4)
                    """,
                    user_id,
                    now,
                    author,
                    broadcast_key,
                )
                if result != "INSERT 0 1":
                    raise ParsecError("Insertion error.")
                await conn.executemany(
                    """INSERT INTO user_devices (user_id, device_name, created_on, verify_key, revocated_on)
                    VALUES ($1, $2, $3, $4, NULL)
                    """,
                    [(user_id, name, now, key) for name, key in devices],
                )

    async def get(self, user_id):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                results = await conn.fetchrow(
                    "SELECT created_on, created_by, broadcast_key FROM users WHERE user_id = $1",
                    user_id,
                )
                if results:
                    created_on, created_by, broadcast_key = results
                else:
                    raise NotFoundError(user_id)

                user = {
                    "user_id": user_id,
                    "broadcast_key": broadcast_key,
                    "created_by": created_by,
                    "created_on": pendulum.from_timestamp(created_on),
                    "devices": {},
                }
                user_devices = await conn.fetch(
                    """
                    SELECT device_name, created_on, configure_token, verify_key, revocated_on
                    FROM user_devices WHERE user_id = $1
                    """,
                    user_id,
                )
                for (
                    d_name,
                    d_created_on,
                    d_configure_token,
                    d_verify_key,
                    d_revocated_on,
                ) in user_devices:
                    user["devices"][d_name] = {
                        "created_on": pendulum.from_timestamp(d_created_on),
                        "configure_token": d_configure_token,
                        "verify_key": d_verify_key if d_verify_key else None,
                        "revocated_on": (
                            pendulum.from_timestamp(d_revocated_on) if d_revocated_on else None
                        ),
                    }

        return user

    async def create_device(self, user_id, device_name, verify_key):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                devices = await conn.fetch(
                    "SELECT device_name FROM user_devices WHERE user_id = $1", user_id
                )
                if not devices:
                    raise NotFoundError("User `%s` doesn't exists" % user_id)

                if device_name in itertools.chain(*devices):
                    raise AlreadyExistsError(
                        "Device `%s@%s` already exists" % (user_id, device_name)
                    )

                result = await conn.execute(
                    "INSERT INTO user_devices (user_id, device_name, created_on, verify_key) VALUES ($1, $2, $3, $4)",
                    user_id,
                    device_name,
                    pendulum.now().int_timestamp,
                    verify_key,
                )
                if result != "INSERT 0 1":
                    raise ParsecError("Insertion error.")

    async def configure_device(self, user_id, device_name, device_verify_key):
        async with self.dbh.pool.acquire() as conn:
            updated = await conn.execute(
                "UPDATE user_devices SET verify_key = $1 WHERE user_id=$2 AND device_name=$3",
                # 'SELECT device_name FROM user_devices WHERE user_id=$1 AND device_name=$2',
                device_verify_key,
                user_id,
                device_name,
            )
            if updated == "UPDATE 0":
                raise NotFoundError("User `%s` doesn't exists" % user_id)

    # TODO
    # raise NotFoundError("Device `%s@%s` doesn't exists" % (user_id, device_name))

    async def declare_unconfigured_device(self, token, user_id, device_name):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                devices = await conn.fetch(
                    "SELECT device_name FROM user_devices WHERE user_id = $1", user_id
                )
                if not devices:
                    raise NotFoundError("User `%s` doesn't exists" % user_id)

                if device_name in itertools.chain(*devices):
                    raise AlreadyExistsError(
                        "Device `%s@%s` already exists" % (user_id, device_name)
                    )

                result = await conn.execute(
                    """
                    INSERT INTO user_devices (
                        user_id, device_name, created_on, configure_token
                    ) VALUES ($1, $2, $3, $4)""",
                    user_id,
                    device_name,
                    pendulum.now().int_timestamp,
                    token,
                )
                if result != "INSERT 0 1":
                    raise ParsecError("Insertion error.")

    async def register_device_configuration_try(
        self, config_try_id, user_id, device_name, device_verify_key, user_privkey_cypherkey
    ):
        async with self.dbh.pool.acquire() as conn:
            # TODO: handle multiple configuration tries on a given device
            result = await conn.execute(
                """
                INSERT INTO device_configure_tries (
                    user_id, config_try_id, status, device_name, device_verify_key,
                    user_privkey_cypherkey
                ) VALUES ($1, $2, $3, $4, $5, $6)
                """,
                user_id,
                config_try_id,
                "waiting_answer",
                device_name,
                device_verify_key,
                user_privkey_cypherkey,
            )
            if result != "INSERT 0 1":
                raise ParsecError("Insertion error.")
        return config_try_id

    async def retrieve_device_configuration_try(self, config_try_id, user_id):
        async with self.dbh.pool.acquire() as conn:
            config_try = await conn.fetchrow(
                """
                SELECT status, device_name, device_verify_key, user_privkey_cypherkey,
                    cyphered_user_privkey, refused_reason
                FROM device_configure_tries WHERE user_id = $1 AND config_try_id = $2
                """,
                user_id,
                config_try_id,
            )
            if not config_try:
                raise NotFoundError()

        return {
            "status": config_try[0],
            "device_name": config_try[1],
            "device_verify_key": config_try[2],
            "user_privkey_cypherkey": config_try[3],
            "cyphered_user_privkey": config_try[4],
            "refused_reason": config_try[5],
        }

        return config_try

    async def accept_device_configuration_try(self, config_try_id, user_id, cyphered_user_privkey):
        async with self.dbh.pool.acquire() as conn:
            updated = await conn.execute(
                """
                UPDATE device_configure_tries SET status = $1, cyphered_user_privkey = $2
                WHERE user_id=$3 AND config_try_id=$4 and status=$5
                """,
                "accepted",
                cyphered_user_privkey,
                user_id,
                config_try_id,
                "waiting_answer",
            )
            if updated == "UPDATE 0":
                raise NotFoundError()

    # TODO: handle this error
    # if config_try['status'] != 'waiting_answer':
    #     raise AlreadyExistsError('Device configuration try already done.')

    async def refuse_device_configuration_try(self, config_try_id, user_id, reason):
        async with self.dbh.pool.acquire() as conn:
            updated = await conn.execute(
                """
                UPDATE device_configure_tries SET status = $1, refused_reason = $2
                WHERE user_id=$3 AND config_try_id=$4 and status=$5
                """,
                "refused",
                reason,
                user_id,
                config_try_id,
                "waiting_answer",
            )
            if updated == "UPDATE 0":
                raise NotFoundError()


# TODO: handle this error
# if config_try['status'] != 'waiting_answer':
#     raise AlreadyExistsError('Device configuration try already done.')
