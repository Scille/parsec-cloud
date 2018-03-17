from parsec.backend.user import BaseUserComponent
import pendulum

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

    async def claim_invitation(self, user_id, invitation_token, broadcast_key, device_name, device_verify_key):
        assert isinstance(broadcast_key, (bytes, bytearray))
        assert isinstance(device_verify_key, (bytes, bytearray))

        try:
            ts, author, invitation_token, claim_tries = await self.dbh.fetch_one(
                'SELECT ts, author, invitation_token, claim_tries FROM invitations WHERE user_id = %s',
                (user_id,)
            )
        except (TypeError, ValueError):
            raise NotFoundError('No invitation for user `%s`' % user_id)

        ts = pendulum.from_timestamp(ts)
        user = await self.dbh.fetch_one(
            'SELECT 1 FROM users WHERE user_id = %s',
            (user_id,)
        )

        try:
            if user is not None:
                raise UserClaimError('User `%s` has already been registered' % user_id)

            now = pendulum.utcnow()

            if (now - ts) > pendulum.interval(hours=1):
                raise OutOfDateError('Claim code is too old.')

            if invitation_token != invitation_token:
                raise UserClaimError('Invalid invitation token')

        except UserClaimError:
            claim_tries = claim_tries + 1

            if claim_tries > 3:
                await self.dbh.delete_one(
                    'DELETE FROM invitations WHERE user_id = %s',
                    (user_id,)
                )

            else:
                await self.dbh.update_one(
                    'UPDATE invitations SET claim_tries = %s WHERE user_id = %s',
                    (claim_tries, user_id)
                )

            raise

        await self.create(
            author,
            user_id,
            broadcast_key,
            devices=[(device_name, device_verify_key)]
        )

    async def create_invitation(self, invitation_token, author, user_id):
        user = await self.dbh.fetch_one(
            'SELECT 1 FROM users WHERE user_id = %s',
            (user_id,)
        )

        if user is not None:
            raise AlreadyExistsError('User `%s` already exists' % user_id)

        # Overwrite previous invitation if any
        await self.dbh.insert_one("""
            INSERT INTO invitations (
                user_id, ts, author, invitation_token, claim_tries
            ) VALUES (%s, %s, %s, %s, 0)
            ON CONFLICT (user_id) DO UPDATE SET
                ts=EXCLUDED.ts,
                author=EXCLUDED.author,
                invitation_token=EXCLUDED.invitation_token,
                claim_tries=EXCLUDED.claim_tries
            """,
            (user_id, pendulum.utcnow().int_timestamp, author, invitation_token)
        )

    async def create(self, author, user_id, broadcast_key, devices):
        assert isinstance(broadcast_key, (bytes, bytearray))

        if isinstance(devices, dict):
            devices = list(devices.items())

        for _, key in devices:
            assert isinstance(key, (bytes, bytearray))

        user = await self.dbh.fetch_one(
            'SELECT 1 FROM users WHERE user_id = %s',
            (user_id,)
        )

        if user is not None:
            raise AlreadyExistsError('User `%s` already exists' % user_id)

        now = pendulum.utcnow().int_timestamp

        await self.dbh.insert_one(
            'INSERT INTO users (user_id, created_on, created_by, broadcast_key) VALUES (%s, %s, %s, %s)',
            (user_id, now, author, broadcast_key)
        )

        await self.dbh.insert_many(
            'INSERT INTO user_devices (user_id, device_name, created_on, verify_key, revocated_on) VALUES (%s, %s, %s, %s, NULL)',
            [
                (user_id, name, now, key)
                for name, key in devices
            ]
        )

    async def get(self, user_id):
        try:
            created_on, created_by, broadcast_key = await self.dbh.fetch_one(
                'SELECT created_on, created_by, broadcast_key FROM users WHERE user_id = %s',
                (user_id,)
            )
        except (TypeError, ValueError):
            raise NotFoundError(user_id)

        user = {
            'user_id': user_id,
            'broadcast_key': broadcast_key.tobytes(),
            'created_by': created_by,
            'created_on': pendulum.from_timestamp(created_on),
            'devices': {
                d_name: {
                    'created_on': pendulum.from_timestamp(d_created_on),
                    'verify_key': d_verify_key.tobytes(),
                    'revocated_on': (pendulum.from_timestamp(d_revocated_on)
                                     if d_revocated_on else None)
                }
                for d_name, d_created_on, d_verify_key, d_revocated_on in await self.dbh.fetch_many(
                    'SELECT device_name, created_on, verify_key, revocated_on FROM user_devices WHERE user_id = %s',
                    (user_id,)
                )
            }
        }

        return user
