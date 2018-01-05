import random
import string

from parsec.schema import BaseCmdSchema, fields
from parsec.backend.exceptions import (
    NotFoundError,
    AlreadyExistsError,
    UserClaimError
)


class UserIDSchema(BaseCmdSchema):
    id = fields.String(required=True)


class DeviceSchema(BaseCmdSchema):
    created_on = fields.DateTime(required=True)
    revocated_on = fields.DateTime()
    verify_key = fields.Base64Bytes(required=True)


class UserSchema(BaseCmdSchema):
    id = fields.String(required=True)
    created_on = fields.DateTime()
    created_by = fields.String()
    broadcast_key = fields.Base64Bytes()
    devices = fields.Map(
        fields.String(),
        fields.Nested(DeviceSchema)
    )


class UserClaimSchema(BaseCmdSchema):
    id = fields.String(required=True)
    token = fields.String(required=True)
    broadcast_key = fields.Base64Bytes(required=True)
    device_name = fields.String(required=True)
    device_verify_key = fields.Base64Bytes(required=True)


def _generate_invitation_token():
    return ''.join([random.choice(string.digits) for _ in range(6)])


class BaseUserComponent:

    def __init__(self, signal_ns):
        self._signal_user_claimed = signal_ns.signal('user_claimed')

    async def api_user_get(self, client_ctx, msg):
        msg = UserIDSchema().load_or_abort(msg)
        try:
            user = await self.get(msg['id'])
        except NotFoundError:
            return {
                'status': 'not_found',
                'reason': 'No user with id `%s`' % msg['id'],
            }
        data, errors = UserSchema().dump(user)
        if errors:
            raise RuntimeError('Dump error with %r: %s' % (user, errors))
        return {'status': 'ok', **data}

    async def api_user_create(self, client_ctx, msg):
        msg = UserIDSchema().load_or_abort(msg)
        token = _generate_invitation_token()
        try:
            await self.create_invitation(client_ctx.id, msg['id'], token)
        except AlreadyExistsError:
            return {
                'status': 'already_exists',
                'reason': 'User `%s` already exists' % msg['id'],
            }
        return {'status': 'ok', 'id': msg['id'], 'token': token}

    async def api_user_claim(self, client_ctx, msg):
        msg = UserClaimSchema().load_or_abort(msg)
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

