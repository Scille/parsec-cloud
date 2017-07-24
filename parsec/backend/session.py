import attr
import random
import string
import json
from marshmallow import fields
from effect2 import do, Effect, TypeDispatcher
from cryptography.exceptions import InvalidSignature

from parsec.backend.pubkey import EPubkeyGet
from parsec.exceptions import PubKeyNotFound, HandshakeError
from parsec.tools import UnknownCheckedSchema, ejson_dumps, ejson_loads


@attr.s
class EGetAuthenticatedUser:
    pass


@attr.s
class EHandshakeSend:
    payload = attr.ib()


@attr.s
class EHandshakeRecv:
    pass


class HandshakeAnswerSchema(UnknownCheckedSchema):
    handshake = fields.String(required=True, validate=lambda x: x == 'answer')
    identity = fields.String(required=True)
    answer = fields.Base64Bytes(required=True)


def _generate_challenge():
    # Use SystemRandom to get cryptographically secure seeds
    return ''.join(random.SystemRandom().choice(string.ascii_uppercase + string.digits)
                   for _ in range(12))


@attr.s
class SessionComponent:
    id = attr.ib(default=None)

    @do
    def handshake(self):
        if self.id:
            raise HandshakeError('Handshake already done.')
        challenge = _generate_challenge()
        query = {'handshake': 'challenge', 'challenge': challenge}
        yield Effect(EHandshakeSend(ejson_dumps(query)))
        raw_resp = yield Effect(EHandshakeRecv())
        try:
            resp = ejson_loads(raw_resp)
        except (TypeError, json.JSONDecodeError):
            error = HandshakeError('Invalid challenge response format')
            yield Effect(EHandshakeSend(error.to_raw()))
            raise error
        resp = HandshakeAnswerSchema().load(resp)
        claimed_identity = resp['identity']
        try:
            pubkey = yield Effect(EPubkeyGet(claimed_identity))
            pubkey.verify(resp['answer'], challenge.encode())
            yield Effect(EHandshakeSend('{"status": "ok", "handshake": "done"}'))
            self.id = claimed_identity
        except (TypeError, PubKeyNotFound, InvalidSignature):
            error = HandshakeError('Invalid signature, challenge or identity')
            yield Effect(EHandshakeSend(error.to_raw()))
            raise error

    @do
    def perform_get_authenticated_user(self, intent):
        if not self.id:
            raise HandshakeError('Handshake not done, no authenticated user.')
        return self.id

    def get_dispatcher(self):
        return TypeDispatcher({
            EGetAuthenticatedUser: self.perform_get_authenticated_user,
        })


def handshake_io_dispatcher_factory(context):

    async def perform_handshake_send(intent):
        return await context.send(intent.payload)

    async def perform_handshake_recv(intent):
        return await context.recv()

    return TypeDispatcher({
        EHandshakeSend: perform_handshake_send,
        EHandshakeRecv: perform_handshake_recv
    })
