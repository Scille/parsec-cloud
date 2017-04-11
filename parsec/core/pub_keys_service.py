import re
import json
import string
import random
from marshmallow import fields

from parsec.service import BaseService, cmd
from parsec.exceptions import ParsecError, HandshakeError
from parsec.tools import BaseCmdSchema, UnknownCheckedSchema
from parsec.session import AuthSession


class PubKeysError(ParsecError):
    pass


class PubKeysNotFound(PubKeysError):
    status = 'not_found'


class cmd_GET_PUB_KEY_Schema(BaseCmdSchema):
    identity = fields.String(required=True)


class cmd_ENCRYPT_Schema(BaseCmdSchema):
    recipient = fields.String(required=True)
    content = fields.String(required=True)


class cmd_VERIFY_Schema(BaseCmdSchema):
    content = fields.String(required=True)


class HandshakeAnswerSchema(UnknownCheckedSchema):
    handshake = fields.String(required=True, validate=lambda x: x == 'answer')
    identity = fields.String(required=True)
    answer = fields.String(required=True)


def _generate_challenge():
    # Use SystemRandom to get cryptographically secure seeds
    return ''.join(random.SystemRandom().choice(string.ascii_uppercase + string.digits)
                        for _ in range(12))


class BasePubKeysService(BaseService):

    # @cmd('get_pub_key')
    # async def _cmd_GET_PUB_KEY(self, session, msg):
    #     msg = cmd_GET_PUB_KEY_Schema().load(msg)
    #     user_key = await self.get_pub_key(msg['identity'])
    #     return {'status': 'ok', 'pub_key': user_key}

    @cmd('encrypt')
    async def _cmd_ENCRYPT(self, session, msg):
        msg = cmd_ENCRYPT_Schema().load(msg)
        cyphertext = await self.encrypt(msg['identity'], msg['content'])
        return {'status': 'ok', 'cyphertext': cyphertext}

    @cmd('verify')
    async def _cmd_VERIFY(self, session, msg):
        msg = cmd_VERIFY_Schema().load(msg)
        user_key = await self.get_pub_key(msg['identity'])
        return {'status': 'ok', 'pub_key': user_key}

    async def get_pub_key(self, identity):
        raise NotImplementedError()

    async def encrypt(self, identity, content):
        raise NotImplementedError()


class GNUPGPubKeysService(BasePubKeysService):

    # gnupg = service('GNUPGService')

    def __init__(self, homedir='~/.gnupg'):
        super().__init__()
        import gnupg
        self.gnupg = gnupg.GPG(homedir=homedir, use_agent=True)

    # async def get_pub_key(self, identity):
    #     key = await self.gnupg.get_pub_key(identity)
    #     if not key:
    #         raise PubKeysNotFound()

    async def encrypt(self, identity, content):
        key = await self.get_pub_key(identity)
        return await self.gnupg.encrypt(key, content)

    async def handshake(self, context):
        challenge = _generate_challenge()
        query = {'handshake': 'challenge', 'challenge': challenge}
        await context.send(json.dumps(query).encode())
        raw_resp = await context.recv()
        try:
            resp = json.loads(raw_resp.decode())
        except (TypeError, json.JSONDecodeError) as exc:
            raise HandshakeError('Invalid JSON format')
        resp, errors = HandshakeAnswerSchema().load(resp)
        if errors:
            raise HandshakeError('Invalid challenge response format: %s' % errors)
        claimed_identity = resp['identity']
        answer = resp['answer']
        # TODO: find a cleaner way to extract the clear text from the signature
        begin_pgp_message = '-----BEGIN PGP SIGNED MESSAGE-----'
        end_pgp_message = '-----BEGIN PGP SIGNATURE-----'
        claimed_challenge = re.match(
            r'^' + begin_pgp_message + '\nHash: .*?\n\n((?:.*)+\n?)\n' + end_pgp_message,
            answer, flags=re.MULTILINE).group(1)
        sign_result = self.gnupg.verify(answer.encode())
        if (challenge == claimed_challenge and sign_result.valid and
                sign_result.key_id == claimed_identity):
            return AuthSession(context, claimed_identity)
        else:
            raise HandshakeError('Invalid signature, challenge or identity')
