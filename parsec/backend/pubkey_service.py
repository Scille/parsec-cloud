import random
import string
import json
from marshmallow import fields
from cryptography.exceptions import InvalidSignature

from parsec.service import BaseService, cmd
from parsec.session import AuthSession, ConnectionClosed
from parsec.exceptions import PubKeyError, PubKeyNotFound, HandshakeError
from parsec.tools import BaseCmdSchema, UnknownCheckedSchema, from_jsonb64
from parsec.crypto import load_public_key


class HandshakeAnswerSchema(UnknownCheckedSchema):
    handshake = fields.String(required=True, validate=lambda x: x == 'answer')
    identity = fields.String(required=True)
    answer = fields.String(required=True)


def _generate_challenge():
    # Use SystemRandom to get cryptographically secure seeds
    return ''.join(random.SystemRandom().choice(string.ascii_uppercase + string.digits)
                        for _ in range(12))


class cmd_PUBKEY_GET_Schema(BaseCmdSchema):
    id = fields.String(required=True)


class BasePubKeyService(BaseService):

    name = 'PubKeyService'

    @cmd('pubkey_get')
    async def _cmd_PUBKEY_GET(self, session, msg):
        msg = cmd_PUBKEY_GET_Schema().load(msg)
        key = await self.get_pubkey(msg['id'], raw=True)
        return {'status': 'ok', 'key': key}

    async def get_pubkey(self, identity):
        raise NotImplementedError()

    async def add_pubkey(self, identity):
        raise NotImplementedError()

    async def handshake(self, context):
        try:
            challenge = _generate_challenge()
            query = {'handshake': 'challenge', 'challenge': challenge}
            await context.send(json.dumps(query))
            raw_resp = await context.recv()
            try:
                resp = json.loads(raw_resp)
            except (TypeError, json.JSONDecodeError):
                raise HandshakeError('Invalid challenge response format')
            resp, errors = HandshakeAnswerSchema().load(resp)
            if errors:
                raise HandshakeError('Invalid challenge response format: %s' % errors)
            claimed_identity = resp['identity']
            try:
                answer = from_jsonb64(resp['answer'])
                pubkey = await self.get_pubkey(claimed_identity)
                pubkey.verify(answer, challenge.encode())
                await context.send('{"status": "ok", "handshake": "done"}')
                return AuthSession(context, claimed_identity)
            except (TypeError, PubKeyNotFound, InvalidSignature):
                error = HandshakeError('Invalid signature, challenge or identity')
                await context.send(error.to_raw())
        except ConnectionClosed:
            pass


class InMemoryPubKeyService(BasePubKeyService):
    def __init__(self):
        super().__init__()
        self._keys = {}

    async def add_pubkey(self, id, key: bytes):
        assert isinstance(key, (bytes, bytearray))
        if id in self._keys:
            raise PubKeyError('Identity `%s` already has a public key' % id)
        else:
            self._keys[id] = key

    async def get_pubkey(self, id, raw=False):
        try:
            key = self._keys[id]
            return key if raw else load_public_key(key)
        except KeyError:
            raise PubKeyNotFound('No public key for identity `%s`' % id)
