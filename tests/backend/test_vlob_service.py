from base64 import encodebytes

from asynctest import mock, MagicMock
# TODO openssl or default backend?
from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives import hashes
import pytest

from parsec.backend import VlobService


class AsyncMock(MagicMock):
    async def __call__(self, *args, **kwargs):
        return super(AsyncMock, self).__call__(*args, **kwargs)


@pytest.fixture(scope='module')
def vlob_svc():
    service = VlobService()

    def asym_encrypt(data, recipient):
        return ('ENCRYPTED:' + data + ':' + recipient + ':END').encode()

    mock_crypto = MagicMock()
    mock_crypto.asym_encrypt = AsyncMock(side_effect=asym_encrypt)
    mock.patch.object(service, 'crypto_service', new=mock_crypto).start()
    return service


@pytest.fixture
def vlob(vlob_svc, event_loop, request):
    return event_loop.run_until_complete(vlob_svc.create(**request.param))


class TestVlobServiceAPI:

    # Helpers

    # Tests

    @pytest.mark.asyncio
    @pytest.mark.parametrize('vlob', [
        {'id': '123',
         'blob': 'Initial commit.',
         'read_trust_seed': 'R6MMT9QUZCUO',
         'write_trust_seed': 'WMT2WEYNPI1T'}
    ], indirect=True)
    async def test_get_sign_challenge(self, vlob_svc, vlob):

        ret = await vlob_svc.dispatch_msg({'cmd': 'get_sign_challenge', 'id': '123'})
        assert ret['status'] == 'ok'
        assert ret['challenge']

    @pytest.mark.asyncio
    @pytest.mark.parametrize('vlob', [
        {'id': '123',
         'blob': 'Initial commit.',
         'read_trust_seed': 'R6MMT9QUZCUO',
         'write_trust_seed': 'WMT2WEYNPI1T'}
    ], indirect=True)
    async def test_get_seed_challenge(self, vlob_svc, vlob):
        # Not found
        ret = await vlob_svc.dispatch_msg({'cmd': 'get_seed_challenge', 'id': '987'})
        assert ret == {'status': 'not_found', 'label': 'Cannot find vlob.'}
        # Working
        ret = await vlob_svc.dispatch_msg({'cmd': 'get_seed_challenge', 'id': '123'})
        assert ret['status'] == 'ok'
        assert ret['challenge']

    @pytest.mark.asyncio
    @pytest.mark.parametrize('vlob_payload', [
        {},
        {'blob': 'Initial commit.'},
        {'id': 'named-vlob-id0'},
        {'id': 'named-vlob-id0', 'blob': 'Initial commit.'}])
    async def test_create(self, vlob_svc, vlob_payload):
        ret = await vlob_svc.dispatch_msg({'cmd': 'create', **vlob_payload})
        assert ret['status'] == 'ok'
        if 'id' in vlob_payload:
            assert ret['id'] == vlob_payload['id']
        else:
            assert ret['id']
        assert ret['read_trust_seed']
        assert ret['write_trust_seed']

    @pytest.mark.asyncio
    @pytest.mark.parametrize('vlob', [
        {'id': '123',
         'blob': 'Initial commit.',
         'read_trust_seed': 'R6MMT9QUZCUO',
         'write_trust_seed': 'WMT2WEYNPI1T'}
    ], indirect=True)
    @pytest.mark.parametrize('hash', [True, None])
    async def test_read(self, vlob_svc, vlob, hash):

        if hash:
            challenge = 'C5D13OYI73F7'
            digest = hashes.Hash(hashes.SHA512(), backend=openssl)
            digest.update(challenge.encode() + vlob.read_trust_seed.encode())
            hash = digest.finalize()
            hash = encodebytes(hash).decode()
        else:
            challenge = '123'

        # Block not found
        vlob_svc._challenges[challenge] = vlob if hash else '987'
        ret = await vlob_svc.dispatch_msg({
            'cmd': 'read',
            'id': '987',
            'hash': hash,
            'challenge': challenge
        })
        assert ret == {'status': 'not_found', 'label': 'Cannot find vlob.'}
        # Working
        vlob_svc._challenges[challenge] = vlob if hash else vlob.id
        ret = await vlob_svc.dispatch_msg({
            'cmd': 'read',
            'id': vlob.id,
            'hash': hash,
            'challenge': challenge
        })
        assert ret == {
            'status': 'ok',
            'version': len(vlob.blob_versions),
            'blob': vlob.blob_versions[-1]
        }

    @pytest.mark.asyncio
    @pytest.mark.parametrize('vlob', [
        {'id': '123',
         'blob': 'Initial commit.',
         'read_trust_seed': 'R6MMT9QUZCUO',
         'write_trust_seed': 'WMT2WEYNPI1T'}
    ], indirect=True)
    @pytest.mark.parametrize('hash', [True, None])
    async def test_update(self, vlob_svc, vlob, hash):
        called_with = '<not called>'

        def on_event(*args):
            nonlocal called_with
            called_with = args

        if hash:
            challenge = 'C5D13OYI73F7'
            digest = hashes.Hash(hashes.SHA512(), backend=openssl)
            digest.update(challenge.encode() + vlob.write_trust_seed.encode())
            hash = digest.finalize()
            hash = encodebytes(hash).decode()
        else:
            challenge = '123'

        blob = 'Next version.'
        # Bad version (to low)
        vlob_svc._challenges[challenge] = vlob if hash else vlob.id
        ret = await vlob_svc.dispatch_msg({
            'cmd': 'update',
            'id': vlob.id,
            'version': 1,
            'blob': blob,
            'hash': hash,
            'challenge': challenge
        })
        assert ret == {'status': 'error', 'label': 'Wrong blob version.'}
        # Bad version (to high)
        vlob_svc._challenges[challenge] = vlob if hash else vlob.id
        vlob_svc.on_vlob_updated.connect(on_event, sender=vlob.id)
        ret = await vlob_svc.dispatch_msg({
            'cmd': 'update',
            'id': vlob.id,
            'version': 3,
            'blob': blob,
            'hash': hash,
            'challenge': challenge
        })
        assert ret == {'status': 'error', 'label': 'Wrong blob version.'}
        # Block not found
        vlob_svc._challenges[challenge] = vlob if hash else '987'
        ret = await vlob_svc.dispatch_msg({
            'cmd': 'update',
            'id': '987',
            'version': 2,
            'blob': blob,
            'hash': hash,
            'challenge': challenge
        })
        assert ret == {'status': 'not_found', 'label': 'Cannot find vlob.'}
        # Challenge response without request
        ret = await vlob_svc.dispatch_msg({
            'cmd': 'update',
            'id': vlob.id,
            'version': 2,
            'blob': blob,
            'hash': hash,
            'challenge': challenge
        })
        assert ret == {'status': 'auth_error', 'label': 'Authentication failure.'}
        # Wrong challenge
        vlob_svc._challenges[challenge] = vlob if hash else vlob.id
        if hash:
            ret = await vlob_svc.dispatch_msg({
                'cmd': 'update',
                'id': vlob.id,
                'version': 2,
                'blob': blob,
                'hash': encodebytes(b'wrong').decode(),
                'challenge': challenge
            })
            assert ret == {'status': 'auth_error', 'label': 'Authentication failure.'}
        else:
            ret = await vlob_svc.dispatch_msg({
                'cmd': 'update',
                'id': '987',
                'version': 2,
                'blob': blob,
                'challenge': challenge
            })
            assert ret == {'status': 'auth_error', 'label': 'Authentication failure.'}
        # Working
        vlob_svc._challenges[challenge] = vlob if hash else vlob.id
        vlob_svc.on_vlob_updated.connect(on_event, sender=vlob.id)
        ret = await vlob_svc.dispatch_msg({
            'cmd': 'update',
            'id': vlob.id,
            'version': 2,
            'blob': blob,
            'hash': hash,
            'challenge': challenge
        })
        assert ret == {'status': 'ok'}
        assert vlob.blob_versions == ['Initial commit.', 'Next version.']
        assert called_with == (vlob.id, )
