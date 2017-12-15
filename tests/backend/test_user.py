import pytest
from unittest.mock import patch
from trio.testing import open_stream_to_socket_listener
from freezegun import freeze_time
from nacl.signing import SigningKey
from nacl.public import PrivateKey

from parsec.utils import to_jsonb64, CookedSocket
from parsec.handshake import ClientHandshake

from tests.common import with_backend


@pytest.mark.trio
@with_backend()
async def test_user_get_ok(backend):
    async with backend.test_connect('alice@test') as sock:
        with freeze_time('2017-07-07'):
            await sock.send({
                'cmd': 'user_get',
                'id': 'bob'
            })
            rep = await sock.recv()
    assert rep == {
        'status': 'ok',
        'id': 'bob',
        'broadcast_key': 'PJyDuQC6ZXlhLqnwP9cwLZE9ye18fydKmzAgT/dvS04=\n',
        'devices': {
            'test': {
                # 'created_on': '2017-07-07T00:00:00+00:00',
                # 'revocated_on': None,
                'verify_key': 'p9KzHZjz4qjlFSTvMNNMU4QVD5CfdWEJ/Yprw/G9Xl4=\n',
            }
        }
    }


@pytest.mark.parametrize('bad_msg', [
    {'id': 42},
    {'id': None},
    {'id': 'alice', 'unknown': 'field'},
    {}
])
@pytest.mark.trio
@with_backend()
async def test_user_get_bad_msg(backend, bad_msg):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'user_get', **bad_msg})
        rep = await sock.recv()
        assert rep['status'] == 'bad_message'


@pytest.mark.trio
@with_backend()
async def test_pubkey_get_not_found(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'user_get', 'id': 'dummy'})
        rep = await sock.recv()
        assert rep == {
            'status': 'not_found',
            'reason': 'No user with id `dummy`'
        }


@pytest.mark.trio
@with_backend()
async def test_user_create_and_claim(backend):
    john_device_sign_key = SigningKey(
        b'\x07\xb5WHe\xf0\xefW\x00&]db6bA\xcf6\x1f\xca'
        b'\xa7\xeb\xf7\x954\x01\x05T\x8f,\xdc^'
    )
    john_broadcast_key = PrivateKey(
        b"\x9cuK\\\xc8\xe3\xbfDE\xd7+E\x9f\xbb&\x8d"
        b"'\xe4\xda\xedx_\xb2\xcc^\xed\x00d\x9faPd"
    )
    token = '<token>'
    async with backend.test_connect('alice@test') as sock:
        with freeze_time('2017-07-07T01:00:00'), \
                patch('parsec.backend.user._generate_invitation_token') \
                as mock_generate_invitation_token:
            mock_generate_invitation_token.return_value = token
            await sock.send({
                'cmd': 'user_create',
                'id': 'john'
            })
            rep = await sock.recv()
    assert rep == {
        'status': 'ok',
        'id': 'john',
        'token': token,
    }

    # Now claim the newly created user
    async with backend.test_connect('anonymous') as sock:
        with freeze_time('2017-07-07T01:59:00'):
            await sock.send({
                'cmd': 'user_claim',
                'id': 'john',
                'token': token,
                'broadcast_key': to_jsonb64(john_broadcast_key.public_key.encode()),
                'device_name': 'phone',
                'device_verify_key': to_jsonb64(john_device_sign_key.verify_key.encode()),
            })
            rep = await sock.recv()
    assert rep == {'status': 'ok'}

    # Finally make sure this user is accepted by the backend
    sockstream = await open_stream_to_socket_listener(backend.listeners[0])
    sock = CookedSocket(sockstream)

    ch = ClientHandshake('john@phone', john_device_sign_key)
    challenge_req = await sock.recv()
    answer_req = ch.process_challenge_req(challenge_req)
    await sock.send(answer_req)
    result_req = await sock.recv()
    ch.process_result_req(result_req)

    await sock.send({'cmd': 'ping', 'ping': 'foo'})
    rep = await sock.recv()
    assert rep == {'status': 'ok', 'pong': 'foo'}


# TODO: test out of date claim code
