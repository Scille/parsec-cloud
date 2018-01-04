import pytest
from trio.testing import trio_test
from nacl.public import PrivateKey

from parsec.utils import to_jsonb64

from tests.common import with_backend, async_patch


def _get_existing_vlob(backend):
    # Backend must have been populated before that
    id, block = list(backend.test_populate_data['vlobs'].items())[0]
    return id, block['rts'], block['wts'], block['blobs']


@pytest.mark.parametrize("id,blob", [
    (None, None),
    (None, b'Initial commit.'),
    ('foo', None),
    ('bar', b'Initial commit.')
], ids=lambda x: 'id=%s, blob=%s' % x)
@trio_test
@with_backend()
async def test_vlob_create_and_read(backend, id, blob):
    async with backend.test_connect('alice@test') as sock:
        payload = {}
        if id:
            payload['id'] = id
        if blob:
            payload['blob'] = to_jsonb64(blob)
        await sock.send({'cmd': 'vlob_create', **payload})
        rep = await sock.recv()
        assert rep['status'] == 'ok'
        assert rep['read_trust_seed']
        assert rep['write_trust_seed']
        if id:
            assert rep['id'] == id
        else:
            assert rep['id']
            id = rep['id']

    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'vlob_read', 'id': rep['id'], 'trust_seed': rep['read_trust_seed']})
        rep = await sock.recv()
        expected_content = to_jsonb64(b'' if not blob else blob)
        assert rep == {
            'status': 'ok',
            'id': id,
            'version': 1,
            'blob': expected_content
        }


@pytest.mark.parametrize('bad_msg', [
    {'blob': to_jsonb64(b'...'), 'bad_field': 'foo'},
    {'blob': 42},
    {'blob': None},
    {'id': 42, 'blob': to_jsonb64(b'...')},
    {'id': '', 'blob': to_jsonb64(b'...')},  # Id is 1 long min
    {'id': 'X' * 33, 'blob': to_jsonb64(b'...')},  # Id is 32 long max
])
@trio_test
@with_backend()
async def test_vlob_create_bad_msg(backend, bad_msg):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'vlob_create', **bad_msg})
        rep = await sock.recv()
        assert rep['status'] == 'bad_message'


@trio_test
@with_backend()
async def test_vlob_read_not_found(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'vlob_read', 'id': '1234', 'trust_seed': 'TS4242'})
        rep = await sock.recv()
        assert rep == {'status': 'not_found_error', 'reason': 'Vlob not found.'}


@trio_test
@with_backend(populated_for='alice')
async def test_vlob_read_ok(backend):
    vid, vrts, vwts, vblobs = _get_existing_vlob(backend)
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'vlob_read', 'id': vid, 'trust_seed': vrts})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'id': vid,
            'blob': to_jsonb64(vblobs[-1]),
            'version': len(vblobs)
    }


@pytest.mark.parametrize('bad_msg', [
    {'id': '1234', 'trust_seed': 'TS4242', 'bad_field': 'foo'},
    {'id': '1234'},
    {'id': '1234', 'trust_seed': 42},
    {'id': '1234', 'trust_seed': None},
    {'id': 42, 'trust_seed': 'TS4242'},
    {'id': None, 'trust_seed': 'TS4242'},
    # {'id': '1234567890', 'trust_seed': 'TS4242'},  # TODO bad?
    {}
])
@trio_test
@with_backend()
async def test_vlob_read_bad_msg(backend, bad_msg):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'vlob_read', **bad_msg})
        rep = await sock.recv()
        # Id and trust_seed are invalid anyway, but here we test another layer
        # so it's not important as long as we get our `bad_message` status
        assert rep['status'] == 'bad_message'


@trio_test
@with_backend(populated_for='alice')
async def test_read_bad_version(backend):
    vid, vrts, vwts, vblobs = _get_existing_vlob(backend)
    async with backend.test_connect('alice@test') as sock:
        await sock.send({
            'cmd': 'vlob_read',
            'id': vid,
            'trust_seed': vrts,
            'version': len(vblobs) + 1
        })
        rep = await sock.recv()
        assert rep == {'status': 'version_error', 'reason': 'Wrong blob version.'}


@trio_test
@with_backend(populated_for='alice')
async def test_vlob_update_ok(backend):
    vid, vrts, vwts, vblobs = _get_existing_vlob(backend)
    blob = to_jsonb64(b'Next version.')
    async with backend.test_connect('alice@test') as sock:
        await sock.send({
            'cmd': 'vlob_update',
            'id': vid,
            'trust_seed': vwts,
            'version': len(vblobs) + 1,
            'blob': blob
        })
        rep = await sock.recv()
        assert rep == {'status': 'ok'}


@trio_test
@with_backend()
async def test_vlob_update_not_found(backend):
    blob = to_jsonb64(b'Next version.')
    async with backend.test_connect('alice@test') as sock:
        await sock.send({
            'cmd': 'vlob_update',
            'id': '123',
            'trust_seed': 'WTS42',
            'version': 2,
            'blob': blob
        })
        rep = await sock.recv()
        assert rep == {'status': 'not_found_error', 'reason': 'Vlob not found.'}


@pytest.mark.parametrize('bad_msg', [
    {'id': '1234', 'trust_seed': 'WTS42',
     'version': '42', 'blob': to_jsonb64(b'...'), 'bad_field': 'foo'},
    {'id': '1234', 'trust_seed': 'WTS42',
     'version': '42', 'blob': None},
    {'id': '1234', 'trust_seed': 'WTS42',
     'version': '42', 'blob': 42},
    {'id': '1234', 'trust_seed': 'WTS42',
     'version': '42'},
    {'id': '1234', 'trust_seed': 'WTS42',
     'version': None, 'blob': to_jsonb64(b'...')},
    {'id': '1234', 'trust_seed': 'WTS42',
     'version': -1, 'blob': to_jsonb64(b'...')},
    {'id': '1234', 'trust_seed': None,
     'version': '42', 'blob': to_jsonb64(b'...')},
    {'id': '1234', 'trust_seed': 42,
     'version': '42', 'blob': to_jsonb64(b'...')},
    {'id': '1234',
     'version': '42', 'blob': to_jsonb64(b'...')},
    {'id': 42, 'trust_seed': 'WTS42',
     'version': '42', 'blob': to_jsonb64(b'...')},
    {'id': None, 'trust_seed': 'WTS42',
     'version': '42', 'blob': to_jsonb64(b'...')},
    {'trust_seed': 'WTS42',
     'version': '42', 'blob': to_jsonb64(b'...')},
    {}
])
@trio_test
@with_backend()
async def test_vlob_update_bad_msg(backend, bad_msg):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'vlob_update', **bad_msg})
        rep = await sock.recv()
        # Id and trust_seed are invalid anyway, but here we test another layer
        # so it's not important as long as we get our `bad_message` status
        assert rep['status'] == 'bad_message'


@trio_test
@with_backend(populated_for='alice')
async def test_update_bad_version(backend):
    vid, vrts, vwts, vblobs = _get_existing_vlob(backend)
    async with backend.test_connect('alice@test') as sock:
        await sock.send({
            'cmd': 'vlob_update',
            'id': vid,
            'trust_seed': vwts,
            'version': len(vblobs) + 2,
            'blob': to_jsonb64(b'Next version.')
        })
        rep = await sock.recv()
        assert rep == {'status': 'version_error', 'reason': 'Wrong blob version.'}


@trio_test
@with_backend(populated_for='alice')
async def test_update_bad_seed(backend):
    vid, vrts, vwts, vblobs = _get_existing_vlob(backend)
    async with backend.test_connect('alice@test') as sock:
        await sock.send({
            'cmd': 'vlob_update',
            'id': vid,
            'trust_seed': 'dummy_seed',
            'version': len(vblobs) + 1,
            'blob': to_jsonb64(b'Next version.')
        })
        rep = await sock.recv()
        assert rep == {'status': 'trust_seed_error', 'reason': 'Invalid write trust seed.'}
