import itertools
import pendulum

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

        try:
            ts, author, invitation_token, claim_tries = await self.dbh.fetch_one(
                """SELECT ts, author, invitation_token, claim_tries
                FROM invitations WHERE user_id = %s
                """,
                (user_id,),
            )
        except (TypeError, ValueError):
            raise NotFoundError("No invitation for user `%s`" % user_id)

        ts = pendulum.from_timestamp(ts)
        user = await self.dbh.fetch_one("SELECT 1 FROM users WHERE user_id = %s", (user_id,))

        try:
            if user is not None:
                raise UserClaimError("User `%s` has already been registered" % user_id)

            now = pendulum.now()

            if (now - ts) > pendulum.duration(hours=1):
                raise OutOfDateError("Claim code is too old.")

            if invitation_token != invitation_token:
                raise UserClaimError("Invalid invitation token")

        except UserClaimError:
            claim_tries = claim_tries + 1

            if claim_tries > 3:
                await self.dbh.delete_one("DELETE FROM invitations WHERE user_id = %s", (user_id,))

            else:
                await self.dbh.update_one(
                    "UPDATE invitations SET claim_tries = %s WHERE user_id = %s",
                    (claim_tries, user_id),
                )

            raise

        await self.create(
            author, user_id, broadcast_key, devices=[(device_name, device_verify_key)]
        )

    async def create_invitation(self, invitation_token, author, user_id):
        user = await self.dbh.fetch_one("SELECT 1 FROM users WHERE user_id = %s", (user_id,))

        if user is not None:
            raise AlreadyExistsError("User `%s` already exists" % user_id)

        # Overwrite previous invitation if any
        await self.dbh.insert_one(
            """
            INSERT INTO invitations (
                user_id, ts, author, invitation_token, claim_tries
            ) VALUES (%s, %s, %s, %s, 0)
            ON CONFLICT (user_id) DO UPDATE SET
                ts=EXCLUDED.ts,
                author=EXCLUDED.author,
                invitation_token=EXCLUDED.invitation_token,
                claim_tries=EXCLUDED.claim_tries
            """,
            (user_id, pendulum.now().int_timestamp, author, invitation_token),
        )

    async def create(self, author, user_id, broadcast_key, devices):
        assert isinstance(broadcast_key, (bytes, bytearray))

        if isinstance(devices, dict):
            devices = list(devices.items())

        for _, key in devices:
            assert isinstance(key, (bytes, bytearray))

        user = await self.dbh.fetch_one("SELECT 1 FROM users WHERE user_id = %s", (user_id,))

        if user is not None:
            raise AlreadyExistsError("User `%s` already exists" % user_id)

        now = pendulum.now().int_timestamp

        await self.dbh.insert_one(
            """INSERT INTO users (user_id, created_on, created_by, broadcast_key)
            VALUES (%s, %s, %s, %s)
            """,
            (user_id, now, author, broadcast_key),
        )

        await self.dbh.insert_many(
            """INSERT INTO user_devices (user_id, device_name, created_on, verify_key, revocated_on)
            VALUES (%s, %s, %s, %s, NULL)
            """,
            [(user_id, name, now, key) for name, key in devices],
        )

    async def get(self, user_id):
        try:
            created_on, created_by, broadcast_key = await self.dbh.fetch_one(
                "SELECT created_on, created_by, broadcast_key FROM users WHERE user_id = %s",
                (user_id,),
            )
        except (TypeError, ValueError):
            raise NotFoundError(user_id)

        user = {
            "user_id": user_id,
            "broadcast_key": broadcast_key.tobytes(),
            "created_by": created_by,
            "created_on": pendulum.from_timestamp(created_on),
            "devices": {
                d_name: {
                    "created_on": pendulum.from_timestamp(d_created_on),
                    "configure_token": d_configure_token,
                    "verify_key": d_verify_key.tobytes() if d_verify_key else None,
                    "revocated_on": (
                        pendulum.from_timestamp(d_revocated_on) if d_revocated_on else None
                    ),
                }
                for d_name, d_created_on, d_configure_token, d_verify_key, d_revocated_on in await self.dbh.fetch_many(
                    """
                    SELECT device_name, created_on, configure_token, verify_key, revocated_on
                    FROM user_devices WHERE user_id = %s
                    """,
                    (user_id,),
                )
            },
        }

        return user

    async def create_device(self, user_id, device_name, verify_key):
        devices = await self.dbh.fetch_many(
            "SELECT device_name FROM user_devices WHERE user_id = %s", (user_id,)
        )
        if not devices:
            raise NotFoundError("User `%s` doesn't exists" % user_id)

        if device_name in itertools.chain(*devices):
            raise AlreadyExistsError("Device `%s@%s` already exists" % (user_id, device_name))

        await self.dbh.insert_one(
            "INSERT INTO user_devices (user_id, device_name, created_on, verify_key) VALUES (%s, %s, %s, %s)",
            (user_id, device_name, pendulum.now().int_timestamp, verify_key),
        )

    async def configure_device(self, user_id, device_name, device_verify_key):
        updated = await self.dbh.update_one(
            "UPDATE user_devices SET verify_key = %s WHERE user_id=%s AND device_name=%s",
            # 'SELECT device_name FROM user_devices WHERE user_id=%s AND device_name=%s',
            (device_verify_key, user_id, device_name),
        )
        if not updated:
            raise NotFoundError("User `%s` doesn't exists" % user_id)

    # TODO
    # raise NotFoundError("Device `%s@%s` doesn't exists" % (user_id, device_name))

    async def declare_unconfigured_device(self, token, user_id, device_name):
        devices = await self.dbh.fetch_many(
            "SELECT device_name FROM user_devices WHERE user_id = %s", (user_id,)
        )
        if not devices:
            raise NotFoundError("User `%s` doesn't exists" % user_id)

        if device_name in itertools.chain(*devices):
            raise AlreadyExistsError("Device `%s@%s` already exists" % (user_id, device_name))

        await self.dbh.insert_one(
            """
            INSERT INTO user_devices (
                user_id, device_name, created_on, configure_token
            ) VALUES (%s, %s, %s, %s)""",
            (user_id, device_name, pendulum.now().int_timestamp, token),
        )

    async def register_device_configuration_try(
        self, config_try_id, user_id, device_name, device_verify_key, user_privkey_cypherkey
    ):
        # TODO: handle multiple configuration tries on a given device
        await self.dbh.insert_one(
            """
            INSERT INTO device_configure_tries (
                user_id, config_try_id, status, device_name, device_verify_key,
                user_privkey_cypherkey
            ) VALUES (%s, %s, %s, %s, %s, %s)
            """,
            (
                user_id,
                config_try_id,
                "waiting_answer",
                device_name,
                device_verify_key,
                user_privkey_cypherkey,
            ),
        )
        return config_try_id

    async def retrieve_device_configuration_try(self, config_try_id, user_id):
        config_try = await self.dbh.fetch_one(
            """
            SELECT status, device_name, device_verify_key, user_privkey_cypherkey,
                cyphered_user_privkey, refused_reason
            FROM device_configure_tries WHERE user_id = %s AND config_try_id = %s
            """,
            (user_id, config_try_id),
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
        updated = await self.dbh.update_one(
            """
            UPDATE device_configure_tries SET status = %s, cyphered_user_privkey = %s
            WHERE user_id=%s AND config_try_id=%s and status=%s
            """,
            ("accepted", cyphered_user_privkey, user_id, config_try_id, "waiting_answer"),
        )
        if not updated:
            raise NotFoundError()

    # TODO: handle this error
    # if config_try['status'] != 'waiting_answer':
    #     raise AlreadyExistsError('Device configuration try already done.')

    async def refuse_device_configuration_try(self, config_try_id, user_id, reason):
        updated = await self.dbh.update_one(
            """
            UPDATE device_configure_tries SET status = %s, refused_reason = %s
            WHERE user_id=%s AND config_try_id=%s and status=%s,
            """,
            ("refused", reason, user_id, config_try_id, "waiting_answer"),
        )
        if not updated:
            raise NotFoundError()


# TODO: handle this error
# if config_try['status'] != 'waiting_answer':
#     raise AlreadyExistsError('Device configuration try already done.')
