import attr
import trio
from marshmallow import fields
import nacl.utils
from nacl.public import PublicKey
from nacl.signing import VerifyKey
from nacl.exceptions import BadSignatureError
from urllib.parse import urlparse

from parsec.utils import (CookedSocket, BaseCmdSchema,
                          ParsecError, to_jsonb64, from_jsonb64)
from parsec.handshake import HandshakeFormatError, ServerHandshake
from parsec.backend.user import MockedUserComponent, UserNotFound
from parsec.backend.pubkey import MockedPubKeyComponent
from parsec.backend.vlob import MockedVlobComponent
from parsec.backend.user_vlob import MockedUserVlobComponent
from parsec.backend.group import MockedGroupComponent
from parsec.backend.message import InMemoryMessageComponent
from parsec.backend.blockstore import MockedBlockStoreComponent


class cmd_LOGIN_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    password = fields.String(missing=None)


class cmd_PING_Schema(BaseCmdSchema):
    ping = fields.String(required=True)


@attr.s
class AnonymousClientContext:
    id = 'anonymous'
    anonymous = True


@attr.s
class ClientContext:
    anonymous = False
    id = attr.ib()
    broadcast_key = attr.ib()
    verify_key = attr.ib()

    @property
    def user_id(self):
        return self.id.split('@')[0]

    @property
    def device_id(self):
        return self.id.split('@')[1]


class BackendApp:

    def __init__(self, config):
        self.config = config
        self.nursery = None
        self.anonymous_verifykey = VerifyKey(config['ANONYMOUS_VERIFY_KEY'])
        self.blockstore_url = config['BLOCKSTORE_URL']
        # TODO: validate BLOCKSTORE_URL value
        if self.blockstore_url == 'backend://':
            self.blockstore = MockedBlockStoreComponent()
        else:
            self.blockstore = None
        self.user = MockedUserComponent()
        self.vlob = MockedVlobComponent()
        self.user_vlob = MockedUserVlobComponent()
        self.pubkey = MockedPubKeyComponent()
        self.message = InMemoryMessageComponent()
        self.group = MockedGroupComponent()

        self.anonymous_cmds = {
            'user_claim': self.user.api_user_claim,
            'ping': self._api_ping
        }

        self.cmds = {
            # 'subscribe_event': self.api_subscribe_event,
            # 'unsubscribe_event': self.api_unsubscribe_event,

            'user_get': self.user.api_user_get,
            'user_create': self.user.api_user_create,

            'blockstore_post': self._api_blockstore_post,
            'blockstore_get': self._api_blockstore_get,
            'blockstore_get_url': self._api_blockstore_get_url,

            'vlob_create': self.vlob.api_vlob_create,
            'vlob_read': self.vlob.api_vlob_read,
            'vlob_update': self.vlob.api_vlob_update,

            'user_vlob_read': self.user_vlob.api_user_vlob_read,
            'user_vlob_update': self.user_vlob.api_user_vlob_update,

            'group_read': self.group.api_group_read,
            'group_create': self.group.api_group_create,
            'group_add_identities': self.group.api_group_add_identities,
            'group_remove_identities': self.group.api_group_remove_identities,

            'message_get': self.message.api_message_get,
            'message_new': self.message.api_message_new,

            'pubkey_get': self.pubkey.api_pubkey_get,

            'ping': self._api_ping
        }

    async def init(self):
        pass

    async def _api_ping(self, client_ctx, msg):
        msg = cmd_PING_Schema().load(msg)
        return {'status': 'ok', 'pong': msg['ping']}

    async def _api_blockstore_post(self, client_ctx, msg):
        if not self.blockstore:
            return {'status': 'not_available'}
        return await self.blockstore.api_blockstore_post(client_ctx, msg)

    async def _api_blockstore_get(self, client_ctx, msg):
        if not self.blockstore:
            return {'status': 'not_available'}
        return await self.blockstore.api_blockstore_get(client_ctx, msg)

    async def _api_blockstore_get_url(self, client_ctx, msg):
        return {'status': 'ok', 'url': self.blockstore_url}

    async def _do_handshake(self, sock):
        context = None
        try:
            hs = ServerHandshake(self.config.get('HANDSHAKE_CHALLENGE_SIZE', 48))
            challenge_req = hs.build_challenge_req()
            await sock.send(challenge_req)
            answer_req = await sock.recv()

            hs.process_answer_req(answer_req)
            if hs.identity == 'anonymous':
                context = AnonymousClientContext()
                result_req = hs.build_result_req(self.anonymous_verifykey)
            else:
                try:
                    userid, deviceid = hs.identity.split('@')
                except ValueError:
                    raise HandshakeFormatError()
                try:
                    user = await self.user.get(userid)
                    device = user['devices'][deviceid]
                except (UserNotFound, KeyError):
                    result_req = hs.build_bad_identity_result_req()
                else:
                    broadcast_key = PublicKey(user['broadcast_key'])
                    verify_key = VerifyKey(device['verify_key'])
                    context = ClientContext(hs.identity, broadcast_key, verify_key)
                    result_req = hs.build_result_req(verify_key)

        except HandshakeFormatError:
            result_req = hs.build_bad_format_result_req()
        await sock.send(result_req)
        return context

    async def handle_client(self, sockstream):
        sock = CookedSocket(sockstream)
        try:
            print('START HANDSHAKE')
            client_ctx = await self._do_handshake(sock)
            if not client_ctx:
                # Invalid handshake
                print('BAD HANDSHAKE')
                return
            print('HANDSHAKE DONE, CLIENT IS `%s`' % client_ctx.id)
            while True:
                req = await sock.recv()
                if not req:  # Client disconnected
                    print('CLIENT DISCONNECTED')
                    break
                print('REQ %s' % req)
                # TODO: handle bad msg
                try:
                    cmd = req.pop('cmd', '<missing>')
                    if client_ctx.anonymous:
                        cmd_func = self.anonymous_cmds[cmd]
                    else:
                        cmd_func = self.cmds[cmd]
                except KeyError:
                    rep = {'status': 'bad_cmd', 'reason': 'Unknown command `%s`' % cmd}
                else:
                    try:
                        rep = await cmd_func(client_ctx, req)
                    except ParsecError as err:
                        rep = err.to_dict()
                print('REP %s' % rep)
                await sock.send(rep)
        except trio.BrokenStreamError:
            # Client has closed connection
            pass
