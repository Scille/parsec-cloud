import attr
import random
import string
import pendulum

from parsec import schema_fields as fields
from parsec.utils import UnknownCheckedSchema


class UserError(Exception):
    pass


class UserNotFound(UserError):
    pass


class UserAlreadyExists(UserError):
    pass


class UserClaimError(UserError):
    pass


class UserIDSchema(UnknownCheckedSchema):
    id = fields.String(required=True)


# TODO: currently it's hard to test created_on date, so it has been disable...
class DeviceSchema(UnknownCheckedSchema):
    # created_on = fields.DateTime(required=True)
    # revocated_on = fields.DateTime()
    verify_key = fields.Base64Bytes(required=True)


class UserSchema(UnknownCheckedSchema):
    id = fields.String(required=True)
    # created_on = fields.DateTime()
    # created_by = fields.String()
    broadcast_key = fields.Base64Bytes()
    devices = fields.Map(
        fields.String(),
        fields.Nested(DeviceSchema)
    )


class UserClaimSchema(UnknownCheckedSchema):
    id = fields.String(required=True)
    token = fields.String(required=True)
    broadcast_key = fields.Base64Bytes(required=True)
    device_name = fields.String(required=True)
    device_verify_key = fields.Base64Bytes(required=True)


def _generate_invitation_token():
    return ''.join([random.choice(string.digits) for _ in range(6)])


class BaseUserComponent:

    async def api_user_get(self, client_ctx, msg):
        msg = UserIDSchema().load(msg)
        try:
            user = await self.get(msg['id'])
        except UserNotFound:
            return {
                'status': 'not_found',
                'reason': 'No user with id `%s`' % msg['id'],
            }
        data, errors = UserSchema().dump(user)
        if errors:
            raise RuntimeError('Dump error with %r: %s' % (user, errors))
        return {'status': 'ok', **data}

    async def api_user_create(self, client_ctx, msg):
        msg = UserIDSchema().load(msg)
        token = _generate_invitation_token()
        try:
            await self.create_invitation(client_ctx.id, msg['id'], token)
        except UserAlreadyExists:
            return {
                'status': 'already_exists',
                'reason': 'User `%s` already exists' % msg['id'],
            }
        return {'status': 'ok', 'id': msg['id'], 'token': token}

    async def api_user_claim(self, client_ctx, msg):
        msg = UserClaimSchema().load(msg)
        try:
            await self.claim_invitation(**msg)
        except UserClaimError as exc:
            return {'status': 'claim_error', 'reason': str(exc)}
        return {'status': 'ok'}

    async def claim_invitation(self, id, token, broadcast_key, device_name, device_verify_key):
        raise NotImplementedError()

    async def create_invitation(self, author, id, token):
        raise NotImplementedError()

    async def create(self, author, id, broadcast_key, devices):
        raise NotImplementedError()

    async def get(self, id):
        raise NotImplementedError()


@attr.s
class MockedUserComponent(BaseUserComponent):
    _users = attr.ib(default=attr.Factory(dict))
    _invitations = attr.ib(default=attr.Factory(dict))

    async def claim_invitation(self, id, token, broadcast_key, device_name, device_verify_key):
        assert isinstance(broadcast_key, (bytes, bytearray))
        assert isinstance(device_verify_key, (bytes, bytearray))

        invitation = self._invitations.get(id)
        try:
            if not invitation:
                raise UserClaimError('No invitation for user `%s`' % id)
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
            raise UserAlreadyExists('User `%s` already exists' % id)
        # Overwrite previous invitation if any
        self._invitations[id] = {
            'date': pendulum.utcnow(),
            'author': author,
            'token': token,
            'claim_tries': 0,
        }

    async def create(self, author, id, broadcast_key, devices):
        assert isinstance(broadcast_key, (bytes, bytearray))
        for _, key in devices:
            assert isinstance(key, (bytes, bytearray))

        if id in self._users:
            raise UserAlreadyExists('User `%s` already exists' % id)

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
            raise UserNotFound(id)
