from parsec.backend.user import BaseUserComponent
import pendulum

from parsec.backend.exceptions import (
    AlreadyExistsError,
    NotFoundError,
    UserClaimError
)


class MemoryUserComponent(BaseUserComponent):
    def __init__(self, *args):
        super().__init__(*args)
        self._users = {}
        self._invitations = {}

    async def claim_invitation(self, id, token, broadcast_key, device_name, device_verify_key):
        assert isinstance(broadcast_key, (bytes, bytearray))
        assert isinstance(device_verify_key, (bytes, bytearray))

        invitation = self._invitations.get(id)
        if not invitation:
            raise UserClaimError('No invitation for user `%s`' % id)
        try:
            if id in self._users:
                raise UserClaimError('User `%s` has already been registered' % id)
            now = pendulum.utcnow()
            if (now - invitation['date']) > pendulum.interval(hours=1):
                raise UserClaimError('Claim code is too old')
            if invitation['token'] != token:
                raise UserClaimError('Invalid token')
        except UserClaimError:
            invitation['claim_tries'] += 1
            if invitation['claim_tries'] > 3:
                del self._invitations[id]
            raise

        await self.create(
            invitation['author'],
            id,
            broadcast_key,
            devices=[(device_name, device_verify_key)]
        )

    async def create_invitation(self, author, id, token):
        if id in self._users:
            raise AlreadyExistsError('User `%s` already exists' % id)
        # Overwrite previous invitation if any
        self._invitations[id] = {
            'date': pendulum.utcnow(),
            'author': author,
            'token': token,
            'claim_tries': 0,
        }

    async def create(self, author, id, broadcast_key, devices):
        assert isinstance(broadcast_key, (bytes, bytearray))

        if isinstance(devices, dict):
            devices = list(devices.items())

        for _, key in devices:
            assert isinstance(key, (bytes, bytearray))

        if id in self._users:
            raise AlreadyExistsError('User `%s` already exists' % id)

        now = pendulum.utcnow()
        self._users[id] = {
            'id': id,
            'created_on': now,
            'created_by': author,
            'broadcast_key': broadcast_key,
            'devices': {
                name: {'created_on': now, 'verify_key': key, 'revocated_on': None}
                for name, key in devices
            },
        }

    async def get(self, id):
        try:
            return self._users[id]
        except KeyError:
            raise NotFoundError(id)
