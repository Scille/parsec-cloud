import attr
import trio
from marshmallow import fields
import nacl.utils
from nacl.public import PrivateKey
from nacl.signing import VerifyKey
from nacl.exceptions import BadSignatureError
from urllib.parse import urlparse

from parsec.utils import (CookedSocket, BaseCmdSchema,
                          ParsecError, to_jsonb64, from_jsonb64)
from parsec.handshake import HandshakeFormatError, ServerHandshake
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
class ClientContext:
    id = attr.ib()
    pubkey = attr.ib()
    signkey = attr.ib()


class BackendApp:

    def __init__(self, config):
        self.config = config
        self.nursery = None
        self.blockstore_url = config.blockstore_url
        # TODO: validate BLOCKSTORE_URL value
        if self.blockstore_url == 'backend://':
            self.blockstore = MockedBlockStoreComponent()
        else:
            self.blockstore = None
        self.vlob = MockedVlobComponent()
        self.user_vlob = MockedUserVlobComponent()
        self.pubkey = MockedPubKeyComponent()
        self.message = InMemoryMessageComponent()
        self.group = MockedGroupComponent()

        self.cmds = {
            # 'subscribe_event': self.api_subscribe_event,
            # 'unsubscribe_event': self.api_unsubscribe_event,

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
            hs = ServerHandshake(self.config.handshake_challenge_size)
            challenge_req = hs.build_challenge_req()
            await sock.send(challenge_req)

            answer_req = await sock.recv()
            hs.process_answer_req(answer_req)
            rawkeys = await self.pubkey.get(hs.identity)
            if not rawkeys:
                result_req = hs.build_bad_identity_result_req()
            else:
                result_req = hs.build_result_req(VerifyKey(rawkeys[1]))
                context = ClientContext(hs.identity, *rawkeys)
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
                cmd_func = self.cmds[req.pop('cmd')]
                try:
                    rep = await cmd_func(client_ctx, req)
                except ParsecError as err:
                    rep = err.to_dict()
                print('REP %s' % rep)
                await sock.send(rep)
        except trio.BrokenStreamError:
            # Client has closed connection
            pass
